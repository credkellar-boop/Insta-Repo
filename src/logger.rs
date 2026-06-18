use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

pub fn log_event(message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!("[{}] {}\n", timestamp, message);
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("insta-repo.log")
        .unwrap();
    
    file.write_all(log_entry.as_bytes()).unwrap();
}
