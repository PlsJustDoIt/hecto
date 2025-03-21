use std::{cmp::min, io::Error};

use super::{command::Edit, Line, Size, Terminal, UIComponent};

#[derive(Default)]
pub struct CommandBar {
    prompt: String,
    value: Line,
    needs_redraw: bool,
    size: Size,
}

impl CommandBar {
    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::Insert(character) => self.value.append_char(character),
            Edit::Delete | Edit::InsertNewline=> {}
            Edit::DeleteBackward => self.value.delete_last(),
        }
        self.set_needs_redraw(true);
    }
    pub fn caret_position_col(&self) -> usize {
        let max_width = self
            .prompt
            .len()
            .saturating_add(self.value.grapheme_count());
        min(max_width, self.size.width)
    }
    pub fn value(&self) -> String {
        self.value.to_string()
    }
    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
    }
}

impl UIComponent for CommandBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }
    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn draw(&mut self, origin: usize) -> Result<(), Error> {
        let area_for_value = self.size.width.saturating_sub(self.prompt.len()); //this is how much space there is between the right side of the prompt and the edge of the bar 1Has a conversation.
        let value_end = self.value.width(); // we always want to show the left part of the value, therefore the end of the visible range we try to access will be equal to the full width
        let value_start = value_end.saturating_sub(area_for_value); //This should give us the start for the grapheme subrange we want to print out.
        let message = format!(
            "{}{}",
            self.prompt,
            self.value.get_visible_graphemes(value_start..value_end)
        );
        let to_print = if message.len() <= self.size.width {
            message
        } else {
            String::new()
        };
        Terminal::print_row(origin, &to_print)
    }
}