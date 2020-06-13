use csv::StringRecord;
use std::fmt;

#[derive(Debug)]
pub struct RedirectDefinition {
    name: Option<String>,
    source: String,
    target: String,
}

impl RedirectDefinition {
    pub fn new(record: StringRecord) -> Result<RedirectDefinition, IncorrectRow> {
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
pub struct IncorrectRow;

impl fmt::Display for IncorrectRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CSV rows need to have 2 or 3 columns.")
    }
}
