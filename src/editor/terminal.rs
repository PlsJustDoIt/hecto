use crossterm::style::Print;
use crossterm::{ queue,Command};
use std::io::stdout;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use std::io::Write;
use std::io::Error;



impl Position {
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}


#[derive(Default,Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}
#[derive(Copy, Clone,Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Terminal {

}

impl Terminal {

    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::show_cursor()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{x:0, y:0})?;
        Self::execute()?;
        Ok(())
    }

    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }
    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_cursor_to(Position { x:0, y: row })?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor_to(position:Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.x as u16, position.y as u16))?;
        
        Ok(())
    }
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        // clippy::as_conversions: See doc above 1Has a conversation.
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        // clippy::as_conversions: See doc above 1Has a conversation.
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        Ok(Size { height, width })
    }

    pub fn hide_cursor() -> Result<(),Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(),Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn print(message: &str) -> Result<(), Error> {
        Self::queue_command(Print(message))?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T:Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }


}