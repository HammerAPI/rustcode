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
    phone_number: String,
}

// Person constructor
impl Person {
    fn new(first_name: String, last_name: String,
            street: String, city: String, state: String,
            zip_code: String, phone_number: String) -> Person {

        Person {
            first_name,
            last_name,
            street,
            city,
            state,
            zip_code,
            phone_number,
        }
    }
}




// Parses command-line arguments 
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




// Constructs a vector of Person structs from the input file
fn build_person_list(input_file: &mut File) -> Vec<Person> {

    let mut person_vec: Vec<Person> = Vec::new();
    let reader = BufReader::new(input_file);

    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.

        let data: Vec<&str> = line.split(", ").collect();

        let p = Person::new(String::from(data[0].trim()), String::from(data[1]), String::from(data[2]), String::from(data[3]), String::from(data[4]), String::from(data[5]), String::from(data[6]));
        person_vec.push(p);
    }
    person_vec
}




// Sorts the vector of Person structs via selection sort
fn sort_person_list(person_list: &mut Vec<Person>) {

    for i in 0..person_list.len() {

        let mut lowest = i;

        for j in (i + 1)..person_list.len() {

            // Temporary variables to hold first and last names
            let j_last = &person_list[j].last_name.to_lowercase();
            let j_first = &person_list[j].first_name.to_lowercase();
            let low_last = &person_list[lowest].last_name.to_lowercase();
            let low_first = &person_list[lowest].first_name.to_lowercase();

            // Swap by last name or first name if last names are equal
            if (j_last < low_last) || (j_last == low_last && j_first < low_first){
                lowest = j;
            }
        }
        person_list.swap(lowest, i);
    }
}




// Write the list of Person structs to the output file
fn write_to_file(person_list: &mut Vec<Person>, output_file: &mut File) {

    for p in person_list {
        // Format the peron's information as a string
        let info = format!("{}, {}, {}, {}, {}, {}, {}\n",
            p.first_name, p.last_name, p.street, p.city,
            p.state, p.zip_code, p.phone_number);

        // Write to output file
        match output_file.write_all(info.as_bytes()) {
            Err(why) => panic!("\ncouldn't write to file: {}", why),
            Ok(_) => (),
        }
    }
}




// Program logic
fn main() {
    let args: Vec<String> = env::args().collect();

    // Get the input and output files
    let (mut input_file, mut output_file) = arg_parser(&args).unwrap_or_else(|err| {
        println!("\nError: {}", err);
        process::exit(1);
    });

    let mut person_list = build_person_list(&mut input_file);

    sort_person_list(&mut person_list);

    write_to_file(&mut person_list, &mut output_file);
}
