#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate boolinator;

// TODO: see issue #25: make this a friendly, minimal, general boilerplate

use boolinator::*;
use hdk::{
    meta::ZomeDefinition,
    holochain_dna::zome::entry_types::Sharing,
};

/// Will soon be replaced by a define_zome! macro
#[no_mangle]
pub extern fn zome_setup(zome_def: &mut ZomeDefinition) {
    zome_def.define(entry!(
        name: "map",
        description: "",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation_function: |_entry: Map, _ctx: hdk::ValidationData| {
            Err(String::from("Not in use yet. Will replace validations! macro below soon."))
        }
    ));
}

#[derive(Serialize, Deserialize)]
struct Map {
    name: String,
    desc: String,
}

validations! {
    [ENTRY] validate_map {
        |map: Map, _ctx: hdk::ValidationData| {
            (map.name.len() < 140)
                .ok_or_else(|| String::from("Name must be less than 140 characters"))
        }
    }
}

// GENESIS
#[no_mangle]
pub extern "C" fn genesis(_offset: i32) -> i32{
    0
}

zome_functions! {
    create_map: |name: String, desc: String| {
        match hdk::commit_entry("map", json!({"name": name, "desc": desc})) {
            Ok(map_address) => {
                json!({"address": map_address})
            },
            Err(hdk_error) => hdk_error.to_json(),
        }
    }
}
