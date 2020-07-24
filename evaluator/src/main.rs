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

static ERROR: i32 = 99999;

struct Tokenizer {
    lexeme: String,
    line: String,
    value: i32,
}

impl Tokenizer {
    fn next_lex(&mut self) {
        /*
        println!("\nAt next_lex()");
        println!("line: '{}'", self.line);
        println!("lexeme: '{}'", self.lexeme);
        */
        self.lexeme.clear();

        let mut i: usize = 0;
        //let mut chars = self.line.chars();
        let mut current_char: char = self.line.chars().nth(0).unwrap_or_default();

        if current_char.is_whitespace() {
            while current_char.is_whitespace() {
                //println!("Skipping whitespace");
                i += 1;
                current_char = self.line.chars().nth(i).unwrap_or_default();
            }
        }

        while !current_char.is_whitespace() {
            self.lexeme.push(current_char);

            if self.current_lex_type() == "" {
                self.lexeme.pop();
                break;
            } else {
                i += 1;
                current_char = self.line.chars().nth(i).unwrap_or_default();
            }
        }
        //self.line = self.line.split_off(i + 1);
        self.line = self.line.drain(i..).collect();
        /*
        println!("After next_lex()");
        println!("line: '{}'", self.line);
        println!("lexeme: '{}'", self.lexeme);
        */

        /*
        println!("\nAt next_lex()");
        println!("line: '{}'", self.line);
        println!("lexeme: '{}'", self.lexeme);
        self.lexeme.clear();

        let mut i: usize = 0;
        //let mut chars = self.line.chars();
        let mut current_char = self.line.chars().nth(i).unwrap_or_default();
        //let mut current_char: char = self.line.chars().nth(i).unwrap_or_default();

        if current_char.is_whitespace() {
            while current_char.is_whitespace() {
                i += 1;
                current_char = self.line.chars().nth(i).unwrap_or_default();
            }
        }
        if current_char.is_numeric() {
            while current_char.is_numeric() {
                self.lexeme.push(current_char);
                i += 1;
                current_char = self.line.chars().nth(i).unwrap_or_default();
            }
        /*
        } else if current_char.is_alphabetic() {
            while current_char.is_alphabetic() {
                self.lexeme.push(current_char);
                //i += 1;
                current_char = self.line.chars().nth(i).unwrap_or_default();
            }
        */
        } else {
            while !current_char.is_alphanumeric() && !current_char.is_whitespace() {
                self.lexeme.push(current_char);
                // Invalid lex- remove the most recently appended character
                if self.current_lex_type() == "" {
                    self.lexeme.pop();
                    i -= 1;
                    break;
                // Valid lex- increment index and try again
                } else {
                    i += 1;
                    //current_char = chars.next().unwrap_or_default();
                    current_char = self.line.chars().nth(i).unwrap_or_default();
                }
            }
        }
        //self.line = chars.collect();
        self.line = self.line.split_off(i);
        println!("After next_lex()");
        println!("line: '{}'", self.line);
        println!("lexeme: '{}'", self.lexeme);
        */
    }

    fn lex_type(&self, lex: &str) -> &str {
        //println!("Entering lex_type({})", lex);
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
        } else if lex.chars().all(char::is_numeric) {
            return "INT_LITERAL";
        } else if lex.chars().all(char::is_alphabetic) && lex.len() > 1 {
            return "WORD";
        } else if lex.chars().all(char::is_alphabetic) && lex.len() == 1 {
            return "LETTER";
        } else {
            return "";
        }
    }

    fn current_lex_type(&self) -> &str {
        self.lex_type(&self.lexeme)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: 'cargo run [input file]'");
        process::exit(1);
    }

    let input_file = File::open(Path::new("src/").join(&args[1]))
        .unwrap_or_else(|err| panic!("Couldn't open that file: {}", err));

    let reader = BufReader::new(input_file);
    //let lexeme: &'static mut String = &mut String::new();
    //let line: &'static mut String = &mut String::new();
    //let mut line: String = String::new();
    //let mut lexeme: String = String::new();
    let mut value: i32;

    //let mut tokenizer = Tokenizer { lexeme, line };
    let mut tokenizer = Tokenizer {
        lexeme: String::new(),
        line: String::new(),
        value: 0,
    };
    for line_in_file in reader.lines() {
        tokenizer.line = line_in_file.unwrap();
        if !tokenizer.line.chars().all(char::is_whitespace) {
            println!("\n{}", tokenizer.line);
            tokenizer.next_lex();
            value = code(&mut tokenizer);
            if value != ERROR {
                println!("Syntax OK");
                println!("Value is {}", value);
            } else {
                if tokenizer.lexeme != ";" && tokenizer.lexeme != "" {
                    println!("Lexical Error: '{}' is not a lexeme", tokenizer.lexeme);
                } else if tokenizer.lexeme == "" && tokenizer.line.len() > 0 {
                    println!("Error: lexeme expected");
                    println!("Got '{}'", tokenizer.line);
                } else if tokenizer.lexeme == "" && !tokenizer.line.len() > 0 {
                    println!("Error: ';' expected");
                    println!("Got '{}'", tokenizer.lexeme);
                } else if tokenizer.lexeme == ";" {
                    println!("Error: ')' expected");
                    println!("Got '{}'", tokenizer.lexeme);
                }
            }
        }
    }
}

