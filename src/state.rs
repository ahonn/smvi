use std::time::Instant;

use crate::{
    cursor::{Cursor, CursorShape, Position},
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
    Message(String),
    Quit,
    None,
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
    mode_type: ModeType,
    cursor: Cursor,
    message: Message,
}

impl State {
    pub fn should_quite(&self) -> bool {
        self.quit
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

    pub fn keypress(&mut self, event: crossterm::event::KeyEvent) -> Result<(), std::io::Error> {
        let mut mode = self.get_mode();
        let action = mode.keypress(event);
        self.dispatch(action);
        Ok(())
    }

    pub fn dispatch(&mut self, action: Action) {
        match action {
            Action::MoveDown => self.cursor.move_down().unwrap(),
            Action::MoveUp => self.cursor.move_up().unwrap(),
            Action::MoveLeft => self.cursor.move_left().unwrap(),
            Action::MoveRight => self.cursor.move_right().unwrap(),
            Action::ShowCursor => self.cursor.show().unwrap(),
            Action::HideCursor => self.cursor.hide().unwrap(),
            Action::SetCursorPositon(position) => self.cursor.set_position(&position).unwrap(),
            Action::SetMode(mode_type) => self.set_mode(mode_type).unwrap(),
            Action::Message(text) => self.set_message(text),
            Action::Quit => self.quit = true,
            Action::None => {}
        }
    }
}
