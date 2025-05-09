use crate::Options;
use std::{io::Result, process::Command};

pub fn run(options: Options) -> Result<()> {
    build()?;
    install(options)?;

    Ok(())
}

fn build() -> Result<()> {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()?;

    println!("\nBuilt binary");

    Ok(())
}

fn install(options: Options) -> Result<()> {
    Command::new("cp")
        .arg(options.binary_path.unwrap_or_default())
        .arg(options.install_directory.unwrap_or_default())
        .status()?;

    Ok(())
}
