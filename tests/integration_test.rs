use std::process::Command;

#[test]
fn test_detect_stager() {
    let output = Command::new("cargo")
        .args(["run", "--", "--target", "tests/fixtures/payload_stager.sh"])
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Remote execution payload detected"));
}

#[test]
fn test_detect_malicious_build_script() {
    let output = Command::new("cargo")
        .args(["run", "--", "--target", "tests/fixtures/malicious_build.rs"])
        .output()
        .expect("Failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Process execution targeting shell/network utilities"));
}
