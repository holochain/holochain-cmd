use config::{App, Zome};
use error::{CliError, CliResult, DefaultResult};
use serde_json;
use std::{
    fs::{self, File}, path::PathBuf,
};

pub fn web(port: u16) -> CliResult<()> {
    Err(CliError::UnknownLanguage)
}

pub fn agent() -> CliResult<()> {
    unimplemented!()
}

pub fn package() -> CliResult<()> {
    unimplemented!()
}

const APP_CONFIG_FILE: &'static str = "app.json";
const ZOMES_DIR: &'static str = "zomes";
const ZOME_CONFIG_FILE: &'static str = "zome.json";

const TESTS_DIR: &'static str = "tests";
const SCENARIOS_DIR: &'static str = "scenarios";
const UI_DIR: &'static str = "ui";

pub fn new(path: PathBuf) -> DefaultResult<()> {
    if path.exists() {
        bail!("project already exists");
    }

    fs::create_dir_all(path.join(ZOMES_DIR))?;
    fs::create_dir_all(path.join(TESTS_DIR))?;
    fs::create_dir_all(path.join(SCENARIOS_DIR))?;
    fs::create_dir_all(path.join(UI_DIR))?;

    let app_config_file = File::create(path.join(APP_CONFIG_FILE))?;
    serde_json::to_writer_pretty(app_config_file, &App::default())?;

    let zome_config_file = File::create(path.join(ZOMES_DIR).join(ZOME_CONFIG_FILE))?;
    serde_json::to_writer_pretty(zome_config_file, &Zome::default())?;

    Ok(())
}
