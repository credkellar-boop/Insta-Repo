use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct RuleConfig {
    pub threat_signatures: Vec<Signature>,
}

#[derive(Debug, Deserialize)]
pub struct Signature {
    pub name: String,
    pub description: String,
    pub pattern: String,
    pub severity: String,
}

pub fn load_rules(path: &str) -> Result<RuleConfig, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: RuleConfig = serde_yaml::from_str(&contents)?;
    Ok(config)
}
