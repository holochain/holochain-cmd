use config::Dht;
use semver::Version;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct App {
    name: String,
    description: String,
    authors: Vec<Author>,
    version: Version,
    dht: Dht,
    properties: HashMap<String, String>,
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
            authors: vec![Author {
                indentifier: "Author Name <author@name.com>".into(),
                public_key_source: "http://eric.harris-braun.com/pk".into(),
                signature: "".into(),
            }],
            version: Version::new(0, 1, 0),
            dht: Dht {},
            properties: HashMap::new(),
        }
    }
}
