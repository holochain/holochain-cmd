use colored::*;
use config_files::App as AppConfig;
use cli::package::{
    IGNORE_FILE_NAME,
    DEFAULT_BUNDLE_FILE_NAME
};
use cli::test::{
    TEST_DIR_NAME,
    DIST_DIR_NAME,
};
use util;
use error::DefaultResult;
use serde_json;
use std::{
    fs::{self, File},
    path::{PathBuf},
    io::Write,
};

struct TestRepo {
    name: String,
    url: String,
    commit: Option<String>
}

fn setup_test_folder(path: &PathBuf, test_folder: &str, test_repo: TestRepo) -> DefaultResult<()> {

    // check if there was a specific commit to checkout, or just latest
    match test_repo.commit {
        Some(sha) => {
            // clone the repo with history
            util::run_cmd(path.clone(), "git".to_string(), vec![
                "clone".to_string(),
                "--quiet".to_string(),
                test_repo.url.to_string(),
                test_folder.to_string(),
            ])?;
            // checkout the desired commit
            let temp_dir = path.join(test_repo.name.to_string());
            util::run_cmd(temp_dir, "git".to_string(), vec![
                "checkout".to_string(),
                sha.to_string(),
                "--quiet".to_string(),
            ])?;
        },
        None => {
            // clone the repo without history
            util::run_cmd(path.clone(), "git".to_string(), vec![
                "clone".to_string(),
                "--quiet".to_string(),
                "--depth".to_string(),
                "1".to_string(),
                test_repo.url.to_string(),
                test_folder.to_string(),
            ])?;
        },
    }

    // remove the .git folder
    fs::remove_dir_all(path.join(test_folder).join(".git"))?;

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
    let ignores = format!("{}\n{}\n{}\n{}\n", &DIST_DIR_NAME, &TEST_DIR_NAME, "README.md", &DEFAULT_BUNDLE_FILE_NAME);
    let mut hcignore_file = File::create(path.join(&IGNORE_FILE_NAME))?;
    hcignore_file.write_all(ignores.as_bytes())?;

    // currently choosing to just clone the latest
    // rather than passing a fixed commit, so that
    // the repo can be updated, and developers
    // don't need to update their command line tools
    let test_repo = TestRepo {
        name: "js-tests-scaffold".to_string(),
        url: "https://github.com/holochain/js-tests-scaffold.git".to_string(),
        commit: None
    };
    setup_test_folder(&path, &TEST_DIR_NAME, test_repo)?;

    println!(
        "{} new Holochain project at: {:?}",
        "Created".green().bold(),
        path
    );

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use cli::test::TEST_DIR_NAME;
    use tempfile::{Builder, TempDir};

    const HOLOCHAIN_TEST_PREFIX: &str = "org.holochain.test";

    fn gen_dir() -> TempDir {
        Builder::new()
            .prefix(HOLOCHAIN_TEST_PREFIX)
            .tempdir()
            .unwrap()
    }

    #[test]
    fn setup_test_folder_test() {
        // because we pass the commit as well
        // this test is deterministic
        let dir = gen_dir();
        let dir_path_buf = &dir.path().to_path_buf();
        let test_repo = TestRepo {
            name: "js-tests-scaffold".to_string(),
            url: "https://github.com/holochain/js-tests-scaffold.git".to_string(),
            commit: Some("26825da5b19a0e4c14273174f9776f929b05c967".to_string())
        };
        setup_test_folder(dir_path_buf, &TEST_DIR_NAME, test_repo);

        assert!(dir_path_buf.join(&TEST_DIR_NAME).join("index.js").exists());
        assert!(dir_path_buf.join(&TEST_DIR_NAME).join("package.json").exists());
        assert!(dir_path_buf.join(&TEST_DIR_NAME).join("webpack.config.js").exists());
        assert!(dir_path_buf.join(&TEST_DIR_NAME).join("README.md").exists());
    }
}
