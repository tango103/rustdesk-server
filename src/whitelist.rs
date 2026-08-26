use std::collections::HashSet;
use std::fs;
use std::sync::Mutex;
use lazy_static::lazy_static;
use hbb_common::log;

lazy_static! {
    static ref WHITELIST: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub fn load(path: &str) {
    let mut set = WHITELIST.lock().unwrap();
    set.clear();
    if let Ok(content) = fs::read_to_string(path) {
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') {
                set.insert(line.to_string());
            }
        }
    }
    log::info!("Whitelist loaded: {} entries", set.len());
}

pub fn contains(id: &str) -> bool {
    let set = WHITELIST.lock().unwrap();
    set.contains(id)
}

pub fn is_empty() -> bool {
    let set = WHITELIST.lock().unwrap();
    set.is_empty()
}
