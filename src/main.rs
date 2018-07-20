extern crate holochain_core;
extern crate holochain_dna;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate semver;
extern crate serde_json;
extern crate uuid;

mod cli;
mod config;
mod error;
mod project;

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
    Package,
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
        Cli::Package => cli::package().or_else(|err| Err(HolochainError::Default(err)))?,
        Cli::Init { path, from } => {
            cli::new(path, from).or_else(|err| Err(HolochainError::Default(err)))?
        }
        Cli::Generate { .. } => unimplemented!(),
    }

    Ok(())
}
