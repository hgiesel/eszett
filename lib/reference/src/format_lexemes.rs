use std::path::Path;
use anyhow::Result;
use crate::read_lexemes::ReadLexemes;
use crate::format_files::format_files;

pub trait FormatLexemes : ReadLexemes {
    fn format_lexemes(&self, path: &Path) -> Result<()>;
}

impl <T: ReadLexemes> FormatLexemes for T {
    fn format_lexemes(&self, path: &Path) -> Result<()> {
        let map = self.read_lexemes(path)?;
        format_files(path, map)?;
        Ok(())
    }
}
