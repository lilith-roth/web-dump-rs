mod args;
mod storage;
mod web;
mod wordlist;

use crate::args::{Args, setup_logging};
use crate::storage::{get_output_dir, prepare_output_dir, save_content_to_disk};
use crate::web::{
    check_online, is_remote_directory, parse_html_and_search_links,
    retrieve_content_from_web_server,
};
use crate::wordlist::load_wordlist;
use bytes::Bytes;
use reqwest::blocking::{Client, Response};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use url::Url;

fn main() {
    let args: Args = setup_logging();
    log::info!("Welcome to web-dump-rs!\n\n");

    // Load wordlist
    log::info!("Wordlist in use: {}", args.wordlist_path);
    let wordlist: BufReader<File> = match File::open(args.wordlist_path) {
        Ok(res) => BufReader::new(res),
        Err(err) => panic!("Could not read wordlist!\n{:?}", err),
    };

    let target_url: Arc<String> = Arc::new(args.target_url);
    log::info!("Target URL: {}", target_url);

    let out_dir: PathBuf = get_output_dir(args.output_directory);
    log::info!("Output directory: {:?}", out_dir);
    let out_dir_str: Arc<String> = Arc::new(prepare_output_dir(out_dir));

    // Load wordlist
    // ToDo: Add switch to enable crawl only mode with no wordlist
    let all_wordlists: Arc<Mutex<Vec<Vec<String>>>> =
        load_wordlist(wordlist, args.threads as usize);

    // Check if target is online
    let is_accessible = check_online(&target_url.clone());
    match is_accessible {
        Ok(_) => log::info!("Target is online! Proceeding..."),
        Err(_) => panic!("Target not online!"),
    }
    create_threads_and_start_crawler(
        args.threads,
        target_url.clone(),
        out_dir_str.clone(),
        all_wordlists.clone(),
        args.append_slash,
        args.crawl_html,
        args.crawl_external,
    );

    log::info!("Done! Thanks for using <3");
}

fn create_threads_and_start_crawler(
    num_threads: u8,
    target_url: Arc<String>,
    out_dir_str: Arc<String>,
    all_wordlists: Arc<Mutex<Vec<Vec<String>>>>,
    append_slash: bool,
    crawl_html: bool,
    crawl_external_domains: bool,
) {
    let mut threads: Vec<JoinHandle<()>> = vec![];
    for i in 0..num_threads {
        // Creating threads
        threads.push(thread::spawn({
            log::debug!("Thread {} spawned", i);
            // Cloning variables into threads
            let url: Arc<String> = target_url.clone();
            let directory: Arc<String> = out_dir_str.clone();
            let thread_wordlist: Vec<String> = all_wordlists
                .lock()
                .expect("Could not load wordlist into thread!")[i as usize]
                .clone();

            // Doing the magic
            move || {
                let client: Client = Client::new();
                // ToDo: Add switch to enable/disable crawling functionality
                let mut newly_found_links: Vec<String> = download_links_from_list(
                    thread_wordlist.clone(),
                    i,
                    url.clone(),
                    append_slash,
                    crawl_html,
                    crawl_external_domains,
                    directory.clone(),
                    &[],
                    client.clone(),
                );
                if crawl_html {
                    let mut checked_links: Vec<String> = thread_wordlist.clone();
                    while !newly_found_links.is_empty() {
                        newly_found_links = download_links_from_list(
                            newly_found_links,
                            i,
                            url.clone(),
                            append_slash,
                            crawl_html,
                            crawl_external_domains,
                            directory.clone(),
                            &checked_links,
                            client.clone(),
                        );
                        checked_links.extend(newly_found_links.clone());
                    }
                }
            }
        }));
    }

    for active_thread in threads {
        active_thread
            .join()
            .expect("Could not wait for threads to finish!");
    }
}

fn download_links_from_list(
    wordlist: Vec<String>,
    thread_id: u8,
    url: Arc<String>,
    append_slash: bool,
    crawl_html: bool,
    crawl_external_domains: bool,
    directory: Arc<String>,
    already_checked_urls: &[String],
    client: Client,
) -> Vec<String> {
    let mut found_new_urls_buffer: Vec<String> = vec![];
    let url_object = match Url::from_str(url.clone().as_str()) {
        Ok(url_object) => url_object,
        Err(err) => {
            log::error!("Can not create URL object, unable to continue!\n{:?}", err);
            return vec![];
        }
    };
    for word in wordlist.clone() {
        // Check if we've already checked the url
        if already_checked_urls.contains(&word) {
            continue;
        }
        let mut download_url: String = match url_object.join(&word) {
            Ok(download_url) => download_url.to_string(),
            Err(err) => {
                log::warn!(
                    "Could not create download URL for word: `{}`! Ignoring...\n{:?}",
                    word,
                    err
                );
                continue;
            }
        };
        let target_domain = url_object.domain().unwrap_or_else(|| {
            log::warn!("Could not retrieve domain from target URL! Ignoring...");
            ""
        });

        if append_slash {
            download_url = format!("{download_url}/")
        }
        log::debug!("Thread {} checking {}", thread_id, download_url);

        // Retrieve content from web server
        let content_raw: Option<Response> =
            retrieve_content_from_web_server(&download_url, &client);
        let content: Response = match content_raw {
            None => continue,
            Some(res) => res,
        };
        let content_url: &Url = content.url();
        log::info!("Thread {} found: {}", thread_id, content_url);
        let url_parse: Url = match Url::parse(content_url.as_str()) {
            Ok(url) => url,
            Err(err) => {
                log::error!("Could not parse url! \n{:?}", err);
                continue;
            }
        };

        let content_bytes_raw: Result<Bytes, _> = content.bytes();
        let content_bytes = match content_bytes_raw {
            Ok(content_bytes) => content_bytes,
            Err(err) => {
                log::error!("Could not parse bytes! \n{:?}", err);
                continue;
            }
        };

        if let Ok(web_content_text) = std::str::from_utf8(&content_bytes) {
            if crawl_html {
                // Try to parse HTML, and search all referenced URLs
                let found_links: Vec<String> = parse_html_and_search_links(
                    web_content_text,
                    target_domain,
                    crawl_external_domains,
                );
                log::info!("HTML Parser found links: {:?}", found_links);
                for link in found_links {
                    found_new_urls_buffer.push(link);
                }
            }

            // If the retrieved content is just a directory listing page ignore
            // ToDo: Add switch for storing these as well, will need some work with the directories though,
            //       as otherwise we will have the same filename & directory name which won't work.
            if is_remote_directory(web_content_text) {
                continue;
            }
        }

        // Save content to disk
        let dir_path: &str = url_parse.path().strip_prefix("/").unwrap();
        let file_path: String = format!("{directory}{dir_path}");
        save_content_to_disk(content_bytes, file_path);
    }
    found_new_urls_buffer
}
