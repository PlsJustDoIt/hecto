#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod editor;
use editor::Editor;
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;




fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len()>2 {
        println!("wrong args\n");
        return;
    }
    if args.len()==2 {
        let filename = args.get(1).unwrap();
        let file = Path::new(filename);
        if !file.exists() || !file.is_file() {
            println!("the path given is not a valid file.\n");
        return;
        }
    }
    
    Editor::new().unwrap().run();
}