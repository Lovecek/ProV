use std::fs;
use std::path::PathBuf;


pub fn run(path: PathBuf) {
    match fs::read_to_string(&path) {
        Ok(contents) => print!("{}", contents),
        Err(e) => eprintln!("Error reading {:?}: {}", path, e),
    }
}