extern crate clap;
extern crate hc_core;

use hc_core::instance::Instance;
use hc_core::state::Action::*;
use hc_core::nucleus::Action::*;
use hc_core::agent::Action::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("philipbeadle <philip.beadle@live.com.au>")
        .about("cmd line for Holochain")
        .arg(Arg::with_name("UUID")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("https://developer.holochain.net/"))
        .get_matches();
    let uuid = matches.value_of("UUID").unwrap();
    println!("Creating instance..");
    let mut instance = Instance::create();

    let dna = hc_core::nucleus::dna::DNA{};
    println!("adding action: {:?}", InitApplication(dna));
    let dna = hc_core::nucleus::dna::DNA{};
    instance.dispatch(Nucleus(InitApplication(dna)));
    println!("pending actions: {:?}", instance.pending_actions());

    let entry = hc_core::common::entry::Entry{};
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

    println!("{}", uuid);
}
