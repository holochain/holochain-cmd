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
use error::DefaultResult;
use serde_json;
use std::{
    fs::{self, File, OpenOptions},
    path::{PathBuf},
    io::Write,
};

fn create_test_file(test_folder_path: &PathBuf, test_file_name: &str, test_file_contents: &str) -> DefaultResult<()> {
    let dest_filepath = test_folder_path.join(test_file_name);
    let mut file = OpenOptions::new().write(true).create(true).open(dest_filepath)?;
    file.write_all(test_file_contents.as_bytes())?;
    Ok(())
}

fn setup_test_folder(path: &PathBuf, test_folder: &str) -> DefaultResult<()> {
    let tests_path = path.join(test_folder);
    fs::create_dir_all(tests_path.clone())?;
    create_test_file(&tests_path, "index.js", include_str!("js-tests-scaffold/index.js"))?;
    create_test_file(&tests_path, "package-lock.json", include_str!("js-tests-scaffold/package-lock.json"))?;
    create_test_file(&tests_path, "package.json", include_str!("js-tests-scaffold/package.json"))?;
    create_test_file(&tests_path, "README.md", include_str!("js-tests-scaffold/README.md"))?;
    create_test_file(&tests_path, "webpack.config.js", include_str!("js-tests-scaffold/webpack.config.js"))?;
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

    setup_test_folder(&path, &TEST_DIR_NAME)?;

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
        setup_test_folder(dir_path_buf, &TEST_DIR_NAME);

        assert!(dir_path_buf.join(&TEST_DIR_NAME).join("index.js").exists());
        assert!(dir_path_buf.join(&TEST_DIR_NAME).join("package.json").exists());
        assert!(dir_path_buf.join(&TEST_DIR_NAME).join("package-lock.json").exists());
        assert!(dir_path_buf.join(&TEST_DIR_NAME).join("webpack.config.js").exists());
        assert!(dir_path_buf.join(&TEST_DIR_NAME).join("README.md").exists());
    }
}
