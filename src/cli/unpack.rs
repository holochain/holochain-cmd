pub use super::package::*;

use base64;
use error::DefaultResult;
use serde_json::{self, Value};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

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
