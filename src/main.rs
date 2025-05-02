extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::rng().random_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Enter your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please, enter a NUMBER!\n");
                continue;
            }
        };

        println!("Your guess: {}\n", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Very low!\n"),
            Ordering::Greater => println!("very high!\n"),
            Ordering::Equal => {
                println!("Bullseye!\n");
                break;
            }
        }
    }
}
