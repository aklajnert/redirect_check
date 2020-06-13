use csv::StringRecord;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fmt, io, process};

#[derive(Debug)]
struct RedirectDefinition {
    name: Option<String>,
    source: String,
    target: String,
}

impl RedirectDefinition {
    fn new(record: StringRecord) -> Result<RedirectDefinition, IncorrectRow> {
        match record.len() {
            2 => Ok(RedirectDefinition {
                name: None,
                source: record[0].to_owned(),
                target: record[1].to_owned(),
            }),
            3 => Ok(RedirectDefinition {
                name: Some(record[0].to_owned()),
                source: record[1].to_owned(),
                target: record[2].to_owned(),
            }),
            _ => Err(IncorrectRow),
        }
    }
}

#[derive(Debug, Clone)]
struct IncorrectRow;

impl fmt::Display for IncorrectRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CSV rows need to have 2 or 3 columns.")
    }
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

    let mut reader = csv::Reader::from_reader(content.as_bytes());
    let mut records = vec![];
    for record in reader.records() {
        let record = record?;
        let record_object = {
            match RedirectDefinition::new(record) {
                Ok(record) => record,
                Err(IncorrectRow) => {
                    eprintln!("{}", IncorrectRow);
                    process::exit(1);
                }
            }
        };
        records.push(record_object);
    }
    println!("{:?}", records);

    Ok(())
}
