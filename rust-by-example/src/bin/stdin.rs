use std::io::stdin;
use std::process::exit;

fn main() {
    let mut name = String::new();
    println!("Enter your name:");
    match stdin().read_line(&mut name) {
        Ok(_) => println!("Hello, {}", name),
        Err(e) => {
            println!("error: {}", e);
            return;
        }
    }

    let mut first_number = String::new();
    let a: i32;
    println!("Enter your first number:");
    stdin().read_line(&mut first_number).unwrap();
    match first_number.trim().parse::<i32>() {
        Ok(value) => a = value,
        Err(err) => {
            println!("error: {}", err);
            exit(1)
        }
    }

    let mut second_number = String::new();
    let b: i32;
    println!("Enter your second number:");
    stdin().read_line(&mut second_number).unwrap();
    match second_number.trim().parse::<i32>() {
        Ok(value) => b = value,
        Err(err) => {
            println!("error: {}", err);
            exit(1)
        }
    }

    println!("{:3} + {:3} = {:3}", a, b, a + b);
}
