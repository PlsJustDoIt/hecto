
/// Indique le status du document en train d'être édité
#[derive(Default, Eq, PartialEq, Debug)]
pub struct DocumentStatus {
    pub total_lines: usize,
    pub current_line_index: usize,
    pub is_modified: bool,
    pub file_name: String,
}

impl DocumentStatus {
    /// status modifié ajouté lorsque le doc est modifié
    pub fn modified_indicator_to_string(&self) -> String {
        if self.is_modified {
            String::from("(modified)")
        } else {
            String::new()
        }
    }
    pub fn line_count_to_string(&self) -> String {
        format!("{} lines", self.total_lines)
    }
    pub fn position_indicator_to_string(&self) -> String {
        format!(
            "{}/{}",
            self.current_line_index.saturating_add(1),
            self.total_lines
        )
    }
}