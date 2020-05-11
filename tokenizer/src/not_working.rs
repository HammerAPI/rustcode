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
    let mut tok: &str;
    let mut statement_cnt: u8 = 0;
    let mut tok_cnt: u8 = 0;
    let mut rollover = true;

    for line in reader.lines() {
        let mut line = line.unwrap();

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

            //next_lex(&mut line, &mut tok);
            tok = next_lex(&mut line);

            if tok_type(&tok) == "" {
                println!("Error: '{}' is not a valid token", current_char);
                line = line.get_mut(1..).unwrap().to_string();
            } else {
                println!(
                    "Lexeme #{} is {} and is an {}",
                    tok_cnt,
                    tok,
                    tok_type(&tok)
                );
                tok_cnt += 1;
                if tok == ";" {
                    println!("----------------------------------");
                    rollover = true;
                }
            }
        }
    }
}

//fn next_lex<'a>(line: &'a mut str, tok: &'a mut str) {
fn next_lex<'a>(line: &'a mut str) -> &str {
    //let mut tok: String = line[..0].into();
    //let mut tok: String = line.to_string();
    let mut tok: Vec<char>;
    let mut newline: Vec<char> = line.chars().collect();
    let mut i: usize = 0;
    let mut current_char: char = newline[0];

    if current_char.is_numeric() {
        while current_char.is_numeric() {
            tok.push(current_char);
            i += 1;
            //current_char = newline.chars().nth(i).unwrap_or_default();
            current_char = newline[i];
        }
    } else if current_char.is_alphabetic() {
        while current_char.is_alphabetic() {
            tok.push(current_char);
            i += 1;
            //current_char = newline.chars().nth(i).unwrap_or_default();
            current_char = newline[i];
        }
    } else {
        while !current_char.is_alphanumeric() && !current_char.is_whitespace() {
            tok.push(current_char);
            // Invalid tok- remove the most recently appended character
            //if tok_type(tok.iter().collect().as_str()) == "" {
            if "" == "" {
                tok.pop();
                break;
            // Valid tok- increment index and try again
            } else {
                i += 1;
                //current_char = newline.chars().nth(i).unwrap_or_default();
                current_char = newline[i];
            }
        }
    }

    //tok.iter().collect().as_str()
    return tok.into_iter().collect().as_str();
    /*
    let mut tok: &str = &line[..0];
    let mut newline: &str = line[..].into();
    let mut i: usize = 0;
    let mut current_char: char = newline.chars().nth(i).unwrap_or_default();

    if current_char.is_numeric() {
        while current_char.is_numeric() {
            //tok.push(current_char);
            tok = &newline[0..i];
            i += 1;
            current_char = newline.chars().nth(i).unwrap_or_default();
        }
    } else if current_char.is_alphabetic() {
        while current_char.is_alphabetic() {
            //tok.push(current_char);
            tok = &newline[0..i];
            i += 1;
            current_char = newline.chars().nth(i).unwrap_or_default();
        }
    } else {
        while !current_char.is_alphanumeric() && !current_char.is_whitespace() {
            //tok.push(current_char);
            tok = &newline[0..i];
            // Invalid tok- remove the most recently appended character
            if tok_type(tok) == "" {
                //tok.pop();
                tok = &newline[0..i - 1];
                break;
            // Valid tok- increment index and try again
            } else {
                i += 1;
                current_char = newline.chars().nth(i).unwrap_or_default();
            }
        }
    }
    //newline = newline.get_mut(i..).unwrap();
    line = &mut newline[..];
    //line = &line[i..];

    tok
    */
}

fn tok_type(tok: &str) -> &str {
    if tok == "+" {
        return "ADD_OP";
    } else if tok == "-" {
        return "SUB_OP";
    } else if tok == "*" {
        return "MULT_OP";
    } else if tok == "/" {
        return "DIV_OP";
    } else if tok == "(" {
        return "LEFT_PAREN";
    } else if tok == ")" {
        return "RIGHT_PAREN";
    } else if tok == "^" {
        return "EXPON_OP";
    } else if tok == "=" {
        return "ASSIGN_OP";
    } else if tok == "<" {
        return "LESS_THAN_OP";
    } else if tok == "<=" {
        return "LESS_THAN_OR_EQUAL_OP";
    } else if tok == ">" {
        return "GREATER_THAN_OP";
    } else if tok == ">=" {
        return "GREATER_THAN_OR_EQUAL_OP";
    } else if tok == "==" {
        return "EQUALS_OP";
    } else if tok == "!" {
        return "NOT_OP";
    } else if tok == "!=" {
        return "NOT_EQUALS_OP";
    } else if tok == ";" {
        return "SEMI_COLON";
    } else if tok.chars().nth(0).unwrap_or_default().is_numeric() {
        return "INT_LITERAL";
    } else if tok.chars().nth(0).unwrap_or_default().is_alphabetic()
        && tok.chars().nth(1).unwrap_or_default().is_alphabetic()
    {
        return "WORD";
    } else if tok.chars().nth(0).unwrap_or_default().is_alphabetic()
        && !tok.chars().nth(1).unwrap_or_default().is_alphabetic()
    {
        return "LETTER";
    } else {
        return "";
    }
}
