mod args;
mod utils;

use crate::args::{Args, setup_logging};
use crate::utils::{get_output_dir, get_wordlist, prepare_output_dir};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{fs::File, io::BufRead, thread};

fn main() {
    let args: Args = setup_logging();
    log::info!("Welcome to web-dump-rs!\n\n");

    // Load wordlist
    log::info!("Wordlist in use: {}", args.wordlist_path);
    let wordlist = get_wordlist(args.wordlist_path);

    let target_url: Arc<String> = Arc::new(args.target_url);
    log::info!("Target URL: {}", target_url);

    let out_dir = get_output_dir(args.output_directory);
    log::info!("Output directory: {:?}", out_dir);
    let out_dir_str = Arc::new(prepare_output_dir(out_dir));

    let wordlists: Arc<Mutex<Vec<Vec<String>>>> =
        Arc::new(Mutex::new(vec![vec![]; usize::from(args.threads)]));

    // Load wordlist
    log::info!("Loading wordlist...");
    let mut i = 0;
    for line in wordlist.lines() {
        let wl = wordlists.clone();
        let buffer = &mut wl.lock().expect("Could not access wordlist!")[i];
        buffer.push(line.expect("Could not read line from wordlist!"));
        i += 1;
        if i == args.threads as usize {
            i = 0;
        }
    }
    log::info!("Wordlist loaded");
    log::info!("{}", args.append_slash);

    let mut threads: Vec<JoinHandle<()>> = vec![];
    for i in 0..args.threads {
        threads.push(thread::spawn({
            log::info!("Thread {} spawned", i);
            let url = target_url.clone();
            let directory = out_dir_str.clone();
            let wl = wordlists
                .lock()
                .expect("Could not load wordlist into thread!")
                .clone();
            move || {
                let client = reqwest::blocking::Client::new();
                for word in wl[i as usize].clone() {
                    let mut download_url: String = format!("{url}{word}");
                    if args.append_slash {
                        download_url = format!("{download_url}/")
                    }
                    log::debug!("Thread {} checking {}", i, download_url);

                    // Retrieve content from web server
                    let content_raw = retrieve_content_from_web_server(&download_url, &client);
                    let content = match content_raw {
                        None => continue,
                        Some(res) => res,
                    };

                    // If the retrieved content is just a directory listing page ignore
                    // ToDo: Add switch for storing these as well, will need some work with the directories though,
                    //       as otherwise we will have the same filename & directory name which won't work.
                    if let Ok(web_content_text) = std::str::from_utf8(&content) {
                        if is_remote_directory(web_content_text) {
                            continue;
                        }
                    }
                    log::info!("Thread {} found: {}", i, download_url);

                    // Save content to disk
                    let file_path: String = format!("{directory}{word}");
                    save_content_to_disk(content, file_path)
                }
            }
        }));
    }
    for active_thread in threads {
        active_thread.join().expect("Could not wait for threads to finish!");
    }
    log::info!("Done! Thanks for using <3");
}

fn is_remote_directory(web_content: &str) -> bool {
    let mut result = false;
    // Python3's http.server
    if web_content.contains("Directory listing") {
        result = true;
    }
    // Apache & nginx
    if web_content.contains("Index of") {
        result = true;
    }
    // ToDo: Find a better way for detecting directories
    result
}

fn retrieve_content_from_web_server(
    download_url: &str,
    client: &reqwest::blocking::Client,
) -> Option<bytes::Bytes> {
    let request = client.get(download_url);
    let response = match request.send() {
        Ok(res) => res,
        Err(_) => return None,
    };
    let response_status_code = response.status();
    if response_status_code != 200 {
        return None;
    }
    response.bytes().ok()
}

fn save_content_to_disk(content: bytes::Bytes, file_path: String) {
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
