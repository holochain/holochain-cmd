extern crate holochain_core;
extern crate holochain_dna;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate assert_cli;
extern crate base64;
extern crate semver;
extern crate serde_json;
extern crate tempdir;
extern crate uuid;

mod cli;
mod config_files;
mod error;
mod package;

use error::{HolochainError, HolochainResult};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "A command line for Holochain")]
enum Cli {
    #[structopt(
        name = "web",
        alias = "w",
        about = "Starts a web server for the current Holochain app"
    )]
    Web {
        #[structopt(long = "port", short = "p", default_value = "3000")]
        port: u16,
    },
    #[structopt(
        name = "agent",
        alias = "a",
        about = "Starts a Holochain node as an agent"
    )]
    Agent,
    #[structopt(
        name = "package",
        alias = "p",
        about = "Builds the current Holochain app into a .hcpkg file"
    )]
    Package {
        #[structopt(
            long = "strip-meta",
            help = "Strips all __META__ sections off the target bundle. Makes unpacking of the bundle impossible"
        )]
        strip_meta: bool,
    },
    #[structopt(name = "unpack")]
    Unpack {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
        #[structopt(parse(from_os_str))]
        to: PathBuf,
    },
    #[structopt(
        name = "init",
        alias = "i",
        about = "Initializes a new Holochain app at the given directory"
    )]
    Init {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
        #[structopt(
            long = "from",
            help = "Specifies the hash of the DNA the new app should be scaffolded from"
        )]
        from: Option<String>,
    },
    #[structopt(
        name = "generate",
        alias = "g",
        about = "Generates a new zome and scaffolds the given capabilities"
    )]
    Generate {
        #[structopt(help = "The name of the zome that will be generated")]
        zome_name: String,
        #[structopt(
            help = "A list of capabilities that will be scaffolded (e.g. blog:rust web_frontend:typescript)",
            raw(required = "true")
        )]
        capabilities: Vec<String>,
    },
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);

        ::std::process::exit(1);
    }
}

fn run() -> HolochainResult<()> {
    let args = Cli::from_args();

    match args {
        Cli::Web { port } => cli::web(port).or_else(|err| Err(HolochainError::Cli(err)))?,
        Cli::Agent => cli::agent().or_else(|err| Err(HolochainError::Cli(err)))?,
        Cli::Package { strip_meta } => {
            cli::package(strip_meta).or_else(|err| Err(HolochainError::Default(err)))?
        }
        Cli::Unpack { path, to } => {
            cli::unpack(path, to).or_else(|err| Err(HolochainError::Default(err)))?
        }
        Cli::Init { path, from } => {
            cli::new(path, from).or_else(|err| Err(HolochainError::Default(err)))?
        }
        Cli::Generate { .. } => unimplemented!(),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cli::Assert;
    use tempdir::TempDir;

    const HOLOCHAIN_TEST_DIR: &str = "holochain_test";

    fn gen_dir() -> TempDir {
        TempDir::new(HOLOCHAIN_TEST_DIR).unwrap()
    }

    #[test]
    fn generate_without_args() {
        Assert::main_binary()
            .with_args(&["generate"])
            .fails()
            .unwrap();

        Assert::main_binary().with_args(&["g"]).fails().unwrap();
    }

    #[test]
    fn init_base_test() {
        const TEST_DIR: &str = "___init_base_test";

        let tmp_dir = gen_dir();
        let file_path = tmp_dir.path().join(TEST_DIR);

        Assert::main_binary()
            .with_args(&["init", file_path.to_str().unwrap()])
            .succeeds()
            .unwrap();
    }

    #[test]
    fn double_init_test() {
        const TEST_DIR: &str = "___double_init_test";

        let tmp_dir = gen_dir();
        let file_path = tmp_dir.path().join(TEST_DIR);

        Assert::main_binary()
            .with_args(&["init", file_path.to_str().unwrap()])
            .succeeds()
            .unwrap();

        Assert::main_binary()
            .with_args(&["init", file_path.to_str().unwrap()])
            .fails()
            .and()
            .stderr()
            .contains("directory is not empty")
            .unwrap();
    }
}
