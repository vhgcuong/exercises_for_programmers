use std::io;
use std::io::Write;

fn main() {
    print!("What is your name? ");
    io::stdout().flush().expect("Error...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error input...");
    println!("Hello, {}, nice to meet you!", input.trim());
}
