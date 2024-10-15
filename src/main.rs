use std::{error::Error, process::Command};
use clap::Parser;

#[derive(Parser)]
struct Options {
    #[clap()] program: String,
    #[clap()] arguments: Vec<String>
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse();
    let status = Command::new(options.program)
        .args(options.arguments)
        .spawn()?
        .wait()?;

    eprintln!("Exited with code: {}", status.code().unwrap());
    return Ok(());
}
