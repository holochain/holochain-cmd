#[derive(Serialize, Deserialize)]
pub struct Zome {
    name: String,
    description: String,
    config: SubConfig,
}

#[derive(Serialize, Deserialize)]
pub struct SubConfig {
    error_handling: ErrorHandling,
}

#[derive(Serialize, Deserialize)]
pub enum ErrorHandling {
    #[serde(rename = "throw-errors")]
    ThrowErrors,
}
