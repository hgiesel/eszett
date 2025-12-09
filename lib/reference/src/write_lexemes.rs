use std::path::PathBuf;
use base::language::Language;
use anyhow::Result;
use crate::lexeme_map::{LexemeMap, EMPTY_LEXEMES};
use crate::read_files::read_files;

pub trait FormatLexemes : ReadLexemes {
    fn format_lexemes(self, path: &mut PathBuf) -> Result<LexemeMap>;
}

impl WriteLexemes for Language {
    fn read_lexemes(self, path: &mut PathBuf) -> Result<()> {
        let map = self.read_lexemes(path)?;
        match self {
            Language::English => {

            },
            _ => Ok(()),
        }
    }
}