use std::env;

use wc::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");
    match Config::build(&args) {
        Ok(config) => match wc::run(config) {
            Err(err) => println!("{}", err),
            _ => return,
        },
        Err(err) => println!("{}", err),
    }
}
