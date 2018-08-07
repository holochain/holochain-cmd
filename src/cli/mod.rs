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
    io::{Read, Write},
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

const FILE_ID: &str = "file";
const DIR_ID: &str = "dir";

const META_SECTION_NAME: &str = "__META__";
const META_TREE_SECTION_NAME: &str = "tree";
const META_CONFIG_SECTION_NAME: &str = "config_file";

const MANIFEST_FILE_NAME: &str = "dna_manifest.json";

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

    // Scan files but discard found json file
    let all_nodes = root.iter().filter(|e| {
        maybe_json_file_path
            .and_then(|path| Some(*e != path))
            .unwrap_or(true)
    });

    let mut meta_section = Object::new();

    // Obtain the config file
    let mut main_tree: Object = if let Some(json_file_path) = maybe_json_file_path {
        let file_name = json_file_path
            .file_name()
            .ok_or_else(|| format_err!("unable to retrieve file name"))?;

        let file_name = file_name
            .to_str()
            .ok_or_else(|| format_err!("unable to retrieve file name"))?;

        meta_section.insert(
            META_CONFIG_SECTION_NAME.into(),
            Value::String(file_name.into()),
        );

        let json_file = fs::read_to_string(json_file_path)?;

        serde_json::from_str(&json_file)?
    } else {
        Object::new()
    };

    // Let's go meta. Way meta!
    let mut meta_tree = Object::new();

    for node in all_nodes {
        let file_name = node
            .file_name()
            .ok_or_else(|| format_err!("unable to retrieve file name"))?;

        let file_name = file_name
            .to_str()
            .ok_or_else(|| format_err!("unable to retrieve file name"))?;

        if node.is_file() {
            meta_tree.insert(file_name.into(), Value::String(FILE_ID.into()));

            let mut buf = Vec::new();
            File::open(node)?.read_to_end(&mut buf)?;
            let encoded_content = base64::encode(&buf);

            main_tree.insert(file_name.into(), Value::String(encoded_content));
        } else if node.is_dir() {
            meta_tree.insert(file_name.into(), Value::String(DIR_ID.into()));

            let sub_tree_content = bundle_recurse(node.clone())?;

            main_tree.insert(file_name.into(), Value::Object(sub_tree_content));
        }
    }

    if meta_tree.len() > 0 {
        meta_section.insert(META_TREE_SECTION_NAME.into(), Value::Object(meta_tree));
    }

    if meta_section.len() > 0 {
        main_tree.insert(META_SECTION_NAME.into(), Value::Object(meta_section));
    }

    Ok(main_tree)
}

pub fn unpack(path: PathBuf, to: PathBuf) -> DefaultResult<()> {
    ensure!(path.is_file(), "'path' doesn't point ot a file");
    ensure!(to.is_dir(), "'to' doesn't point ot a directory");

    let raw_bundle_content = fs::read_to_string(&path)?;
    let bundle_content: Object = serde_json::from_str(&raw_bundle_content)?;

    unpack_recurse(bundle_content, to)?;

    Ok(())
}

fn unpack_recurse(mut obj: Object, to: PathBuf) -> DefaultResult<()> {
    if let Some(Value::Object(mut main_meta_obj)) = obj.remove(META_SECTION_NAME) {
        // unpack the tree
        if let Some(Value::Object(tree_meta_obj)) = main_meta_obj.remove(META_TREE_SECTION_NAME) {
            for (meta_entry, meta_value) in tree_meta_obj {
                let entry = obj
                    .remove(&meta_entry)
                    .ok_or_else(|| format_err!("INCOMPATIBLE META SECTION"))?;

                if let Value::String(node_type) = meta_value {
                    match node_type.as_str() {
                        FILE_ID if entry.is_string() => {
                            let base64_content = entry.as_str().unwrap().to_string();
                            let content = base64::decode(&base64_content)?;

                            File::create(to.join(meta_entry))?.write_all(&content[..])?;
                        }
                        DIR_ID if entry.is_object() => {
                            let directory_obj = entry.as_object().unwrap();
                            let dir_path = to.join(meta_entry);

                            fs::create_dir(dir_path.clone())?;

                            unpack_recurse(directory_obj.clone(), dir_path.clone())?;
                        }
                        _ => bail!("YOU SUCK AT META DATA!"),
                    }
                } else {
                    bail!("YOU SUCK AT META DATA!");
                }
            }
        }

        // unpack the config file
        if let Some(config_file_meta) = main_meta_obj.remove(META_CONFIG_SECTION_NAME) {
            ensure!(
                config_file_meta.is_string(),
                "config file has to be a string"
            );

            if obj.len() > 0 {
                let dna_file = File::create(to.join(config_file_meta.as_str().unwrap()))?;
                serde_json::to_writer_pretty(dna_file, &obj)?;
            }
        }
    }

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
