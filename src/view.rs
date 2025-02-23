
use super::terminal::{Size,Position , Terminal};
use crate::buffer::{self, Buffer};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct View {
    buffer:Buffer,
    needs_redraw: bool,
    size: Size,
}



impl View {

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }


    // pub fn render_buffer(&self) -> Result<(), Error> {


    //     let  rows = Terminal::size()?.height;
    //     for n in 0..rows {
    //         // Terminal::move_cursor_to(0,n).unwrap(); // ma tech à la base
    //         if let Some(line) = self.buffer.lines.get(n) {
    //             Terminal::print(line)?;
    //             Terminal::print("\r\n")?;
    //         } else {
    //             Self::draw_empty_row()?;
    //         }
    //     }
    //     Ok(())

    // }


    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();

        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }
        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;

        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message

    }

    pub fn render(&mut self) {

        

        if !self.needs_redraw {
            return;
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }
        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit too far up or down
        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;

        for n in 0..height {
            // Terminal::move_cursor_to(0,n).unwrap(); // ma tech à la base
            if let Some(line) = self.buffer.lines.get(n) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(n, truncated_line);
            }  else if n == vertical_center && self.buffer.is_empty() {
                Self::render_line(n, &Self::build_welcome_message(width));
            } else {
                Self::render_line(n, "~");
            }
        }

        self.needs_redraw = false;
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }


}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

