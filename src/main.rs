mod args;
mod utils;

use crate::args::setup_logging;
use crate::utils::{get_output_dir, get_wordlist, prepare_output_dir};
use std::io::Write;
use std::path::Path;
use std::{fs::File, io::BufRead};

fn main() {
    let args = setup_logging();
    log::info!("Welcome to web-dump-rs!\n\n");

    // Load wordlist
    log::info!("Wordlist in use: {}", args.wordlist_path);
    let wordlist = get_wordlist(args.wordlist_path);

    let target_url: &str = args.target_url.as_str();
    log::info!("Target URL: {}", target_url);

    let out_dir = get_output_dir(args.output_directory);
    log::info!("Output directory: {:?}", out_dir);
    let out_dir_str = prepare_output_dir(out_dir);

    let client = reqwest::blocking::Client::new();
    for line in wordlist.lines() {
        for word in line.unwrap().split_whitespace() {
            let download_url: String = format!("{target_url}{word}");
            log::debug!("Checking {}", download_url);

            // Retrieve content from web server
            let content_raw = retrieve_content_from_web_server(&download_url, &client);
            let content = match content_raw {
                None => continue,
                Some(res) => res,
            };

            // If the retrieved content is just a directory listing page ignore
            // ToDo: Add switch for storing these as well, will need some work with the directories though,
            //       as otherwise we will have the same filename & directory name which won't work.
            if let Ok(res) = std::str::from_utf8(&content) {
                if res.contains("Directory listing") {
                    continue;
                }
            }

            // Save content to disk
            let file_path: String = format!("{out_dir_str}{word}");
            save_content_to_disk(content, file_path)
        }
    }
    log::info!("Done! Thanks for using <3");
}

fn retrieve_content_from_web_server(
    download_url: &str,
    client: &reqwest::blocking::Client,
) -> Option<bytes::Bytes> {
    let request = client.get(download_url);
    let response = request.send().expect("err");
    let response_status_code = response.status();
    if response_status_code != 200 {
        return None;
    }
    match response.bytes() {
        Ok(res) => Some(res),
        Err(_) => None,
    }
}

fn save_content_to_disk(content: bytes::Bytes, file_path: String) {
    log::debug!("Trying to save new file: {:?}", &file_path);

    // Making sure the directory exists
    if !Path::new(&file_path).parent().unwrap().exists() {
        log::debug!(
            "Creating new directory: {:?}",
            Path::new(&file_path).parent().unwrap()
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
