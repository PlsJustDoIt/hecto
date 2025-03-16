use crossterm::style::{Attribute,Print};
use crossterm::{ queue,Command};
use std::io::stdout;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,EnableLineWrap,DisableLineWrap,SetTitle,
    LeaveAlternateScreen,
};
use super::{Position, Size};
use std::io::Write;
use std::io::Error;


pub struct Terminal {

}

impl Terminal {

    /// Termine l'application
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::enable_line_wrap()?;
        Self::show_cursor()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    /// Initialise le terminal
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::disable_line_wrap()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{x:0, y:0})?;
        Self::execute()?;
        Ok(())
    }

    /// Désactive le retour à la ligne
    pub fn disable_line_wrap() -> Result<(), Error> {
        Self::queue_command(DisableLineWrap)?;
        Ok(())
    }

    /// Active le retour à la ligne
    pub fn enable_line_wrap() -> Result<(), Error> {
        Self::queue_command(EnableLineWrap)?;
        Ok(())
    }

    /// Définit le titre de la fenêtre
    pub fn set_title(title: &str) -> Result<(), Error> {
        Self::queue_command(SetTitle(title))?;
        Ok(())
    }

    /// Imprime une ligne de texte inversée
    pub fn print_inverted_row(row: usize, line_text: &str) -> Result<(), Error> {
        let width = Self::size()?.width;
        Self::print_row(
            row,
            &format!(
                "{}{:width$.width$}{}",
                Attribute::Reverse,
                line_text,
                Attribute::Reset
            ),
        )
    }

    /// rentre dans le mode d'écran alternatif
    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    /// quitte le mode d'écran alternatif
    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    /// Imprime une ligne de texte
    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_cursor_to(Position { x:0, y: row })?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }

    /// Efface l'écran
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    /// Efface la ligne actuelle
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }


    /// Déplace le curseur à une position donnée
    pub fn move_cursor_to(position:Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.x as u16, position.y as u16))?;
        
        Ok(())
    }

    /// Récupère la taille du terminal
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

    /// Cache le curseur
    pub fn hide_cursor() -> Result<(),Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    /// Affiche le curseur
    pub fn show_cursor() -> Result<(),Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    /// Imprime un message dans le terminal
    pub fn print(message: &str) -> Result<(), Error> {
        Self::queue_command(Print(message))?;
        Ok(())
    }

    /// Exécute les commandes en attente
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    /// Ajoute une commande dans la queue
    fn queue_command<T:Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }


}