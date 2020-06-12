use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct RedirectDefinition {
    name: String,
    source: String,
    target: String,
}

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

    let content = content.to_owned();
    let mut reader = csv::Reader::from_reader(content.as_bytes());
    let mut records = vec![];
    for record in reader.records() {
        let record = record?;
        records.push(RedirectDefinition {
            name: record[0].to_owned(),
            source: record[1].to_owned(),
            target: record[2].to_owned(),
        });
    }
    println!("{:?}", records);

    Ok(())
}
