use std::path::Path;
use base::language::Language;
use anyhow::Result;
use crate::lexeme_map::{LexemeMap, EMPTY_LEXEMES};
use crate::read_files::read_files;

pub trait ReadLexemes {
    fn read_lexemes(&self, path: &Path) -> Result<LexemeMap>;
}

impl ReadLexemes for Language {
    fn read_lexemes(&self, root: &Path) -> Result<LexemeMap> {
        match self {
            Language::English => {
                let path = root.join("data/en");
                read_files(path.as_path())
            },
            Language::Spanish => {
                let path = root.join("data/es");
                read_files(path.as_path())
            },
            Language::French => {
                let path = root.join("data/fr");
                read_files(path.as_path())
            },
            Language::Italian => {
                let path = root.join("data/it");
                read_files(path.as_path())
            },
            _ => Ok(EMPTY_LEXEMES.clone()),
        }
    }
}