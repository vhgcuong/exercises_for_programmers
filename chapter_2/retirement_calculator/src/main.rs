use std::io;
use std::io::Write;
use chrono::{Local, Datelike};

fn main() {

    loop {
        let current_age = input_keyboard(String::from("What is your current age?"));
        let retire = input_keyboard(String::from("At what age would you like to retire?"));

        if retire < current_age {
            println!("Current age < retire ===> Error...");
            continue
        }

        println!("You have {} years left until you can retire.", retire - current_age);

        let local = Local::now();
        let year = local.year();

        println!("It's {year}, so you can retire in {}.", year + retire - current_age);
        break;
    }

}


fn input_keyboard(text: String) -> i32 {
    return loop {
        print!("{text} ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        if let Ok(num) = input.trim().parse::<i32>() {
            break num;
        }

        println!("Invalid input. Please enter a number.");
    }
}