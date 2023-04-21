mod editor;
mod document;
mod state;
mod mode;

use clap::Parser;
use editor::Editor;

#[derive(Parser, Debug)]
struct Args {
    #[arg(required = false)]
    filename: String,
}

fn main() {
    let args = Args::parse();
    Editor::open(Some(args.filename)).run();
}
