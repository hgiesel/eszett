use std::path::Path;
use base::language::Language;

pub trait GetDirectory {
    fn get_directory(&self) -> &'static Path;
}

impl GetDirectory for Language {
    fn get_directory(&self) -> &'static Path {
        match self {
            Language::English => Path::new("data/en"),
            Language::Spanish => Path::new("data/es"),
            Language::French => Path::new("data/fr"),
            Language::Italian => Path::new("data/it"),
            _ => Path::new("."),
        }
    }
}
