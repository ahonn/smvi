use crossterm::event::KeyCode;

use crate::state::Action;

use super::Mode;

#[derive(Debug, Default)]
pub struct NormalMode {}

impl Mode for NormalMode {
    fn display(&self) -> &'static str {
        "NORMAL"
    }

    fn keypress(&mut self, event: crossterm::event::KeyEvent) -> Action {
        match event.code {
            KeyCode::Char('q') => Action::Quit,
            _ => Action::None,
        }
    }
}
