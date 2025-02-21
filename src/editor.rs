
use crate::terminal::{Terminal,Position,Size}; // If you want to use Terminal directly
use crate::view::View;
use crossterm::event::{read, Event, Event::Key, KeyEventKind::Press, KeyCode, KeyEvent, KeyModifiers};
use std::env;
use std::io::Error;
use core::cmp::min;




#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}


#[derive(Default)]
pub struct Editor {
    should_quit:bool,
    location:Location,
    view:View
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();

        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            self.view.load(file_name);
        }
        // Terminal::p
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn move_point(&mut self, key_code:KeyCode) -> Result<(),Error> {
        let Location{mut x, mut y} = self.location;
        let Size { height, width } = Terminal::size()?;
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            },
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            },
            KeyCode::Left => {
                x = x.saturating_sub(1);
            },

            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            },

            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            },

            _ => (),
        }

        self.location = Location { x, y };
        Ok(())
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
            
        }   
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code, modifiers,  kind: Press, ..
        }) = event
        {
            match code {

                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;
                }
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },

                KeyCode::Char('a') => Terminal::print("le perou est en danger").unwrap(),
                
                _ => (),
            }
            

        }
        Ok(())
    }

    

    
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_cursor_to(Position {
                x: self.location.x,
                y: self.location.y,
            })?;
            
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;

        Ok(())
    }

    

}