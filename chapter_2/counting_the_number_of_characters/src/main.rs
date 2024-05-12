use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("What is the input string? ");
        io::stdout().flush().expect("Error...");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error input...");

        let str_input = input.trim();

        if str_input.is_empty() {
            println!("Must enter something into the program.");
        } else {
            println!("{} has {} characters", str_input, str_input.chars().count());
            break;
        }

    }

}
