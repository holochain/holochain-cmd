# Holochain Command Line

Holochain is being rewritten in Rust as a set of crates that make up the Holochain Library. This means we need a container app to load Holochain and use its features. Using the cmd line wrapper [CLAP](https://github.com/kbknapp/clap-rs ) it was easy to parse the command line and pass those values to Holochain.

```rust
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("philipbeadle <philip.beadle@live.com.au>")
        .about("cmd line for Holochain")
        .subcommand(SubCommand::with_name("web")
          .arg(Arg::with_name("UUID")
                   .required(true)
                   .takes_value(true)
                   .index(1)
                   .help("https://developer.holochain.net/"))
                                      .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))
        
        .get_matches();
    let uuid = matches.value_of("UUID").unwrap();
```
As the library progresses we will add more exposed functions to the command line.
