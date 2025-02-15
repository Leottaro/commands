use std::{env::set_current_dir, path::Path, process::exit};

use zsh_commands::Inputs;

fn main() {
    let inputs = Inputs::parse();
    if inputs.contains_help || inputs.arguments.len() < 2 {
        println!(
            "{} <dir1> [dir2] ...\ncreate the dirs (can create foo/bar) and cd into the first one",
            inputs.name
        );
        exit(0);
    }
    if inputs.arguments.iter().any(|dir| Path::new(dir).exists()) {
        eprintln!("One or more directories already exist.");
        exit(1);
    }
    for dir in inputs.arguments.iter() {
        std::fs::create_dir_all(dir).unwrap();
    }
    set_current_dir(&inputs.arguments[0]).unwrap();
}
