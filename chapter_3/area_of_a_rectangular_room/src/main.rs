use std::io;
use std::io::Write;
use std::num::ParseIntError;

const CONSTANT_METER: f64 = 0.09290304;

struct Rectangular<T> {
    width: T,
    length: T
}

impl<T> Rectangular<T>  {
    fn square<T>(&self) -> T {
        match self.length.checked_mul(self.width) {
            None => panic!(""),
            Some(_) => self.length * self.width
        }
    }

    fn convert_to_meters(&self) -> f64 {
        self.square() as f64 * CONSTANT_METER
    }

    fn get_width<T>(&self) -> &T {
        self.width
    }

    fn get_length(&self) -> &T {
        self.length
    }
}

fn main() {
    let length = match input_terminal("What is the length of the room in feet?") {
        Ok(number) => number,
        Err(_) => panic!("Input validate")
    };
    let width = match input_terminal("What is the width of the room in feet?") {
        Ok(number) => number,
        Err(_) => panic!("Input validate")
    };

    let rec = Rectangular {
        width,
        length
    };

    println!("You entered dimensions of {} feet by {} feet.", rec.get_length(), rec.get_width());
    println!("The area is {} square feet {:.3} square meters", rec.square(), rec.convert_to_meters());
}

fn input_terminal(text: &str) -> Result<u32, ParseIntError> {
    print!("{text} ");
    io::stdout().flush().expect("Error...");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error...");

    input.trim().parse::<u32>()
}
