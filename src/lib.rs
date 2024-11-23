use std::error::Error;
use std::fs;
use std::io::{self, Read};

mod config;
mod wc;

pub use config::Config;
use wc::{CommandOption, Wc};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let Some(file_path) = config.file_path {
        let file = fs::read_to_string(file_path)?;
        let wc = Wc(file);
        process_option(&wc, config.option);
        return Ok(());
    } else {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        let wc = Wc(s);
        process_option(&wc, config.option);
        return Ok(());
    }
}

fn process_option(wc: &Wc, option: Option<CommandOption>) {
    match option {
        Some(CommandOption::Bytes) => println!("{}", wc.bytes_count()),
        Some(CommandOption::Lines) => println!("{}", wc.lines_count()),
        Some(CommandOption::Words) => println!("{}", wc.words_count()),
        Some(CommandOption::Characters) => println!("{}", wc.characters_count()),
        _ => println!(
            "{} {} {}",
            wc.lines_count(),
            wc.words_count(),
            wc.bytes_count()
        ),
    }
}
