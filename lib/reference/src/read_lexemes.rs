use std::path::Path;
use anyhow::Result;
use crate::get_directory::GetDirectory;
use crate::lexeme_map::LexemeMap;
use crate::read_files::read_files;

pub trait ReadLexemes {
    fn read_lexemes(&self, path: &Path) -> Result<LexemeMap>;
}

impl <T: GetDirectory> ReadLexemes for T {
    fn read_lexemes(&self, root: &Path) -> Result<LexemeMap> {
        let path = root.join(self.get_directory());
        read_files(path.as_path())
    }
}
