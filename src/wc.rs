use std::str::FromStr;

pub enum CommandOption {
    Bytes,
    Lines,
    Words,
    Characters,
}

impl FromStr for CommandOption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-c" => Ok(CommandOption::Bytes),
            "-l" => Ok(CommandOption::Lines),
            "-w" => Ok(CommandOption::Words),
            "-m" => Ok(CommandOption::Characters),
            _ => Err(format!("'{}' is not recognized as an option.", s)),
        }
    }
}

pub struct Wc(pub String);

impl Wc {
    pub fn bytes_count(&self) -> usize {
        self.0.bytes().len()
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