fn code(tokenizer: &mut Tokenizer) -> i32 {
    //println!("Entering code with value {}", tokenizer.value);
    let value: i32 = expression(tokenizer);

    if value == ERROR {
        return ERROR;
    } else {
        if tokenizer.lexeme == ";" {
            return value;
        } else {
            return ERROR;
        }
    }
}

fn expression(tokenizer: &mut Tokenizer) -> i32 {
    //println!("Entering expression with value {}", tokenizer.value);
    let value: i32 = term(tokenizer);

    return if value == ERROR {
        ERROR
    } else {
        tokenizer.value = value;
        ttail(tokenizer)
    };
}

fn ttail(tokenizer: &mut Tokenizer) -> i32 {
    //println!("Entering ttail with value {}", tokenizer.value);
    let value: i32;

    if tokenizer.lexeme == "+" {
        value = tokenizer.value;
        tokenizer.next_lex();
        tokenizer.value = term(tokenizer);
        if tokenizer.value == ERROR {
            return ERROR;
        } else {
            tokenizer.value = tokenizer.value + value;
            return ttail(tokenizer);
        }
    } else if tokenizer.lexeme == "-" {
        value = tokenizer.value;
        tokenizer.next_lex();
        tokenizer.value = term(tokenizer);
        if tokenizer.value == ERROR {
            return ERROR;
        } else {
            tokenizer.value = tokenizer.value - value;
            return ttail(tokenizer);
        }
    } else {
        return tokenizer.value;
    }
}

fn term(tokenizer: &mut Tokenizer) -> i32 {
    //println!("Entering term with value {}", tokenizer.value);
    let value: i32;
    value = statement(tokenizer);

    return if value == ERROR {
        ERROR
    } else {
        tokenizer.value = value;
        stail(tokenizer)
    };
}

fn stail(tokenizer: &mut Tokenizer) -> i32 {
    //println!("Entering stail with value {}", tokenizer.value);
    let value: i32;

    if tokenizer.lexeme == "*" {
        value = tokenizer.value;
        tokenizer.next_lex();
        tokenizer.value = statement(tokenizer);
        if tokenizer.value == ERROR {
            return ERROR;
        } else {
            tokenizer.value = tokenizer.value * value;
            return ttail(tokenizer);
        }
    } else if tokenizer.lexeme == "/" {
        value = tokenizer.value;
        tokenizer.next_lex();
        tokenizer.value = statement(tokenizer);
        if tokenizer.value == ERROR {
            return ERROR;
        } else {
            tokenizer.value = tokenizer.value / value;
            return ttail(tokenizer);
        }
    } else {
        return tokenizer.value;
    }
}

