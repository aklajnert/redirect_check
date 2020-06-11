use std::io;
use std::path::Path;

fn main() {
    loop {
        println!("Provide path to the redirect definitions file:");
        let mut definitions_path = String::new();
        io::stdin().read_line(&mut definitions_path).unwrap();
        if !Path::new(&definitions_path.trim()).exists() {
            eprintln!("File doesn't exists.");
        } else {
            break;
        }
    }
}
