use super::{Mode, ModeType};
use crate::state::Action;
use crossterm::event::KeyCode;

#[derive(Debug, Default)]
pub struct NormalMode {}

impl Mode for NormalMode {
    fn display(&self) -> &'static str {
        "NORMAL"
    }

    fn keypress(&mut self, event: crossterm::event::KeyEvent) -> Action {
        match event.code {
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Char('i') => Action::SetMode(ModeType::Insert),
            KeyCode::Char('h') => Action::MoveLeft,
            KeyCode::Char('j') => Action::MoveDown,
            KeyCode::Char('k') => Action::MoveUp,
            KeyCode::Char('l') => Action::MoveRight,
            _ => Action::None,
        }
    }
}
