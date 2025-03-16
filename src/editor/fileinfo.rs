use std::{
    fmt::{self, Display},
    path::PathBuf,
};

/// structure pour avoir des informations par rapport à un fichier
#[derive(Default, Debug, Clone)]
pub struct FileInfo {
    pub path: Option<PathBuf>,
}

impl FileInfo {
    /// permet de créer une instance de FileInfo à partir d'un nom de fichier
    pub fn from(file_name: &str) -> Self {
        Self {
            path: Some(PathBuf::from(file_name)),
        }
    }
}

/// permet d'afficher le nom du fichier
impl Display for FileInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self
            .path
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No Name]");
        write!(formatter, "{name}")
    }
}