use cli::{package, scaffold::Scaffold};
use colored::*;
use config_files::Build;
use error::DefaultResult;
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::Read,
    io::Write,
    io::Seek,
    io::SeekFrom,
    path::Path,
};
use toml;
use toml::{
    value::{
        Value
    }
};
use util;

pub const CARGO_FILE_NAME: &str = "Cargo.toml";

pub struct RustScaffold {
    build_template: Build,
}

#[derive(Serialize, Deserialize, Debug)]
struct CargoPackage {
    name: String,
    version: String,
    authors: Vec<String>,
    edition: String
}

#[derive(Serialize, Deserialize, Debug)]
struct CargoLib {
    path: String,
    #[serde(rename="crate-type")]
    crate_type: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct CargoFile {
    package: CargoPackage,
    #[serde(serialize_with = "toml::ser::tables_last")]
    dependencies: toml::value::Table,
    lib: Option<CargoLib>
}

fn toml_string(s: &str) -> toml::value::Value {
    toml::value::Value::String(s.to_string())
}

fn rewrite_cargo_toml(cargo_file_path: &Path) -> DefaultResult<()> {
    let mut cargo_file = OpenOptions::new().read(true).write(true).open(cargo_file_path)?;
    let mut contents = String::new();
    cargo_file.read_to_string(&mut contents)?;
    let new_toml = flesh_out_cargo_toml(contents.as_str())
        .expect("Couldn't parse Cargo.toml");
    cargo_file.seek(SeekFrom::Start(0));
    // let mut new_cargo_file = OpenOptions::new().write(true).open(cargo_file_path)?;
    cargo_file.write_all(new_toml.as_bytes());
    Ok(())

}
fn flesh_out_cargo_toml(contents: &str) -> Result<String, toml::ser::Error> {
    let mut config: CargoFile = toml::from_str(contents)
        .expect("Couldn't parse Cargo.toml");
    let deps: BTreeMap<String, Value> = btreemap! {
        "serde".to_string() => toml_string("1.0"),
        "serde_json".to_string() => toml_string("1.0"),
        "serde_derive".to_string() => toml_string("1.0"),
        "hdk".to_string() => Value::Table(btreemap! {
            "git".to_string() => toml_string("https://github.com/holochain/hdk-rust")
        }),
        "holochain_wasm_utils".to_string() => Value::Table(btreemap! {
            "git".to_string() => toml_string("https://github.com/holochain/holochain-rust"),
            "branch".to_string() => toml_string("develop")
        })
    };
    let lib = CargoLib {
        path: "src/lib.rs".to_string(),
        crate_type: vec!["cdylib".to_string()]
    };
    config.dependencies = deps.into();
    config.lib = Some(lib);
    toml::to_string(&config)
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
        ).expect("Couldn't initialize zome with cargo init");

        // add hdk-rust dependency by default
        let cargo_file_path = base_path.as_ref().join(CARGO_FILE_NAME);
        rewrite_cargo_toml(&cargo_file_path).expect("OK?");

        // create and fill in a build file appropriate for Rust
        let build_file_path = base_path.as_ref().join(package::BUILD_CONFIG_FILE_NAME);

        self.build_template.save_as(build_file_path)?;

        Ok(())
    }
}
