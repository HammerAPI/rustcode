use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: 'cargo run [input file] [first word] [second word]'");
        process::exit(1);
    }

    let dictionary = build_dictionary(&args[1]);
    let first_word = String::from(&args[2]);
    let second_word = String::from(&args[3]);

    check_words(&first_word, &second_word, &dictionary);

    let word_list = build_word_list(first_word.len(), dictionary);

    println!(
        "Attempting to find a ladder between {} and {}",
        first_word, second_word
    );

    let ladder = get_ladder(first_word, second_word, word_list);
    //let ladder = build_ladder(first_word, second_word, word_list);

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

/*
fn build_ladder(first: String, second: String, word_list: Vec<String>) -> Vec<String> {
    // Start the ladder with a word
    let mut ladder = vec![second.clone()];

    // Start the list with the first ladder
    let mut list_of_ladders = vec![ladder.clone()];

    // Start an empty used_words list
    let mut used_words = vec![];

    // Loop as long as a ladder is not found
    while list_of_ladders.len() > 0 {
        // Obtain the first ladder
        ladder = list_of_ladders.remove(0);
        //println!("{:?}", ladder);
        let one_away = get_words_one_letter_away(ladder.last().unwrap(), &word_list);

        for word in one_away {
            if !ladder.contains(&word) && !used_words.contains(&word) {
                ladder.push(word.clone());
                if ladder.contains(&first) {
                    println!("LADDER FOUND");
                    return ladder;
                } else {
                    list_of_ladders.push(ladder.clone());
                    ladder.pop().unwrap_or_default();
                    used_words.push(word);
                }
            }
        }
    }
    ladder
}
*/

/*
fn get_ladder(first_word: String, second_word: String, word_list: Vec<String>) -> Vec<String> {
    // Start the list with the first ladder
    let ladder_list = Arc::new(Mutex::new(vec![vec![second_word]]));
    let word_list = Arc::new(word_list);
    let found = Arc::new(Mutex::new(false));
    let first = Arc::new(first_word);

    let (tx, rx): (mpsc::Sender<Vec<String>>, mpsc::Receiver<Vec<String>>) = mpsc::channel();

    while !ladder_list.lock().unwrap().is_empty() {
        // Individual sender for each ladder pulled from the ladder_list
        let ladder_sender = tx.clone();
        //let cloned_list = ladder_list.clone();
        let cloned_list = Arc::clone(&ladder_list);
        // Get the bottom ladder
        let mut ladder = ladder_list.lock().unwrap().remove(0);

        // Clone the first word (change this to Arc)
        let first = first.clone();

        // Clone the word list (change this to Arc)
        //let word_list = word_list.clone();
        let cloned_word_list = Arc::clone(&word_list);

        let f = Arc::clone(&found);
        //thread::spawn(move || {
        let child = thread::spawn(move || {
            //println!("Entering new thread");

            let one_away: Vec<String> =
                get_words_one_letter_away(ladder.last().unwrap(), &cloned_word_list);
            //println!("{:?}", one_away);

            // For every one-letter-away word
            for word in one_away {
                //println!("{}", word);
                // If the ladder does not already contain it, push it
                if !ladder.contains(&word) {
                    ladder.push(word);

                    // If the ladder is complete, say so
                    if ladder.contains(&first) {
                        println!("LADDER FOUND\n{:?}", ladder);
                        // Clone the ladder and send it back
                        ladder_sender.send(ladder.clone()).unwrap();
                        (*f.lock().unwrap()) = true;
                        return;
                    } else {
                        // Push the new ladder to the ladder_list
                        cloned_list.lock().unwrap().push(ladder.clone());
                        // Remove the word just added so the ladder can be reused
                        ladder.pop().unwrap_or_default();
                    }
                }
            }
        });
        child.join().unwrap();
        if *(found.lock().unwrap()) {
            println!("LADDER FOUND");
            return rx.recv().unwrap();
            //break;
        }
    }

    /*
    for received in rx {
        if received.contains(&first) && received.contains(&second) {
            return received;
        }
    }
    */
    Vec::new()
}
*/

