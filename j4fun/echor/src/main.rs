use clap::{Command, Arg};

fn main() {
    // println!("{:?}", std::env::args()); // This will not work

    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Yotoko <vhgcuong95@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
        )
        .get_matches();

    println!("{:#?}", matches);

}
