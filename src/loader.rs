use crate::incremental::State;
use anyhow::{anyhow, Result};
use std::{fs::File, io::BufReader, path::Path};

pub fn load(file_path: &Path) -> Result<State> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);
    match serde_yaml::from_reader(reader) {
        Ok(state) => Ok(state),
        Err(err) => Err(anyhow!("{}", err.to_string())),
    }
}
