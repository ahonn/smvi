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
            KeyCode::Esc => Action::SetMode(super::ModeType::Normal),
            KeyCode::Left => Action::MoveLeft,
            KeyCode::Down => Action::MoveDown,
            KeyCode::Up => Action::MoveUp,
            KeyCode::Right => Action::MoveRight,
            _ => Action::None,
        }
    }
}
