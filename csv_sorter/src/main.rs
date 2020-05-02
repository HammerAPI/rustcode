/**
 * # CSV Sorter
 *
 * ## Author: Daniel Hammer
 *
 * ### data: 2020/5/2
 *
 * ### Description:
 * This program reads in a CSV composed of information about people, such as
 * names and addresses. It then stores each entry into a struct, and those
 * structs into a vector. The vector is sorted by last name (or first, if
 * last names are identical) and the newly sorted data is written to an
 * output file.
 */

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
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

// Person constructor
impl Person {
    fn new(first_name: String, last_name: String,
            street: String, city: String, state: String,
            zip_code: String) -> Person {

        Person {
            first_name,
            last_name,
            street,
            city,
            state,
            zip_code,
        }
    }
}




/**
 * Processes command-line arguments
 *
 * # Description
 * This function processes the passed-in command line arguments and attempts
 * to open and create valid input/output files from the names given.
 *
 * # Arguments
 * * `args` - A string array of command-line arguments.
 *
 * # Returns
 * * A tuple of the input file and output file if they are found, else errors.
 */
fn arg_parser(args: &[String]) -> Result<(File, File), &'static str> {
    
    // Exit if too many or too few arguments were passed
    if args.len() != 3 {
        return Err("Usage: 'cargo run [input file] [output file]");
    }

    // Get the input file
    let input_file = match File::open(format!("{}{}", "src/", &args[1])) {
        Err(why) => panic!("\ncouldn't open file: {}", why),
        Ok(file) => file,
    };

    // Get the output file
    let output_file = match File::create(format!("{}{}", "src/", &args[2])) {
        Err(why) => panic!("\ncouldn't create file: {}", why),
        Ok(file) => file,
    };

    // Return both files as a tuple
    Ok((input_file, output_file))
}




/**
 * Builds a list of Person structs
 *
 * # Description
 * This function reads the input file line by line and creates a Person
 * struct based on the line's contents. It then adds that struct to a vector
 * and repeats for every line in the file. The final vector contains every
 * Person struct read in from the file.
 *
 * # Arguments
 * * `input_file` - The input file to read from.
 *
 * # Returns
 * * A vector of type Person containing all Person structs from the file.
 */
fn build_person_vec(input_file: &mut File) -> Vec<Person> {

    let mut person_vec: Vec<Person> = Vec::new();
    let reader = BufReader::new(input_file);

    for line in reader.lines() {

        let line = line.unwrap();

        let data: Vec<&str> = line.split(", ").collect();

        let p = Person::new(String::from(data[0].trim()),
                            String::from(data[1].trim()),
                            String::from(data[2].trim()),
                            String::from(data[3].trim()),
                            String::from(data[4].trim()),
                            String::from(data[5].trim()));
        person_vec.push(p);
    }
    person_vec
}




/**
 * Sorts the list of Person structs
 *
 * # Description
 * Sorts via Selection Sort.
 *
 * # Arguments
 * * `person_vec` - A vector containing Person structs.
 */
fn sort_person_vec(person_vec: &mut Vec<Person>) {

    for i in 0..person_vec.len() {

        let mut lowest = i;

        for j in (i + 1)..person_vec.len() {

            // Temporary variables to hold first and last names
            let j_last = &person_vec[j].last_name.to_lowercase();
            let j_first = &person_vec[j].first_name.to_lowercase();
            let low_last = &person_vec[lowest].last_name.to_lowercase();
            let low_first = &person_vec[lowest].first_name.to_lowercase();

            // Swap by last name or first name if last names are equal
            if (j_last < low_last) || (j_last == low_last && j_first < low_first){
                lowest = j;
            }
        }
        person_vec.swap(lowest, i);
    }
}




/**
 * Writes data to the output file
 *
 * # Description
 * Writes all Person structs to the output file, catching errors if the file
 * is not available to be written to.
 *
 * # Arguments
 * * `person_vec` - A vector containing Person structs.
 * * `output_file` - The file to write to.
 */
fn write_to_file(person_vec: &mut Vec<Person>, output_file: &mut File) {

    for p in person_vec {

        // Format the peron's information as a string
        let info = format!("{}, {}, {}, {}, {}, {}\n",
            p.first_name, p.last_name, p.street, p.city,
            p.state, p.zip_code);

        // Write to output file
        match output_file.write_all(info.as_bytes()) {
            Err(why) => panic!("\ncouldn't write to file: {}", why),
            Ok(_) => (),
        }
    }
}




fn main() {

    let args: Vec<String> = env::args().collect();

    // Get the input and output files
    let (mut input_file, mut output_file) = arg_parser(&args).unwrap_or_else(|err| {
        println!("\nError: {}", err);
        process::exit(1);
    });

    let mut person_vec = build_person_vec(&mut input_file);

    sort_person_vec(&mut person_vec);

    write_to_file(&mut person_vec, &mut output_file);
}
