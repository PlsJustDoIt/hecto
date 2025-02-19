use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {

}

impl Editor {
    pub fn default() -> Self {            
        Editor{}            
    }
    pub fn run(&self){
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?} \r", event);
                    match (event.code) {
                        Char(c) => {
                            // ok
                            if c == 'q' {
                                break;
                            }
                        },
                        _ => (),   
                    }   
                },
                Err(err) => println!("broken : {}", err),
                _ => () // anything else
            }
        }
        disable_raw_mode().unwrap();
    }
}