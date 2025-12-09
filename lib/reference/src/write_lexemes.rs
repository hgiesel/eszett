use std::path::Path;
use base::language::Language;
use anyhow::Result;
use crate::lexeme_map::{LexemeMap, EMPTY_LEXEMES};
use crate::read_files::read_files;
use crate::read_lexemes::ReadLexemes;
use crate::format_files::format_files;

pub trait FormatLexemes : ReadLexemes {
    fn format_lexemes(&self, path: &mut Path) -> Result<()>;
}

impl FormatLexemes for Language {
    fn format_lexemes(&self, path: &mut Path) -> Result<()> {
        let map = self.read_lexemes(path)?;
        match self {
            Language::English => {
                format_files(path, map)?;
                Ok(())
            },
            _ => Ok(()),
        }
    }
}