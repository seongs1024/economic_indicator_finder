use anyhow::Result;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn deserialize_from_file<P, T>(path: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    Ok(serde_json::from_reader(&mut reader)?)
}
