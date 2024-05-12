use std::io::{self, Write};
use chrono::{Local, Datelike};

fn main() {
    loop {
        let current_age = input_keyboard("What is your current age?");
        let retire = input_keyboard("At what age would you like to retire?");

        if retire < current_age {
            println!("Current age is greater than retirement age. Please enter valid ages.");
            continue;
        }

        let years_left = retire - current_age;
        println!("You have {} years left until you can retire.", years_left);

        let current_year = Local::now().year();
        let retirement_year = current_year + years_left;
        println!("It's {}, so you can retire in {}.", current_year, retirement_year);
        break;
    }
}

fn input_keyboard(prompt: &str) -> i32 {
    loop {
        print!("{} ", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if let Ok(num) = input.trim().parse::<i32>() {
            break num;
        }

        println!("Invalid input. Please enter a number.");
    }
}
