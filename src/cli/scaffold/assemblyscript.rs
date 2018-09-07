use cli::{package, scaffold::Scaffold};
use config_files::Build;
use error::DefaultResult;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};
use util;

pub struct AssemblyScriptScaffold {
    build_template: Build,
}

impl AssemblyScriptScaffold {
    pub fn new() -> AssemblyScriptScaffold {
        AssemblyScriptScaffold {
            build_template: Build::with_artifact("module.wasm")
                .cmd(
                    "./node_modules/assemblyscript/bin/asc",
                    &["index.ts", "-b", "module.wasm"],
                ),
        }
    }
}

impl Scaffold for AssemblyScriptScaffold {
    fn gen<P: AsRef<Path>>(&self, base_path: P) -> DefaultResult<()> {
        fs::create_dir_all(&base_path)?;
        
        // use npm to initialise a nodejs project 
        util::run_cmd(
            base_path.as_ref().to_path_buf(),
            "npm".into(),
            vec![
                "init".to_owned(),
                "-y".to_owned(),
            ],
        )?;

        // add hdk-assemblyscript as a dependency
        util::run_cmd(
            base_path.as_ref().to_path_buf(),
            "npm".into(),
            vec![
                "install".to_owned(),
                "--save".to_owned(),
                "holochain/hdk-assemblyscript".to_owned()
            ],
        )?;

        // add assemblyscript as a dev dependency
        util::run_cmd(
            base_path.as_ref().to_path_buf(),
            "npm".into(),
            vec![
                "install".to_owned(),
                "--save-dev".to_owned(),
                "AssemblyScript/assemblyscript".to_owned()
            ],
        )?;

        // create a index.ts file
        let typescript_file_path = base_path.as_ref().join(package::TYPESCRIPT_FILE_NAME);

        let mut typescript_file = OpenOptions::new().write(true).create(true).open(typescript_file_path)?;

        let require: &str =
            "import { debug } from \"./node_modules/hdk-assemblyscript\"";

        typescript_file.write_all(require.as_bytes())?;

        // create and fill in a build file appropriate for AssemblyScript
        let build_file_path = base_path.as_ref().join(package::BUILD_CONFIG_FILE_NAME);

        self.build_template.save_as(build_file_path)?;

        Ok(())
    }
}
