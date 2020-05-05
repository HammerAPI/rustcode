//! # CSV Sorter
//!
//! ## Author: Daniel Hammer
//!
//! ### Date: 2020/5/2
//!
//! ### Description:
//! This program reads in a CSV composed of information about people, such as
//! names and addresses. It then stores each entry into a struct, and those
//! structs into a vector. The vector is sorted by last name (or first, if
//! last names are identical) and the newly sorted data is written to an
//! output file.
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process;

// Person struct to hold relevant data
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    street: String,
    city: String,
    state: String,
    zip_code: String,
}

/// Builds a list of Person structs
///
/// # Description
/// This function reads the input file line by line and creates a Person
/// struct based on the line's contents. It then adds that struct to a vector
/// and repeats for every line in the file. The final vector contains every
/// Person struct read in from the file.
///
/// # Arguments
/// * `input_file` - The input file to read from.
///
/// # Returns
/// * A vector of type Person containing all Person structs from the file.
fn build_person_vec(input_file: &mut File) -> Vec<Person> {
    let mut person_vec: Vec<Person> = Vec::new();
    let reader = BufReader::new(input_file);

    for line in reader.lines() {
        let line = line.unwrap();

        let mut data = line.split(',').map(|s| s.trim());

        let p = Person {
            first_name: String::from(data.next().unwrap()),
            last_name: String::from(data.next().unwrap()),
            street: String::from(data.next().unwrap()),
            city: String::from(data.next().unwrap()),
            state: String::from(data.next().unwrap()),
            zip_code: String::from(data.next().unwrap()),
        };
        person_vec.push(p);
    }
    person_vec
}

/// Sorts the list of Person structs
///
/// # Description
/// Sorts via Selection Sort.
///
/// # Arguments
/// * `person_vec` - A vector containing Person structs.
fn sort_person_vec(person_vec: &mut Vec<Person>) {
    person_vec.sort_by_key(|person| {
        (
            person.last_name.to_lowercase(),
            person.first_name.to_lowercase(),
        )
    });
}

/// Writes data to the output file
///
/// # Description
/// Writes all Person structs to the output file, catching errors if the file
/// is not available to be written to.
///
/// # Arguments
/// * `person_vec` - A vector containing Person structs.
/// * `output_file` - The file to write to.
fn write_to_file(person_vec: &mut Vec<Person>, output_file: &mut File) {
    for p in person_vec {
        // Format the peron's information as a string
        let info = format!(
            "{}, {}, {}, {}, {}, {}\n",
            p.first_name, p.last_name, p.street, p.city, p.state, p.zip_code
        );

        output_file
            .write_all(info.as_bytes())
            .unwrap_or_else(|err| panic!("Could not write data to file: {}", err));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: 'cargo run [input file] [output file]'");
        process::exit(1);
    }

    let mut input_file = File::open(Path::new("src/").join(&args[1]))
        .unwrap_or_else(|err| panic!("Couldn't open that file: {}", err));

    let mut output_file = File::create(Path::new("src/").join(&args[2]))
        .unwrap_or_else(|err| panic!("Couldn't create that file: {}", err));

    let mut person_vec = build_person_vec(&mut input_file);

    sort_person_vec(&mut person_vec);

    write_to_file(&mut person_vec, &mut output_file);
}
