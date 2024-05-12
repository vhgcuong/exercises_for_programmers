use std::io;
use std::io::Write;

fn input_keyboard(text: String) -> String {
    print!("{} ", text);
    io::stdout().flush().expect("Error...");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error... ");

    String::from(input.trim())
}

fn main() {
    let noun = input_keyboard(String::from("Enter a noun: "));
    let verb = input_keyboard(String::from("Enter a verb: "));
    let adjective = input_keyboard(String::from("Enter an adjective:"));
    let adverb = input_keyboard(String::from("Enter an adverb:"));

    println!("Do you {verb} your {adjective} {noun} {adverb}? That's hilarious!");
}
