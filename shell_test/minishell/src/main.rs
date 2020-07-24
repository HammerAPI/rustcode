/*
The main file for rustymsh. Contains the core logic, including everything in
msh.c, and the signal handlers.

## TODO

See how many places it's possible to remove uses of to_string() or cloning. May
involve parameterizing over lifetimes.

Rework the handler printing code to be less convoluted.

Parameterize a set of enums for error reporting so that it can be centralized?

Clean up the joblist API to make it easier to use.
*/
pub mod jobs;
pub mod util;

use crate::jobs::{JobList, JobState};
use crate::util::{
    cast_execve_args, format_string_int, parse_numerical, parse_shell_args, signal_write_err,
    signal_write_out, unix_error, ParseFailReason,
};

use libc::{c_int, sigset_t};
use nix::sys::signal::{kill, sigprocmask, SigHandler, SigSet, SigmaskHow, Signal};
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::{fork, setpgid, ForkResult, Pid};
use std::ffi::CStr;
use std::io::{self, Write};
use std::mem::MaybeUninit;

const PROMPT_STR: &str = "msh> ";

/* Since we'll need to access this variable from signal handlers, it needs to be
 * global, and unprotected by synchronization mechanisms. This makes it unsafe
 * to access. Fortunately, this also clearly illuminates all potential race
 * conditions in the shell (anytime we touch JOBS_LIST in an unsafe block without
 * blocking signals first).
 *
 * Also note that we will initialize JOBS_LIST to Some() in the start of main(),
 * so all other accesses will unwrap() instead of smart handling.
 */
static mut JOBS_LIST: Option<JobList> = None;

/* Overall architecture is a REPL: we read a command from stdin, execute it, and
then wait for the next command. Whether we let the user enter a new command
immediately or not is contingent on whether a fg/bg job was requested: if bg,
they can enter a new command immediately, if fg, they're forced to wait until
the command is finished.

Detection of job completion is tricky. Whenever a job completes, a SIGCHILD
signal is sent to us. We don't want to handle child reaping in both the main
code and signal handlers, because this could result in a race condition where
the main loop reaps a child and then the signal handler tries to reap the same
child, resulting in a race. On the other hand, we need to be able to block on
child completion (in an fg job) and respond to child termination asynchronously
(for bg jobs).

The solution is to place all child reaping logic in the sigchld handler, so that
we can respond to child reaping in one spot, and block fg jobs using sigsuspend.
sigsuspend() will block the current process until a signal arrives, so we can
block, wait until a signal arrives, see if the appropriate job was reaped, and
then either keep waiting or start a new eval-loop as appropriate */

fn main() {
    // Redirect stderr to stdout so that driver will get all output on the pipe connected to stdout
    nix::unistd::dup2(1, 2).expect("Could not redirect stderr to stdout");

    // As amazing as clap and friends are, I don't want to pull in a dependency
    // to parse three flags for a toy project. Just do it manually...
    let emit_prompt;
    let (_, emit) = parse_shell_args();
    emit_prompt = emit;

    util::install_sighandler(SigHandler::Handler(sigint_handler), Signal::SIGINT);
    util::install_sighandler(SigHandler::Handler(sigchld_handler), Signal::SIGCHLD);
    util::install_sighandler(SigHandler::Handler(sigtstp_handler), Signal::SIGTSTP);
    util::install_sighandler(SigHandler::Handler(sigquit_handler), Signal::SIGQUIT);

    // Initialize the jobs list
    unsafe {
        JOBS_LIST = Some(JobList::new());
    }

    /* The following block is the main REPL of rustymsh. The REPL reads a line
     * from stdin, executes it, and fflushes the result so that we don't have
     * to worry about buffer issues */
    let mut inp_buf = String::new();
    let stdin = io::stdin();
    loop {
        if emit_prompt {
            print!("{}", PROMPT_STR);
            std::io::stdout().flush().expect("Failed to flush stdout.");
        }

        let bytes = stdin
            .read_line(&mut inp_buf)
            .expect("Could not read from stdin");
        if bytes == 0 {
            std::process::exit(0); // Reached EOF--exit successfully
        }

        eval(&inp_buf[..]);
        inp_buf.clear();
        std::io::stdout().flush().expect("Failed to flush stdout.");
    }
}

