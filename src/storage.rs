use console::style;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, path};

pub(crate) fn save_content_to_disk(content: bytes::Bytes, mut file_path: String) {
    log::debug!("Trying to save new file: {:?}", &file_path);
    if PathBuf::from(&file_path).exists() {
        log::warn!("File {} already exists!", style(file_path).red());
        return
    }
    
    // Making sure the directory exists
    if !Path::new(&file_path).exists() {
        log::debug!(
            "Creating new directory: {:?}",
            Path::new(&file_path)
                .parent()
                .expect("Could not create directory to save content!")
        );

        let mut path: &Path = Path::new(&file_path);
        if !file_path.ends_with("/") {
            path = match Path::new(&file_path).parent() {
                Some(res) => res,
                None => {
                    log::error!(
                        "Could not retrieve directory to store file in!\nHow did this happen??? O.o"
                    );
                    return;
                }
            };
        }
        match fs::create_dir_all(path) {
            Ok(_) => (),
            Err(err) => {
                log::error!("Could not create directory for: {:?}\n{:?}", path, err);
                return;
            }
        }
    }

    if file_path.ends_with("/") {
        file_path += "index.html";
    }
    
    // Creating a new file on disk
    let mut downloaded_file: File = match File::create(&file_path) {
        Ok(res) => res,
        Err(err) => {
            log::error!("Unable to create new file: {}\n{:?}", file_path, err);
            return;
        }
    };

    // Writing content to file
    match downloaded_file.write_all(&content) {
        Ok(_) => log::info!("Saved {}", style(file_path).green()),
        Err(err) => log::error!("Unable to save: {}\n{:?}", file_path, err),
    }
}

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
    if !Path::new(&output_directory).exists() {
        fs::create_dir_all(&output_directory).expect("Could not create output directory!");
    }
    match output_directory.to_str() {
        None => panic!("Could not convert output directory to string!\nHow did this happen?"),
        Some(res) => String::from(res),
    }
}
