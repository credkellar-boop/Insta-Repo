use notify::{Watcher, RecursiveMode, Result};
use std::path::Path;

pub fn watch_repo(path: &str) -> Result<()> {
    let mut watcher = notify::recommended_watcher(|res: notify::Result<notify::Event>| {
        match res {
            Ok(event) => println!("[*] File system event detected: {:?}", event.kind),
            Err(e) => println!("[!] Watch error: {:?}", e),
        }
    })?;

    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
    println!("[+] Watchdog active on {}. Monitoring for new threats...", path);
    Ok(())
}
