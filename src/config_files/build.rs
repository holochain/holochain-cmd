use error::DefaultResult;
use serde_json;
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Clone, Deserialize)]
pub struct Build {
    pub steps: HashMap<String, Vec<String>>,
    pub artifact: PathBuf,
}

impl Build {
    pub fn is_valid(&self) -> bool {
        self.steps.len() > 0
    }

    pub fn from_file<T: AsRef<Path>>(path: T) -> DefaultResult<Build> {
        let file = File::open(path)?;

        let build = serde_json::from_reader(&file)?;

        Ok(build)
    }
}
