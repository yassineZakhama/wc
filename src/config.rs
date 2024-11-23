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
            3 => Ok(Config::do_build(&args[2].clone())),
            4 => Config::do_build_with_option(&args[2].clone(), args[3].clone()),
            _ => return Err(String::from("Too much arguments.")),
        }
    }

    fn do_build(candidate: &str) -> Self {
        match candidate.parse::<CommandOption>() {
            Ok(option) => Config {
                file_path: None,
                option: Some(option),
            },
            Err(_) => Config {
                file_path: Some(candidate.to_string()),
                option: None,
            },
        }
    }

    fn do_build_with_option(option: &str, file_path: String) -> Result<Config, String> {
        let option = option.parse::<CommandOption>()?;
        Ok(Config {
            file_path: Some(file_path),
            option: Some(option),
        })
    }
}
