use base64;
use colored::*;
use error::DefaultResult;
use serde_json;
use std::{
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Clone, Deserialize)]
pub struct Build {
    pub steps: HashMap<String, Vec<String>>,
    pub artifact: PathBuf,
}

impl Build {
    pub fn from_file<T: AsRef<Path>>(path: T) -> DefaultResult<Build> {
        let file = File::open(path)?;

        let build = serde_json::from_reader(&file)?;

        Ok(build)
    }

    /// Starts the build using the supplied build steps and returns the contents of the artifact
    pub fn run(&self, base_path: &PathBuf) -> DefaultResult<String> {
        for (bin, args) in &self.steps {
            let pretty_command = format!("{} {}", bin.green(), args.join(" ").cyan());

            println!("Executing command {}", pretty_command);

            let status = Command::new(bin)
                .args(args)
                .current_dir(base_path)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?;

            ensure!(
                status.success(),
                "command {:?} was not successful",
                pretty_command
            );
        }

        let artifact_path = base_path.join(&self.artifact);

        if artifact_path.exists() && artifact_path.is_file() {
            let wasm = fs::read_to_string(&artifact_path)?;

            Ok(base64::encode(&wasm))
        } else {
            bail!("artifact path either doesn't point to a file or doesn't exist")
        }
    }
}
