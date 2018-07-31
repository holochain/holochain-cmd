use config_files::{App as AppConfig, Author, Dht};
use holochain_dna::Dna;
use semver::Version;
use serde_json::{self, Value};
use std::{fs::File, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub spec_version: Version,
    pub authors: Vec<Author>,
    pub dna: Dna,
    pub properties: Value,
    pub dht: Dht,
}

impl Package {
    pub fn from_app_config(base: AppConfig) -> Package {
        let AppConfig {
            authors,
            dht,
            properties, // use properties in the future
            ..
        } = base;

        Package {
            authors,
            spec_version: (2, 0, 0).into(),
            dht,
            properties,
            dna: Default::default(),
        }
    }

    pub fn save_as<T: AsRef<Path>>(&self, path: T) {
        let path = path.as_ref().to_path_buf();

        let file = File::create(path).unwrap();

        serde_json::to_writer_pretty(file, self).unwrap();
    }
}
