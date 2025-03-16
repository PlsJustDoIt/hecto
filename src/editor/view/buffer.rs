use std::fs::{read_to_string, File};
use std::io::Error;
use super::line::Line;
use super::Location;
use std::io::Write;
use crate::editor::fileinfo::FileInfo;



#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub dirty: bool, // en gros pour savoir si on a save ou pas
    pub file_info: FileInfo,
}

impl  Buffer {

    /// permet de charger le texte d'un fichier dans le buffer
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(Line::from(value));
        }
        Ok(Self { lines, file_info: FileInfo::from(file_name),dirty: false, })
    }

    /// permet de savoir si le buffer est vide
    pub fn is_empty(&self) -> bool {
        return self.lines.is_empty()
    }

    /// permet de sauvegarder le texte écrit dans le terminal dans un fichier
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(path) = &self.file_info.path {
            let mut file = File::create(path)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    /// retourne le nb de lignes
    pub fn height(&self) -> usize {
        self.lines.len()
    }


    /// permet d'insérer un charactère au niveau du curseur
    pub fn insert_char(&mut self, character: char, at: Location) {
        if at.line_index > self.height() {
            return;
        }
        if at.line_index == self.height() {
            self.lines.push(Line::from(&character.to_string()));
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(character, at.grapheme_index);
            self.dirty = true;
        }
    }

    /// permet d'insérer une ligne
    pub fn insert_newline(&mut self, at: Location) {
        if at.line_index == self.height() {
            self.lines.push(Line::default());
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            let new = line.split(at.grapheme_index);
            self.lines.insert(at.line_index.saturating_add(1), new);
            self.dirty = true;
        }
    }

    /// permet de supprimer un charactère
    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get(at.line_index) {
            if at.grapheme_index >= line.grapheme_count()
                && self.height() > at.line_index.saturating_add(1)
            {
                let next_line = self.lines.remove(at.line_index.saturating_add(1));
                // clippy::indexing_slicing: We checked for existence of this line in the surrounding if statment
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_index].append(&next_line);
                self.dirty = true;
            } else if at.grapheme_index < line.grapheme_count() {
                // clippy::indexing_slicing: We checked for existence of this line in the surrounding if statment
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_index].delete(at.grapheme_index);
                self.dirty = true;
            }
        }
    }

}