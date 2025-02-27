use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub fn load_xml(path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
