use config::Dht;
use semver::Version;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Dna {
    name: String,
    description: String,
    version: Version,
    uuid: String,
    dna_spec_version: Version,
    dht: Dht,
    properties: HashMap<String, String>,
}
