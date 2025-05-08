mod rust;
use std::{fs, process};

enum Command {
    UPDATE,
    HELP,
    INSTALL,
}

struct Options {
    command: Option<Command>,
    module: Option<String>,
    binary_directory: Option<String>,
    install_directory: Option<String>,
    compile: Option<bool>,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            command: Some(Command::HELP),
            module: Some(String::from("rust")),
            binary_directory: None,
            install_directory: Some(String::from("/usr/bin")),
            compile: Some(true),
        }
    }
}

fn main() {
    // Check that install directory exists
    match fs::exists("/usr/local/bin") {
        Ok(true) => {
            println!("Install directory OK")
        }
        Ok(false) => {
            eprintln!("Install directory does not exist.");
            process::exit(1)
        }
        Err(e) => {
            eprintln!("Install directory error: {}", e);
            process::exit(1)
        }
    }

    let parsed_args = parse_args();
    let options = generate_options(parsed_args);

    match crate::rust::run(options) {
        Ok(_) => println!("Successfuly installed!"),
        Err(e) => {
            eprintln!("Error installing: {}", e)
        }
    }
}

/**
    Parses command line arguments into `Vec<(String, Option<String>)>`, with the first
    value being an argument and the second value being the argument after.
*/
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

/**
    Returns an `option` struct from given command line arguments
*/
fn generate_options(parsed_args: Vec<(String, Option<String>)>) -> Options {
    let iter = parsed_args.iter();
    let mut options = Options::default();

    if let Some(a) = parsed_args.get(1) {
        match a.0.as_str() {
            "install" => options.command = Some(Command::INSTALL),
            "update" => options.command = Some(Command::UPDATE),
            _ => {}
        }
    }

    for arg in iter {
        match arg.0.as_str() {
            "-h" | "--help" => options.command = Some(Command::HELP),
            "-m" | "--module" => options.module = arg.1.clone(),
            "-d" | "--binary-directory" => options.binary_directory = arg.1.clone(),
            "-i" | "--install-dir" => options.install_directory = arg.1.clone(),
            "-n" | "--no-compile" => options.compile = Some(false),
            _ => {}
        }
    }

    options
}
