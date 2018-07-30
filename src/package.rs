use config_files::{App as AppConfig, Author, Dht};
use holochain_dna::zome::Zome;
use semver::Version;
use serde_json;
use std::{collections::HashMap, fs::File, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub description: String,
    pub authors: Vec<Author>,
    pub version: Version,
    pub dht: Dht,
    pub properties: HashMap<String, String>,
    pub zomes: Vec<Zome>,
}

impl Package {
    pub fn from_app_config(base: AppConfig) -> Package {
        let AppConfig {
            name,
            description,
            authors,
            version,
            dht,
            properties,
        } = base;

        Package {
            name,
            description,
            authors,
            version,
            dht,
            properties,
            zomes: Vec::new(),
        }
    }

    pub fn save_as<T: AsRef<Path>>(&self, path: T) {
        let path = path.as_ref().to_path_buf();

        let file = File::create(path).unwrap();

        serde_json::to_writer_pretty(file, self).unwrap();
    }
}
