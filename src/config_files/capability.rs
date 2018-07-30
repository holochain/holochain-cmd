use error::DefaultResult;
use serde_json;
use std::{fs::File, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Capability {}

impl Capability {
    pub fn from_file<T: AsRef<Path>>(path: T) -> DefaultResult<Capability> {
        let file = File::open(&path)?;

        let zome = serde_json::from_reader(file)?;

        Ok(zome)
    }

    pub fn save_as<T: AsRef<Path>>(&self, path: T) -> DefaultResult<()> {
        let file = File::create(&path)?;

        serde_json::to_writer_pretty(file, self)?;

        Ok(())
    }
}
