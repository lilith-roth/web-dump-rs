mod args;
mod storage;
mod utils;
mod web;
mod wordlist;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use crate::args::{Args, setup_logging};
use crate::storage::{save_content_to_disk};
use crate::utils::{get_output_dir, get_wordlist, prepare_output_dir};
use crate::web::{is_remote_directory, parse_html_and_search_links, retrieve_content_from_web_server};
use crate::wordlist::load_wordlist;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use bytes::Bytes;
use reqwest::blocking::Client;
use url::Url;

fn main() {
    let args: Args = setup_logging();
    log::info!("Welcome to web-dump-rs!\n\n");

    // Load wordlist
    log::info!("Wordlist in use: {}", args.wordlist_path);
    let wordlist: BufReader<File> = get_wordlist(args.wordlist_path);

    let target_url: Arc<String> = Arc::new(args.target_url);
    log::info!("Target URL: {}", target_url);

    let out_dir: PathBuf = get_output_dir(args.output_directory);
    log::info!("Output directory: {:?}", out_dir);
    let out_dir_str: Arc<String> = Arc::new(prepare_output_dir(out_dir));

    // Load wordlist
    let wordlists: Arc<Mutex<Vec<Vec<String>>>> = load_wordlist(wordlist, args.threads as usize);

    crawl_and_download(args.threads, target_url.clone(), out_dir_str.clone(), wordlists.clone(), args.append_slash);

    log::info!("Done! Thanks for using <3");
}

fn crawl_and_download(num_threads: u8, target_url: Arc<String>, out_dir_str: Arc<String>, wordlists: Arc<Mutex<Vec<Vec<String>>>>, append_slash: bool) {
    let mut threads: Vec<JoinHandle<()>> = vec![];
    for i in 0..num_threads {
        // Creating threads
        threads.push(thread::spawn({
            log::debug!("Thread {} spawned", i);
            // Cloning variables into threads
            let url: Arc<String> = target_url.clone();
            let directory: Arc<String> = out_dir_str.clone();
            let wl: Vec<Vec<String>> = wordlists
                .lock()
                .expect("Could not load wordlist into thread!")
                .clone();

            // Doing the magic
            move || {
                let client: Client = Client::new();
                let mut newly_found_links: Vec<String> = download_links_from_list(wl.clone(), i, url.clone(), append_slash, directory.clone(), client.clone());
                while !newly_found_links.is_empty() {
                    newly_found_links = download_links_from_list(wl.clone(), i, url.clone(), append_slash, directory.clone(), client.clone());
                }
            }
        }));
    };

    for active_thread in threads {
        active_thread
            .join()
            .expect("Could not wait for threads to finish!");
    }
}

fn download_links_from_list(wl: Vec<Vec<String>>, thread_id: u8, url: Arc<String>, append_slash: bool, directory: Arc<String>, client: Client) -> Vec<String> {
    let mut found_new_urls_buffer: Vec<String> = vec![];
    for word in wl[thread_id as usize].clone() {
        let mut download_url: String = format!("{url}{word}");
        if append_slash {
            download_url = format!("{download_url}/")
        }
        log::debug!("Thread {} checking {}", thread_id, download_url);

        // Retrieve content from web server
        let content_raw: Option<Bytes> = retrieve_content_from_web_server(&download_url, &client);
        let content: Bytes = match content_raw {
            None => continue,
            Some(res) => res,
        };

        // Create directories for web path
        // create_directories(Url::parse(&download_url).unwrap().path());

        // If the retrieved content is just a directory listing page ignore
        // ToDo: Add switch for storing these as well, will need some work with the directories though,
        //       as otherwise we will have the same filename & directory name which won't work.
        if let Ok(web_content_text) = std::str::from_utf8(&content) {

            // Try to parse HTML, and search all referenced URLs
            let found_links: Vec<String> = parse_html_and_search_links(web_content_text);
            log::info!("HTML Parser found links: {:?}", found_links);
            for link in found_links {
                found_new_urls_buffer.push(link);
            }


            // If remote page is a directory listing, continue with next item in wordlist
            if is_remote_directory(web_content_text) {
                continue;
            }
        }
        log::info!("Thread {} found: {}", thread_id, download_url);

        // Save content to disk
        let url_parse: Url = Url::parse(&download_url).expect("Could not parse URL");
        let dir_path: &str = url_parse.path();
        let file_path: String = format!("{directory}{dir_path}");
        save_content_to_disk(content, file_path);
    }
    found_new_urls_buffer
}
