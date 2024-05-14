use std::io;
use std::io::Write;
use std::num::ParseIntError;

fn main() {
    let length = match input_terminal("What is the length of the room in feet?") {
        Ok(number) => number,
        Err(_) => panic!("Input validate")
    };
    let width = match input_terminal("What is the width of the room in feet?") {
        Ok(number) => number,
        Err(_) => panic!("Input validate")
    };

    let square = length * width;
    println!("You entered dimensions of {length} feet by {width} feet.");
    println!("The area is {} square feet {:.3} square meters", length * width, square as f64 * 0.09290304);
}

fn input_terminal(text: &str) -> Result<u32, ParseIntError> {
    print!("{text} ");
    io::stdout().flush().expect("Error...");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error...");

    input.trim().parse::<u32>()
}
