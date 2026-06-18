use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize)]
pub struct ScanReport {
    pub target_directory: String,
    pub total_flags_found: usize,
    pub detections: Vec<DetectionRecord>,
}

#[derive(Serialize)]
pub struct DetectionRecord {
    pub file_path: String,
    pub threat_description: String,
    pub ai_analysis: Option<String>,
}

impl ScanReport {
    pub fn export_to_json(&self, output_path: &Path) -> Result<(), std::io::Error> {
        let json_output = serde_json::to_string_pretty(self)?;
        let mut file = File::create(output_path)?;
        file.write_all(json_output.as_bytes())?;
        println!("[+] JSON report successfully written to: {}", output_path.display());
        Ok(())
    }
}
