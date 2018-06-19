extern crate clap;
extern crate hc_core;

use hc_core::instance::Instance;
use hc_core::state::Action::*;
use hc_core::nucleus::Action::*;
use hc_core::agent::Action::*;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("philipbeadle <philip.beadle@live.com.au>")
        .about("cmd line for Holochain")
        .arg(Arg::with_name("agent")
          .short("a")
          .long("agent")
          .takes_value(true)
          .help("https://developer.holochain.net/"))
        .subcommand(SubCommand::with_name("web")
            .version("0.1.0")
            .author("philipbeadle <philip.beadle@live.com.au>")
            .about("cmd line for Holochain")
            .arg(Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .help("https://developer.holochain.org/Command_Line_Tools#hcdev_web")))
        .get_matches();

        let agent = matches.value_of("agent").unwrap().to_string();
        let mut port = String::new();
        if let Some(matches) = matches.subcommand_matches("web") {
            match matches.value_of("port") {
                Some(value) => port = value.to_string(),
                None => (),
            }
       }

    println!("Creating instance..");
    let mut instance = Instance::create();

    let dna = hc_core::nucleus::dna::DNA{};
    println!("adding action: {:?}", InitApplication(dna));
    let dna = hc_core::nucleus::dna::DNA{};
    instance.dispatch(Nucleus(InitApplication(dna)));
    println!("pending actions: {:?}", instance.pending_actions());

    let entry = hc_core::common::entry::Entry{};
    println!("commit an Agent Entry for Agent {:?}", agent);
    let action = Agent(Commit(entry));
    println!("adding action: {:?}", action);
    instance.dispatch(action);
    println!("pending actions: {:?}", instance.pending_actions());

    let dna = hc_core::nucleus::dna::DNA{};
    instance.dispatch(Nucleus(InitApplication(dna)));

    println!("consuming action...");
    instance.consume_next_action();
    println!("pending actions: {:?}", instance.pending_actions());

    println!("consuming action...");
    instance.consume_next_action();
    println!("pending actions: {:?}", instance.pending_actions());
    instance.consume_next_action();

    println!("Agent {}", agent);
    println!("Port {}", port);
}
