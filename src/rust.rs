use crate::Options;
use std::io::Result;

pub fn run(options: Options) -> Result<()> {
    println!(
        "{}",
        options.install_directory.unwrap_or(String::from("default"))
    );
    Ok(())
}
