use anyhow::Result;
use regex::RegexSet;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs::{File, read_to_string};
use std::io::{BufWriter, BufReader};
use std::collections::BTreeSet;

#[derive(Serialize, Deserialize, Debug)]
struct Indicator {
    name: String,
    alternatives: Vec<String>,
}

#[derive(Deserialize)]
struct Speech {
    speaker: String,
    file: PathBuf,
}

fn main() -> Result<()> {
    let speechs = {
        let path = PathBuf::from("speech/speech.json");
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let deserialized: Vec<Speech> = serde_json::from_reader(&mut reader)?;
        deserialized
    };

    let path = PathBuf::from("asset/indicators.json");
    let indicators = {
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let deserialized: Vec<Indicator> = serde_json::from_reader(&mut reader)?;
        let deserialized: BTreeSet<String> = deserialized
            .into_iter()
            .flat_map(|i| {
                i.alternatives.into_iter().chain([i.name])
            })
            .collect();
        Vec::from_iter(deserialized)
    };

    let regex_query: Vec<String> = indicators
        .iter()
        .map(|indicator| format!(r"(?i)\b{}\b", indicator))
        .collect();

    let regex_set = RegexSet::new(&regex_query)?;

    for speech in speechs {
        let speaker = &speech.speaker;
        let speech = read_to_string(speech.file)?;
        let matches: Vec<_> = regex_set
        .matches(&speech)
        .into_iter()
        .map(|i| &indicators[i])
        .collect();
        println!("From the speech of {}: {:#?}", speaker, matches);
    }
    Ok(())
}
