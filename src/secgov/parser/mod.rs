pub mod index;
pub mod atomfilings;

use std::error::Error;

use regex::Regex;

use self::atomfilings::XMLFiling;
use super::models::FilingTransaction;

pub struct FilingDoc;

impl FilingDoc {
    pub fn new(url: &str, content: &str) -> Result<Vec<FilingTransaction>, Box<dyn Error>>{
        let mut filing = XMLFiling::new(url);
        let content = Self::extract_xml(content);

        filing
            .extract_transactions(
                Self::extract_xml(&content)
                    .as_str()
            )
    }

    fn extract_xml(input: &str) -> String {
        let pattern: Regex =
            Regex::new(r#"<\?xml version="1\.0"\?>[\W\S]*</ownershipDocument>"#).unwrap();

        let result = pattern
            .captures(input)
            .and_then(|cap| {
                cap.iter()
                    .next()
                    .expect("Failed to parse XML")
                    .map(|m| m.as_str())
            })
            .expect("XML regex match failed");

        let result = result
            .replace("<ownershipDocument>", "<ownershipDocument xmlns=\"\">");

        result
    }
}