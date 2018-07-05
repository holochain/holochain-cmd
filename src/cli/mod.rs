use error::{CliError, CliResult, DefaultResult};
use std::{fs::OpenOptions, io::Write, path::PathBuf, process::Command, str::FromStr};

const SDK_VERSION: &str = "0.1.0";

pub enum Language {
    Rust,
    TypeScript,
}

impl FromStr for Language {
    type Err = CliError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "rust" => Ok(Language::Rust),
            "typescript" => Ok(Language::TypeScript),
            _ => Err(CliError::UnknownLanguage),
        }
    }
}

pub fn web(port: u16) -> CliResult<()> {
    Err(CliError::UnknownLanguage)
}

pub fn agent() -> CliResult<()> {
    unimplemented!()
}

pub fn package() -> CliResult<()> {
    unimplemented!()
}

pub fn new(path: PathBuf, language: Language) -> DefaultResult<()> {
    if path.exists() {
        bail!("project already exists");
    }

    let project_name = path
        .file_name()
        .ok_or_else(|| format_err!("unable to get file name"))?;

    match language {
        Language::Rust => {
            Command::new("cargo")
                .arg("new")
                .arg(project_name)
                .arg("--lib")
                .output()
                .unwrap();

            let cargo_file = path.join("Cargo.toml").canonicalize()?;

            let mut file = OpenOptions::new().append(true).open(cargo_file)?;

            let input_line: Vec<_> = format!("holochain_sdk = \"{}\"\n", SDK_VERSION)
                .as_bytes()
                .to_vec();

            file.write_all(&input_line)?;

            println!("Holochain project successfully created");
        }
        Language::TypeScript => unimplemented!(),
    }

    Ok(())
}
