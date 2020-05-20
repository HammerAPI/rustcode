use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: 'cargo run [input file] [first word] [second word]'");
        process::exit(1);
    }

    let dictionary = build_dictionary(&args[1]);
    let first_word = &args[2];
    let second_word = &args[3];

    check_words(&first_word, &second_word, &dictionary);

    let word_list = build_word_list(first_word.len(), dictionary);

    println!(
        "Attempting to find a ladder between {} and {}",
        first_word, second_word
    );

    let ladder = build_ladder(&first_word, &second_word, &word_list);

    if ladder.len() > 0 {
        println!("A ladder was found!");
        for word in ladder {
            println!("{}", word);
        }
    } else {
        println!("No ladder was found.");
    }
}

fn check_words(first_word: &str, second_word: &str, dictionary: &Vec<String>) {
    if first_word.len() != second_word.len() {
        println!("Words must be the same length");
        process::exit(1);
    }
    if !dictionary.contains(&first_word.to_string()) {
        println!("'{}' is not in the dictionary.", first_word);
        process::exit(1);
    }
    if !dictionary.contains(&second_word.to_string()) {
        println!("'{}' is not in the dictionary.", second_word);
        process::exit(1);
    }
}

fn build_dictionary(filename: &str) -> Vec<String> {
    let input_file = File::open(Path::new("src/").join(filename))
        .unwrap_or_else(|err| panic!("Couldn't open that file: {}", err));

    let reader = BufReader::new(input_file);

    let mut dictionary: Vec<String> = Vec::new();
    for line in reader.lines() {
        dictionary.push(line.unwrap());
    }

    dictionary
}

fn build_word_list(len: usize, dictionary: Vec<String>) -> Vec<String> {
    let mut word_list: Vec<String> = Vec::new();

    for word in dictionary {
        if word.len() == len {
            word_list.push(word);
        }
    }

    word_list
}

fn get_words_one_letter_away(base_word: &str, word_list: &Vec<String>) -> Vec<String> {
    let mut one_letter_away: Vec<String> = Vec::new();
    let mut letters_different: u8;
    let mut chars: std::str::Chars;

    for word in word_list {
        letters_different = 0;
        chars = base_word.chars();
        for c in word.chars() {
            if c != chars.next().unwrap() {
                letters_different += 1;
            }
        }
        if letters_different == 1 {
            one_letter_away.push(word.clone());
        }
    }

    one_letter_away
}

fn build_ladder(first_word: &str, second_word: &str, word_list: &Vec<String>) -> Vec<String> {
    let mut list_of_ladders: Vec<Vec<String>> = Vec::new();
    let mut used_words: Vec<String> = Vec::new();
    let mut ladder: Vec<String> = Vec::new();
    let mut one_away: Vec<String>;

    ladder.push(second_word.to_string());
    used_words.push(second_word.to_string());
    list_of_ladders.push(ladder);

    while list_of_ladders.len() > 0 {
        ladder = list_of_ladders.remove(0);
        one_away = get_words_one_letter_away(&ladder[ladder.len() - 1], word_list);

        for word in one_away {
            if !ladder.contains(&word) && !used_words.contains(&word) {
                ladder.push(word.clone());

                if ladder.last().unwrap() == first_word {
                    return ladder;
                }

                used_words.extend_from_slice(&ladder[..]);

                list_of_ladders.push(ladder.clone());
                if let Some(pos) = ladder.iter().position(|x| *x == word) {
                    ladder.remove(pos);
                }
            }
        }
    }
    return Vec::new();
}
