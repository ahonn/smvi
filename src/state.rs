use std::time::Instant;

use crossterm::terminal;

use crate::{
    cursor::{Cursor, CursorShape},
    document::Document,
    mode::{insert_mode::InsertMode, normal_mode::NormalMode, Mode, ModeType},
};

#[derive(Debug)]
pub enum Action<'a> {
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
    ShowCursor,
    HideCursor,
    SetCursorPositon(&'a Position),
    SetMode(ModeType),
    // Message(String),
    Quit,
    None,
}

#[derive(Debug, Default)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub text: String,
    pub time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::from(" "),
            time: Instant::now(),
        }
    }
}

#[derive(Debug, Default)]
pub struct State {
    quit: bool,
    document: Document,
    mode_type: ModeType,
    cursor: Cursor,
    offset: Position,
    message: Message,
}

impl State {
    pub fn should_quite(&self) -> bool {
        self.quit
    }

    pub fn get_document(&self) -> &Document {
        &self.document
    }

    pub fn set_document(&mut self, document: Document) {
        self.document = document;
    }

    pub fn get_mode(&self) -> Box<dyn Mode> {
        match self.mode_type {
            ModeType::Normal => Box::<NormalMode>::default(),
            ModeType::Insert => Box::<InsertMode>::default(),
        }
    }

    pub fn set_mode(&mut self, mode_type: ModeType) -> Result<(), std::io::Error> {
        self.mode_type = mode_type;
        match self.mode_type {
            ModeType::Normal => self.cursor.set_shape(CursorShape::Block)?,
            ModeType::Insert => self.cursor.set_shape(CursorShape::Bar)?,
        };
        Ok(())
    }

    pub fn get_message(&self) -> &Message {
        &self.message
    }

    pub fn set_message(&mut self, text: String) {
        self.message.text = text;
        self.message.time = Instant::now();
    }

    pub fn get_cursor_position(&self) -> &Position {
        &self.cursor.position
    }

    pub fn get_offset(&self) -> &Position {
        &self.offset
    }

    pub fn keypress(&mut self, event: crossterm::event::KeyEvent) -> Result<(), std::io::Error> {
        let mut mode = self.get_mode();
        let action = mode.keypress(event);
        self.dispatch(action);
        self.scroll();
        Ok(())
    }

    pub fn dispatch(&mut self, action: Action) {
        match action {
            Action::MoveUp | Action::MoveDown | Action::MoveLeft | Action::MoveRight => {
                self.move_cursor(action)
            }
            Action::ShowCursor => self.cursor.show().unwrap(),
            Action::HideCursor => self.cursor.hide().unwrap(),
            Action::SetCursorPositon(position) => self.cursor.set_position(position).unwrap(),
            Action::SetMode(mode_type) => self.set_mode(mode_type).unwrap(),
            // Action::Message(text) => self.set_message(text),
            Action::Quit => self.quit = true,
            Action::None => {}
        }
    }

    fn move_cursor(&mut self, action: Action) {
        let Position { mut row, mut col } = self.cursor.position;
        let height = self.document.len();

        let width = if let Some(row) = self.document.row(row) {
            row.len()
        } else {
            0
        };

        match action {
            Action::MoveUp => {
                if row > 0 {
                    row = row.saturating_sub(1);
                }
            }
            Action::MoveDown => {
                if row.saturating_add(1) <= height {
                    row = row.saturating_add(1);
                }
            }
            Action::MoveLeft => {
                if col > 0 {
                    col = col.saturating_sub(1);
                } else if row > 0 {
                    row = row.saturating_sub(1);
                    if let Some(row) = self.document.row(row) {
                        col = row.len();
                    } else {
                        col = 0;
                    }
                }
            }
            Action::MoveRight => {
                if col < width {
                    col = col.saturating_add(1);
                } else if row < height {
                    row = row.saturating_add(1);
                    col = 0;
                }
            }
            _ => {}
        }
        self.cursor.position = Position { row, col };
    }

    fn scroll(&mut self) {
        let Position { row, col } = self.cursor.position;
        let (width, height) = terminal::size().unwrap();
        let height = height.saturating_sub(2);

        if col < self.offset.col {
            self.offset.col = col;
        } else if col >= self.offset.col.saturating_add(width as usize) {
            self.offset.col = col.saturating_sub(width as usize).saturating_add(1);
        }

        if row < self.offset.row {
            self.offset.row = row;
        } else if row >= self.offset.row.saturating_add(height as usize) {
            self.offset.row = row.saturating_sub(height as usize).saturating_add(1);
        }
    }
}
