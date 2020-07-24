extern crate pest;

use pest::Parser;
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CLIParser;

pub fn parse(input: &str) -> Vec<String> {
    // We are parsing the input string via the `line` rule in grammar.pest
    let parsed = CLIParser::parse(Rule::line, input)
        .expect("Failed to parse")
        .next()
        .unwrap();

    let mut tokens: Vec<String> = Vec::new();
    // Gets `line`
    for line in parsed.into_inner() {
        // `line` is either `quoted` or `commands`
        match line.as_rule() {
            Rule::quoted => {
                // Trim the quotation marks off of the string
                let token = line.as_str();
                let token = String::from(&token[1..token.len() - 1]);
                tokens.push(token);
            }
            Rule::commands => {
                // Get each `command` present
                for command in line.into_inner() {
                    tokens.push(String::from(command.as_str()));
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    tokens
}
