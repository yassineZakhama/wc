use std::{env, io};

use wc::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    match Config::build(&args) {
        Ok(config) => match wc::run(config) {
            Err(err) => eprintln!("{}", err),
            _ => {
                println!("Enter anything to exit.");
                let _ = io::stdin().read_line(&mut String::new());
                return;
            }
        },
        Err(err) => eprintln!("{}", err),
    }
}
