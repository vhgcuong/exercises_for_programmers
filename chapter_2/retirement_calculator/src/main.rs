use std::io;
use std::io::Write;

fn main() {
    let current_age = input_keyboard(String::from("What is your current age?"));
    let retire = input_keyboard(String::from("At what age would you like to retire?"));

}


fn input_keyboard(text: String) -> u32 {
    return loop {
        print!("{text}");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        if let Ok(num) = input.trim().parse::<u32>() {
            break num;
        }

        println!("Invalid input. Please enter a number.");
    }
}