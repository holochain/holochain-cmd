use colored::*;
use error::DefaultResult;
use std::{path::PathBuf, process::Command};

pub fn run_cmd(base_path: PathBuf, bin: String, args: Vec<String>) -> DefaultResult<()> {
    let pretty_command = format!("{} {}", bin.green(), args.join(" ").cyan());

    println!("> {}", pretty_command);

    let status = Command::new(bin)
        .args(args)
        .current_dir(base_path)
        .status()?;

    ensure!(
        status.success(),
        "command {} was not successful",
        pretty_command
    );

    Ok(())
}
