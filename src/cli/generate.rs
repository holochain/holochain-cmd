use cli::{
    package::CODE_DIR_NAME,
    scaffold::{self, Scaffold},
};
use error::DefaultResult;
use std::{
    fs,
    path::PathBuf,
};

pub fn generate(zome_name: &PathBuf, language: &str) -> DefaultResult<()> {
    if !zome_name.exists() {
        fs::create_dir_all(&zome_name)?;
    }

    ensure!(
        zome_name.is_dir(),
        "argument \"zome_name\" doesn't point to a directory"
    );

    let code_dir = zome_name.join(CODE_DIR_NAME);
    fs::create_dir_all(&code_dir)?;
    let zome_name_string = zome_name.to_str()
        .expect("Invalid zome path given")
        .to_string()
        .replace("/", "_")
        .replace("zomes_", "");

    // match against all supported languages
    match language {
        "rust" => scaffold(&scaffold::rust::RustScaffold::new(zome_name_string), code_dir)?,
        // "assemblyscript" => scaffold(&scaffold::assemblyscript::AssemblyScriptScaffold::new(), code_dir)?,
        // TODO: supply zome name for AssemblyScriptScaffold as well
        _ => bail!("unsupported language: {}", language),
    }

    Ok(())
}

fn scaffold<S: Scaffold>(tooling: &S, base_path: PathBuf) -> DefaultResult<()> {
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
    fn can_generate_scaffolds() {
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

         Command::main_binary()
            .unwrap()
            .current_dir(&tmp.path())
            .args(&["g", "zomes/zubblebat", "assemblyscript"])
            .assert()
            .success();

    }
}
