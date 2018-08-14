use cli::{
    package::CODE_DIR_NAME,
    scaffold::{self, Scaffold},
};
use error::DefaultResult;
use std::{fs, path::PathBuf};

pub fn generate(zome_name: PathBuf, language: String) -> DefaultResult<()> {
    if !zome_name.exists() {
        fs::create_dir_all(&zome_name)?;
    }

    match language.as_str() {
        "rust" => scaffold(
            scaffold::rust::RustScaffold::new(),
            zome_name.join(CODE_DIR_NAME),
        )?,
        _ => bail!("unsupported language: {}", language),
    }

    Ok(())
}

fn scaffold<S: Scaffold>(tooling: S, base_path: PathBuf) -> DefaultResult<()> {
    tooling.gen(base_path)
}
