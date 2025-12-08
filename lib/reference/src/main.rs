use reference::read_files;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x= read_files()?;
    println!("{:?}", x);
    Ok(())
}