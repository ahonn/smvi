use crate::{document::Document, state::State, status_bar::StatusBar};
use crossterm::{
    event::{read, Event},
    execute, terminal,
};
use std::io::{stdout, Write};

#[derive(Debug)]
pub struct Editor {
    state: State,
    document: Document,
    status_bar: StatusBar,
}

impl Editor {
    pub fn open(filename: Option<String>) -> Self {
        let document = if let Some(filename) = &filename {
            if let Ok(doc) = Document::from_file(filename.clone()) {
                doc
            } else {
                Document::default()
            }
        } else {
            Document::default()
        };
        let filename = if let Some(filename) = &filename {
            filename.clone()
        } else {
            String::from("[No Name]")
        };

        let state = State::default();
        let status_bar = StatusBar::new(filename, &state.get_mode().display());
        Self {
            document,
            state,
            status_bar,
        }
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
        if self.state.should_quite() {
            execute!(stdout, terminal::Clear(terminal::ClearType::Purge))?;
        } else {
            execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
            self.draw_rows();
            self.status_bar.update(&self.state);
            self.status_bar.draw()?;
        }
        stdout.flush()
    }

    fn read_keypress(&mut self) -> Result<Event, std::io::Error> {
        let event = read()?;
        match event {
            Event::Key(event) => {
                self.state.keypress(event);
            }
            _ => {}
        }
        Ok(event)
    }

    fn draw_rows(&self) {
        let (_, rows) = terminal::size().unwrap();
        for index in 0..(rows - 1) {
            if let Some(row) = self.document.row(index as usize) {
                self.draw_row(row);
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_row(&self, row: &str) {
        println!("{}\r", row.strip_suffix('\n').unwrap());
    }
}