/// Evaluate a given command string read from the REPL.
fn eval(cmdline: &str) -> () {
    let parse_result = match crate::util::parseline(cmdline) {
        Ok(args) => args,
        Err(ParseFailReason::EmptyLine) => return, // Empty line is not an error
        Err(e) => {
            eprintln!("Error in parse: {:?}", e);
            std::process::exit(1);
        }
    };
    let jobstate = parse_result.0;
    let argv = parse_result.1;
    if builtin_cmd(&argv) {
        return (); // builtin_cmd executes the command, so we should just return
    }

    /* We need to disable interrupts until we've added the child into the
    jobstructs--otherwise, we can be interrupted when the jobstruct does not
    reflect the state of the world (bad!). This is really more important for
    the parent--the child can immediately unblock the signals */
    let mut needs_block = SigSet::empty();
    let mut old_blockset = SigSet::empty();
    needs_block.add(Signal::SIGINT);
    needs_block.add(Signal::SIGTSTP);
    needs_block.add(Signal::SIGCHLD);

    sigprocmask(
        SigmaskHow::SIG_BLOCK,
        Some(&needs_block),
        Some(&mut old_blockset),
    )
    .expect("Could not block signals in sigprocmask.");

    match fork() {
        Ok(ForkResult::Parent { child: pid, .. }) => {
            let jid;

            unsafe {
                jid = JOBS_LIST
                    .as_mut()
                    .unwrap()
                    .addjob(pid, jobstate, cmdline)
                    .unwrap();
            }

            if jobstate == JobState::FG {
                waitfg(pid);
            } else {
                println!("[{}] ({}) {}", jid, pid, cmdline.trim());
            }

            sigprocmask(SigmaskHow::SIG_SETMASK, Some(&old_blockset), None)
                .expect("Could not unblock signals from parent process");
        }
        Ok(ForkResult::Child) => {
            // Detach the child into its own process group so that it doesn't get
            // signals that are only meant for the shell/foreground process
            setpgid(Pid::from_raw(0), Pid::from_raw(0)).expect("Could not setpgid");
            sigprocmask(SigmaskHow::SIG_SETMASK, Some(&old_blockset), None)
                .expect("Could not unblock signals from child process");

            // Type-level faffery to convert things into &[CStr], which is needed for execve
            let (argv, env) = cast_execve_args(argv);
            let argv: Vec<&CStr> = argv.iter().map(|x| &x[..]).collect();
            let env: Vec<&CStr> = env.iter().map(|x| &x[..]).collect();

            if let Err(_) = nix::unistd::execve(&argv[0], &argv, &env) {
                println!("{}: Command not found", argv[0].to_str().unwrap());
                std::process::exit(-1);
            }
        }
        Err(_) => unix_error("Call to fork() failed."),
    }
}

/// Checks to see if argv corresponds to built-in, and executes it if so.
/// Returns true if the command was a built-in.
fn builtin_cmd(argv: &Vec<String>) -> bool {
    match &(argv[0])[..] {
        "jobs" => {
            unsafe {
                let job_str = JOBS_LIST.as_ref().unwrap().listjobs();
                print!("{}", job_str.unwrap());
            }
            true
        }
        "quit" => {
            std::process::exit(0);
        }
        "fg" | "bg" => {
            do_bgfg(argv);
            true
        }
        _ => false,
    }
}

