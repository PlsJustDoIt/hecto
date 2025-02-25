use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::convert::TryFrom;

use super::terminal::Size;

#[derive(Clone, Copy)]
pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Left,
    Right,
    Down,
}
#[derive(Clone, Copy)]
pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
    PrintChar(char),
    Nothing,
    Delete,
    Backspace,
    Tab,
    Enter,
    Save,

}

impl TryFrom<Event> for EditorCommand {
    type Error = String;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                (KeyCode::Backspace, _) => Ok(Self::Backspace),
                (KeyCode::Delete, _) => Ok(Self::Delete),
                (KeyCode::Tab,_) => Ok(Self::Tab),
                (KeyCode::Enter,_) => Ok(Self::Enter),
                (KeyCode::Char(char),  KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(Self::PrintChar(char)),
                (KeyCode::Char('s'),KeyModifiers::CONTROL) => {
                    Ok(Self::Save)
                },
                _ => Ok(Self::Nothing),
                
            },
            Event::Resize(width_u16, height_u16) => {
                // clippy::as_conversions: Will run into problems for rare edge case systems where usize < u16
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                // clippy::as_conversions: Will run into problems for rare edge case systems where usize < u16
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                Ok(Self::Resize(Size { height, width }))
            }
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}