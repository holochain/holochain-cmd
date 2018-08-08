use error::CliResult;

pub fn agent() -> CliResult<()> {
    println!("Starting agent...");
    println!("Agent successfully started!");
    println!("Stopping agent...");
    println!("Agent stopped. Bye!");

    Ok(())
}
