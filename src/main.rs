mod args;
mod storage;
mod utils;
mod web;
mod wordlist;

use crate::args::{Args, setup_logging};
use crate::storage::save_content_to_disk;
use crate::utils::{get_output_dir, get_wordlist, prepare_output_dir};
use crate::web::{is_remote_directory, parse_html_and_search_links, retrieve_content_from_web_server};
use crate::wordlist::load_wordlist;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use lol_html::{element, HtmlRewriter, Settings};

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

    // Load wordlist
    let wordlists: Arc<Mutex<Vec<Vec<String>>>> = load_wordlist(wordlist, args.threads as usize);

    let mut threads: Vec<JoinHandle<()>> = vec![];
    for i in 0..args.threads {
        // Creating threads
        threads.push(thread::spawn({
            log::debug!("Thread {} spawned", i);
            // Cloning variables into threads
            let url = target_url.clone();
            let directory = out_dir_str.clone();
            let mut wl = wordlists
                .lock()
                .expect("Could not load wordlist into thread!")
                .clone();

            // Doing the magic
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
                        // If remote page is a directory listing, continue with next item in wordlist
                        if is_remote_directory(web_content_text) {
                            continue;
                        }
                        
                        // Try to parse HTML, and search all referenced URLs
                        let found_links: Vec<String> = parse_html_and_search_links(web_content_text);
                        log::debug!("HTML Parser found links: {:?}", found_links);
                        for link in found_links {
                            wl[i as usize].push(link);   
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
        active_thread
            .join()
            .expect("Could not wait for threads to finish!");
    }
    log::info!("Done! Thanks for using <3");
}

