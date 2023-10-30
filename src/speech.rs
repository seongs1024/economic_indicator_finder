use crate::util::deserialize_from_file;
use anyhow::Result;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct Speech {
    pub speaker: String,
    pub file: PathBuf,
}

impl Speech {
    pub fn from_json_file<P>(path: P) -> Result<Vec<Self>>
    where
        P: AsRef<Path>,
    {
        Ok(deserialize_from_file(path)?)
    }
}
