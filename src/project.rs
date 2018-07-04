use std::path::PathBuf;

pub trait Project {
    const TYPE: &'static str;

    fn init(path: PathBuf) -> Self;

    fn build(&mut self);

    fn is_project(path: PathBuf) -> bool;

    fn get_artifact(&mut self) -> Vec<u8>;
}
