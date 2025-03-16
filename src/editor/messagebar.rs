
use std::{
    io::Error,
    time::{Duration, Instant},
};

const DEFAULT_DURATION: Duration = Duration::new(5, 0);

use super::{
    terminal::{Size, Terminal},
    UIComponent
};

/// message affichée en bas de l'écran
struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message {
    /// Vérifie si le message est expiré
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

/// Barre de message
#[derive(Default)]
pub struct MessageBar {
    current_message: Message,
    needs_redraw: bool,
    cleared_after_expiry: bool, //ensures we can properly hide expired messages
}

impl MessageBar {
    /// Met à jour le message
    pub fn update_message(&mut self, new_message: &str) {
        self.current_message = Message {
            text: new_message.to_string(),
            time: Instant::now(),
        };
        self.cleared_after_expiry = false;
        self.set_needs_redraw(true);
    }
}
/// implémentation de UIComponent pour MessageBar
impl UIComponent for MessageBar {
    /// Met à jour le besoin de redessiner
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    /// Vérifie si le composant a besoin d'être redessiné
    fn needs_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_message.is_expired()) || self.needs_redraw
    }

    /// Met à jour la taille
    fn set_size(&mut self, _: Size) {}

    /// Dessine le composant
    fn draw(&mut self, origin: usize) -> Result<(), Error> {
        if self.current_message.is_expired() {
            self.cleared_after_expiry = true; // Upon expiration, we need to write out "" once to clear the message. To avoid clearing more than necessary, we  keep track of the fact that we've already cleared the expired message once..
        }
        let message = if self.current_message.is_expired() {
            ""
        } else {
            &self.current_message.text
        };

        Terminal::print_row(origin, message)
    }
}