
use crate::terminal::{Terminal,Position}; // If you want to use Terminal directly
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};


pub struct Editor {
    should_quit:bool,
}

impl Editor {
    pub const fn default() -> Self {            
        Self { should_quit: false }            
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
            
        }   
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },

                Char('a') => Terminal::print("le perou est en danger").unwrap(),
                
                _ => (),
            }

        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
                    Terminal::clear_screen()?;
                    Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position{x:0, y:0})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;

        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        
        let  rows = Terminal::size()?.height;
        for n in 0..rows {
            // Terminal::move_cursor_to(0,n).unwrap(); // ma tech à la base
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if n + 1 < rows {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())

    }

}