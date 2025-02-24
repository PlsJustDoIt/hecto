
// use crate::terminal::Terminal; // If you want to use Terminal directly
// use crate::view::View;
mod terminal;
mod view;
mod editorcommand;
use view::View;
use terminal::Terminal;
use editorcommand::EditorCommand;
use crossterm::event::{read, Event, KeyEvent, KeyEventKind::{self},};
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};

pub struct Editor {
    should_quit:bool,
    view:View
}

impl Editor {

    pub fn new() -> Result<Self,Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move | panic_info | { // closure
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }
        Ok(Self {
            should_quit: false,
            view,
        })
    }
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                #[cfg(debug_assertions)]
                Err(err) => {
                    panic!("Could not read event: {err:?}");
                }
            }
        }
    }

    // fn move_point(&mut self, key_code:KeyCode) {
    //     let Location{mut x, mut y} = self.location;
    //     let Size { height, width } = Terminal::size().unwrap_or_default();
    //     match key_code {
    //         KeyCode::Up => {
    //             y = y.saturating_sub(1);
    //         },
    //         KeyCode::Down => {
    //             y = min(height.saturating_sub(1), y.saturating_add(1));
    //         },
    //         KeyCode::Left => {
    //             x = x.saturating_sub(1);
    //         },

    //         KeyCode::Right => {
    //             x = min(width.saturating_sub(1), x.saturating_add(1));
    //         },

    //         KeyCode::PageUp => {
    //             y = 0;
    //         }
    //         KeyCode::PageDown => {
    //             y = height.saturating_sub(1);
    //         }
    //         KeyCode::Home => {
    //             x = 0;
    //         }
    //         KeyCode::End => {
    //             x = width.saturating_sub(1);
    //         },

    //         _ => (),
    //     }

    //     self.location = Location { x, y };
    // }


    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind,..}) => kind == &KeyEventKind::Press,
            Event::Resize(_,_) => true,
            _ => false,

        };

        if should_process {
            match EditorCommand::try_from(event) {
                Ok(command) => {
                    if matches!(command,EditorCommand::Quit) {
                        self.should_quit = true;

                    } else {
                        self.view.handle_command(command);
                    }
                }
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not handle command: {err}");
                    }
                }
            }
        } else {
            #[cfg(debug_assertions)]
            {
                panic!("Received and discarded unsupported or non-press event.");
            }
        }
    }

    

    
    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        self.view.render();
        let _ = Terminal::move_cursor_to(self.view.get_position());
        let _ = Terminal::show_cursor();
        let _ = Terminal::execute();
    }

    

}


impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}