
use crate::terminal::{Terminal,Position}; // If you want to use Terminal directly
use crate::buffer::{self, Buffer};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


#[derive(Default)]
pub struct View {
    buffer:Buffer,
    needs_review:bool
}

impl View {
    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    pub fn render_welcome_screen() -> Result<(), Error> {
        let  rows = Terminal::size()?.height;
        for n in 0..rows {
            // Terminal::move_cursor_to(0,n).unwrap(); // ma tech à la base
            Terminal::clear_line()?;
            if n == rows / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if n.saturating_add(1) < rows {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    // fn draw_rows(&self) -> Result<(), Error> {
    //     let  rows = Terminal::size()?.height;
    //     for n in 0..rows {
    //         // Terminal::move_cursor_to(0,n).unwrap(); // ma tech à la base
    //         Terminal::clear_line()?;
    //         if n == rows / 3 {
    //             Self::draw_welcome_message()?;
    //         } else {
    //             Self::draw_empty_row()?;
    //         }
    //         if n + 1 < rows {
    //             Terminal::print("\r\n")?;
    //         }
    //     }
    //     Ok(())

    // }

    pub fn render_buffer(&self) -> Result<(), Error> {


        let  rows = Terminal::size()?.height;
        for n in 0..rows {
            // Terminal::move_cursor_to(0,n).unwrap(); // ma tech à la base
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(n) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
            } else {
                Self::draw_empty_row()?;
            }
        }
        Ok(())

    }


    pub fn render(&self) -> Result<(),Error> {
        if self.buffer.is_empty() {
            Self::render_welcome_screen()?;
        } else {
            self.render_buffer()?;
        }
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
        }
    }

}