use config_files::{
    App as AppConfig, Capability as CapabilityConfig, EntryType as EntryTypeConfig,
    Zome as ZomeConfig,
};
use error::{CliError, CliResult, DefaultResult};
use holochain_dna::{zome::Zome, Dna};
use package::Package;
use serde_json;
use std::{
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};

pub const APP_CONFIG_FILE: &str = "app.json";
pub const ZOMES_DIR: &str = "zomes";
pub const ZOME_CONFIG_FILE: &str = "zome.json";

pub const CAPABILITIES_DIR: &str = "capabilities";
pub const CAPABILITY_CONFIG_FILE: &str = "capability.json";

pub const ENTRY_TYPES_DIR: &str = "entry_types";
pub const ENTRY_TYPE_VALIDATION_FILE: &str = "validation.wasm";
pub const ENTRY_TYPE_CONFIG_FILE: &str = "type.json";

pub const TESTS_DIR: &str = "tests";
pub const SCENARIOS_DIR: &str = "scenarios";
pub const UI_DIR: &str = "ui";
pub const TARGET_DIR: &str = "target";
pub const PACKAGE_ARTIFACT: &str = "out.hcpkg";

pub const ZOME_WASM_BIN_NAME: &str = "main.wasm";

pub fn web(port: u16) -> CliResult<()> {
    Err(CliError::UnknownLanguage)
}

pub fn agent() -> CliResult<()> {
    println!("Starting agent...");
    println!("Agent successfully started!");
    println!("Stopping agent...");
    println!("Agent stopped. Bye!");

    Ok(())
}

pub fn package() -> DefaultResult<()> {
    let zomes_dir_path = PathBuf::from(ZOMES_DIR);

    let zomes_dir: Vec<_> = fs::read_dir(&zomes_dir_path)?
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap().path())
        .collect();

    if zomes_dir.is_empty() {
        bail!("no zomes found");
    }

    let app_config_file = File::open(APP_CONFIG_FILE)?;
    let app_config: AppConfig = serde_json::from_reader(app_config_file)?;

    let mut compiled_zomes = Vec::new();

    for zome_path in zomes_dir {
        if !zome_path.is_dir() {
            bail!("the path {:?} is not a directory", zome_path);
        }

        let config_file_path = zome_path.join(ZOME_CONFIG_FILE);

        if !config_file_path.exists() {
            bail!(
                "the path {:?} doesn't contain a {} file",
                zome_path,
                ZOME_CONFIG_FILE
            );
        }

        let config_file = ZomeConfig::from_file(&config_file_path)?;

        compiled_zomes.push(compile_zome(&zome_path, &config_file)?);
    }

    let mut pack = Package::from_app_config(app_config);

    pack.zomes = compiled_zomes;

    pack.save_as(PathBuf::from(TARGET_DIR).join(PACKAGE_ARTIFACT));

    Ok(())
}

fn compile_zome<T: AsRef<Path>>(path: T, zome_config: &ZomeConfig) -> DefaultResult<Zome> {
    let caps_dir_path = path.as_ref().join(CAPABILITIES_DIR);

    let caps_dir: Vec<_> = fs::read_dir(&caps_dir_path)?
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap().path())
        .collect();

    for cap_path in caps_dir {
        if !cap_path.is_dir() {
            bail!("the path {:?} is not a directory", cap_path);
        }

        let config_file_path = cap_path.join(CAPABILITY_CONFIG_FILE);

        if !config_file_path.exists() {
            bail!(
                "the path {:?} doesn't contain a {} file",
                cap_path,
                CAPABILITY_CONFIG_FILE
            );
        }

        let cap_config_file = CapabilityConfig::from_file(config_file_path)?;

        let compiled_wasm = compile_capabiliy(cap_path, &cap_config_file)?;
    }

    let entry_types_dir_path = path.as_ref().join(ENTRY_TYPES_DIR);

    let entry_types_dir: Vec<_> = fs::read_dir(&entry_types_dir_path)?
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap().path())
        .collect();

    for entry_path in entry_types_dir {
        if !entry_path.is_dir() {
            bail!("{:?} is not a directory", entry_path);
        }

        let mut config_file = EntryTypeConfig::from_file(entry_path.join(ENTRY_TYPE_CONFIG_FILE))?;

        let mut validation_file = File::open(entry_path.join(ENTRY_TYPE_VALIDATION_FILE))?;

        let mut wasm_data = Vec::new();

        validation_file.read_to_end(&mut wasm_data)?;
    }

    bail!("NOPE! NO SOY BLA BLA BLA");
}

fn compile_capabiliy<T: AsRef<Path>>(
    path: T,
    cap_config: &CapabilityConfig,
) -> DefaultResult<Vec<u8>> {
    let path = PathBuf::from(path.as_ref());

    let wasm_bin_path = path.join(ZOME_WASM_BIN_NAME);

    let mut file = File::open(wasm_bin_path)?;

    let mut wasm_data = Vec::new();

    file.read_to_end(&mut wasm_data)?;

    Ok(wasm_data)
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
    serde_json::to_writer_pretty(app_config_file, &AppConfig::default())?;

    println!("Created new Holochain project at: {:?}", path);

    Ok(())
}