/*
fn get_ladder(first_word: String, second_word: String, word_list: Vec<String>) -> Vec<String> {
    // Start the list with the first ladder
    let ladder_list = Arc::new(Mutex::new(vec![vec![second_word]]));
    let word_list = Arc::new(word_list);
    let found = Arc::new(Mutex::new(false));
    let first = Arc::new(first_word);
    let (tx, rx): (mpsc::Sender<Vec<String>>, mpsc::Receiver<Vec<String>>) = mpsc::channel();

    while !ladder_list.lock().unwrap().is_empty() {
        let mut threads = vec![];
        let lad = ladder_list.lock().unwrap().remove(0);
        let one_away = get_words_one_letter_away(lad.last().unwrap(), &word_list);

        for word in one_away {
            // Individual sender for each ladder pulled from the ladder_list
            let ladder_sender = tx.clone();
            //let cloned_list = ladder_list.clone();
            let cloned_list = Arc::clone(&ladder_list);
            // Get the bottom ladder
            let mut ladder = lad.clone();

            // Clone the first word (change this to Arc)
            let first = first.clone();

            // found?
            let f = Arc::clone(&found);

            let child = thread::spawn(move || {
                //println!("Entering new thread for {}", word);
                // If the ladder does not already contain it, push it
                if !ladder.contains(&word) {
                    ladder.push(word);

                    // If the ladder is complete, say so
                    if ladder.contains(&first) {
                        println!("LADDER FOUND\n{:?}", ladder);
                        // Clone the ladder and send it back
                        ladder_sender.send(ladder).unwrap();
                        (*f.lock().unwrap()) = true;
                        return;
                    } else {
                        // Push the new ladder to the ladder_list
                        if !cloned_list.lock().unwrap().contains(&ladder) {
                            cloned_list.lock().unwrap().push(ladder);
                        }
                    }
                }
            });
            threads.push(child);
        }
        for child in threads {
            child.join().unwrap();
            println!("{:?}", lad);
        }
        if *(found.lock().unwrap()) {
            println!("LADDER FOUND");
            return rx.recv().unwrap();
            //break;
        }
    }

    /*
    for received in rx {
        println!("{:?}", received);
        /*
        if received.contains(&first) && received.contains(&second_word) {
            return received;
        }
        */
    }
    */
    Vec::new()
}
*/

/* doesn't work
fn get_ladder(first_word: String, second_word: String, word_list: Vec<String>) -> Vec<String> {
    // Start the list with the first ladder
    let ladder_list = Arc::new(Mutex::new(vec![vec![second_word]]));
    let word_list = Arc::new(word_list);
    let found = Arc::new(Mutex::new(false));
    let first = Arc::new(first_word);

    let (tx, rx): (mpsc::Sender<Vec<String>>, mpsc::Receiver<Vec<String>>) = mpsc::channel();

    while *found.lock().unwrap() == false {
        let mut threads = vec![];

        //for ladder in &mut ladder_list.lock().unwrap().iter_mut() {
        while ladder_list.lock().unwrap().len() > 0 {
            //let ladder_sender = tx.clone();
            let mut ladder = ladder_list.lock().unwrap().remove(0);
            //let first = first.clone();
            let cloned_word_list = word_list.clone();
            //let f = found.clone();
            let last_word: String = ladder.last().unwrap().clone();

            let child = thread::spawn(move || {
                //println!("Entering new thread");

                let one_away = get_words_one_letter_away(&last_word[..], &cloned_word_list);
                let mut new_ladders: Vec<Vec<String>> = Vec::new();

                for word in one_away {
                    if !ladder.contains(&word) {
                        /*
                        ladder.push(word);

                        if ladder.contains(&first) {
                            println!("LADDER FOUND");
                            ladder_sender.send(ladder.clone()).unwrap();
                            (*f.lock().unwrap()) = true;
                            return;
                        }
                        */

                        ladder.push(word);
                        new_ladders.push(ladder.clone());
                        ladder.pop();
                    }
                }
                new_ladders
            });
            threads.push(child);
        }
        for child in threads {
            let new_ladders = child.join().unwrap();
            for lad in new_ladders {
                if lad.contains(&first) {
                    println!("LADDER FOUND");
                }
                ladder_list.lock().unwrap().push(lad);
            }
            //ladder_list.lock().unwrap().extend(new_ladders);
        }
        if *(found.lock().unwrap()) {
            println!("LADDER FOUND");
            return rx.recv().unwrap();
            //break;
        }
    }

    /*
    for received in rx {
        if received.contains(&first) && received.contains(&second) {
            return received;
        }
    }
    */
    Vec::new()
}
*/

fn get_ladder(first_word: String, second_word: String, word_list: Vec<String>) -> Vec<String> {
    println!("Entering get_ladder");
    let mut found = false;
    let word_list = Arc::new(word_list);

    // Start the list with the first ladder
    let mut list_of_ladders = vec![vec![second_word.clone()]];

    while !found {
        let mut threads = vec![];

        while list_of_ladders.len() > 0 {
            let mut ladder = list_of_ladders.remove(0);
            let cloned_word_list = word_list.clone();
            let child = thread::spawn(move || {
                //println!("Entering new thread");
                let one_away = get_words_one_letter_away(ladder.last().unwrap(), &cloned_word_list);
                //println!("one away from {} is {:?}", ladder.last().unwrap(), one_away);

                let mut new_ladders = vec![];
                for word in one_away {
                    if !ladder.contains(&word) {
                        ladder.push(word);
                        if !new_ladders.contains(&ladder) {
                            new_ladders.push(ladder.clone());
                        }
                        ladder.pop();
                    }
                }
                println!("\n{:?}", new_ladders);
                new_ladders
            });
            threads.push(child);
        }
        for child in threads {
            let new_ladders = child.join().unwrap();
            for lad in new_ladders {
                //println!("{:?}", lad);
                if lad.contains(&first_word) {
                    println!("LADDER FOUND");
                    found = true;
                    break;
                }
                if !list_of_ladders.contains(&lad) {
                    list_of_ladders.push(lad);
                }
            }
            //ladder_list.lock().unwrap().extend(new_ladders);
        }
    }

    Vec::new()
}
