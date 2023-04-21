use crossterm::{
    execute,
    style::{Color, SetColors, Colors, Print, ResetColor}, terminal,
};
use std::io::stdout;

use crate::state::State;

const STATUS_FG_COLOR: Color = Color::Rgb {
    r: 63,
    g: 63,
    b: 63,
};
const STATUS_BG_COLOR: Color = Color::Rgb {
    r: 239,
    g: 239,
    b: 239,
};

#[derive(Debug)]
pub struct StatusBar {
    pub filename: String,
    pub mode: &'static str,
}

impl StatusBar {
    pub fn new(filename: String, mode: &'static str) -> Self {
        Self { filename, mode }
    }

    pub fn draw(&self) -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        let mut status = format!("{} {}", self.mode, self.filename);

        let (cols, _) = terminal::size()?;
        if (status.len() as u16) < cols {
            status.push_str(&" ".repeat((cols - status.len() as u16) as usize));
        }

        execute!(
            stdout,
            SetColors(Colors::new(STATUS_FG_COLOR, STATUS_BG_COLOR)),
            Print(status),
            ResetColor,
        )
    }

    pub fn update(&mut self, state: &State) {
        self.mode = state.get_mode().display();
    }
}
