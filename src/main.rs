extern crate holochain_core;
extern crate holochain_dna;
extern crate structopt;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate assert_cmd;
extern crate base64;
extern crate colored;
extern crate dir_diff;
extern crate semver;
#[macro_use]
extern crate serde_json;
extern crate ignore;
extern crate tempfile;
extern crate uuid;

mod cli;
mod config_files;
mod error;
mod util;

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
        #[structopt(long = "output", short = "o", parse(from_os_str))]
        output: Option<PathBuf>,
    },
    #[structopt(
        name = "unpack",
        about = "Unpacks a Holochain bundle into it's original file system structure"
    )]
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
        #[structopt(
            help = "The path to the zome that should be generated (usually in ./zomes/)",
            parse(from_os_str)
        )]
        zome: PathBuf,
        #[structopt(
            help = "The language of the generated zome",
            default_value = "rust"
        )]
        language: String,
    },
    #[structopt(
        name = "test",
        alias = "t",
        about = "Runs tests"
    )]
    Test,
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
        Cli::Web { port } => cli::web(port).or_else(|err| Err(HolochainError::Default(err)))?,
        Cli::Agent => cli::agent().or_else(|err| Err(HolochainError::Default(err)))?,
        Cli::Package { strip_meta, output } => {
            cli::package(strip_meta, output).or_else(|err| Err(HolochainError::Default(err)))?
        }
        Cli::Unpack { path, to } => {
            cli::unpack(&path, &to).or_else(|err| Err(HolochainError::Default(err)))?
        }
        Cli::Init { path, from } => {
            cli::new(&path, &from).or_else(|err| Err(HolochainError::Default(err)))?
        }
        Cli::Generate { zome, language } => {
            cli::generate(&zome, &language).or_else(|err| Err(HolochainError::Default(err)))?
        }
        Cli::Test => {
            cli::test().or_else(|err| Err(HolochainError::Default(err)))?
        }
    }

    Ok(())
}
