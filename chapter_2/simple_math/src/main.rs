use std::io;
use std::io::Write;

fn main() {
    let get_num = |msg| loop {
        print!("{msg}");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        if let Ok(num) = input.trim().parse::<u32>() {
            break num;
        }

        println!("Invalid input. Please enter a number.");
    };

    let first = get_num("What is the first number? ");
    let second = get_num("What is the second number? ");

    for op in ["+", "-", "*", "/"] {
        let result = match op {
            "+" => first.checked_add(second),
            "-" => first.checked_sub(second),
            "*" => first.checked_mul(second),
            "/" => first.checked_div(second),
            _ => unreachable!()
        };

        match result {
            Some(value) => println!("{first} {op} {second} = {value}"),
            None => println!("Error: Overflow for operation {first} {op} {second}")
        }
    }
}