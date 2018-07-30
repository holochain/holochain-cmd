use error::DefaultResult;
use holochain_dna::zome::entry_types::Sharing;
use serde_json;
use std::{fs::File, path::Path};

#[derive(Serialize, Deserialize)]
pub struct EntryType {
    pub name: String,
    pub description: String,
    pub sharing: Sharing,
    pub links_to: Vec<Link>,
}

impl EntryType {
    pub fn from_file<T: AsRef<Path>>(path: T) -> DefaultResult<EntryType> {
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

#[derive(Serialize, Deserialize)]
pub struct Link {
    target_type: String,
    tag: String,
}
