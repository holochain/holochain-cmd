use cli::{package, scaffold::Scaffold};
use config_files::Build;
use error::DefaultResult;
use std::{
    fs::{self, OpenOptions},
    io::Read,
    io::Write,
    io::Seek,
    io::SeekFrom,
    path::Path,
};
use toml::{
    self,
    value::{
        Value
    }
};
use util;

pub const CARGO_FILE_NAME: &str = "Cargo.toml";

pub struct RustScaffold {
    build_template: Build,
}

/// Modify Cargo.toml in place, using pieces of the original
fn rewrite_cargo_toml(base_path: &Path) -> DefaultResult<()> {
    let cargo_file_path = base_path.join(CARGO_FILE_NAME);
    let mut cargo_file = OpenOptions::new()
        .read(true).write(true).open(cargo_file_path)?;
    let mut contents = String::new();
    cargo_file.read_to_string(&mut contents)?;

    // create new Cargo.toml using pieces of the original
    let zome_name = get_zome_name(base_path);
    let new_toml = generate_cargo_toml(zome_name.as_str(), contents.as_str())?;
    cargo_file.seek(SeekFrom::Start(0))?;
    cargo_file.write_all(new_toml.as_bytes())?;
    Ok(())
}

/// Assuming the base_path is at `[zomename]/code`, get the zome name
fn get_zome_name(base_path: &Path) -> String {
    base_path.parent()
        .and_then(|p| p.to_str())
        .unwrap_or("myzome")
        .replace("/", "-")
}

/// Given existing Cargo.toml string, pull out some values and return a new
/// string with values pulled from template
fn generate_cargo_toml(name: &str, contents: &str) -> DefaultResult<String> {
    let config: Value = toml::from_str(contents)?;

    let authors_default = Value::from("[\"TODO\"]");
    let edition_default = Value::from("\"TODO\"");
    let maybe_package = config.get("package");

    let name = Value::from(name);
    let authors = maybe_package
        .and_then(|p| p.get("authors"))
        .unwrap_or(&authors_default);
    let edition = maybe_package
        .and_then(|p| p.get("edition"))
        .unwrap_or(&edition_default);

    interpolate_cargo_template(&name, authors, edition)
}

/// Use the Cargo.toml.template file and interpolate values into the placeholders
fn interpolate_cargo_template(name: &Value, authors: &Value, edition: &Value) -> DefaultResult<String> {
    let template = include_str!("rust/Cargo.template.toml");
    Ok(
        template
        .replace("<<NAME>>", toml::to_string(name)?.as_str())
        .replace("<<AUTHORS>>", toml::to_string(authors)?.as_str())
        .replace("<<EDITION>>", toml::to_string(edition)?.as_str())
    )
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

        // immediately rewrite the generated Cargo file, using some values
        // and throwing away the rest
        rewrite_cargo_toml(base_path.as_ref())?;

        // create and fill in a build file appropriate for Rust
        let build_file_path = base_path.as_ref().join(package::BUILD_CONFIG_FILE_NAME);
        self.build_template.save_as(build_file_path)?;

        Ok(())
    }
}
