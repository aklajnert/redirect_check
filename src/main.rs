use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{io, process};

use ansi_term::Color;

use crate::redirect_definition::RedirectDefinition;

mod redirect_definition;

#[tokio::main]
async fn main() {
    let path = get_path();

    let mut records = match read_csv(path) {
        Ok(records) => records,
        Err(error) => {
            eprintln!("Failed to load CSV data: {}", error);
            process::exit(1);
        }
    };
    let records_count = records.len();

    let mut failed_records = vec![];
    let tasks: Vec<_> = records
        .into_iter()
        .map(|mut record| {
            tokio::spawn(async {
                record.resolve().await;
                record
            })
        })
        .collect();
    let mut records = vec![];
    for task in tasks {
        records.push(task.await.unwrap());
    }

    for mut record in records.iter() {
        if record.is_correct() {
            println!("{}: {}", Color::Green.paint("OK"), record);
        } else {
            println!("{}: {}", Color::Red.paint("Fail"), record);
            failed_records.push(record);
        }
    }
    show_summary(records_count, &mut failed_records);
    pause();
}

fn get_path() -> PathBuf {
    loop {
        let mut definitions_path = String::new();
        println!("Provide path to the redirect definitions file:");
        io::stdin().read_line(&mut definitions_path).unwrap();

        // strip whitespaces and quotations (in case if there's a space in the path on Windows)
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

fn show_summary(records_count: usize, failed_records: &mut Vec<&RedirectDefinition>) {
    if failed_records.len() == 0 {
        println!("{}", Color::Green.paint("\nAll redirects are correct."));
    } else {
        println!(
            "\n{}/{} tests failed.\n\n{}\n---------",
            Color::Red.paint(failed_records.len().to_string()),
            Color::Blue.paint(records_count.to_string()),
            Color::Red.paint("Failures:")
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

fn pause() {
    println!("\nPress ENTER to exit...");
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
}
