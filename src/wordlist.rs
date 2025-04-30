use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};

pub(crate) fn load_wordlist(
    wordlist: BufReader<File>,
    amount_threads: usize,
) -> Arc<Mutex<Vec<Vec<String>>>> {
    let wordlists: Arc<Mutex<Vec<Vec<String>>>> =
        Arc::new(Mutex::new(vec![vec![]; amount_threads]));
    log::info!("Loading wordlist...");
    // Splits wordlist into partial wordlists for N threads
    let mut i: usize = 0;
    for line in wordlist.lines() {
        let wl: Arc<Mutex<Vec<Vec<String>>>> = wordlists.clone();
        let buffer: &mut Vec<String> = &mut wl.lock().expect("Could not access wordlist!")[i];
        buffer.push(line.expect("Could not read line from wordlist!"));
        i += 1;
        if i == amount_threads {
            i = 0;
        }
    }
    log::info!("Wordlist loaded");
    wordlists
}
