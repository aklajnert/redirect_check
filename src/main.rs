use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{io, process};

mod redirect_definition;
use crate::redirect_definition::RedirectDefinition;

fn main() {
    let path = get_path();
    println!("Provided path: {:?}", path);

    read_csv(path).unwrap();
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

fn read_csv(path: PathBuf) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut reader = csv::Reader::from_reader(content.as_bytes());
    let mut records = vec![];
    for record in reader.records() {
        let record = record?;
        let record_object = match RedirectDefinition::new(record) {
            Ok(record) => record,
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            }
        };
        records.push(record_object);
    }
    println!("{:?}", records);

    Ok(())
}
