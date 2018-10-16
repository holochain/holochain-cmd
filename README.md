# Holochain Command Line Tools

[![Project](https://img.shields.io/badge/project-holochain-blue.svg?style=flat-square)](http://holochain.org/)
[![PM](https://img.shields.io/badge/pm-waffle-blue.svg?style=flat-square)](https://waffle.io/holochain/org)
[![Chat](https://img.shields.io/badge/chat-chat%2eholochain%2enet-blue.svg?style=flat-square)](https://chat.holochain.net)

This repo provides a set of tools for building and running Holochain DNA from the command line. The tools are written in Rust, and delivered as binary executables.

## Install

These dependencies need to be installed in order to compile, and use `holochain-cmd`:

- [Rust](https://www.rust-lang.org/en-US/install.html)
  - needs to be the `nightly` build, so use the following commands, once you have first installed Rust
  - `rustup toolchain install nightly`
  - `rustup default nightly`
  - Also, if you are going to be developing Zomes in Rust, install the WASM build target for Rust, by running:
  - `rustup target add wasm32-unknown-unknown --toolchain nightly`
- [Node.js](https://nodejs.org) (needed for running tests)
- [hcshell](https://github.com/holochain/holosqape) (also needed for running tests, hcshell is currently installed as part of holosqape)


To install the Holochain command line, run the following commands in a terminal
```shell
$ git clone https://github.com/holochain/holochain-cmd.git
$ cd holochain-cmd
$ git submodule init
$ git submodule update
$ cargo install -f --path .
```

The command line tools are now available in your command line using the `hc` command.
Run `hc -V` to confirm.
Run `hc help` for help.

## Usage

`(u)` means the command is not yet implemented.

| Command   | Use                                                                 |
|-----------|---------------------------------------------------------------------|
| init      | Initializes a new Holochain app at the given directory              |
| generate  | Generates a new Zome                                                |
| package   | Builds the current Holochain app into a `.dna.json` file            |
| unpack    | Unpacks a Holochain bundle into its original file system structure  |
| test      | Runs tests written in the test folder                               |
| web (u)   | Starts a web server for the current Holochain app                   |
| agent (u) | Starts a Holochain node as an agent                                 |

## How To Get Started Building An App

In your terminal, change directories to one where you wish to initialize a new Holochain app.
Run the following, replacing `your_app_name` with your actual app name:
```shell
$ hc init your_app_name
$ cd your_app_name
```

We now have the empty shell of a Holochain app. From here, we will want to generate at least one Zome.
To do this, run the following, replacing `your_zome_name` with a name related to the functionality you wish to develop. For example: `users`.
```shell
$ hc generate zomes/your_zome_name rust
```

Currently, Zomes can be generated as `rust`, or as `assemblyscript`. `hc generate` scaffolds the files and config you need to get started. In the command above, we declared `rust` just to be explicit, even though it's the default language.

What this did is generate a new folder under `zomes` called `users`. Here is the folder structure of it.
- users
  - code
    - src
      - lib.rs
    - .build
    - Cargo.toml
  - zome.json

So in a given Zome we have two things:
1. a JSON file, `zome.json`, which defines and configures the Zome
2. a `code` folder, which can be compiled into a single `WASM` file with the code for this Zome

In order for Holochain to run your app, you have to build your code into a single packaged file. Those instructions follow.

## What are .dna.json files?

A Holochain app can be fully contained in a file known as a `.dna.json` file.
It is a JSON file, with a particular structure that Holochain can understand, and execute.

This is an unusual JSON file; it is part configuration, and part executable.

The configuration part comes from the `json` files that are throughout your app. One at the top level for the application (`app.json`) and one for each Zome. Ultimately, these get stitched together into a single tree structure in the `.dna.json` file.

The executable part comes from having embedded Base64 encoded WebAssembly code in the file. *What does that mean?* [WebAssembly](https://webassembly.org/) is a fast and secure low-level language.
Rather than storing the code in its ugly raw WASM bytecode format, Holochain expects the code to be [encoded using Base64](https://en.wikipedia.org/wiki/Base64) , for legibility and simplicity reasons.

If you haven't heard of WebAssembly (WASM for short), that's ok. Important to know is that WASM is intended as a "compilation target" for other languages, not a language to write code in. So instead of writing code in WASM, write code in a language that's familiar to you, and [supports WASM](https://github.com/appcypher/awesome-wasm-langs). When it's time to run your code in Holochain, compile it.

In order to avoid having to handcraft this complex JSON structure, with lots of room for error, the `hc package` command streamlines the process of taking your "raw" application folder, and packaging it up into the final `.dna.json` file.

More information about this follows.

## Using Built-in Compilation

The `hc package` tool will automate the process of compiling your Zome code, encoding it, and inserting into the `.dna.json` file. In order to get these benefits, you just need to make sure that you have the right compilation tools installed on the machine you are using the command line tools from, and that you have the proper configuration files in your Zome folders.

`hc package` works with two special files called `.hcignore` files and `.build` files.

### .build files
In the process of building a `.dna.json` file, here is what Holochain does.
- It iterates Zome by Zome adding them to the JSON
- For each Zome, it looks for any folders containing a `.build` file
- For any folder with a `.build` file, it executes one or more commands from the `.build` file to create a WASM file
- It takes that built WASM file and Base64 encodes it, then stores a key/value pair for the Zome with the key as the folder name and the encoded WASM as the value

When using `hc generate` to scaffold a Zome, you will have a `.build` file automatically. If you create your Zome manually however, you will need to create the file yourself. Here's the structure of a `.build` file, using a Rust Zome which builds using Cargo as an example:
```json
{
  "steps": {
    "cargo": [
      "build",
      "--release",
      "--target=wasm32-unknown-unknown"
    ]
  },
  "artifact": "target/wasm32-unknown-unknown/release/code.wasm"
}
```

The two top level properties are `steps` and `artifact`. `steps` is a list of commands which will be sequentially executed to build a WASM file. `artifact` is the expected path to the built WASM file. Under `steps`, each key refers to the bin of the command that will be executed, such as `cargo`. The value of `cargo`, the command, is an array of arguments: `build`, and the two `--` flags. In order to determine what should go here, just try running the commands yourself from a terminal, while in the directory of the Zome code.

### Ignoring using .hcignore files

Sometimes, you'll want to exclude files and folders in your project directory to get a straight `.dna.json` file that can be understood by Holochain. In order to do that, just create a `.hcignore` file. It has a similar structure to `.gitignore` files:

```
README.md
dist
.DS_Store
```

The `package` command includes patterns inside `.gitignore` files automatically, so you don't have to write everything twice. Also *hidden* files are ignored by default as well.

### Rust -> WASM compilation tools
If we take Zome code in Rust as an example, you will need Rust and Cargo set up appropriately to build WASM from Rust code. WASM compilation is available on the `nightly` Rust toolchain. To enable it, run the following:
```shell
$ rustup toolchain install nightly
$ rustup target add wasm32-unknown-unknown --toolchain nightly # adds WASM as a compilation target
$ rustup default nightly # switch to the nightly rust toolchain as your default
```

Once that's done, you should be able to run commands like `cargo build --target=wasm32-unknown-unknown` and have it work.

Once all of this is set up, you can build and run your `.dna.json` file with Holochain!

### Writing and Running Tests
By default, when you use `hc init` to create a new project folder, it creates a sub-directory called `test`. The files in that folder are equipped for testing your project. 

Checkout the README of our default testing configuration to understand how to write your tests in Javascript: [https://github.com/holochain/js-tests-scaffold](https://github.com/holochain/js-tests-scaffold).

Once you have a project folder initiated, you can run `hc test` to execute your tests. This combines the following steps:
  1. Packaging your files into a DNA file, located at `dist/bundle.json`. This step will fail if your packaging step fails.
  2. Installing build and testing dependencies, if they're not installed (`npm install`)
  3. Building a single JS file used for testing, placed at `test/dist/bundle.js` (`npm run build`)
  4. Executing (with [hcshell](https://github.com/holochain/holosqape)) the test file found at `test/dist/bundle.js`

`hc test` also has some configurable options.

If you want to run it without repackaging the DNA, run it with
```shell
hc test --no-package
```

If you want to run it without running the `npm` commands, run it with
```shell
hc test --skip-npm
```

If your tests are in a different folder than `test`, run it with
```shell
hc test --dir tests
```
 where `tests` is the name of the folder.

If the file you wish to actually execute is somewhere besides `test/dist/bundle.js` then run it with
```shell
hc test --testfile test/test.js
```
where `test/test.js` is the path of the file.

You have the flexibility to write tests in quite a variety of ways, open to you to explore.

**Note about default configuration with TAPE testing**: If you use the default configuration with Tape for testing, to get an improved CLI visual output (with colors! and accurate script exit codes), we recommend adjusting the command you use to run tests as follows:
```
hc test | test/node_modules/faucet/bin/cmd.js
```


## Contribute
Holochain is an open source project.  We welcome all sorts of participation and are actively working on increasing surface area to accept it.  Please see our [contributing guidelines](https://github.com/holochain/org/blob/master/CONTRIBUTING.md) for our general practices and protocols on participating in the community.

## License
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

Copyright (C) 2018, Holochain Trust

This program is free software: you can redistribute it and/or modify it under the terms of the license p
rovided in the LICENSE file (GPLv3).  This program is distributed in the hope that it will be useful, bu
t WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
 PURPOSE.

**Note:** We are considering other 'looser' licensing options (like MIT license) but at this stage are using GPL while we're getting the matter sorted out.  See [this article](https://medium.com/holochain/licensing-needs-for-truly-p2p-software-a3e0fa42be6c) for some of our thinking on licensing for distributed application frameworks.
