extern crate hc_core;

use hc_core::instance::Instance;
use hc_core::state::Action::*;
use hc_core::nucleus::Action::*;
use hc_core::agent::Action::*;

#[no_mangle]
pub extern fn run_holochain(input: i32) -> i32 {
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
    input + 9
}
