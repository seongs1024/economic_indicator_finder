use anyhow::Result;
use regex::RegexSet;

pub struct Finder {
    indicators: Vec<String>,
    regex_set: RegexSet,
}

impl Finder {
    pub fn new(indicators: Vec<String>) -> Result<Self> {
        let regex_query: Vec<String> = indicators
            .iter()
            .map(|ref indicator| format!(r"(?i)\b{}\b", indicator))
            .collect();

        let regex_set = RegexSet::new(&regex_query)?;

        Ok(Self {
            indicators,
            regex_set,
        })
    }

    pub fn matches<S1>(&self, sentence: S1) -> Vec<String>
    where
        S1: AsRef<str>,
    {
        self.regex_set
            .matches(sentence.as_ref())
            .into_iter()
            .map(|i| self.indicators[i].clone())
            .collect()
    }
}
