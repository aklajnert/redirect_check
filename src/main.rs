use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let path = get_path();
    println!("Provided path: {:?}", path);
}

fn get_path() -> PathBuf {
    loop {
        let mut definitions_path = String::new();
        println!("Provide path to the redirect definitions file:");
        io::stdin().read_line(&mut definitions_path).unwrap();
        let definitions_path = definitions_path.trim();
        let path = Path::new(&definitions_path);
        if !path.exists() {
            eprintln!("File doesn't exists.");
        } else {
            return path.to_owned();
        }
    }
}
