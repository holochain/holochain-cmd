use base64;
use config_files::App as AppConfig;
use error::{CliError, CliResult, DefaultResult};

use serde_json::{self, Map, Value};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

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

const DEFAULT_BUNDLE_FILE_NAME: &str = "bundle.json";

pub fn package(strip_meta: bool, output: Option<PathBuf>) -> DefaultResult<()> {
    let output = output.unwrap_or(PathBuf::from(DEFAULT_BUNDLE_FILE_NAME));

    if !output.exists() {
        fs::create_dir_all(&output)?;
    }

    let dir_obj_bundle = bundle_recurse(PathBuf::from("."), strip_meta)?;

    let out_file = File::create(&output)?;

    serde_json::to_writer_pretty(&out_file, &Value::Object(dir_obj_bundle))?;

    println!("Wrote bundle file to {:?}", output);

    Ok(())
}

const FILE_ID: &str = "file";
const DIR_ID: &str = "dir";

const META_SECTION_NAME: &str = "__META__";
const META_TREE_SECTION_NAME: &str = "tree";
const META_CONFIG_SECTION_NAME: &str = "config_file";

type Object = Map<String, Value>;

pub fn bundle_recurse(path: PathBuf, strip_meta: bool) -> DefaultResult<Object> {
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
    let all_nodes = root.iter().filter(|node_path| {
        maybe_json_file_path
            .and_then(|path| Some(node_path != &path))
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

            let sub_tree_content = bundle_recurse(node.clone(), strip_meta)?;

            main_tree.insert(file_name.into(), Value::Object(sub_tree_content));
        }
    }

    if !strip_meta {
        if meta_tree.len() > 0 {
            meta_section.insert(META_TREE_SECTION_NAME.into(), Value::Object(meta_tree));
        }

        if meta_section.len() > 0 {
            main_tree.insert(META_SECTION_NAME.into(), Value::Object(meta_section));
        }
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
                    .ok_or_else(|| format_err!("incompatible meta section"))?;

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
                        _ => bail!("incompatible meta section"),
                    }
                } else {
                    bail!("incompatible meta section");
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

pub fn new(path: PathBuf, from: Option<String>) -> DefaultResult<()> {
    if !path.exists() {
        fs::create_dir_all(&path)?;
    } else {
        let zomes_dir = fs::read_dir(&path)?;

        if zomes_dir.count() > 0 {
            bail!("directory is not empty");
        }
    }

    fs::create_dir_all(path.join("zomes"))?;
    fs::create_dir_all(path.join("tests"))?;
    fs::create_dir_all(path.join("scenarios"))?;
    fs::create_dir_all(path.join("ui"))?;

    let app_config_file = File::create(path.join("app.json"))?;
    serde_json::to_writer_pretty(app_config_file, &AppConfig::default())?;

    println!("Created new Holochain project at: {:?}", path);

    Ok(())
}
