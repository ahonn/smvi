use crate::{document::Document, state::{State, Action}};
use crossterm::{
    event::{read, Event, KeyCode},
    execute, terminal,
};
use std::io::stdout;

#[derive(Debug)]
pub struct Editor {
    document: Document,
    state: State,
}

impl Editor {
    pub fn open(filename: Option<String>) -> Self {
        let document = if let Some(filename) = filename {
            if let Ok(doc) = Document::from_file(filename) {
                doc
            } else {
                Document::default()
            }
        } else {
            Document::default()
        };
        Self {
            document,
            state: State::default(),
        }
    }

    pub fn run(&mut self) {
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

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        terminal::enable_raw_mode().unwrap();
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        self.draw_rows();
        Ok(())
    }

    fn read_keypress(&mut self) -> Result<Event, std::io::Error> {
        let event = read()?;
        match event {
            Event::Key(event) => {
                self.state.keypress(event);
            },
            _ => {}
        }
        Ok(event)
    }

    fn draw_rows(&self) {
        let (_, rows) = terminal::size().unwrap();
        for index in 0..rows {
            if let Some(row) = self.document.row(index as usize) {
                self.draw_row(row);
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_row(&self, row: &str) {
        println!("{}\r", row.strip_suffix("\n").unwrap());
    }
}
