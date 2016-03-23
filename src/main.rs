/* The Ultimate Guessing Game */
/* TODO Game Features:
*  - Score keeping
*  - Secret number changes
*  - Powerups/Traps
*  - Collect points to buy guesses/hints
*/

use std::io;
use std::cmp::Ordering;

mod helpers;


fn menu() -> u32 {
    let mut selection = 1;
    let mut selected = false;
    println!("Please select a difficulty level or exit:");
    println!("1. Easy\n2. Medium\n3. Hard\n4. Exit");
    while !selected {
        let mut selected_level = String::new();
        io::stdin().read_line(&mut selected_level).expect("Failed to read line.");
        selection = match selected_level.trim().parse() {
            Ok(num) if num < 1 || num > 4 => {
                println!("Must be 1-4.");
                continue;
            },
            Ok(num) => {
                selected = true;
                num
            }
            Err(_) => {
                println!("Please input a number 1-4.");
                continue;
            }
        };
    }
    selection
}


fn game() -> bool {
    let menu_selection = menu();
    let max_number = match menu_selection {
        1 => 50,
        2 => 100,
        3 => 200,
        _ => {
            // Exit game
            println!("Goodbye.");
            return false;
        }
    };
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
    true
}


fn main() {
    println!("Welcome to the Guessing Game!");
    let mut play = true;
    while play {
        play = game();
    }
}