fn statement(tokenizer: &mut Tokenizer) -> i32 {
    //println!("Entering statement with value {}", tokenizer.value);
    let value: i32;
    value = factor(tokenizer);

    return if value == ERROR {
        ERROR
    } else {
        tokenizer.value = value;
        ftail(tokenizer)
    };
}

fn ftail(tokenizer: &mut Tokenizer) -> i32 {
    //println!("Entering ftail with value {}", tokenizer.value);
    let value: i32;

    if tokenizer.lexeme == "==" {
        value = tokenizer.value;
        tokenizer.next_lex();
        tokenizer.value = factor(tokenizer);
        if tokenizer.value == ERROR {
            return ERROR;
        } else {
            tokenizer.value = (tokenizer.value == value) as i32;
            return ttail(tokenizer);
        }
    } else if tokenizer.lexeme == "!=" {
        value = tokenizer.value;
        tokenizer.next_lex();
        tokenizer.value = factor(tokenizer);
        if tokenizer.value == ERROR {
            return ERROR;
        } else {
            tokenizer.value = (tokenizer.value != value) as i32;
            return ttail(tokenizer);
        }
    } else if tokenizer.lexeme == "<=" {
        value = tokenizer.value;
        tokenizer.next_lex();
        tokenizer.value = factor(tokenizer);
        if tokenizer.value == ERROR {
            return ERROR;
        } else {
            tokenizer.value = (tokenizer.value <= value) as i32;
            return ttail(tokenizer);
        }
    } else if tokenizer.lexeme == ">=" {
        value = tokenizer.value;
        tokenizer.next_lex();
        tokenizer.value = factor(tokenizer);
        if tokenizer.value == ERROR {
            return ERROR;
        } else {
            tokenizer.value = (tokenizer.value >= value) as i32;
            return ttail(tokenizer);
        }
    } else if tokenizer.lexeme == "<" {
        value = tokenizer.value;
        tokenizer.next_lex();
        tokenizer.value = factor(tokenizer);
        if tokenizer.value == ERROR {
            return ERROR;
        } else {
            tokenizer.value = (tokenizer.value < value) as i32;
            return ttail(tokenizer);
        }
    } else if tokenizer.lexeme == ">" {
        value = tokenizer.value;
        tokenizer.next_lex();
        tokenizer.value = factor(tokenizer);
        if tokenizer.value == ERROR {
            return ERROR;
        } else {
            tokenizer.value = (tokenizer.value > value) as i32;
            return ttail(tokenizer);
        }
    } else {
        return tokenizer.value;
    }
}

fn factor(tokenizer: &mut Tokenizer) -> i32 {
    //println!("Entering factor with value {}", tokenizer.value);
    let factor_value: i32;
    let value: i32;

    value = exponentiation(tokenizer);

    if value == ERROR {
        return ERROR;
    } else if tokenizer.lexeme == "^" {
        tokenizer.next_lex();
        factor_value = factor(tokenizer);
        if factor_value == ERROR {
            return ERROR;
        } else {
            return value.pow(factor_value as u32);
        }
    } else {
        return value;
    }
}

fn exponentiation(tokenizer: &mut Tokenizer) -> i32 {
    //println!("Entering exponentiation with value {}", tokenizer.value);
    let value: i32;
    if tokenizer.lexeme.chars().all(char::is_numeric) && tokenizer.lexeme != "" {
        return num(tokenizer);
    } else if tokenizer.lexeme == "(" {
        tokenizer.next_lex();

        value = expression(tokenizer);

        if tokenizer.lexeme == ")" {
            tokenizer.next_lex();
            return value;
        } else {
            return ERROR;
        }
    } else {
        return ERROR;
    }
}

fn num(tokenizer: &mut Tokenizer) -> i32 {
    //println!("Entering num with value {}", tokenizer.value);
    let number: i32 = tokenizer.lexeme.parse().unwrap();
    tokenizer.next_lex();
    return number;
}
