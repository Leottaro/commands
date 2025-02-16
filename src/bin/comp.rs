use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::{fs::read_dir, process::exit};

use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use leottaro_commands::{match_path, Inputs};

fn tar_dir_without_gitignore(
    src_path: &PathBuf,
    tar_path: &PathBuf,
    tar: Arc<Mutex<tar::Builder<GzEncoder<BufWriter<File>>>>>,
    my_ignored_paths: HashSet<PathBuf>,
    verbose: bool,
) {
    if verbose {
        println!("taring folder {:?} as path {:?}", src_path, tar_path);
    }
    {
        let mut usable_tar = tar.lock().unwrap();
        usable_tar.append_dir(&tar_path, &src_path).expect(&format!(
            "Unable to add folder {:?} to the tar archive as path {:?}",
            src_path, tar_path
        ));
    }

    let paths: Vec<PathBuf> = read_dir(src_path)
        .expect(&format!("Unable to read the directory {:?}", src_path))
        .into_iter()
        .map(|entry| {
            entry
                .expect(&format!(
                    "Unable to read entry from directory {:?}",
                    src_path
                ))
                .path()
        })
        .collect();

    let mut new_ignored_paths = HashSet::new();
    for path in paths.iter() {
        if !path
            .file_name()
            .expect(&format!("Unable to get file_name from path {:?}", path))
            .eq(".gitignore")
        {
            continue;
        }
        let file = File::open(path).expect("Unable to open .gitignore file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let line_path = PathBuf::from(line);
                new_ignored_paths.insert(
                    line_path
                        .strip_prefix("./")
                        .map(|path| path.to_path_buf())
                        .unwrap_or(line_path),
                );
            }
        }
    }

    let join_handles = paths.into_iter().map(|from_path| {
        let my_ignored_paths = my_ignored_paths.clone();
        let new_ignored_paths = new_ignored_paths.clone();
        let tar_path = tar_path.clone();
        let tar = Arc::clone(&tar);
        std::thread::spawn(move || {
            let name = from_path.file_name().expect(&format!(
                "Unable to get file_name from path {:?}",
                from_path
            ));
            let tar_path = tar_path.join(name);

            if new_ignored_paths
                .iter()
                .any(|ignore| match_path(ignore, &PathBuf::from(name)))
                || my_ignored_paths
                    .iter()
                    .any(|ignore| match_path(ignore, &PathBuf::from(name)))
            {
                return;
            }

            if from_path.is_file() {
                if name.eq(".DS_Store") {
                    return;
                }
                if verbose {
                    println!("taring file {:?} as path {:?}", from_path, tar_path);
                }
                let mut file = File::open(&from_path)
                    .expect(&format!("Unable to open file to tar {:?}", from_path));
                {
                    let mut usable_tar = tar.lock().unwrap();
                    usable_tar
                        .append_file(&tar_path, &mut file)
                        .expect(&format!(
                            "Unable to add file {:?} to the tar archive as path {:?}",
                            from_path, tar_path
                        ));
                }
                return;
            }

            let this_path_to_ignore: HashSet<PathBuf> = new_ignored_paths
                .iter()
                .filter_map(|path| {
                    let new_path = path.iter().skip(1).collect::<PathBuf>();
                    if new_path.as_os_str().eq("") {
                        return None;
                    }
                    if let Some(parent) = path.iter().next() {
                        if parent.eq("**") || parent.eq(name) {
                            return Some(new_path);
                        }
                    }
                    if match_path(&path, &PathBuf::from(name)) {
                        return Some(new_path);
                    }

                    None
                })
                .chain(my_ignored_paths.iter().filter_map(|path| {
                    let new_path = path.iter().skip(1).collect::<PathBuf>();
                    if new_path.as_os_str().eq("") {
                        None
                    } else {
                        Some(new_path)
                    }
                }))
                .collect();

            tar_dir_without_gitignore(&from_path, &tar_path, tar, this_path_to_ignore, verbose);
        })
    });

    for handle in join_handles {
        handle.join().expect("Thread panicked");
    }
}

fn main() {
    let inputs = Inputs::parse();
    if inputs.contains_help || inputs.arguments.is_empty() {
        println!(
            "{} <path1> [path2] ...\nOPTIONS: \n\t--compression=[0-9] \tadjust the compression level (default: 6)\n\t-v --verbose     \toutput everything",
            inputs.name
        );
        exit(0);
    }

    let verbose = inputs.options.contains_key("v") || inputs.options.contains_key("verbose");
    let compression_level = {
        let compression = inputs
            .options
            .get("compression")
            .and_then(|str| Some(str.as_str()))
            .unwrap_or("6");
        let compresison_number = compression.parse::<u32>().expect(&format!(
            "Unable to read the compression level of {compression}"
        ));
        Compression::new(compresison_number)
    };

    let existing_paths: Vec<PathBuf> = inputs
        .arguments
        .iter()
        .filter_map(|path_string| {
            let path = PathBuf::from(path_string);
            if path.exists() {
                return Some(path);
            }
            None
        })
        .collect();
    if existing_paths.len() != inputs.arguments.len() {
        eprintln!("One or more paths do not exist.");
        exit(1);
    }

    for src_path in existing_paths {
        let src_path = src_path
            .canonicalize()
            .expect(&format!("Unable to get absolute path of {:?}", src_path));
        let src_filename = src_path
            .file_name()
            .expect(&format!("Unable to get {:?} filename", src_path));

        // Compress the copied directory
        let tar_path = PathBuf::from(src_filename).with_extension("tar.gz");
        println!("taring {:?} as {:?}", src_filename, tar_path);
        let tar_gz =
            File::create(&tar_path).expect(&format!("Unable to create {:?} achive", tar_path));
        let encoder = GzEncoder::new(BufWriter::new(tar_gz), compression_level);
        let tar = Arc::new(Mutex::new(tar::Builder::new(encoder)));
        tar.lock().unwrap().mode(tar::HeaderMode::Deterministic);

        tar_dir_without_gitignore(
            &src_path,
            &PathBuf::from(src_filename),
            tar,
            HashSet::new(),
            verbose,
        );
    }
}
