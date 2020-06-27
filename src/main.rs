use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{io, process};

mod redirect_definition;
use crate::redirect_definition::RedirectDefinition;

fn main() {
    let path = get_path();
    println!("Provided path: {:?}", path);

    let records = match read_csv(path) {
        Ok(records) => records,
        Err(error) => {
            eprintln!("Failed to load CSV data: {}", error);
            process::exit(1);
        }
    };
    let records_count = records.len();

    let mut failed_records = vec![];
    for mut record in records {
        record.resolve();
        if record.is_correct() {
            println!("OK: {}", record);
        } else {
            println!("Fail: {}", record);
            failed_records.push(record);
        }
    }
    if failed_records.len() == 0 {
        println!("\nAll redirects are correct.");
    } else {
        println!(
            "\n{}/{} tests failed.\nFailures:\n---------",
            failed_records.len(),
            records_count
        );
        for failure in failed_records {
            let resolved = match &failure.resolved_url {
                Some(result) => result.to_string(),
                None => "Failed to resolve".to_string(),
            };
            println!(
                "\n{}\n\tExpected: {}\n\tGot: {}",
                failure, failure.source, resolved
            );
        }
    }
}

fn get_path() -> PathBuf {
    loop {
        let mut definitions_path = String::new();
        println!("Provide path to the redirect definitions file:");
        io::stdin().read_line(&mut definitions_path).unwrap();

        // strip whitespaces and quotations (from drag&drop on Windows)
        let definitions_path = definitions_path.trim().trim_matches('"');

        let path = Path::new(&definitions_path);
        if !path.exists() {
            eprintln!("File doesn't exists.");
        } else {
            return path.to_owned();
        }
    }
}

fn read_csv(path: PathBuf) -> std::io::Result<Vec<RedirectDefinition>> {
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
        if record_object.source.len() > 0 && record_object.target.len() > 0 {
            records.push(record_object);
        }
    }

    Ok(records)
}
