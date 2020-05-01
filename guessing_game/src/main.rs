use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess: ");

        // Create a mutable string
        let mut guess = String::new();

        // Read a line of input, storing in the 'guess' variable, and crashing
        // if unable to read
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Shadows 'guess' by converting it to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            //Err(_) => continue,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };
        
        println!("You guessed: {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
