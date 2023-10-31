use anyhow::Result;
use economic_indicator_finder::prelude::*;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let speechs = Speech::from_json_file("speech/speech.json")?;
    let finder = Finder::new(Indicator::from_json_file("asset/indicators.json")?)?;

    for speech in speechs {
        let speaker = &speech.speaker;
        let speech = read_to_string(speech.file)?;
        let matches: Vec<_> = finder.matches(speech);
        println!("From the speech of {}: {:#?}", speaker, matches);
    }
    Ok(())
}
