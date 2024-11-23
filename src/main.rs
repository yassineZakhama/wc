use std::env;

use wc::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    match Config::build(&args) {
        Ok(config) => match wc::run(config) {
            Err(err) => eprintln!("{}", err),
            _ => return,
        },
        Err(err) => eprintln!("{}", err),
    }
}
