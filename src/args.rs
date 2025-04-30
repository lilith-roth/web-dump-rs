use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,

    #[arg(
        short = 'w',
        long,
        required = true,
        help = "</usr/share/wordlists/wordlist.txt>"
    )]
    pub(crate) wordlist_path: String,

    #[arg(short = 'u', long, required = true, help = "<http://127.0.0.1/>")]
    pub(crate) target_url: String,

    #[arg(short = 'o', long, default_value = "./out/")]
    pub(crate) output_directory: String,

    #[arg(short = 's', long, action = clap::ArgAction::SetTrue, help = "appends '/' to each request [default: false]")]
    pub(crate) append_slash: bool,

    #[arg(short = 't', long, default_value = "1")]
    pub(crate) threads: u8,

    #[arg(short = 'c', long, action = clap::ArgAction::SetFalse, help = "Enables HTML interpreter to find referencing links [default: true]")]
    pub(crate) crawl_html: bool,

    #[arg(short = 'e', long, action = clap::ArgAction::SetTrue, help = "Enables HTML interpreter to also save external domains/hosts [default: false]")]
    pub(crate) crawl_external: bool,
}

pub(crate) fn setup_logging() -> Args {
    let args: Args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    args
}
