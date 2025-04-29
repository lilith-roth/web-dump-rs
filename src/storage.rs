use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn save_content_to_disk(content: bytes::Bytes, file_path: String) {
    log::debug!("Trying to save new file: {:?}", &file_path);

    // Making sure the directory exists
    if !Path::new(&file_path)
        .parent()
        .expect("Could not access parent directory!")
        .exists()
    {
        log::debug!(
            "Creating new directory: {:?}",
            Path::new(&file_path)
                .parent()
                .expect("Could not create directory to save content!")
        );
        let path = match Path::new(&file_path).parent() {
            Some(res) => res,
            None => {
                log::error!(
                    "Could not retrieve directory to store file in!\nHow did this happen??? O.o"
                );
                return;
            }
        };
        match std::fs::create_dir_all(path) {
            Ok(_) => (),
            Err(err) => {
                log::error!("Could not create directory for: {:?}\n{:?}", path, err);
                return;
            }
        }
    }

    // Creating a new file on disk
    let downloaded_file: Option<File> = match File::create(&file_path) {
        Ok(res) => Some(res),
        Err(err) => {
            println!("{}", &file_path);
            log::error!("Unable to create new file: {}\n{:?}", file_path, err);
            return;
        }
    };

    // Writing content to file
    match downloaded_file
        .expect("In case of none we return earlier!")
        .write_all(&content)
    {
        Ok(_) => log::info!("Saved {}", file_path),
        Err(err) => log::error!("Unable to save: {}\n{:?}", file_path, err),
    }
}

pub(crate) fn create_directories(path: &str) {
    // std::fs::create_dir_all().unwrap()
    
}
