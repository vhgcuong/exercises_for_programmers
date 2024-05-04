use std::io;
use std::io::Write;

fn logic(first: u32, second: u32) {
    match first.checked_add(second) {
        Some(_) => {
            println!("{first} + {second} = {}", first + second);
        },
        None => {
            println!("Error: {first} + {second} (u32:MAX = 4_294_967_295u32)");
        }
    }

    match first.checked_sub(second) {
        Some(_) => {
            println!("{first} - {second} = {}", first - second);
        },
        None => {
            println!("Error: {first} - {second} (u32:MIN = 0)");
        }
    }

    match first.checked_mul(second) {
        Some(_) => {
            println!("{first} * {second} = {}", first * second);
        },
        None => {
            println!("Error: {first} * {second} (u32:MAX = 4_294_967_295u32)");
        }
    }

    match first.checked_div(second) {
        Some(_) => {
            println!("{first} / {second} = {}", first / second);
        },
        None => {
            println!("Error: {first} / {second} (u32:MAX = 4_294_967_295u32)");
        }
    }
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
