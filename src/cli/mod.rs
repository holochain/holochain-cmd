use base64;
use config_files::{App as AppConfig, Capability as CapabilityConfig, Zome as ZomeConfig};
use error::{CliError, CliResult, DefaultResult};
use holochain_dna::{
    wasm::DnaWasm,
    zome::{capabilities::Capability, Zome},
    Dna,
};
use package::Package;
use serde_json::{self, Map, Value};
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

const BUNDLE_FILE_NAME: &str = "__bundle.json";

pub fn bundle() -> DefaultResult<()> {
    let dir_obj_bundle = bundle_recurse(PathBuf::from("."))?;

    let out_file = File::create(BUNDLE_FILE_NAME)?;

    serde_json::to_writer_pretty(&out_file, &Value::Object(dir_obj_bundle))?;

    println!("Wrote bundle file to {}", BUNDLE_FILE_NAME);

    Ok(())
}

const FILES_SECTION_NAME: &str = "__FILES";
const DIRS_SECTION_NAME: &str = "__DIRS";

type Object = Map<String, Value>;

pub fn bundle_recurse(path: PathBuf) -> DefaultResult<Object> {
    let root: Vec<_> = path
        .read_dir()?
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap().path())
        .collect();

    let maybe_json_file_path = root
        .iter()
        .filter(|e| e.is_file())
        .find(|e| e.to_str().unwrap().ends_with(".json"));

    // Scan files
    let other_files = root.iter().filter(|e| {
        if let Some(json_file_path) = maybe_json_file_path {
            e.is_file() && *e != json_file_path
        } else {
            e.is_file()
        }
    });

    let mut files_obj = Object::new();
    for file in other_files {
        let mut buf = Vec::new();
        File::open(file)?.read_to_end(&mut buf)?;
        let encoded_content = base64::encode(&buf);

        let file_name = file
            .file_name()
            .ok_or_else(|| format_err!("unable to retrieve file name"))?;

        let file_name = file_name
            .to_str()
            .ok_or_else(|| format_err!("unable to retrieve file name"))?;

        files_obj.insert(file_name.into(), Value::String(encoded_content));
    }

    let other_dirs = root.iter().filter(|e| e.is_dir());

    let mut dirs_obj = Object::new();

    for dir in other_dirs {
        let file_name = dir
            .file_name()
            .ok_or_else(|| format_err!("unable to retrieve file name"))?;

        let file_name = file_name
            .to_str()
            .ok_or_else(|| format_err!("unable to retrieve file name"))?;

        let dir_obj = bundle_recurse(dir.clone())?;

        dirs_obj.insert(file_name.into(), Value::Object(dir_obj));
    }

    let mut config_file: Object = if let Some(json_file_path) = maybe_json_file_path {
        let json_file = fs::read_to_string(json_file_path)?;

        serde_json::from_str(&json_file)?
    } else {
        Object::new()
    };

    if files_obj.len() > 0 {
        config_file.insert(FILES_SECTION_NAME.into(), Value::Object(files_obj));
    }

    if dirs_obj.len() > 0 {
        config_file.insert(DIRS_SECTION_NAME.into(), Value::Object(dirs_obj));
    }

    Ok(config_file)
}

pub fn unpack(path: PathBuf, to: PathBuf) -> DefaultResult<()> {
    ensure!(path.is_file(), "'path' doesn't point ot a file");
    ensure!(to.is_dir(), "'to' doesn't point ot a directory");

    let raw_bundle_content = fs::read_to_string(&path)?;
    let bundle_content: Object = serde_json::from_str(&raw_bundle_content)?;

    Ok(())
}

/// Package smart
pub fn package(check: bool) -> DefaultResult<()> {
    if !check {
        return bundle();
    }

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

    let main_dna = Dna {
        name: app_config.name.clone(),

        description: app_config.description.clone(),

        version: app_config.version.to_string(),

        uuid: "00000000-00000-000-00000000".into(),

        dna_spec_version: "2.0.0".into(),

        properties: Default::default(), // should come from the config in the future

        zomes: compiled_zomes,
    };

    let mut pack = Package::from_app_config(app_config);

    pack.dna = main_dna;

    pack.save_as(PathBuf::from(TARGET_DIR).join(PACKAGE_ARTIFACT));

    Ok(())
}

fn compile_zome<T: AsRef<Path>>(path: T, zome_config: &ZomeConfig) -> DefaultResult<Zome> {
    let caps_dir_path = path.as_ref().join(CAPABILITIES_DIR);

    let caps_dir: Vec<_> = fs::read_dir(&caps_dir_path)?
        .filter(|e| e.is_ok())
        .map(|e| e.unwrap().path())
        .collect();

    let mut caps = Vec::new();

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

        let compiled_cap = compile_capabiliy(cap_path, &cap_config_file)?;

        caps.push(compiled_cap);
    }

    Ok(Zome {
        name: zome_config.name.clone(),
        description: zome_config.description.clone().unwrap_or_default(),
        config: zome_config.config.clone(),
        entry_types: Vec::new(),
        capabilities: caps,
    })

    // let entry_types_dir_path = path.as_ref().join(ENTRY_TYPES_DIR);
    //
    // let entry_types_dir: Vec<_> = fs::read_dir(&entry_types_dir_path)?
    //     .filter(|e| e.is_ok())
    //     .map(|e| e.unwrap().path())
    //     .collect();
    //
    // for entry_path in entry_types_dir {
    //     if !entry_path.is_dir() {
    //         bail!("{:?} is not a directory", entry_path);
    //     }
    //
    //     let mut config_file = EntryTypeConfig::from_file(entry_path.join(ENTRY_TYPE_CONFIG_FILE))?;
    //
    //     let mut validation_file = File::open(entry_path.join(ENTRY_TYPE_VALIDATION_FILE))?;
    //
    //     let mut wasm_data = Vec::new();
    //
    //     validation_file.read_to_end(&mut wasm_data)?;
    // }
}

fn compile_capabiliy<T: AsRef<Path>>(
    path: T,
    cap_config: &CapabilityConfig,
) -> DefaultResult<Capability> {
    let path = PathBuf::from(path.as_ref());

    let wasm_bin_path = path.join(ZOME_WASM_BIN_NAME);

    let mut file = File::open(wasm_bin_path)?;

    let mut wasm_data = Vec::new();

    file.read_to_end(&mut wasm_data)?;

    Ok(Capability {
        name: Default::default(),
        capability: Default::default(),
        fn_declarations: Vec::new(),
        code: DnaWasm { code: wasm_data },
    })
}

pub fn new(path: PathBuf, from: Option<String>) -> DefaultResult<()> {
    if !path.exists() {
        fs::create_dir_all(&path)?;
    } else {
        let zomes_dir = fs::read_dir(&path)?;

        if zomes_dir.count() > 0 {
            bail!("directory is not empty");
        }
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
