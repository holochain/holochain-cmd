use config_files::Dht;
use semver::Version;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub description: String,
    pub authors: Vec<Author>,
    pub version: Version,
    pub dht: Dht,
    pub properties: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    indentifier: String,
    public_key_source: String,
    signature: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            name: "Holochain App Name".into(),
            description: "A Holochain app".into(),
            version: Version::new(0, 1, 0),
            authors: vec![Author {
                indentifier: "Author Name <author@name.com>".into(),
                public_key_source: "http://eric.harris-braun.com/pk".into(),
                signature: "".into(),
            }],
            dht: Dht {},
            properties: HashMap::new(),
        }
    }
}
