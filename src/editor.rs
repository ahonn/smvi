use crate::{
    document::Document,
    state::{Action, Position, State},
};
use crossterm::{
    event::{poll, read, Event},
    execute,
    style::{Color, Colors, Print, ResetColor, SetColors},
    terminal,
};
use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};

const STATUS_FG_COLOR: Color = Color::Rgb {
    r: 63,
    g: 63,
    b: 63,
};
const STATUS_BG_COLOR: Color = Color::Rgb {
    r: 239,
    g: 239,
    b: 239,
};

#[derive(Debug)]
pub struct Editor {
    filename: Option<String>,
    state: State,
}

impl Editor {
    pub fn open(filename: Option<String>) -> Self {
        let mut state = State::default();
        if let Some(filename) = &filename {
            if let Ok(document) = Document::from_file(filename.clone()) {
                state.set_document(document);
            }
        }

        Self { filename, state }
    }

    pub fn run(&mut self) {
        terminal::enable_raw_mode().unwrap();

        loop {
            if let Err(err) = self.refresh_screen() {
                panic!("{}", err);
            }
            if self.state.should_quite() {
                break;
            }
            if let Err(err) = self.read_keypress() {
                panic!("{}", err);
            }
        }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        let mut stdout = stdout();

        self.state.dispatch(Action::HideCursor);
        self.state
            .dispatch(Action::SetCursorPositon(&Position::default()));
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        if !self.state.should_quite() {
            self.draw_rows();
            self.draw_status_bar()?;
            self.draw_message_bar()?;

            let Position { row, col } = self.state.get_cursor_position();
            let offset = self.state.get_offset();
            self.state.dispatch(Action::SetCursorPositon(&Position {
                row: row - offset.row,
                col: col - offset.col,
            }));
        }
        self.state.dispatch(Action::ShowCursor);
        stdout.flush()
    }

    fn read_keypress(&mut self) -> Result<(), std::io::Error> {
        if poll(Duration::from_millis(10))? {
            let event = read()?;
            if let Event::Key(event) = event {
                self.state.keypress(event)?;
            }
        }
        Ok(())
    }

    fn draw_rows(&self) {
        let document = self.state.get_document();
        let (_, rows) = terminal::size().unwrap();
        for index in 0..(rows - 2) {
            if let Some(row) = document.row(index as usize + self.state.get_offset().row) {
                self.draw_row(row);
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_row(&self, row: &str) {
        println!("{}\r", row.replace('\n', ""));
    }

    fn draw_status_bar(&self) -> Result<(), std::io::Error> {
        let filename = if let Some(name) = &self.filename {
            name.to_string()
        } else {
            "[No Name]".to_string()
        };
        let status = format!("[{}] {}", self.state.get_mode().display(), filename);

        let Position { row, col } = self.state.get_cursor_position();
        let line_indicator = format!("{}:{}", row, col);

        let (cols, _) = terminal::size()?;
        let len = (status.len() + line_indicator.len()) as u16;
        let spaces = if len < cols {
            " ".repeat((cols - len) as usize)
        } else {
            "".to_string()
        };

        let status = format!("{}{}{}", status, spaces, line_indicator);

        execute!(
            stdout(),
            SetColors(Colors::new(STATUS_FG_COLOR, STATUS_BG_COLOR)),
            Print(status),
            ResetColor,
        )
    }

    fn draw_message_bar(&self) -> Result<(), std::io::Error> {
        let message = &self.state.get_message();
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(terminal::size().unwrap().0 as usize);
            execute!(stdout(), Print(text))?;
        }
        Ok(())
    }
}
