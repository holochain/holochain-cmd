use error::DefaultResult;
use util;
use cli::package;
use std::{
    fs,
    path::{Path, PathBuf}
};

pub fn test() -> DefaultResult<()> {

    // create dist folder
    let dist_path = Path::new("dist");
    
    if !dist_path.exists() {
        fs::create_dir(dist_path)?;
    }

    // build the package file
    let bundle_file_path = dist_path.join(package::DEFAULT_BUNDLE_FILE_NAME);
    package(true, Some(bundle_file_path.to_path_buf()))?;

    // build tests
    let test_file_path = PathBuf::new().join("test");

    // npm install, if no node_modules yet
    let node_modules_path = test_file_path.join("node_modules");
    if !node_modules_path.exists() {
        let args: Vec<String> = [
            "install".to_string(),
        ].to_vec();
        util::run_cmd(test_file_path.clone(), "npm".to_string(), args)?;
    }

    // npm run build
    let args: Vec<String> = [
        "run".to_string(),
        "build".to_string(),
    ].to_vec();
    util::run_cmd(test_file_path.clone(), "npm".to_string(), args)?;

    // holoconsole test/dist/bundle.js
    let main_dir = PathBuf::new().join(".");
    let args: Vec<String> = [
        "test/dist/bundle.js".to_string(),
    ].to_vec();
    util::run_cmd(main_dir, "holoconsole".to_string(), args)?;

    Ok(())
}