use nix::errno::{errno, from_i32};
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal};
use std::ffi::CString;
use std::process::exit;
use std::vec::Vec;

use crate::jobs::JobState;

pub const MAX_LINE_SIZE: usize = 1024;
pub const MAX_NUM_ARGS: usize = 128;
pub const MAX_NUM_JOBS: usize = 16;
pub const MAX_JOBID: i32 = 1 << 16;

pub fn unix_error(msg: &str) {
    eprintln!("{}: {}", msg, from_i32(errno()));
    exit(1);
}

// If a signal arrives during a syscall, we want to let the system automatically
// restart the syscall instead of automatically failing. SA_RESTART will do this
// for us, see `man 7 signal` for a discussion of this behavior.
pub fn install_sighandler(handler: SigHandler, sig: Signal) -> () {
    let action = SigAction::new(handler, SaFlags::SA_RESTART, SigSet::empty());
    unsafe {
        sigaction(sig, &action).expect("Could not set signal handler with sigaction");
    }
}

fn usage() -> () {
    println!("Usage: shell [-hvp]\n");
    println!("   -h   print this message\n");
    println!("   -v   print additional diagnostic information\n");
    println!("   -p   do not emit a command prompt\n");
}

/// Parse shell arguments. The only flags that are passed to the shell right now
/// alter its global behavior. Reflect the requested behavior changes by passing
/// a tuple of (verbose, emit_prompt) booleans back to the caller.
pub fn parse_shell_args() -> (bool, bool) {
    let mut verbose = false;
    let mut emit_prompt = true;
    let args: Vec<String> = std::env::args().collect();
    for arg in args[1..].iter().map(|x| &x[..]) {
        match arg {
            "-h" => {
                usage();
                std::process::exit(0) // User requested help: exit success
            }
            "-v" => {
                verbose = true;
            }
            "-p" => {
                emit_prompt = false;
            }
            _ => {
                usage();
                std::process::exit(1) // Bad input: exit with error
            }
        }
    }
    (verbose, emit_prompt)
}

/// Help manage the args for execve syscall
///
/// The execve syscall needs some odd types, and formats that aren't necessarily
/// provided natively by Rust. This helper function creates the correct forms
/// for execve() so that the return value just has to be casted and passed in.
pub fn cast_execve_args(args: Vec<String>) -> (Vec<CString>, Vec<CString>) {
    let argv: Vec<CString> = args.into_iter().map(|x| CString::new(x).unwrap()).collect();
    let env: Vec<CString> = std::env::vars()
        .map(|(x, y)| {
            // Manually place vars in the expected form
            let kvps = format!("{}={}", x, y);
            CString::new(kvps).unwrap()
        })
        .collect();

    return (argv, env);
}

const PANIC_STR: &str = "Handler write failed!";

/// Safely write a message to stdout from a signal handler. See the format_string_int
/// and format_string_str functions for ways to generate this buffer without allocation
pub extern "C" fn signal_write_out(msg: &[u8]) {
    const STDERR: std::os::unix::io::RawFd = 1;
    let nwritten = nix::unistd::write(STDERR, msg).unwrap();
    if nwritten != msg.len() {
        panic!(PANIC_STR);
    }
}

/// Safely write a message to stderr from a signal handler. See the format_string_int
/// and format_string_str functions for ways to generate this buffer without allocation
pub extern "C" fn signal_write_err(msg: &[u8]) {
    const STDERR: std::os::unix::io::RawFd = 2;
    let nwritten = nix::unistd::write(STDERR, msg).unwrap();
    if nwritten != msg.len() {
        panic!(PANIC_STR);
    }
}

pub const MSGBUF_LEN: usize = 256;
/// A custom string formatting function to allow simple string formatting without
/// needing heap allocations. Takes in a buffer of MAXLENGTH and replaces all instances
/// of %d with integer representations.
pub fn format_string_int(buf: &[u8], x: i32) -> [u8; MSGBUF_LEN] {
    let mut numbuf = [0u8; 16];
    let mut x = x;

    // Compute the base-10 representation of the number
    let mut j = 15;
    while x >= 10 {
        numbuf[j] = (x % 10) as u8 + '0' as u8;
        x = x / 10;
        j -= 1;
        if j == 0 {
            break;
        }
    }
    numbuf[j] = x as u8 + '0' as u8;

    // Compute the correct representation with no null bytes
    let mut offset = 0;
    for i in 0..16 {
        if numbuf[i] != 0 {
            offset = i;
            break;
        }
    }
    let mut localcopy = [0u8; MSGBUF_LEN];

    for i in 0..std::cmp::min(buf.len(), MSGBUF_LEN) {
        localcopy[i] = buf[i];
    }

    // Replace the first %d with %s
    for i in 0..buf.len() {
        if localcopy[i] == b'%' && localcopy[i + 1] == b'd' {
            localcopy[i + 1] = b's';
            break;
        }
    }
    format_string_str(&localcopy[..], &numbuf[offset..])
}

/// A custom string formatting function to allow simple string formatting without
/// needing heap allocations. Takes in a buffer of MAXLENGTH and replaces all instances
/// of %s with string representations.
pub fn format_string_str(buf: &[u8], x: &[u8]) -> [u8; MSGBUF_LEN] {
    let mut localcopy = [0; MSGBUF_LEN];
    let mut already_done = false;

    let mut from = 0;
    let mut to = 0;

    loop {
        // Write number into buffer using offset
        if from + 1 < buf.len() && buf[from] == b'%' && buf[from + 1] == b's' && !already_done {
            for j in 0..x.len() {
                localcopy[to] = x[j];
                to += 1;
            }
            from += 2;
            already_done = true;
        }
        if to >= MSGBUF_LEN || from >= buf.len() {
            break;
        }
        localcopy[to] = buf[from];
        to += 1;
        from += 1;
    }
    localcopy
}
/// The result of an attempt to parse a command line.
#[derive(Clone, Debug, PartialEq)]
pub struct ParseResult(pub JobState, pub Vec<String>);

