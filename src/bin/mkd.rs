use std::{
    env::{args, set_current_dir},
    process::exit,
};

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    if args.iter().any(|dir| std::path::Path::new(dir).exists()) {
        eprintln!("One or more directories already exist.");
        exit(1);
    }
    for dir in args.iter() {
        std::fs::create_dir_all(dir).unwrap();
    }
    set_current_dir(&args[0]).unwrap();
}
