use project_root::get_project_root;
use base::language::Language;
use reference::format_lexemes::FormatLexemes;
use reference::get_directory;
use reference::read_lexemes;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = get_project_root()?;
    Language::English.format_lexemes(&root.as_path())?;
    Ok(())
}
