mod rust;
use colored::Colorize;
use serde::Deserialize;
use std::{
    fs,
    io::Result,
    path::{Path, PathBuf},
    process,
};

#[derive(Debug, Deserialize)]
enum Command {
    HELP,
    INSTALL,
}

// Missing fields in the .toml should be filled with default values
#[derive(Debug, Deserialize)]
#[serde(default)]
struct Options {
    command: Option<Command>,
    module: Option<String>,
    binary_path: Option<String>,
    install_directory: Option<String>,
    build: Option<bool>,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            command: Some(Command::HELP),
            module: Some(String::from("rust")),
            binary_path: None,
            install_directory: Some(String::from("/usr/local/bin")),
            build: Some(true),
        }
    }
}

fn main() -> Result<()> {
    let parsed_args = parse_args();
    let project_root = find_config(&std::env::current_dir()?)?;
    let opts = generate_options(parsed_args, project_root)?;

    match fs::exists(opts.install_directory.clone().unwrap_or_default().as_str())? {
        true => {
            println!("Install directory {}", "OK".green().bold())
        }
        false => {
            eprintln!("Install directory {}", "does not exist.".red().bold());
            process::exit(1)
        }
    }

    match opts.command {
        Some(Command::HELP) => {
            println!("THIS WILL BE THE HELP MESSAGE");
            return Ok(());
        }
        _ => {}
    }

    crate::rust::run(opts)?;

    Ok(())
}

/// Parses command line arguments into `Vec<(String, Option<String>)>`, with the first
/// value being an argument and the second value being the argument after.
fn parse_args() -> Vec<(String, Option<String>)> {
    let mut arguments = Vec::new();
    let mut iter = std::env::args().into_iter().peekable();

    while let Some(cur_arg) = iter.next() {
        match iter.peek() {
            Some(next_arg) => arguments.push((cur_arg, Some(next_arg.clone()))),
            None => arguments.push((cur_arg, None)),
        }
    }

    return arguments;
}

/// Returns an `option` struct from given command line arguments
/// and `freighter.toml` path.
fn generate_options<P: AsRef<Path>>(
    parsed_args: Vec<(String, Option<String>)>,
    proj_root: Option<P>,
) -> Result<Options> {
    let mut opts = Options::default();

    // Parse options from proj_root/freighter.toml
    match proj_root {
        Some(s) => {
            let config_str = fs::read_to_string(s.as_ref().join("freighter.toml"))?;
            opts = toml::from_str(&config_str).unwrap();
        }
        None => {}
    }

    let iter = parsed_args.iter();

    // Set command
    if let Some(a) = parsed_args.get(1) {
        match a.0.as_str() {
            "install" => opts.command = Some(Command::INSTALL),
            "help" | _ => opts.command = Some(Command::HELP),
        }
    }

    // Flags n stuff ig
    for arg in iter {
        match arg.0.as_str() {
            "-h" | "--help" => opts.command = Some(Command::HELP),
            "-m" | "--module" => opts.module = arg.1.clone(),
            "-b" | "--binary-path" => opts.binary_path = arg.1.clone(),
            "-i" | "--install-dir" => opts.install_directory = arg.1.clone(),
            "-n" | "--no-build" => opts.build = Some(false),
            _ => {}
        }
    }

    Ok(opts)
}

/// Searches down from working directory to find `freighter.toml` file in project root
pub fn find_config<P: AsRef<Path>>(dir: &P) -> Result<Option<PathBuf>> {
    let components = dir.as_ref().ancestors();

    for dir in components {
        match std::fs::exists(dir.join("freighter.toml"))? {
            true => {
                println!("Found freighter.toml in {}", dir.display());
                return Ok(Some(dir.to_path_buf()));
            }
            false => {}
        }
    }

    println!(
        "{}",
        "Could not find freighter.toml from working directory.".red()
    );

    Ok(None)
}
