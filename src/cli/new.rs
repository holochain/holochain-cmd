use colored::*;
use config_files::App as AppConfig;
use cli::package::{
    IGNORE_FILE_NAME,
    DEFAULT_BUNDLE_FILE_NAME
};
use util;
use error::DefaultResult;
use serde_json;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    io::Write,
};

fn setup_test_folder(path: &PathBuf) -> DefaultResult<()> {
    let test_dir = path.join("test");
    fs::create_dir_all(&test_dir)?;

    // clone the default test files in, without git history
    let args: Vec<String> = [
        "clone".to_string(),
        "--depth".to_string(),
        "1".to_string(),
        "https://github.com/holochain/js-tests-scaffold.git".to_string()
    ].to_vec();
    util::run_cmd(path.clone(), "git".to_string(), args)?;

    // copy each file from 'js-tests-scaffold/files' into 'test' folder
    let temp_dir = path.join("js-tests-scaffold");
    for entry in fs::read_dir(temp_dir.join("files").as_path())? {
        let entry = entry?;
        let from_file = entry.path();
        let to_file = test_dir.join(Path::new(&entry.file_name()));
        fs::copy(from_file, to_file.as_path())?;
    }

    // remove the cloned folder
    fs::remove_dir_all(temp_dir)?;

    Ok(())
}

pub fn new(path: &PathBuf, _from: &Option<String>) -> DefaultResult<()> {
    if !path.exists() {
        fs::create_dir_all(&path)?;
    } else {
        let zomes_dir = fs::read_dir(&path)?;

        if zomes_dir.count() > 0 {
            bail!("directory is not empty");
        }
    }

    // create empty zomes folder
    fs::create_dir_all(path.join("zomes"))?;

    let app_config_file = File::create(path.join("app.json"))?;
    serde_json::to_writer_pretty(app_config_file, &AppConfig::default())?;

    // create a default .hcignore file with good defaults
    let ignores = format!("{}\n{}\n{}\n", "test", "README.md", &DEFAULT_BUNDLE_FILE_NAME);
    let mut hcignore_file = File::create(path.join(&IGNORE_FILE_NAME))?;
    hcignore_file.write_all(ignores.as_bytes())?;

    setup_test_folder(&path)?;

    println!(
        "{} new Holochain project at: {:?}",
        "Created".green().bold(),
        path
    );

    Ok(())
}
