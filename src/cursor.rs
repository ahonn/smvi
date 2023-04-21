use std::io::{stdout, Write};

use crossterm::{cursor, execute};

#[derive(Debug, Default)]
pub enum CursorShape {
    #[default]
    Block,
    Bar,
}

#[derive(Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self { row: 0, col: 0 }
    }
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

    pub fn move_down(&mut self) -> Result<(), std::io::Error> {
        self.position.row += 1;
        execute!(stdout(), cursor::MoveDown(1))
    }

    pub fn move_up(&mut self) -> Result<(), std::io::Error> {
        self.position.row -= 1;
        execute!(stdout(), cursor::MoveUp(1))
    }

    pub fn move_left(&mut self) -> Result<(), std::io::Error> {
        self.position.col -= 1;
        execute!(stdout(), cursor::MoveLeft(1))
    }

    pub fn move_right(&mut self) -> Result<(), std::io::Error> {
        self.position.col += 1;
        execute!(stdout(), cursor::MoveRight(1))
    }
}
