use base64;
use colored::*;
use config_files::Build;
use error::DefaultResult;
use serde_json::{self, Map, Value};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};
use util;

pub const CODE_DIR_NAME: &str = "code";

pub const BUILD_CONFIG_FILE_NAME: &str = ".build";

pub const CARGO_FILE_NAME: &str = "Cargo.toml";

pub const WASM_FILE_EXTENSION: &str = "wasm";

pub const DEFAULT_BUNDLE_FILE_NAME: &str = "bundle.json";

pub const META_FILE_ID: &str = "file";
pub const META_DIR_ID: &str = "dir";
pub const META_BIN_ID: &str = "bin";

pub const META_SECTION_NAME: &str = "__META__";
pub const META_TREE_SECTION_NAME: &str = "tree";
pub const META_CONFIG_SECTION_NAME: &str = "config_file";

pub type Object = Map<String, Value>;

struct Packager {
    strip_meta: bool,
    preserve_dotfiles: bool,
}

impl Packager {
    fn new(strip_meta: bool, preserve_dotfiles: bool) -> Packager {
        Packager {
            strip_meta,
            preserve_dotfiles,
        }
    }

    pub fn package(
        strip_meta: bool,
        preserve_dotfiles: bool,
        output: Option<PathBuf>,
    ) -> DefaultResult<()> {
        let output = output.unwrap_or(PathBuf::from(DEFAULT_BUNDLE_FILE_NAME));

        Packager::new(strip_meta, preserve_dotfiles).run(output)
    }

    fn run(&self, output: PathBuf) -> DefaultResult<()> {
        let dir_obj_bundle = self.bundle_recurse(PathBuf::from("."))?;

        let out_file = File::create(&output)?;

        serde_json::to_writer_pretty(&out_file, &Value::Object(dir_obj_bundle))?;

        println!("{} bundle file at {:?}", "Created".green().bold(), output);

        Ok(())
    }

    fn bundle_recurse(&self, path: PathBuf) -> DefaultResult<Object> {
        let root: Vec<_> = path
            .read_dir()?
            .filter(|e| e.is_ok())
            .map(|e| e.unwrap().path())
            .filter(|node| {
                if self.preserve_dotfiles {
                    return true;
                }

                util::file_name_string(node.to_path_buf())
                    .and_then(|file_name| Ok(!file_name.starts_with(".")))
                    .unwrap_or(false)
            }).collect();

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
            let file_name = util::file_name_string(json_file_path.clone())?;

            meta_section.insert(
                META_CONFIG_SECTION_NAME.into(),
                Value::String(file_name.clone()),
            );

            let json_file = fs::read_to_string(json_file_path)?;

            serde_json::from_str(&json_file)?
        } else {
            Object::new()
        };

        // Let's go meta. Way meta!
        let mut meta_tree = Object::new();

        for node in all_nodes {
            let file_name = util::file_name_string(node.clone())?;

            if node.is_file() {
                meta_tree.insert(file_name.clone(), Value::String(META_FILE_ID.into()));

                let mut buf = Vec::new();
                File::open(node)?.read_to_end(&mut buf)?;
                let encoded_content = base64::encode(&buf);

                main_tree.insert(file_name.clone(), Value::String(encoded_content));
            } else if node.is_dir() {
                if let Some(build_config) = node
                    .read_dir()?
                    .filter(|e| e.is_ok())
                    .map(|e| e.unwrap().path())
                    .find(|path| path.ends_with(BUILD_CONFIG_FILE_NAME))
                {
                    meta_tree.insert(file_name.clone(), Value::String(META_BIN_ID.into()));

                    let build = Build::from_file(build_config)?;

                    let wasm = build.run(&node)?;

                    main_tree.insert(file_name.clone(), Value::String(wasm));
                } else {
                    meta_tree.insert(file_name.clone(), Value::String(META_DIR_ID.into()));

                    let sub_tree_content = self.bundle_recurse(node.clone())?;

                    main_tree.insert(file_name.clone(), Value::Object(sub_tree_content));
                }
            }
        }

        if !self.strip_meta {
            if meta_tree.len() > 0 {
                meta_section.insert(META_TREE_SECTION_NAME.into(), Value::Object(meta_tree));
            }

            if meta_section.len() > 0 {
                main_tree.insert(META_SECTION_NAME.into(), Value::Object(meta_section));
            }
        }

        Ok(main_tree)
    }
}

pub fn package(
    strip_meta: bool,
    preserve_dotfiles: bool,
    output: Option<PathBuf>,
) -> DefaultResult<()> {
    Packager::package(strip_meta, preserve_dotfiles, output)
}

