
// use crate::terminal::Terminal; // If you want to use Terminal directly
// use crate::view::View;
mod messagebar;
mod uicomponents;
use uicomponents::UIComponent;
mod documentstatus;
use documentstatus::DocumentStatus;
use self::{messagebar::MessageBar, terminal::Size};
mod fileinfo;
use fileinfo::FileInfo;
mod command;
mod terminal;
mod view;
mod statusbar;
use statusbar::StatusBar;
use view::View;
use terminal::Terminal;
use crossterm::event::{read, Event, KeyEvent, KeyEventKind::{self},};
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
use command::{
    Command::{self,Edit,Move,System},
    System::{Quit,Resize,Save},
};

const QUIT_TIMES: u8 = 3;

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    should_quit:bool,
    view:View,
    status_bar: StatusBar,
    title:String,
    message_bar: MessageBar,
    terminal_size: Size,
    quit_times: u8,
}

impl Editor {
    /// Crée un nouvel éditeur
    pub fn new() -> Result<Self,Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move | panic_info | { // closure
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.resize(size);
        editor
        .message_bar
        .update_message("HELP: Ctrl-S = save | Ctrl-Q = quit");
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            editor.view.load(file_name);
        }
        editor.refresh_status();
        Ok(editor)
    }

    /// Redimensionne l'éditeur
    fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });
        self.message_bar.resize(Size {
            height: 1,
            width: size.width,
        });
        self.status_bar.resize(Size {
            height: 1,
            width: size.width,
        });
    }

    /// Rafraîchit le status
    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name);
        self.status_bar.update_status(status);

        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
        }
    }

    /// Exécute l'éditeur
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

            let status = self.view.get_status();
            self.status_bar.update_status(status);
        }
    }

    /// Évalue un événement (touche pressée ou redimensionnement)
    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind,..}) => kind == &KeyEventKind::Press,
            Event::Resize(_,_) => true,
            _ => false,

        };

        if should_process {
            if let Ok(command) = Command::try_from(event) {
                self.process_command(command);
            }
        }
    }
    
    /// Rafraîchit l'écran
    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }
        let _ = Terminal::hide_cursor();
        self.message_bar
        .render(self.terminal_size.height.saturating_sub(1));
        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }
        if self.terminal_size.height > 2 {
            self.view.render(0);
        }
        let _ = Terminal::move_cursor_to(self.view.caret_position());
        let _ = Terminal::show_cursor();
        let _ = Terminal::execute();
    }

    /// Traite une commande
    fn process_command(&mut self, command: Command) {
        match command {
            System(Quit) => self.handle_quit(),
            System(Resize(size)) => self.resize(size),
            _ => self.reset_quit_times(), // Reset quit times for all other commands
        }

        match command {
            System(Quit | Resize(_)) => {} // already handled above 1Has a conversation.
            System(Save) => self.handle_save(),
            Edit(edit_command) => self.view.handle_edit_command(edit_command),
            Move(move_command) => self.view.handle_move_command(move_command),
        }

        
    }
    /// Gère la commande de quitter
    // clippy::arithmetic_side_effects: quit_times is guaranteed to be between 0 and QUIT_TIMES
    #[allow(clippy::arithmetic_side_effects)]
    fn handle_quit(&mut self) {
        if !self.view.get_status().is_modified || self.quit_times + 1 == QUIT_TIMES {
            self.should_quit = true;
        } else if self.view.get_status().is_modified {
            self.message_bar.update_message(&format!(
                "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                QUIT_TIMES - self.quit_times - 1
            ));

            self.quit_times += 1;
        }
    }

    /// Réinitialise le nombre de tentatives de quitter
    fn reset_quit_times(&mut self) {
        if self.quit_times > 0 {
            self.quit_times = 0;
            self.message_bar.update_message("");
        }
    }

    /// affiche le message de sauvegarde
    fn handle_save(&mut self) {
        if self.view.save().is_ok() {
            self.message_bar.update_message("File saved successfully.");
        } else {
            self.message_bar.update_message("Error writing file!");
        }
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