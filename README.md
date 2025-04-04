# web-dump-rs

Simple tool to retrieve all files from a web server based on a provided wordlist text file.


## Build

`cargo build --release`


## Run

`web-dump-rs --wordlist-path /usr/share/wordlists/wordlist.txt --target-url http://127.0.0.1`

### Usage
```
Usage: web-dump-rs [OPTIONS] --wordlist-path <WORDLIST_PATH> --target-url <TARGET_URL>

Options:
  -v, --verbose...                           Increase logging verbosity
  -q, --quiet...                             Decrease logging verbosity
  -w, --wordlist-path <WORDLIST_PATH>        </usr/share/wordlists/wordlist.txt>
  -u, --target-url <TARGET_URL>              <http://127.0.0.1/>
  -o, --output-directory <OUTPUT_DIRECTORY>  [default: ./out/]
  -h, --help                                 Print help
  -V, --version                              Print version
```


---

If there are any questions, missing features or problems please create a new issue here on GitHub.

If you're interested in helping this project, you're very welcome to create a pull requests.
