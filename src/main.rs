extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Input a number between 1 and 100");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        let trimmed_guess = guess.trim();

        if trimmed_guess == "exit" {
            println!("Goodbye.");
            break;
        }

        let guess: u32 = match trimmed_guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number...");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low."),
            Ordering::Greater => println!("Too high."),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }

        println!("Please guess again");
    }
}