#[derive(Clone, Debug, PartialEq)]
pub enum ParseFailReason {
    EmptyArg,
    EmptyLine,
    Unmatched(char),
    Invalid(char),
    Other(String),
}
/// Eats a token and returns the token along with the remainder of buf
/// Tokens are split on first whitespace or next matching single quote.
fn chomp_tok(buf: &str) -> Result<(&str, &str), ParseFailReason> {
    let nows = buf.trim();
    let delim_char = match nows.chars().next() {
        Some('\'') => '\'',
        Some(_) => ' ',
        None => return Err(ParseFailReason::EmptyArg),
    };

    let newbuf = if delim_char == '\'' {
        &nows[1..] // Convenience def! Lets us avoid the initial quote
    } else {
        nows
    };
    let delim_index = newbuf.find(delim_char);

    // Somewhat ugly nested matches to deal with four different cases
    match delim_char {
        '\'' => match delim_index {
            Some(i) => {
                let (tok, rest) = newbuf.split_at(i);
                Ok((tok, &rest[1..]))
            }
            None => Err(ParseFailReason::Unmatched('\'')),
        },
        ' ' => match delim_index {
            Some(i) => Ok(newbuf.split_at(i)),
            None => Ok((newbuf, "")),
        },
        _ => unreachable!(),
    }
}

/// Parse the command line and build an argv array
pub fn parseline(buf: &str) -> Result<ParseResult, ParseFailReason> {
    let mut argv: Vec<String> = Vec::new();
    let mut remainder = buf;

    loop {
        match chomp_tok(remainder) {
            Ok((tok, rem)) => {
                argv.push(tok.to_string());
                remainder = rem;
            }
            Err(ParseFailReason::EmptyArg) => break,
            Err(x) => return Err(x),
        }
    }

    if argv.is_empty() {
        return Err(ParseFailReason::EmptyLine);
    }

    // argv cannot be empty here, so last is always a Some
    let lastarg = argv.last().unwrap();
    if lastarg == "&" {
        argv.pop(); // Clear out the &--it's not an arg to the prog, it's to the shell
        Ok(ParseResult(JobState::BG, argv))
    } else {
        Ok(ParseResult(JobState::FG, argv))
    }
}

// Parse a &[u8] into an i32.
pub fn parse_numerical(inp: &[u8]) -> Result<i32, ParseFailReason> {
    let mut res = 0i32;
    for i in inp.iter() {
        let j = (*i as char).to_digit(10);
        let val = match j {
            Some(z) => z,
            None => return Err(ParseFailReason::Invalid(*i as char)),
        };
        res *= 10;
        res += val as i32;
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    // A cursed macro to create a vec of strings from [str] literals
    macro_rules! string_vec {
        // match a list of expressions separated by comma:
        ($($str:expr),*) => ({
            // create a Vec with this list of expressions,
            // calling String::from on each:
            vec![$(String::from($str),)*] as Vec<String>
        });
    }

    #[test]
    fn test_simple_parse() {
        let input = "run this program";
        let output = parseline(input).unwrap();
        assert_eq!(
            output,
            ParseResult(JobState::FG, string_vec!["run", "this", "program"])
        )
    }

    #[test]
    fn test_ws_parse() {
        let input = "  run        this program           ";
        let output = parseline(input).unwrap();
        assert_eq!(
            output,
            ParseResult(JobState::FG, string_vec!["run", "this", "program"])
        )
    }

    #[test]
    fn test_quoted_parse() {
        let input = "run 'this program'";
        let output = parseline(input).unwrap();
        assert_eq!(
            output,
            ParseResult(JobState::FG, string_vec!["run", "this program"])
        )
    }

    #[test]
    fn test_fail_parse() {
        let input = "run 'this program";
        let output = parseline(input);
        assert_eq!(output, Err(ParseFailReason::Unmatched('\'')));
    }

    #[test]
    fn test_simple_bg() {
        let input = "run this program &";
        let output = parseline(input).unwrap();
        assert_eq!(
            output,
            ParseResult(JobState::BG, string_vec!["run", "this", "program"])
        )
    }

    #[test]
    fn test_format_int() {
        let test_string = b"H %d";
        let x: i32 = 618206;
        let ret = format_string_int(test_string, x);
        let mut correct = [0u8; MSGBUF_LEN];
        correct[0] = b'H';
        correct[1] = b' ';
        correct[2] = '6' as u8;
        correct[3] = '1' as u8;
        correct[4] = '8' as u8;
        correct[5] = '2' as u8;
        correct[6] = '0' as u8;
        correct[7] = '6' as u8;
        println!("{:?}", &ret[0..32]);
        println!("{:?}", &correct[0..9]);
        for i in 0..MSGBUF_LEN {
            assert_eq!(correct[i], ret[i]);
        }
    }

    #[test]
    fn test_format_ids() {
        let msgbuf = b"Job [%d] (%d) stopped by %s\n";
        let jid = 2;
        let pid = 1058514;
        let msg = format_string_int(msgbuf, jid);
        let msg = format_string_int(&msg[..], pid);
        let msg = format_string_str(&msg[..], "SIGTSTP".as_bytes());

        let correct = "Job [2] (1058514) stopped by SIGTSTP\n".as_bytes();
        println!("{:?}", &msg[0..32]);
        println!("{:?}", &correct[0..32]);
        for i in 0..correct.len() {
            assert_eq!(msg[i], correct[i]);
        }
    }
}
