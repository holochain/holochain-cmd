use cli::scaffold::{self, Scaffold};
use error::DefaultResult;
use regex::Regex;
use std::{fs, path::PathBuf};

lazy_static! {
    pub static ref SCAFFOLD_REGEX: Regex = Regex::new(r"^(\S+):(\S+)$").unwrap();
}

struct CapScaffold {
    name: String,
    language: String,
}

pub fn generate(zome_name: PathBuf, capabilities: Vec<String>) -> DefaultResult<()> {
    // Vaildate capabilities
    for cap in &capabilities {
        ensure!(
            SCAFFOLD_REGEX.is_match(&cap),
            "invalid capability scaffold: {}",
            cap
        );
    }

    if !zome_name.exists() {
        fs::create_dir_all(&zome_name)?;
    }

    let caps: Vec<_> = capabilities
        .iter()
        .map(|cap| {
            let matching = SCAFFOLD_REGEX.captures_iter(cap).next().unwrap();

            CapScaffold {
                name: matching[0].to_owned(),
                language: matching[1].to_owned(),
            }
        })
        .collect();

    for cap in caps {
        match cap.language.as_str() {
            "rust" => scaffold(scaffold::rust::RustScaffold, zome_name.join(cap.name))?,
            _ => bail!("unsupported language: {}", cap.language),
        }
    }

    Ok(())
}

fn scaffold<S: Scaffold>(tooling: S, base_path: PathBuf) -> DefaultResult<()> {
    tooling.gen(base_path)
}
