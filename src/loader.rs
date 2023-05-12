use crate::incremental::State;
use anyhow::{anyhow, Result};
use std::{fs::File, io::BufReader, path::Path};

pub fn load(file_path: &Path) -> Result<State> {
    let ext = match match file_path.extension() {
        Some(ext) => ext.to_str(),
        None => return Err(anyhow!("Unknown file extension")),
    } {
        Some(ext) => ext,
        None => return Err(anyhow!("Unable to parse file extension")),
    };

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    if ext == "yaml" || ext == "yml" {
        match serde_yaml::from_reader(reader) {
            Ok(state) => Ok(state),
            Err(err) => Err(anyhow!("{}", err.to_string())),
        }
    } else if ext == "json" {
        match serde_json::from_reader(reader) {
            Ok(state) => Ok(state),
            Err(err) => Err(anyhow!("{}", err.to_string())),
        }
    } else {
        Err(anyhow!("Unsupported extension {}", ext))
    }
}
