pub mod parser;
use crate::parser::parse;
use execute::command;
use execute::Execute;
use std::env::{current_dir, var};
use std::fs::OpenOptions;
use std::io::{stderr, stdin, stdout, Error, Write};
use std::process::{id, Command, Stdio};

fn main() {
    while let Ok(tokens) = tokenize_input() {
        if let Some(process) = setup_process(&tokens) {
            if let Err(e) = execute(process) {
                eprintln!("Error executing process:\n{}", e);
            }
        }
    }
}

fn execute(mut process: Command) -> Result<(), Error> {
    match process.spawn() {
        Ok(child) => {
            let pid = child.id();
            println!("Process {} is executing", pid);

            let output = child.wait_with_output()?;

            stdout().write_all(&output.stdout)?;
            stderr().write_all(&output.stderr)?;
            println!(
                "\nProcess {} finished with exit code {}",
                pid, output.status
            );

            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn tokenize_input() -> Result<Vec<String>, Error> {
    let curr = current_dir()?;
    let curr = curr.display();
    let user = var("USER").unwrap();
    print!("{}@{} {} > ", user, id(), curr);
    stdout().flush()?;
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let parsed = parse(&buffer);

    Ok(parsed)
}

fn setup_process(tokens: &[String]) -> Option<Command> {
    let commands = tokens.iter().fold(Vec::new(), |mut acc, token| {
        // If the token is a redirector or the accumulator is empty,
        // create a new vector in the accumulator
        if is_redirect(&token) || acc.is_empty() {
            acc.push(Vec::new());
        }
        // Obtain the last vector within the accumulator and push the token onto it
        acc.last_mut().unwrap().push(&token[..]);

        // Return the current accumulator and continue processing the tokens
        acc
    });
    let mut process: Option<Command> = None;
    let mut iter = commands.iter();

    while let Some(command) = iter.next() {
        process = redirect(&command[..], process).expect("Failed");
    }

    process
}

fn is_redirect(token: &str) -> bool {
    match token {
        "<" | ">" | ">>" | "<<" | "1>" | "2>" | "|" | "&>" | "1>&2" | "2>&1" => true,
        _ => false,
    }
}

fn redirect(command: &[&str], process: Option<Command>) -> Result<Option<Command>, Error> {
    let redirection = command.first().unwrap();
    match &redirection[..] {
        /*
        // ---- Append redirection ----
        ">>" => handle_append_redirect(command, process),

        // ---- stderr redirection ----
        "2>" => handle_stderr_redirect(command, process),

        // ---- stdout and stderr redirection ----
        "&>" => handle_stdout_stderr_redirect(command, process),

        */
        // ---- Stdout redirection ----
        ">" | "1>" => redirect_stdout(&command[1..], process),

        // ---- Stdin redirection ----
        "<" => redirect_stdin(command, process),
        // ---- pipe in between processes ----
        "|" => pipe(&command[1..], process),
        _ => {
            let mut proc: Command = Command::new(&command[0]);
            proc.args(&command[1..]);

            Ok(Some(proc))
        }
    }
}

fn redirect_stdout(command: &[&str], process: Option<Command>) -> Result<Option<Command>, Error> {
    match process {
        Some(mut process) => {
            let filename = command.first().unwrap_or(&"ERROR_DEFAULT.txt");
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(filename)?;

            process.stdout(Stdio::from(file));

            Ok(Some(process))
        }
        None => Ok(None),
    }
}
fn redirect_stdin(command: &[&str], process: Option<Command>) -> Result<Option<Command>, Error> {
    match process {
        Some(mut process) => {
            let filename = command.first().unwrap_or(&"ERROR_DEFAULT.txt");
            let file = OpenOptions::new().read(true).open(filename)?;

            process.stdin(Stdio::from(file));

            Ok(Some(process))
        }
        None => Ok(None),
    }
}
fn pipe(command: &[&str], process: Option<Command>) -> Result<Option<Command>, Error> {
    if let Some(mut process) = process {
        process.stdout(Stdio::piped());
        let child = process.spawn()?;
        if let Some(output) = child.stdout {
            let mut new_proc = Command::new(&command[0]);
            new_proc.args(&command[1..]).stdin(Stdio::from(output));

            return Ok(Some(new_proc));
        }
    }
    Ok(None)
}
