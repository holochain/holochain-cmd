use config_files::App as AppConfig;
use error::{CliError, CliResult, DefaultResult};
use serde_json;
use std::{
    fs::{self, File},
    path::PathBuf,
};

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
