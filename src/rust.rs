use colored::Colorize;

use crate::{Command, Options};
use std::io::Result;

pub fn run(options: Options) -> Result<()> {
    match options.build {
        Some(true) | None => {
            build()?;
        }
        Some(false) => println!("Not building binary"),
    }

    match options.command {
        Some(Command::INSTALL) => {
            install(options)?;
            println!("{}", "Successfully installed!".green().bold())
        }
        _ => {}
    }

    Ok(())
}

fn build() -> Result<()> {
    std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()?;

    Ok(())
}

fn install(options: Options) -> Result<()> {
    std::process::Command::new("install")
        .arg(options.binary_path.unwrap_or_default())
        .arg(options.install_directory.unwrap_or_default())
        .status()?;

    Ok(())
}
