use crate::mode::{insert_mode::InsertMode, normal_mode::NormalMode, Mode, ModeType};

pub enum Action {
    SwitchMode(ModeType),
    Quit,
    None,
}

#[derive(Debug, Default)]
pub struct State {
    quit: bool,
    mode_type: ModeType,
}

impl State {
    pub fn should_quite(&self) -> bool {
        self.quit
    }

    pub fn switch_mode_type(&mut self, mode_type: ModeType) {
        self.mode_type = mode_type;
    }

    pub fn get_mode(&self) -> Box<dyn Mode> {
        match self.mode_type {
            ModeType::Normal => Box::<NormalMode>::default(),
            ModeType::Insert => Box::<InsertMode>::default(),
        }
    }

    pub fn keypress(&mut self, event: crossterm::event::KeyEvent) {
        let action = self.get_mode().keypress(event);
        self.dispatch(action);
    }

    pub fn dispatch(&mut self, action: Action) {
        match action {
            Action::Quit => self.quit = true,
            Action::SwitchMode(mode_type) => {
                self.switch_mode_type(mode_type);
            }
            Action::None => {}
        }
    }
}


