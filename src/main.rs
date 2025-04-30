mod args;
mod handler;
mod intro;
mod storage;
mod web;
mod wordlist;

use crate::args::{Args, setup_logging};
use crate::handler::create_threads_and_start_crawler;
use crate::intro::write_welcome_message;
use crate::storage::{get_output_dir, prepare_output_dir};
use crate::web::check_online;
use crate::wordlist::load_wordlist;
use console::style;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn main() {
    let args: Args = setup_logging();
    write_welcome_message();
    log::debug!("Startup args:\n{:?}", args);

    let target_url: Arc<String> = Arc::new(args.target_url);
    log::info!("Target URL: {}", style(target_url.clone()).red());

    // Load wordlist
    log::info!(
        "Wordlist in use: {}",
        style(args.wordlist_path.clone()).red()
    );
    let wordlist: BufReader<File> = match File::open(args.wordlist_path) {
        Ok(res) => BufReader::new(res),
        Err(err) => panic!("Could not read wordlist!\n{:?}", err),
    };

    let out_dir: PathBuf = get_output_dir(args.output_directory);
    log::info!("Output directory: {:?}", style(out_dir.clone()).red());
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
