extern crate holochain_core;
extern crate holochain_dna;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod cli;
mod error;
mod project;

use cli::Language;
use error::{HolochainError, HolochainResult};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "A command line for Holochain")]
enum Cli {
    #[structopt(
        name = "web", alias = "w", about = "Starts a web server for the current Holochain app"
    )]
    Web {
        #[structopt(long = "port", short = "p", default_value = "3000")]
        port: u16,
    },
    #[structopt(name = "agent", alias = "a", about = "Starts a Holochain node as an agent")]
    Agent,
    #[structopt(
        name = "package", alias = "b", about = "Builds the current Holochain app into a DNA file"
    )]
    Package,
    #[structopt(
        name = "new", alias = "n", about = "Initializes a new Holochain app at the given directory"
    )]
    New {
        #[structopt(parse(from_os_str))]
        path: PathBuf,
        #[structopt(
            short = "l",
            long = "lang",
            help = "The language of the generated project scaffold",
            default_value = "rust"
        )]
        language: Language,
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
        Cli::Package => cli::package().or_else(|err| Err(HolochainError::Cli(err)))?,
        Cli::New { path, language } => {
            cli::new(path, language).or_else(|err| Err(HolochainError::Default(err)))?
        }
    }

    Ok(())
}
