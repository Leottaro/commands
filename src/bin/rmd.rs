use std::env::{current_dir, set_current_dir};
use std::fs::remove_dir_all;
use std::io::{stdin, stdout, Write};

fn main() {
    let current_dir = current_dir().unwrap();
    print!("Are you sure you want to delete the current directory and all of its contents? (y/N) ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() != "y" {
        println!("Aborted.");
        return;
    }
    set_current_dir("..").unwrap();
    remove_dir_all(current_dir).unwrap();
}
