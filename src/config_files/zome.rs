use error::DefaultResult;
use holochain_dna::zome::ErrorHandling;
use serde_json;
use std::{fs::File, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Zome {
    name: String,
    description: Option<String>,
    config: ZomeConfig,
}

impl Zome {
    pub fn from_file<T: AsRef<Path>>(path: T) -> DefaultResult<Zome> {
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
pub struct ZomeConfig {
    error_handling: ErrorHandling,
}

impl Default for Zome {
    fn default() -> Self {
        Zome {
            name: "zome_name".into(),
            description: "Desciption of zome".to_string().into(),
            config: ZomeConfig {
                error_handling: ErrorHandling::ThrowErrors,
            },
        }
    }
}
