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

impl Default for Zome {
    fn default() -> Self {
        Zome {
            name: "zome_name".into(),
            description: "Desciption of zome".into(),
            config: ZomeConfig {
                error_handling: ErrorHandling::ThrowErrors,
            },
        }
    }
}
