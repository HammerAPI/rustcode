//! Basic lexical classifier (tokenizer)
//!
//! Author: Daniel Hammer
//!
//! Date: 2020/5/10
//!
//! Description:
//! This program reads in a text file composed of basic arithmetic expressions
//! and classifies each lexeme, rejecting invalid lexemes.
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader /*Write*/};
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: 'cargo run [input file]'");
        process::exit(1);
    }

    let input_file = File::open(Path::new("src/").join(&args[1]))
        .unwrap_or_else(|err| panic!("Couldn't open that file: {}", err));

    let reader = BufReader::new(input_file);
    let mut lex: String;
    let mut line: String;
    let mut lex_cnt: u32 = 0;

    for line_in_file in reader.lines() {
        line = line_in_file.unwrap();

        while line.len() > 0 {
            line = line.trim().to_string();

            lex = next_lex(&mut line);

            // Display lexeme type or catch errors
            if lex_type(&lex) == "" {
                println!(
                    "Error: '{}' is not a valid lexeme",
                    line.get(..1).unwrap().to_string()
                );
                line = line.get_mut(1..).unwrap().to_string();
            } else {
                println!("Lexeme #{} is {} and is {}", lex_cnt, lex, lex_type(&lex));
                lex_cnt += 1;
            }
        }
    }
}

/// Obtains the next valid lexeme
///
/// # Description
/// Create new string to hold the next lexeme, then scan the passed-in line
/// until a valid lexeme is found. Return that lexeme and iterate the line
/// forward until the lexeme is consumed.
///
/// # Arguments
/// * 'line' - The line of text to be scanned for lexemes.
///
/// # Returns
/// * A string of a lexeme.
fn next_lex(line: &mut String) -> String {
    let mut lex: String = String::new();

    let mut i: usize = 0;
    let mut current_char: char = line.chars().nth(i).unwrap_or_default();

    if current_char.is_numeric() {
        while current_char.is_numeric() {
            lex.push(current_char);
            i += 1;
            current_char = line.chars().nth(i).unwrap_or_default();
        }
    } else if current_char.is_alphabetic() {
        while current_char.is_alphabetic() {
            lex.push(current_char);
            i += 1;
            current_char = line.chars().nth(i).unwrap_or_default();
        }
    } else {
        while !current_char.is_alphanumeric() && !current_char.is_whitespace() {
            lex.push(current_char);
            // Invalid lex- remove the most recently appended character
            if lex_type(&lex[..]) == "" {
                lex.pop();
                break;
            // Valid lex- increment index and try again
            } else {
                i += 1;
                current_char = line.chars().nth(i).unwrap_or_default();
            }
        }
    }
    *line = line.get_mut(i..).unwrap().to_string();

    lex
}

/// Gets lexeme type
///
/// # Description
/// Matches the lexeme and returns a string of its type.
///
/// # Arguments
/// * 'lex' - the lexeme to be classified.
///
/// # Returns
/// * A string literal of the lexeme's type.
fn lex_type(lex: &str) -> &str {
    if lex == "+" {
        return "ADD_OP";
    } else if lex == "-" {
        return "SUB_OP";
    } else if lex == "*" {
        return "MULT_OP";
    } else if lex == "/" {
        return "DIV_OP";
    } else if lex == "(" {
        return "LEFT_PAREN";
    } else if lex == ")" {
        return "RIGHT_PAREN";
    } else if lex == "^" {
        return "EXPON_OP";
    } else if lex == "=" {
        return "ASSIGN_OP";
    } else if lex == "<" {
        return "LESS_THAN_OP";
    } else if lex == "<=" {
        return "LESS_THAN_OR_EQUAL_OP";
    } else if lex == ">" {
        return "GREATER_THAN_OP";
    } else if lex == ">=" {
        return "GREATER_THAN_OR_EQUAL_OP";
    } else if lex == "==" {
        return "EQUALS_OP";
    } else if lex == "!" {
        return "NOT_OP";
    } else if lex == "!=" {
        return "NOT_EQUALS_OP";
    } else if lex == ";" {
        return "SEMI_COLON";
    } else if lex.chars().nth(0).unwrap_or_default().is_numeric() {
        return "INT_LITERAL";
    } else if lex.chars().nth(0).unwrap_or_default().is_alphabetic()
        && lex.chars().nth(1).unwrap_or_default().is_alphabetic()
    {
        return "WORD";
    } else if lex.chars().nth(0).unwrap_or_default().is_alphabetic()
        && !lex.chars().nth(1).unwrap_or_default().is_alphabetic()
    {
        return "LETTER";
    } else {
        return "";
    }
}
