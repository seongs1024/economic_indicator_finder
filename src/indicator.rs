use crate::util::deserialize_from_file;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Indicator {
    name: String,
    alternatives: Vec<String>,
}

impl Indicator {
    pub fn from_json_file<P>(path: P) -> Result<Vec<String>>
    where
        P: AsRef<Path>,
    {
        let deserialized: Vec<Self> = deserialize_from_file(path)?;
        let deserialized: BTreeSet<String> = deserialized
            .into_iter()
            .flat_map(|i| i.alternatives.into_iter().chain([i.name]))
            .collect();
        Ok(Vec::from_iter(deserialized))
    }
}
