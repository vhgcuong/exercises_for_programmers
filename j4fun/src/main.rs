use std::env::consts::FAMILY;

#[derive(Debug)]
struct Configuration {
    version: u32,
    active: bool
}

impl Configuration {
    fn default() -> Configuration {
        Configuration {
            version: 0,
            active: false
        }
    }
}

fn main() {
    let default_config = Configuration::default();
    println!("{:?}", default_config)
}
