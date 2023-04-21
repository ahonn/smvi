use crossterm::event::KeyCode;

use crate::state::Action;

use super::Mode;

#[derive(Debug, Default)]
pub struct InsertMode {}

impl Mode for InsertMode {
    fn display(&self) -> &'static str {
        "INSERT"
    }

    fn keypress(&mut self, event: crossterm::event::KeyEvent) -> Action {
        match event.code {
            KeyCode::Esc => Action::SwitchMode(super::ModeType::Normal),
            _ => Action::None,
        }
    }
}
