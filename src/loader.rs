use std::{path::Path, io::BufReader, fs::File};
use anyhow::{anyhow, Result};
use crate::incremental::State;

pub fn load(file_path: &Path) -> Result<State> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);
    match serde_yaml::from_reader(reader) {
        Ok(state) => Ok(state),
        Err(err) => Err(anyhow!("{}", err.to_string())),
    }
}