/// Handles moving jobs into the foreground and running jobs in the background.
fn do_bgfg(argv: &Vec<String>) -> () {
    /* The logic of this function is a little nasty: because we need to do fg/bg
     * and lookups on pid/jid, there's a lot of potentially redundant code. My
     * solution is to use these flags to gather data in one phase, then execute
     * all the actions at once */
    let usepid;
    let tofg;
    let id;

    if argv.len() == 1 {
        println!("{} command requires PID or %jobid argument", argv[0]);
        return ();
    }

    if argv[0] == "fg" {
        tofg = true;
    } else if argv[0] == "bg" {
        tofg = false;
    } else {
        // This function should only be entered if argv[0] is fg or bg, but I don't
        // want to rely on someone never changing the caller code...
        unreachable!();
    }

    let arg = &argv[1];
    let arg = arg.as_bytes();
    if arg[0] == b'%' {
        usepid = false;
        id = match parse_numerical(&arg[1..]) {
            Ok(x) => x,
            Err(_) => {
                println!("{}: argument must be a PID or %jobid", argv[0]);
                return ();
            }
        }
    } else {
        usepid = true;
        id = match parse_numerical(&arg[..]) {
            Ok(x) => x,
            Err(_) => {
                println!("{}: argument must be a PID or %jobid", argv[0]);
                return ();
            }
        }
    }

    let maybejob = unsafe {
        if usepid {
            JOBS_LIST.as_mut().unwrap().getjob_pid(Pid::from_raw(id))
        } else {
            JOBS_LIST.as_mut().unwrap().getjob_jid(id)
        }
    };

    let job = match maybejob {
        Some(j) => j,
        None => {
            if usepid {
                println!("({}): No such process", id);
            } else {
                println!("%{}: No such job", id);
            }
            return ();
        }
    };

    let jid = job.jid();
    let pid = job.pid();
    let cmdline = job.cmdline().to_string();

    let group_id = Pid::from_raw(-pid.as_raw());
    kill(group_id, Signal::SIGCONT).expect("Could not send SIGCONT");

    if tofg {
        job.set_state(JobState::FG);
        waitfg(pid);
    } else {
        job.set_state(JobState::BG);
        print!("[{}] ({}) {}", jid, pid, cmdline);
    }
}

/// Wait for a foreground process given by pid. Assumes signals that could affect
/// child process state
fn waitfg(pid: Pid) -> () {
    // Signals are blocked in this function by the calling scope eval(), which
    // blocks INT, TSTP, and CHLD. Signals will be temporarily unblocked by the
    // sigsuspend call while in this scope.

    // Unfortunately, nix does not provide a nice wrapper around sigsuspend, so
    // we're going to to have to do this using unsafe libc crate calls (ew)
    // This must be carefully done, or we're going to footgun ourself. See
    // https://gankra.github.io/blah/initialize-me-maybe/ for more details
    let emptysigset = unsafe {
        let mut emptyset = MaybeUninit::<sigset_t>::uninit();
        let err_code = libc::sigemptyset(emptyset.as_mut_ptr());
        if err_code == -1 {
            unix_error("Could not create an empty sigset_t. Something's broken.");
        }
        emptyset.assume_init()
    };

    // Check the jobs list for our foreground job. This is safe ONLY because eval()
    // has blocked signals for us--otherwise we'd be at risk of race conditions
    let mut fgpid = unsafe { JOBS_LIST.as_mut().unwrap().fgpid() };
    while fgpid.is_some() && fgpid.unwrap() == pid {
        unsafe {
            /* sigsuspend() blocks the program until a signal is recieved. Hopefully,
            at some point, we'll get a SIGCHLD, the sigchld_handler will remove
            foreground job from the joblist, and we can exit this loop! */
            libc::sigsuspend(&emptysigset as *const sigset_t);
            fgpid = JOBS_LIST.as_mut().unwrap().fgpid();
        }
    }

    ()
}

extern "C" fn sigquit_handler(_: c_int) -> () {
    const QUIT_MSG: &[u8] = b"Terminating after receipt of SIGQUIT signal\n";
    signal_write_err(QUIT_MSG);
    std::process::exit(1);
}

