/* The Ultimate Guessing Game */
/* TODO Game Features:
*  - Difficulty levels
*  - Score keeping
*  - Secret number changes
*  - Powerups/Traps
*  - Collect points to buy guesses/hints
*/

use std::io;
use std::cmp::Ordering;

mod helpers;


fn game() {

    let mut diff_level: u32 = 1;
    let mut begin: bool = false;
    while !begin {
        println!("Please select a difficulty level:");
        println!("1. Easy");
        println!("2. Medium");
        println!("3. Hard");

        let mut selected_level = String::new();
        io::stdin().read_line(&mut selected_level).expect("Failed to read line.");
        diff_level = match selected_level.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 3 {
                    println!("Must be 1, 2, or 3.");
                    continue;
                } else {
                    begin = true;
                    num
                }
            }
            Err(_) => {
                println!("Please input a number 1, 2, or 3.");
                continue;
            }
        };
    }

    let mut max_number: i32 = 50;
    if diff_level == 1 {
        max_number = 50;
    } else if diff_level == 2 {
        max_number = 100;
    } else if diff_level == 3 {
        max_number = 200;
    }
    let secret_number = helpers::get_random_number(1, max_number);

    println!("Input a number between 1 and {}", max_number);

    let mut count = 5;
    while count > 0 {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        let trimmed_guess = guess.trim();

        if trimmed_guess == "exit" {
            println!("Goodbye.");
            break;
        }

        let guess: i32 = match trimmed_guess.parse() {
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
    println!("Welcome to the Guessing Game!");
    game();
}
