use std::path::PathBuf;
use std::{fs, path};

pub(crate) fn get_output_dir(output_directory: String) -> PathBuf {
    match path::absolute(output_directory) {
        Ok(res) => res,
        Err(err) => panic!(
            "Absolute path to output directory could not be retrieved!\n{:?}",
            err
        ),
    }
}

pub(crate) fn prepare_output_dir(output_directory: PathBuf) -> String {
    if !path::Path::new(&output_directory).exists() {
        fs::create_dir_all(&output_directory).expect("TODO: panic message");
    }
    match output_directory.to_str() {
        None => panic!("Could not convert output directory to string!\nHow did this happen?"),
        Some(res) => String::from(res),
    }
}

pub(crate) fn get_wordlist(wordlist_path: String) -> std::io::BufReader<std::fs::File> {
    match std::fs::File::open(wordlist_path) {
        Ok(res) => std::io::BufReader::new(res),
        Err(err) => panic!("Could not read wordlist!\n{:?}", err),
    }
}
