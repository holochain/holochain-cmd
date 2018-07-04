use std::{fs::OpenOptions, io::Write, path::PathBuf, process::Command, str::FromStr};

const SDK_VERSION: &str = "0.1.0";

pub enum Language {
    Rust,
    TypeScript,
}

impl FromStr for Language {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "rust" => Ok(Language::Rust),
            "typescript" => Ok(Language::TypeScript),
            _ => Err("unrecognized language"),
        }
    }
}

pub fn web(port: u16) {
    unimplemented!()
}

pub fn agent() {
    unimplemented!()
}

pub fn build() {
    unimplemented!()
}

pub fn new(path: PathBuf, language: Language) {
    if path.exists() {
        panic!("project already exists");
    }

    let project_name = path.file_name().unwrap();

    match language {
        Language::Rust => {
            Command::new("cargo")
                .arg("new")
                .arg(project_name)
                .arg("--lib")
                .output()
                .unwrap();

            let cargo_file = path.join("Cargo.toml").canonicalize().unwrap();

            let mut file = OpenOptions::new().append(true).open(cargo_file).unwrap();

            let input_line: Vec<_> = format!("holochain_sdk = \"{}\"\n", SDK_VERSION)
                .as_bytes()
                .to_vec();

            file.write_all(&input_line).unwrap();

            println!("Holochain project successfully created");
        }
        _ => unreachable!(),
    }
}
