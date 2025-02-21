#![warn(clippy::all, clippy::pedantic,clippy::arithmetic_side_effects, clippy::as_conversions, clippy::integer_division)]
mod editor;
mod terminal;
mod view;
mod buffer;
use editor::Editor;



fn main() {

    // let args: Vec<String> = std::env::args().collect();
    // if args.len()>2 {
    //     println!("wrong args\n");
    //     return;
    // }
    // if let Some(first_arg) = args.get(1) {
    //     let file_contents = std::fs::read_to_string(first_arg).unwrap();
    //     for line in file_contents.lines() {
    //         // Do something with the line

    //    }
    // } else {
    //    println!("No arg given");
    // }
    Editor::default().run();
}