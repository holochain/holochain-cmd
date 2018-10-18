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

/// Modify Cargo.toml in place, using pieces of the original
fn rewrite_cargo_toml(cargo_file_path: &Path) -> DefaultResult<()> {
    let mut cargo_file = OpenOptions::new()
        .read(true).write(true).open(cargo_file_path)?;
    let mut contents = String::new();
    cargo_file.read_to_string(&mut contents)?;

    // create new Cargo.toml using pieces of the original
    let new_toml = flesh_out_cargo_toml(contents.as_str())?;
    cargo_file.seek(SeekFrom::Start(0))?;
    cargo_file.write_all(new_toml.as_bytes())?;
    Ok(())
}

/// Use the Cargo.toml.template file and interpolate values into the placeholders
fn interpolate_cargo_template(authors: &Value, edition: &Value) -> DefaultResult<String> {
    let authors = toml::to_string(authors)?;
    let edition = toml::to_string(edition)?;
    let template = include_str!("rust/Cargo.toml.template");
    Ok(
        template
        .replace("<<AUTHORS>>", authors.as_str())
        .replace("<<EDITION>>", edition.as_str())
    )
}

/// Given existing Cargo.toml string, pull out some values and return a new
/// string with values pulled from template
fn flesh_out_cargo_toml(contents: &str) -> DefaultResult<String> {
    let config: Value = toml::from_str(contents)?;

    let authors_default = Value::from("[\"TODO\"]");
    let edition_default = Value::from("\"TODO\"");
    let maybe_package = config.get("package");
    let authors = maybe_package
        .and_then(|p| p.get("authors"))
        .unwrap_or(&authors_default);
    let edition = maybe_package
        .and_then(|p| p.get("edition"))
        .unwrap_or(&edition_default);

    interpolate_cargo_template(authors, edition)
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
        let cargo_file_path = base_path.as_ref().join(CARGO_FILE_NAME);
        rewrite_cargo_toml(&cargo_file_path)?;

        // create and fill in a build file appropriate for Rust
        let build_file_path = base_path.as_ref().join(package::BUILD_CONFIG_FILE_NAME);
        self.build_template.save_as(build_file_path)?;

        Ok(())
    }
}
