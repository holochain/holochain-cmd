# Holochain Command Line

Holochain is being rewritten in Rust as a set of crates that make up the Holochain Library. This means we need a container app to load Holochain and use its features. Using the cmd line wrapper [CLAP](https://github.com/kbknapp/clap-rs ) it was easy to parse the command line and pass those values to Holochain.
As the library progresses we will add more exposed functions to the command line.
