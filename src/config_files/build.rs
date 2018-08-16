use base64;
use error::DefaultResult;
use serde_json;
use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use util;

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
            util::run_cmd(base_path.to_path_buf(), bin.to_string(), args.clone())?;
        }

        let artifact_path = base_path.join(&self.artifact);

        if artifact_path.exists() && artifact_path.is_file() {
            let mut wasm_buf = Vec::new();
            File::open(&artifact_path)?.read_to_end(&mut wasm_buf)?;

            Ok(base64::encode(&wasm_buf))
        } else {
            bail!("artifact path either doesn't point to a file or doesn't exist")
        }
    }
}
