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
    package(true, Some(bundle_file_path.to_path_buf()))?;

    // build tests
    let tests_path = path.join(&tests_folder);

    // npm install, if no node_modules yet
    let node_modules_path = tests_path.join("node_modules");
    if !node_modules_path.exists() {
        util::run_cmd(tests_path.clone(), "npm".to_string(), vec![
            "install".to_string(),
        ])?;
    }

    // npm run build
    util::run_cmd(tests_path.clone(), "npm".to_string(), vec![
        "run".to_string(),
        "build".to_string(),
    ])?;

    // execute the built test file using holoconsole
    // this "magic string" comes from the webpack config
    // in the js-tests-scaffold: https://github.com/holochain/js-tests-scaffold/blob/master/webpack.config.js#L5-L8
    // they need to stay in sync
    let js_test_path = format!("{}{}", &tests_folder, "/dist/bundle.js");
    util::run_cmd(path.to_path_buf(), "holoconsole".to_string(), vec![
        js_test_path,
    ])?;

    Ok(())
}