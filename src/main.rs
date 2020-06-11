use std::io;
use std::path::Path;

fn main() {
    let mut definitions_path = String::new();
    get_path(&mut definitions_path);
}

fn get_path(mut definitions_path: &mut String) {
    loop {
        println!("Provide path to the redirect definitions file:");
        io::stdin().read_line(&mut definitions_path).unwrap();
        if !Path::new(&definitions_path.trim()).exists() {
            eprintln!("File doesn't exists.");
        } else {
            break;
        }
    }
}
