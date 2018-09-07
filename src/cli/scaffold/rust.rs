use cli::{package, scaffold::Scaffold};
use config_files::Build;
use error::DefaultResult;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};
use util;

pub const CARGO_FILE_NAME: &str = "Cargo.toml";

pub struct RustScaffold {
    build_template: Build,
}

impl RustScaffold {
    pub fn new() -> RustScaffold {
        RustScaffold {
            build_template: Build::with_artifact("target/wasm32-unknown-unknown/release/code.wasm")
                .cmd(
                    "cargo",
                    &["build", "--release", "--target=wasm32-unknown-unknown"],
                ),
        }
    }
}

impl Scaffold for RustScaffold {
    fn gen<P: AsRef<Path>>(&self, base_path: P) -> DefaultResult<()> {
        fs::create_dir_all(&base_path)?;

        // use cargo to initialise a library Rust crate without any version control
        util::run_cmd(
            base_path.as_ref().to_path_buf(),
            "cargo".into(),
            vec![
                "init".to_owned(),
                "--lib".to_owned(),
                "--vcs".to_owned(),
                "none".to_owned(),
            ],
        )?;

        // add hdk-rust dependency by default
        let cargo_file_path = base_path.as_ref().join(CARGO_FILE_NAME);

        let mut cargo_file = OpenOptions::new().append(true).open(cargo_file_path)?;

        // @TODO switch to crates.io ref when hdk-rust gets published
        // @see https://github.com/holochain/holochain-cmd/issues/19
        // Also, caution, if hdk-rust switches to git flow style and "develop" branch, and
        // the above TODO hasn't been addressed, this should also be updated
        let hdk_dep: &str =
            "hdk = { git = \"https://github.com/holochain/hdk-rust\", branch = \"master\" }";

        cargo_file.write_all(hdk_dep.as_bytes())?;

        // add WASM friendly lib configuration properties
        let lib_config: &str = "\n\n[lib]\npath = \"src/lib.rs\"\ncrate-type = [\"cdylib\"]\n";

        cargo_file.write_all(lib_config.as_bytes())?;

        // create and fill in a build file appropriate for Rust
        let build_file_path = base_path.as_ref().join(package::BUILD_CONFIG_FILE_NAME);

        self.build_template.save_as(build_file_path)?;

        Ok(())
    }
}
