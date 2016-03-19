/* The Ultimate Guessing Game */
/* TODO Game Features:
*  - Difficulty levels
*  - Score keeping
*  - Secret number changes
*  - Powerups/Traps
*  - Collect points to buy guesses/hints
*/

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn game() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Input a number between 1 and 100");

    let mut count = 5;
    while count > 0 {
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

        count = count - 1;
        if count > 0 {
            println!("Please guess again. {} guesses remainging.", count);
        } else {
            println!("Oh no! You ran out of guesses. The answer was {}.", secret_number);
        }
    }
}


fn main() {
    game();
}
