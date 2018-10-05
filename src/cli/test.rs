use colored::*;
use error::DefaultResult;
use util;
use cli::package;
use std::{
    fs,
    path::{PathBuf}
};

pub const TEST_DIR_NAME: &str = "test";
pub const DIST_DIR_NAME: &str = "dist";

pub fn test(path: &PathBuf, tests_folder: &str) -> DefaultResult<()> {

    // create dist folder
    let dist_path = path.join(&DIST_DIR_NAME);
    
    if !dist_path.exists() {
        fs::create_dir(dist_path.as_path())?;
    }

    // build the package file, within the dist folder
    let bundle_file_path = dist_path.join(package::DEFAULT_BUNDLE_FILE_NAME);
    println!(
        "{} files for testing to file: {:?}",
        "Packaging".green().bold(),
        bundle_file_path
    );
    package(true, Some(bundle_file_path.to_path_buf()))?;

    // build tests
    let tests_path = path.join(&tests_folder);

    // npm install, if no node_modules yet
    let node_modules_path = tests_path.join("node_modules");
    if !node_modules_path.exists() {
        // execute the built test file using holoconsole
        println!("{}", "Installing node_modules".green().bold());
        util::run_cmd(tests_path.clone(), "npm".to_string(), vec![
            "install".to_string(),
            "--silent".to_string(),
        ])?;
    }

    // npm run build
    // this "magic string" comes from the webpack config
    // in the js-tests-scaffold: https://github.com/holochain/js-tests-scaffold/blob/master/webpack.config.js#L5-L8
    // they need to stay in sync
    let js_test_path = format!("{}{}", &tests_folder, "/dist/bundle.js");
    println!(
        "{} an executable test file: {}",
        "Building".green().bold(),
        js_test_path
    );
    util::run_cmd(tests_path.clone(), "npm".to_string(), vec![
        "run".to_string(),
        "build".to_string(),
    ])?;

    // execute the built test file using holoconsole
    println!(
        "{} tests in {}",
        "Running".green().bold(),
        js_test_path
    );
    util::run_cmd(path.to_path_buf(), "holoconsole".to_string(), vec![
        js_test_path,
    ])?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use cli::package;
    use std::process::Command;
    use assert_cmd::prelude::*;
    use tempfile::{Builder, TempDir};

    const HOLOCHAIN_TEST_PREFIX: &str = "org.holochain.test";

    fn gen_dir() -> TempDir {
        Builder::new()
            .prefix(HOLOCHAIN_TEST_PREFIX)
            .tempdir()
            .unwrap()
    }

    #[test]
    fn test_command_test() {
        // because we pass the commit as well
        // this test is deterministic
        let temp_space = gen_dir();
        let temp_dir_path = temp_space.path();
        let temp_dir_path_buf = temp_space.path().to_path_buf();

        // do init first, so theres a project
        Command::main_binary()
                .unwrap()
                .args(&["init", temp_dir_path.to_str().unwrap()])
                .assert()
                .success();

        test(&temp_dir_path_buf, &TEST_DIR_NAME);

        // check success of packaging step
        assert!(temp_dir_path_buf.join(&DIST_DIR_NAME).join(package::DEFAULT_BUNDLE_FILE_NAME).exists());
        // check success of npm install step
        assert!(temp_dir_path_buf.join(&TEST_DIR_NAME).join("node_modules").exists());
        // check success of js webpack build step
        assert!(temp_dir_path_buf.join(&TEST_DIR_NAME).join("dist/bundle.js").exists());
    }
}