use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader /*Write*/};
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: 'cargo run [input file] [output file]'");
        process::exit(1);
    }

    let input_file = File::open(Path::new("src/").join(&args[1]))
        .unwrap_or_else(|err| panic!("Couldn't open that file: {}", err));

    /*
    let mut output_file = File::create(Path::new("src/").join(&args[2]))
        .unwrap_or_else(|err| panic!("Couldn't create that file: {}", err));
    */

    let reader = BufReader::new(input_file);
    let mut tok: String = String::from("");
    let mut line: String;
    let mut statement_cnt: u8 = 0;
    let mut tok_cnt: u8 = 0;
    let mut rollover = true;

    for line_in_file in reader.lines() {
        let line_in_file = line_in_file.unwrap();
        line = line_in_file;

        if rollover == true {
            tok_cnt = 0;
            statement_cnt += 1;
            println!("Statement #{}", statement_cnt);
            rollover = false;
        }

        while line.len() != 0 {
            let mut current_char: char = line.chars().nth(0).unwrap_or_default();

            if current_char.is_whitespace() {
                while current_char.is_whitespace() {
                    line = line.get_mut(1..).unwrap().to_string();
                    current_char = line.chars().nth(0).unwrap_or_default();
                }
                continue;
            }

            next_lex(&mut line, &mut tok);

            println!(
                "Lexeme #{} is {} and is an {}",
                tok_cnt,
                tok,
                get_tok_type(&tok)
            );
            tok_cnt += 1;
            if tok == ";" {
                println!("----------------------------------");
                rollover = true;
            }
        }
    }
}

fn next_lex<'a>(line: &'a mut String, tok: &'a mut String) {
    tok.clear();

    let mut i: usize = 0;
    let mut current_char: char = line.chars().nth(i).unwrap_or_default();

    if current_char.is_numeric() {
        while current_char.is_numeric() {
            tok.push(current_char);
            i += 1;
            current_char = line.chars().nth(i).unwrap_or_default();
        }
    } else if current_char.is_alphabetic() {
        while current_char.is_alphabetic() {
            tok.push(current_char);
            i += 1;
            current_char = line.chars().nth(i).unwrap_or_default();
        }
    } else {
        while !current_char.is_alphanumeric() && !current_char.is_whitespace() {
            tok.push(current_char);
            // Invalid tok- remove the most recently appended character
            if get_tok_type(tok) == "" {
                tok.pop();
                break;
            // Valid tok- increment index and try again
            } else {
                i += 1;
                current_char = line.chars().nth(i).unwrap_or_default();
            }
        }
    }
    *line = line.get_mut(i..).unwrap().to_string();
}

fn get_tok_type(tok: &String) -> String {
    if tok == "+" {
        return String::from("ADD_OP");
    } else if tok == "-" {
        return String::from("SUB_OP");
    } else if tok == "*" {
        return String::from("MULT_OP");
    } else if tok == "/" {
        return String::from("DIV_OP");
    } else if tok == "(" {
        return String::from("LEFT_PAREN");
    } else if tok == ")" {
        return String::from("RIGHT_PAREN");
    } else if tok == "^" {
        return String::from("EXPON_OP");
    } else if tok == "=" {
        return String::from("ASSIGN_OP");
    } else if tok == "<" {
        return String::from("LESS_THAN_OP");
    } else if tok == "<=" {
        return String::from("LESS_THAN_OR_EQUAL_OP");
    } else if tok == ">" {
        return String::from("GREATER_THAN_OP");
    } else if tok == ">=" {
        return String::from("GREATER_THAN_OR_EQUAL_OP");
    } else if tok == "==" {
        return String::from("EQUALS_OP");
    } else if tok == "!" {
        return String::from("NOT_OP");
    } else if tok == "!=" {
        return String::from("NOT_EQUALS_OP");
    } else if tok == ";" {
        return String::from("SEMI_COLON");
    } else if tok.chars().nth(0).unwrap_or_default().is_numeric() {
        return String::from("INT_LITERAL");
    } else if tok.chars().nth(0).unwrap_or_default().is_alphabetic()
        && tok.chars().nth(1).unwrap_or_default().is_alphabetic()
    {
        return String::from("WORD");
    } else if tok.chars().nth(0).unwrap_or_default().is_alphabetic()
        && !tok.chars().nth(1).unwrap_or_default().is_alphabetic()
    {
        return String::from("LETTER");
    } else {
        return String::new();
    }
}
