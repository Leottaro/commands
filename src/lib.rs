use std::{collections::HashMap, env::args, ffi::OsString, path::PathBuf, process::exit};

pub struct Inputs {
    pub name: String,
    pub arguments: Vec<String>,
    pub options: HashMap<String, String>,
    pub contains_help: bool,
}

impl Inputs {
    pub fn parse() -> Self {
        let mut args = args();

        let name = args.next().unwrap();
        let mut arguments = Vec::new();
        let mut options = HashMap::new();
        let mut contains_help = false;

        for arg in args.into_iter() {
            if arg == "--help" || arg == "-h" {
                contains_help = true;
                continue;
            }

            let chars: Vec<char> = arg.chars().collect();
            if !chars[0].eq(&'-') {
                arguments.push(arg);
                continue;
            }

            let splits: Vec<&str> = arg.trim_matches('-').split('=').collect();
            if splits.len() != 2 {
                eprintln!("Error while parsing options: Syntax error");
                exit(1);
            }
            options.insert(splits[0].to_string(), splits[1].to_string());
        }

        Self {
            name,
            arguments,
            options,
            contains_help,
        }
    }
}

impl std::fmt::Display for Inputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nArguments: {:?}\nOptions: {:?}\nContains Help: {}",
            self.name, self.arguments, self.options, self.contains_help
        )
    }
}
