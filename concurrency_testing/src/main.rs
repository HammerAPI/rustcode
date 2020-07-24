//use std::env;
//use std::process;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let list = Arc::new(Mutex::new(vec![
        "this is",
        "a list",
        "of words",
        "to search",
    ]));
    let mut handles = vec![];
    let found = Arc::new(Mutex::new("override"));

    while list.lock().unwrap().len() > 0 {
        //let list = Arc::clone(&list);
        let list = list.clone();
        let found = found.clone();

        let handle = thread::spawn(move || {
            let phrase = list.lock().unwrap().pop().unwrap_or_default();
            if phrase.contains("to") {
                let mut f = found.lock().unwrap();
                println!("FOUND\n{}", phrase);
                *f = phrase;
            }
        });
        handles.push(handle);
    }

    println!("Phrase is '{}'", found.lock().unwrap());

    for handle in handles {
        handle.join().unwrap();
    }
}
