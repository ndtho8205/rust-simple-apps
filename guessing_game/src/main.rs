use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::stdin;
use std::process::exit;

const TIMES: i32 = 10;

fn main() {
    println!("GUESS THE NUMBER!");

    let secret_number: i32 = thread_rng().gen_range(1, 101);
    let mut win = false;

    for i in 1..=TIMES {
        println!("[{}] Input your guess:", i);

        let mut guess = String::new();

        if let Err(error) = stdin().read_line(&mut guess) {
            println!("error: Failed to read your input!");
            println!("{}", error);
            exit(1);
        }

        match guess.trim().parse::<i32>() {
            Ok(value) => {
                println!("You guessed: {}", value);

                match value.cmp(&secret_number) {
                    Ordering::Equal => {
                        win = true;
                        break;
                    }
                    Ordering::Less => println!("BIGGER, please!"),
                    Ordering::Greater => println!("SMALLER, please!"),
                }
            }
            Err(error) => {
                println!("error: Failed to parse your input into a valid number!");
                println!("{}", error);
            }
        }

        println!();
    }

    if win {
        println!("CONGRATULATIONS! You win!");
    } else {
        println!("The number is {}. Let's try again later!", secret_number);
    }
}
