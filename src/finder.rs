
use anyhow::Result;
use regex::Regex;

#[cfg(feature = "slow_finder")]
use regex::RegexSet;

use std::collections::BTreeSet;

pub trait Matches {
    fn matches<S1>(&self, sentence: S1) -> Vec<String>
    where
    S1: AsRef<str>;
}

pub struct Finder {
    #[allow(dead_code)]
    indicators: Vec<String>,
    regex: Regex,
}

impl Finder {
    pub fn new(indicators: Vec<String>) -> Result<Self> {
        let joined = indicators
            .iter()
            .map(|i| format!(r"\b{}\b", i))
            .collect::<Vec<_>>()
            .join("|");
        let indicator_pattern = format!("(?i)({})", joined);

        let regex = Regex::new(&indicator_pattern)?;

        Ok(Self {
            indicators,
            regex,
        })
    }
}

impl Matches for Finder {
    fn matches<S1>(&self, sentence: S1) -> Vec<String>
    where
        S1: AsRef<str>,
    {
        let indicators: BTreeSet<_> = self.regex
            .captures_iter(sentence.as_ref())
            .map(|c| c.extract())
            .map(|(_, [indicator])| indicator.to_lowercase())
            .collect();
        Vec::from_iter(indicators)
    }
}

#[cfg(feature = "slow_finder")]
pub struct SlowFinder {
    indicators: Vec<String>,
    regex_set: RegexSet,
}

#[cfg(feature = "slow_finder")]
impl SlowFinder {
    #[allow(dead_code)]
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
}

#[cfg(feature = "slow_finder")]
impl Matches for SlowFinder {
    fn matches<S1>(&self, sentence: S1) -> Vec<String>
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
