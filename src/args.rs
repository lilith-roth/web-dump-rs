use clap::Parser;
use clap_verbosity_flag::InfoLevel;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity<InfoLevel>,

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
}

pub(crate) fn setup_logging() -> Args {
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    args
}
