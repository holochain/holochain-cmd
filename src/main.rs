extern crate holochain_core;
extern crate holochain_dna;
#[macro_use]
extern crate structopt;

mod cli;
mod project;

use cli::Language;
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
        name = "build", alias = "b", about = "Builds the current Holochain app into a DNA file"
    )]
    Build,
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
    let args = Cli::from_args();

    match args {
        Cli::Web { port } => cli::web(port),
        Cli::Agent => cli::agent(),
        Cli::Build => cli::build(),
        Cli::New { path, language } => cli::new(path, language),
    }
}
