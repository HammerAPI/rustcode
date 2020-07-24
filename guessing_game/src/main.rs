use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    let mut rands = vec![];
    for i in (0..100).rev() {
        //let random = rand::thread_rng().gen_range(1, 301);
        rands.push(i);
    }
    println!("{:?}", rands);

    loop {
        println!("\nPlease input your guess: ");

        // Create a mutable string
        let mut guess = String::new();

        // Read a line of input, storing in the 'guess' variable, and crashing
        // if unable to read
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadows 'guess' by converting it to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            //Err(_) => continue,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
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
