
mod buffer;
use buffer::Buffer;
use line::Line;
use super::{
    editorcommand::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};
mod location;
use location::Location;
mod line;
use std::cmp::min;



const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct View {
    buffer:Buffer,
    needs_redraw: bool,
    size: Size,
    location:Location,
    scroll_offset:Location
}



impl View {

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
        let top = self.scroll_offset.y;

        for n in 0..height {
            // Terminal::move_cursor_to(0,n).unwrap(); // ma tech à la base
            // If there's a line in the buffer at this row index
            if let Some(line) = self.buffer.lines.get(n.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let  right = self.scroll_offset.x.saturating_add(width);
                Self::render_line(n, &line.get(left..right));
            }  // If the buffer is empty and we're at the vertical center of the screen
            else if n == vertical_center && self.buffer.is_empty() {
                // Render the welcome message
                Self::render_line(n, &Self::build_welcome_message(width));
            } else {

                // Otherwise, draw an empty row
                // TODO : à enlever plus tard 
                let str_temp = format!("~");
                Self::render_line(n, str_temp.as_str());
            }
        }

        self.needs_redraw = false;
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Print(message) => Terminal::print(message.as_str()).unwrap(),
            EditorCommand::Quit => {}
        }
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn get_position(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }

    fn move_text_location(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, .. } = self.size;
        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
                
            }
            Direction::Down => {
                y = y.saturating_add(1);
            }
            Direction::Left => {
                if x > 0 {
                    x = x-1;
                } else if y > 0 {
                    y = y-1;
                    x = self.buffer.lines.get(y).map_or(0, Line::len);
                }
                
            }

            // TODO : a voir plus tard
            Direction::Right => {

                let width = self.buffer.lines.get(y).map_or(0, Line::len);
                if x < width {
                    x += 1;
                } else {
                    y = y.saturating_add(1);
                    x = 0;
                }
                
            }
            Direction::PageUp => y = y.saturating_sub(height).saturating_sub(1),
            Direction::PageDown => y = y.saturating_add(height).saturating_sub(1),
            Direction::Home => x = 0,
            Direction::End => x = self.buffer.lines.get(y).map_or(0, Line::len),
        }

         //snap x to valid position
         x = self.buffer.lines.get(y).map_or(0, |line| min(line.len(), x));
        
         //snap y to valid position
         y = min(y, self.buffer.lines.len());

        self.location = Location { x, y };
        self.scroll_location_into_view();
    }

    fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_location_into_view();
        self.needs_redraw = true;
    }

    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height } = self.size;
        let mut offset_changed = false;

        // Scroll vertically
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        //Scroll horizontally
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }
        self.needs_redraw = offset_changed;
    }

    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location:Location::default(),
            scroll_offset:Location::default(),
        }
    }
}

