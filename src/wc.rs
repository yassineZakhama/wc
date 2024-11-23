pub enum CommandOption {
    Bytes,
    Lines,
    Words,
    Characters,
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
