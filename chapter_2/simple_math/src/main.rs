use std::io;
use std::io::Write;

fn logic(first: u32, second: u32) {
    println!("{first} + {second} = {}", first + second);
    println!("{first} - {second} = {}", first - second);
    println!("{first} * {second} = {}", first * second);
    println!("{first} / {second} = {}", first / second);
}

fn input_keyboard(text: String) -> u32 {
    loop {
        print!("{text} ");
        io::stdout().flush().expect("Error...");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error...");

        match input.trim().parse::<u32>() {
            Ok(number) => return number,
            Err(_) => {
                println!("The value entered is not numeric. Please try again!");
                continue
            }
        }
    }
}

fn main() {
    let first = input_keyboard(String::from("What is the first number?"));
    let second = input_keyboard(String::from("What is the second number?"));

    logic(first, second);
}
