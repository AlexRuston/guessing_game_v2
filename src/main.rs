extern crate rand;

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    // generate a random number
    let generated_number = rand::thread_rng().gen_range(1, 100);

    // use this to break out of the loop
    let mut correct_flag: bool = false;
    // count the users guesses
    let mut guess_count = 0;

    while !correct_flag {
        // incrememnt guesses
        guess_count = guess_count + 1;

        if guess_count <= 1 {
            println!("Enter a number between 1 and 100...");
        }else {
            println!("Guess again...");
        }

        // read user input
        let mut user_guess_input = String::new();

        io::stdin()
            .read_line(&mut user_guess_input)
            .expect("Failed to read line");

        // make it a u32 and validate it
        let user_guess: i32 = match user_guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Shutting down..."),
        };

        // get the difference between the users guess and the generated number
        let difference = (generated_number - user_guess).abs();

        // we can help the user out a bit and tell them how far away they are
        match difference {
            0 => println!("Woah, nice!"),
            1 ..= 10 => println!("You're between 1 and 10 away..."),
            11 ..= 20 => println!("You're between 11 and 20 away..."),
            21 ..= 35 => println!("You're between 21 and 35 away..."),
            _ => println!("Nope..."),
        }

        // point the user in the direction of up or down, or spot on
        match user_guess.cmp(&generated_number) {
            Ordering::Less => println!("{} is too low ", user_guess),
            Ordering::Greater => println!("{} is too high ", user_guess),
            Ordering::Equal => {
                println!("{}!\nWinner!", generated_number);
                println!("Good work!\nAnd in: {} guesses!", guess_count);
                // breaks us out of the while
                correct_flag = true;
            }
        }
    }
}
