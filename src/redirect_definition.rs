use std::fmt;

use csv::StringRecord;

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

    pub fn check_redirect(&self) -> String {
        let response = match self.query() {
            Ok(status) => status,
            Err(_) => String::from(""),
        };

        match response.as_str() {
            "200" => String::from(format!("OK: {}", &self.source)),
            _ => String::from(format!("Fail: {}", &self.source)),
        }
    }

    fn query(&self) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                (KHTML, like Gecko) Chrome/83.0.4103.97 Safari/537.36",
            )
            .build()?;
        let resp = client.get(&self.source.to_owned()).send()?;
        // println!("{:#?}", resp);
        Ok(String::from(resp.status().as_str()))
    }
}

#[derive(Debug, Clone)]
pub struct IncorrectRow;

impl fmt::Display for IncorrectRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CSV rows need to have 2 or 3 columns.")
    }
}
