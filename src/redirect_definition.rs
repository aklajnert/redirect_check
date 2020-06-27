use csv::StringRecord;
use std::error::Error;
use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct RedirectDefinition {
    pub name: Option<String>,
    pub source: String,
    pub target: String,
    pub resolved_url: Option<String>,
}

impl RedirectDefinition {
    pub fn new(record: StringRecord) -> Result<RedirectDefinition, IncorrectRow> {
        match record.len() {
            2 => Ok(RedirectDefinition {
                source: record[0].to_string(),
                target: record[1].trim_end_matches('/').to_string(),
                ..Default::default()
            }),
            3 => Ok(RedirectDefinition {
                name: Some(record[0].to_string()),
                source: record[1].to_string(),
                target: record[2].trim_end_matches('/').to_string(),
                ..Default::default()
            }),
            _ => Err(IncorrectRow),
        }
    }

    pub fn resolve(&mut self) {
        let query_result = self.query();
        if query_result.is_ok() {
            self.resolved_url = Some(query_result.unwrap().trim_end_matches('/').to_string())
        }
    }

    pub fn is_correct(&self) -> bool {
        match &self.resolved_url {
            Some(url) => url.ends_with(&self.target),
            None => false,
        }
    }

    fn query(&self) -> Result<String, Box<dyn Error>> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                (KHTML, like Gecko) Chrome/83.0.4103.97 Safari/537.36",
            )
            .build()?;
        let resp = client.get(&self.source.to_string()).send()?;
        let status = String::from(resp.status().as_str());
        if status != "200" {
            return Err(Box::new(HttpError { error: status }));
        };
        Ok(String::from(resp.url().as_str()))
    }
}

#[derive(Debug, Clone)]
pub struct IncorrectRow;

impl fmt::Display for IncorrectRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CSV rows need to have 2 or 3 columns.")
    }
}
impl Error for IncorrectRow {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    error: String,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP Error: {}", self.error)
    }
}
impl Error for HttpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}
