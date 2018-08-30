use cli::{package, scaffold::Scaffold};
use error::DefaultResult;
use serde_json;
use std::{
    fs::{self, File, OpenOptions},
    path::Path,
    io::{Write}
};
use util;

pub struct RustScaffold {
    build_template: serde_json::Value,
}

impl RustScaffold {
    pub fn new() -> RustScaffold {
        RustScaffold {
            build_template: json!(
                {
                    "steps": {
                        "cargo": [ "build", "--release", "--target=wasm32-unknown-unknown" ]
                    },
                    "artifact": "target/wasm32-unknown-unknown/release/code.wasm"
                }
            ),
        }
    }
}

impl Scaffold for RustScaffold {
    fn gen<P: AsRef<Path>>(&self, base_path: P) -> DefaultResult<()> {
        fs::create_dir_all(&base_path)?;

        // use cargo to initialise a Rust app without any version control
        util::run_cmd(
            base_path.as_ref().to_path_buf(),
            "cargo".into(),
            vec!["init".to_owned(), "--vcs".to_owned(), "none".to_owned()],
        )?;

        // add hdk-rust dependency by default
        let cargo_file_path = base_path.as_ref().join(package::CARGO_FILE_NAME);

        let mut f = OpenOptions::new()
            .append(true)
            .open(cargo_file_path)?;

        // @TODO switch to crates.io ref when hdk-rust gets published
        // @see https://github.com/holochain/holochain-cmd/issues/19
        // Also, caution, if hdk-rust switches to git flow style and "develop" branch, and
        // the above TODO hasn't been addressed, this should also be updated
        let hdk_dep: &str = "hdk = { git = \"https://github.com/holochain/hdk-rust\", branch = \"master\" }";

        f.write_all(hdk_dep.as_bytes())?;

        // create and fill in a build file appropriate for Rust
        let build_file_path = base_path.as_ref().join(package::BUILD_CONFIG_FILE_NAME);

        let file = File::create(build_file_path)?;

        serde_json::to_writer_pretty(file, &self.build_template)?;

        Ok(())
    }
}
