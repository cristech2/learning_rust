use rand::Rng;
use std::cmp::Ordering; // Importing the Ordering enum for comparison results.
use std::io; // Importing the IO Module for the standard library. // Importing the Rng trait from the rand crate for random number generation.

fn main() {
    // Generate a random number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // Print a welcome message to the user.
    println!("Guess the number if you can! :)");
    //Generate a infinit loop to keep asking the user for a guess until they get it right.
    loop {
        // Prompt the user to enter their guess.
        println!("Please input the number you guess:");

        // Create a new mutable String to hold the user's input.
        let mut guess = String::new();

        // Read the user's input from standard input and store it in the 'guess' variable.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Convert the user's input from a String to an unsigned 32-bit integer.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Print out what the user guessed.
        println!("You guessed: {guess}");

        // Compare the user's guess to the secret number.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Good job!");
                break;
            }
        }
    }
}
