use cli::{package, scaffold::Scaffold};
use error::DefaultResult;
use serde_json;
use std::{fs::File, path::Path};
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
                    "artifact": "target/release/code.wasm"
                }
            ),
        }
    }
}

impl Scaffold for RustScaffold {
    fn gen<P: AsRef<Path>>(&self, base_path: P) -> DefaultResult<()> {
        let code_dir = base_path
            .as_ref()
            .to_path_buf()
            .join(package::CODE_DIR_NAME);

        util::run_cmd(code_dir.clone(), "cargo".into(), vec!["init".to_owned()])?;

        let build_file_path = code_dir.join(package::BUILD_CONFIG_FILE_NAME);

        let file = File::create(build_file_path)?;

        serde_json::to_writer_pretty(file, &self.build_template)?;

        Ok(())
    }
}