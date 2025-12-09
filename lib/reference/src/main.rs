use project_root::get_project_root;
use base::language::Language;
use reference::read_lexemes::ReadLexemes;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut root = get_project_root()?;
    let x = Language::English.read_lexemes(&mut root)?;
    println!("{:#?}", x);
    println!("{}", x);
    Ok(())
}