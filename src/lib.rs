use std::error::Error;
use std::fs;
use std::io::{self, Read};

pub struct Wc(String);

impl Wc {
    pub fn bytes_count(&self) -> usize {
        self.0.len()
    }

    pub fn lines_count(&self) -> usize {
        self.0.lines().count()
    }

    pub fn words_count(&self) -> usize {
        self.0.split_whitespace().count()
    }

    pub fn characters_count(&self) -> usize {
        self.0.chars().count()
    }
}

enum CommandOption {
    Bytes,
    Lines,
    Words,
    Characters,
}

pub struct Config {
    file_path: Option<String>,
    option: Option<CommandOption>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() == 1 {
            return Err(String::from("Not enough arguments."));
        }

        if args[1] != "wc" {
            let error = String::from(format!("'{}' is not recognized as a command.", args[1]));
            return Err(error);
        }

        match args.len() {
            2 => Ok(Config {
                file_path: None,
                option: None,
            }),
            3 => Ok(Config::build_config(&args[2].clone())),
            4 => Config::build_config_with_option(&args[2].clone(), args[3].clone()),
            _ => return Err(String::from("Too much arguments.")),
        }
    }

    fn build_config(candidate: &str) -> Self {
        match candidate {
            "-c" => Config {
                file_path: None,
                option: Some(CommandOption::Bytes),
            },
            "-l" => Config {
                file_path: None,
                option: Some(CommandOption::Lines),
            },
            "-w" => Config {
                file_path: None,
                option: Some(CommandOption::Words),
            },
            "-m" => Config {
                file_path: None,
                option: Some(CommandOption::Characters),
            },
            file_path => Config {
                file_path: Some(file_path.to_string()),
                option: None,
            },
        }
    }

    fn build_config_with_option(option: &str, file_path: String) -> Result<Config, String> {
        match option {
            "-c" => Ok(Config {
                file_path: Some(file_path),
                option: Some(CommandOption::Bytes),
            }),
            "-l" => Ok(Config {
                file_path: Some(file_path),
                option: Some(CommandOption::Lines),
            }),
            "-w" => Ok(Config {
                file_path: Some(file_path),
                option: Some(CommandOption::Words),
            }),
            "-m" => Ok(Config {
                file_path: Some(file_path),
                option: Some(CommandOption::Characters),
            }),
            _ => Err(format!("'{}' is not recognized as an option.", option)),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let Some(file_path) = config.file_path {
        let file = fs::read_to_string(file_path)?;
        let wc = Wc(file);
        match config.option {
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
        return Ok(());
    } else {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        let wc = Wc(s);
        match config.option {
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
        return Ok(());
    }
}
