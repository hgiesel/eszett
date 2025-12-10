use project_root::get_project_root;
use base::language::Language;
use reference::format_lexemes::FormatLexemes;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut root = get_project_root()?;

    Language::English.format_lexemes(&mut root)?;
    Ok(())
}