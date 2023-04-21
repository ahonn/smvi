use std::io::stdout;
use crossterm::{cursor, execute};
use crate::state::Position;

#[derive(Debug, Default)]
pub enum CursorShape {
    #[default]
    Block,
    Bar,
}

#[derive(Debug, Default)]
pub struct Cursor {
    pub position: Position,
    pub shape: CursorShape,
}

impl Cursor {
    pub fn show(&mut self) -> Result<(), std::io::Error> {
        let Position { row, col } = self.position;
        execute!(
            stdout(),
            cursor::Show,
            cursor::MoveTo(col as u16, row as u16)
        )
    }

    pub fn hide(&mut self) -> Result<(), std::io::Error> {
        execute!(stdout(), cursor::Hide)
    }

    pub fn set_position(&self, position: &Position) -> Result<(), std::io::Error> {
        let Position { row, col } = position;
        let mut stdout = stdout();
        execute!(
            stdout,
            cursor::Show,
            cursor::MoveTo(*col as u16, *row as u16)
        )
    }

    pub fn set_shape(&self, shape: CursorShape) -> Result<(), std::io::Error> {
        let style = match shape {
            CursorShape::Block => cursor::SetCursorStyle::SteadyBlock,
            CursorShape::Bar => cursor::SetCursorStyle::SteadyBar,
        };
        execute!(stdout(), style)
    }
}
