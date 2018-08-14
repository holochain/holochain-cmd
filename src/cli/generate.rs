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

    let code_dir = zome_name.join(CODE_DIR_NAME);
    fs::create_dir_all(&code_dir)?;

    match language.as_str() {
        "rust" => scaffold(scaffold::rust::RustScaffold::new(), code_dir)?,
        _ => bail!("unsupported language: {}", language),
    }

    Ok(())
}

fn scaffold<S: Scaffold>(tooling: S, base_path: PathBuf) -> DefaultResult<()> {
    tooling.gen(base_path)
}

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use std::process::Command;
    use tempfile::{Builder, TempDir};

    const HOLOCHAIN_TEST_PREFIX: &str = "org.holochain.test";

    fn gen_dir() -> TempDir {
        Builder::new()
            .prefix(HOLOCHAIN_TEST_PREFIX)
            .tempdir()
            .unwrap()
    }

    #[test]
    fn can_generate_rust_scaffold() {
        let tmp = gen_dir();

        Command::main_binary()
            .unwrap()
            .current_dir(&tmp.path())
            .args(&["init", "."])
            .assert()
            .success();

        Command::main_binary()
            .unwrap()
            .current_dir(&tmp.path())
            .args(&["g", "zomes/bubblechat", "rust"])
            .assert()
            .success();
    }
}
