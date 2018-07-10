use holochain_dna::zome::ErrorHandling;

#[derive(Serialize, Deserialize)]
pub struct Zome {
    name: String,
    description: String,
    config: ZomeConfig,
}

#[derive(Serialize, Deserialize)]
pub struct ZomeConfig {
    error_handling: ErrorHandling,
}