pub fn unpack(path: PathBuf, to: PathBuf) -> DefaultResult<()> {
    ensure!(path.is_file(), "argument \"path\" doesn't point ot a file");

    if !to.exists() {
        fs::create_dir_all(&to)?;
    }

    ensure!(to.is_dir(), "argument \"to\" doesn't point to a directory");

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
                        id @ META_FILE_ID | id @ META_BIN_ID if entry.is_string() => {
                            let base64_content = entry.as_str().unwrap().to_string();
                            let content = base64::decode(&base64_content)?;

                            let mut file_path = to.join(meta_entry);

                            if id == META_BIN_ID {
                                file_path.set_extension(WASM_FILE_EXTENSION);
                            }

                            File::create(file_path)?.write_all(&content[..])?;
                        }
                        META_DIR_ID if entry.is_object() => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::prelude::*;
    use dir_diff;
    use std::process::Command;
    use tempfile::{Builder, TempDir};

    const HOLOCHAIN_TEST_PREFIX: &str = "org.holochain.test";

    fn gen_dir() -> TempDir {
        Builder::new()
            .prefix(HOLOCHAIN_TEST_PREFIX)
            .tempdir()
            .unwrap()
    }

    #[test]
    fn package_and_unpack_isolated() {
        const DEFAULT_BUNDLE_FILE_NAME: &str = "bundle.json";

        fn package(shared_file_path: &PathBuf) {
            let temp_space = gen_dir();
            let temp_dir_path = temp_space.path();

            Command::main_binary()
                .unwrap()
                .args(&["init", temp_dir_path.to_str().unwrap()])
                .assert()
                .success();

            let bundle_file_path = shared_file_path.join(DEFAULT_BUNDLE_FILE_NAME);

            Command::main_binary()
                .unwrap()
                .args(&["package", "-o", bundle_file_path.to_str().unwrap()])
                .current_dir(&temp_dir_path)
                .assert()
                .success();
        }

        fn unpack(shared_file_path: &PathBuf) {
            let temp_space = gen_dir();
            let temp_dir_path = temp_space.path();

            Command::main_binary()
                .unwrap()
                .current_dir(&shared_file_path)
                .args(&[
                    "unpack",
                    DEFAULT_BUNDLE_FILE_NAME,
                    temp_dir_path.to_str().unwrap(),
                ]).assert()
                .success();
        }

        let shared_space = gen_dir();

        package(&shared_space.path().to_path_buf());

        unpack(&shared_space.path().to_path_buf());

        shared_space.close().unwrap();
    }

    #[test]
    /// A test ensuring that packaging and unpacking a project results in the very same project
    fn package_reverse() {
        const DEFAULT_BUNDLE_FILE_NAME: &str = "bundle.json";

        const SOURCE_DIR_NAME: &str = "source_app";
        const DEST_DIR_NAME: &str = "dest_app";

        let shared_space = gen_dir();

        let root_path = shared_space.path().to_path_buf();

        let source_path = shared_space.path().join(SOURCE_DIR_NAME);
        fs::create_dir_all(&source_path).unwrap();

        // Initialize and package a project
        Command::main_binary()
            .unwrap()
            .args(&["init", source_path.to_str().unwrap()])
            .assert()
            .success();

        let bundle_file_path = root_path.join(DEFAULT_BUNDLE_FILE_NAME);

        Command::main_binary()
            .unwrap()
            .args(&["package", "-o", bundle_file_path.to_str().unwrap()])
            .current_dir(&source_path)
            .assert()
            .success();

        // Unpack the project from the generated bundle
        let dest_path = shared_space.path().join(DEST_DIR_NAME);
        fs::create_dir_all(&dest_path).unwrap();

        Command::main_binary()
            .unwrap()
            .args(&[
                "unpack",
                bundle_file_path.to_str().unwrap(),
                dest_path.to_str().unwrap(),
            ]).assert()
            .success();

        // Assert for equality
        assert!(!dir_diff::is_different(&source_path, &dest_path).unwrap());
    }

    #[test]
    fn auto_compilation() {
        let tmp = gen_dir();

        Command::main_binary()
            .unwrap()
            .current_dir(&tmp.path())
            .args(&["init", "."])
            .assert()
            .success();

        Command::main_binary()
            .unwrap()
            .current_dir(&tmp.path())
            .args(&["g", "zomes/bubblechat", "rust"])
            .assert()
            .success();

        Command::main_binary()
            .unwrap()
            .current_dir(&tmp.path())
            .args(&["package"])
            .assert()
            .success();
    }
}
