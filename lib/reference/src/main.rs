use project_root::get_project_root;
use reference::read_files;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut root = get_project_root()?;
    let x= read_files(&mut root)?;
    println!("{:?}", x);
    Ok(())
}