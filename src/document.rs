use std::fs;

use ropey::Rope;

#[derive(Debug, Default)]
pub struct Document {
    text: Rope,
}

impl Document {
    pub fn from_file(filename: String) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(filename)?;
        let text = Rope::from(content);
        Ok(Self { text })
    }

    pub fn row(&self, row: usize) -> Option<&str> {
        self.text.line(row).as_str()
    }
}
