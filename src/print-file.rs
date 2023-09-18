use std::fs::File;
use std::io::{Read, Result};
use std::path::{Path, PathBuf};

fn read_file(dir: &Path, filename: &str) -> Result<String> {
    let mut full_path = PathBuf::from(dir);
    full_path.push(filename);

    let mut file = File::open(full_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn main() -> Result<()> {
    let dir = Path::new("./");
    let content = read_file(dir, "example.txt")?;
    println!("File content: {}", content);

    Ok(())
}
