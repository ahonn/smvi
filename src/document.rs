use std::fs;

#[derive(Debug, Default)]
pub struct Document {
    pub rows: Vec<String>,
}

impl Document {
    pub fn from_file(filename: String) -> Result<Self, std::io::Error> {
        let content = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in content.lines() {
            rows.push(value.to_string());
        }
        Ok(Self { rows })
    }

    pub fn row(&self, row: usize) -> Option<&str> {
        if row < self.rows.len() {
            Some(&self.rows[row])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
