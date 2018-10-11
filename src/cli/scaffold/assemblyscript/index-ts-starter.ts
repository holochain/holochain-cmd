import {
    debug,
    commit_entry,
    get_entry,
    serialize,
    deserialize,
    stringify
} from "./node_modules/hdk-assemblyscript"

/*

There are decorators available to simplify development.

You can delete the following examples, or modify them to get started.

The following decorator enables an object of a particular class to be converted to a string,
by calling .toString() on it. You DON'T need to do this manually if passing it
into `debug` or `commit_entry`
*/

@can_stringify
class TestClass {
    key: string
    otherKey: i32
}

/*
The following decorator will support the conversion and exporting of your function automatically,
and make it compatible with the Holochain API.
*/

@zome_function
function testfunction(param1: string, param2: i32): string {
    const myTest: TestClass = {
        key: "hello",
        otherKey: 23
    };
    debug(myTest);
    return "something";
}
