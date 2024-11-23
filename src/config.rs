use crate::wc::CommandOption;

pub struct Config {
    pub file_path: Option<String>,
    pub option: Option<CommandOption>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() == 1 {
            return Err(String::from("Not enough arguments."));
        }

        if args[1] != "wc" {
            let error = String::from(format!(
                "'{}' is not recognized as a command. Use 'wc' to run the program.",
                args[1]
            ));
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
