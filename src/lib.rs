use std::{collections::HashMap, env::args, path::PathBuf, process::exit};

pub struct Inputs {
    pub name: String,
    pub arguments: Vec<String>,
    pub options: HashMap<String, String>,
    pub contains_help: bool,
}

impl Inputs {
    pub fn parse() -> Self {
        let mut args = args();

        let name = args
            .next()
            .unwrap_or_else(|| panic!("Unable to get first arg from Args {:?}", args));
        let mut arguments = Vec::new();
        let mut options = HashMap::new();
        let mut contains_help = false;

        for arg in args {
            if arg == "--help" || arg == "-h" {
                contains_help = true;
                continue;
            }

            let arg_chars = arg.chars();
            if arg.starts_with("--") {
                let arg = arg_chars.skip(2).collect::<String>();
                let splits: Vec<&str> = arg.split('=').collect();
                if splits.len() > 2 {
                    eprintln!(
                        "Error while parsing options: \nSyntax error in arg {}: splits = {:?}",
                        arg, splits
                    );
                    exit(1);
                }
                options.insert(
                    splits[0].to_string(),
                    splits.get(1).unwrap_or(&"").to_string(),
                );
            } else if arg.starts_with("-") {
                options.insert(arg_chars.skip(1).collect(), "".to_string());
            } else {
                arguments.push(arg);
            }
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

pub fn match_path(path1: &PathBuf, path2: &PathBuf) -> bool {
    if path1.eq(path2) {
        return true;
    }

    if let (Some(path1), Some(path2)) = (path1.to_str(), path2.to_str()) {
        if let Ok(pattern) = glob::Pattern::new(path1) {
            return pattern.matches(path2);
        }
    }

    false
}
