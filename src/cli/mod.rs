use config::{App, Zome};
use error::{CliError, CliResult, DefaultResult};
use serde_json;
use std::{
    collections::HashMap, fs::{self, File}, path::{Path, PathBuf},
};

const APP_CONFIG_FILE: &str = "app.json";
const ZOMES_DIR: &str = "zomes";
const ZOME_CONFIG_FILE: &str = "zome.json";

const TESTS_DIR: &str = "tests";
const SCENARIOS_DIR: &str = "scenarios";
const UI_DIR: &str = "ui";

pub fn web(port: u16) -> CliResult<()> {
    Err(CliError::UnknownLanguage)
}

pub fn agent() -> CliResult<()> {
    unimplemented!()
}

pub fn package() -> DefaultResult<()> {
    let zomes_dir_path = PathBuf::from(ZOMES_DIR);

    let zomes_dir = fs::read_dir(&zomes_dir_path)?;

    let zomes_dir: Vec<_> = zomes_dir
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap().path())
        .collect();

    if zomes_dir.is_empty() {
        bail!("no zomes found");
    }

    let mut zome_config_files = HashMap::new();

    for zome in zomes_dir {
        if !zome.is_dir() {
            bail!("{:?} is not a directory", zome);
        }

        let config_file_path = zome.join(ZOME_CONFIG_FILE);

        if !config_file_path.exists() {
            bail!("{:?} doesn't contain a zome.json file", zome);
        }

        let config_file = Zome::from_file(&config_file_path)?;

        let zome_wasm = compile_zome(&config_file_path, &config_file);

        zome_config_files.insert(zome, zome_wasm);
    }

    println!("{:#?}", zome_config_files);

    Ok(())
}

fn compile_zome<T: AsRef<Path>>(path: T, config: &Zome) -> Vec<u8> {
    Vec::new()
}

pub fn new(path: PathBuf, from: Option<String>) -> DefaultResult<()> {
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    let zomes_dir = fs::read_dir(&path)?;

    if zomes_dir.count() > 0 {
        bail!("directory is not empty");
    }

    fs::create_dir_all(path.join(ZOMES_DIR))?;
    fs::create_dir_all(path.join(TESTS_DIR))?;
    fs::create_dir_all(path.join(SCENARIOS_DIR))?;
    fs::create_dir_all(path.join(UI_DIR))?;

    let app_config_file = File::create(path.join(APP_CONFIG_FILE))?;
    serde_json::to_writer_pretty(app_config_file, &App::default())?;

    println!("Created new Holochain project at: {:?}", path);

    Ok(())
}