extern "C" fn sigint_handler(_: c_int) -> () {
    let fgpid = unsafe { JOBS_LIST.as_ref().unwrap().fgpid() };

    if let Some(pid) = fgpid {
        let group_id = Pid::from_raw(-pid.as_raw());
        kill(group_id, Signal::SIGINT).expect("Could not send SIGINT");
    }
}

extern "C" fn sigtstp_handler(_: c_int) -> () {
    let fgpid = unsafe { JOBS_LIST.as_ref().unwrap().fgpid() };

    if let Some(pid) = fgpid {
        let group_id = Pid::from_raw(-pid.as_raw());
        kill(group_id, Signal::SIGTSTP).expect("Could not send SIGTSTP");
    }
}

extern "C" fn sigchld_handler(_: c_int) -> () {
    // Somebody's dead! Let's gather all their bodies! Note that sigchld may fire
    // multiple times, but we will only recieve the last one, so it's not
    // sufficient to just reap one child here--we have to keep collecting them
    // until no unreaped children are left.
    let mut flags = WaitPidFlag::empty();
    flags.insert(WaitPidFlag::WNOHANG);
    flags.insert(WaitPidFlag::WUNTRACED);
    flags.insert(WaitPidFlag::WCONTINUED);

    // pid = None converts into waitpid(-1), i.e. wait for all children.
    // Figure that one out without reading the nix source code, I dare you.
    while let Ok(status) = waitpid(None, Some(flags)) {
        match status {
            WaitStatus::Exited(pid, _) => unsafe {
                JOBS_LIST
                    .as_mut()
                    .unwrap()
                    .deletejob(pid)
                    .expect("Nonexistent job exited! Bug or race condition?")
            },
            WaitStatus::Signaled(pid, signal, _) => {
                let job = unsafe {
                    JOBS_LIST
                        .as_mut()
                        .unwrap()
                        .getjob_pid(pid)
                        .expect("Nonexistent job killed by signal! Bug or race condition?")
                };
                let jid = job.jid();
                let pid = job.pid();
                unsafe {
                    JOBS_LIST.as_mut().unwrap().deletejob(pid).unwrap();
                }

                let msgbuf = b"Job [%d] (%d) terminated by signal %d\n";
                let msg = format_string_int(msgbuf, jid);
                let msg = format_string_int(&msg[..], pid.as_raw());
                let msg = format_string_int(&msg[..], signal as i32);
                signal_write_out(&msg[..]);
            }
            WaitStatus::Stopped(pid, signal) => {
                let job = unsafe {
                    JOBS_LIST
                        .as_mut()
                        .unwrap()
                        .getjob_pid(pid)
                        .expect("Nonexistent job killed by signal! Bug or race condition?")
                };
                job.set_state(JobState::Stop);
                let msgbuf = b"Job [%d] (%d) stopped by signal %d\n";
                let msg = format_string_int(msgbuf, job.jid());
                let msg = format_string_int(&msg[..], job.pid().as_raw());
                let msg = format_string_int(&msg[..], signal as i32);
                signal_write_out(&msg[..]);
            }
            WaitStatus::Continued(pid) => {
                let job = unsafe { JOBS_LIST.as_mut().unwrap().getjob_pid(pid) };
                if job.is_none() {
                    panic!("Nonexistent job killed by signal! Bug or race condition?")
                }
                /* Skeleton handler code: this may need to be reworked or removed
                 * later. It's here for now because mshref doesn't handle SIGCONT
                 * from outside the shell, and I want to fix that at some point.
                 * The correct behavior should be to pretend that the user requested
                 * CONT of the job in the background via bg, but without printing
                 * anything. It doesn't make sense to put the job in the fg (what
                 * if another job is there?), and leaving it in a stopped state
                 * means that jobs shows incorrect information.
                 */
            }
            WaitStatus::StillAlive => {
                // This was a triumph
                let _note = "huge success!";
                break;
            }
            _ => unreachable!(),
        }
    }
}
