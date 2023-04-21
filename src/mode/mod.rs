use crossterm::event::KeyEvent;
use crate::state::Action;

pub mod normal_mode;
pub mod insert_mode;

#[derive(Debug, Default)]
pub enum ModeType {
    #[default]
    Normal,
    Insert,
}

pub trait Mode {
    fn display(&self) -> &'static str;
    fn keypress(&mut self, event: KeyEvent) -> Action;
}
