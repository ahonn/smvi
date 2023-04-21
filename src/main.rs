mod editor;
mod document;
mod state;
mod mode;
mod cursor;

use clap::Parser;
use editor::Editor;

#[derive(Parser, Debug)]
struct Args {
    filename: Option<String>,
}

fn main() {
    let args = Args::parse();
    Editor::open(args.filename).run();
}
