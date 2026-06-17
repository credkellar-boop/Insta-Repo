use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub struct AiAnalyzer {
    client: Client,
    api_key: String,
    model: String,
}

impl AiAnalyzer {
    pub fn new(api_key: &str, model: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            // Allows flexibility for gemini-1.5-pro or gemini-3.1-flash-lite
            model: model.to_string(), 
        }
    }

    pub async fn evaluate_payload(&self, file_path: &str, snippet: &str) -> Result<String, Box<dyn Error>> {
        let endpoint = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let prompt = format!(
            "You are an expert offensive security researcher. Analyze the following code snippet found in '{}'. \
            It triggered a heuristic malware flag. Determine if this code is malicious, what it does, and if it attempts \
            to execute remote payloads or exfiltrate data. Be concise.\n\nCode:\n{}",
            file_path, snippet
        );

        let body = json!({
            "contents": [{
                "parts": [{"text": prompt}]
            }],
            "generationConfig": {
                "temperature": 0.2
            }
        });

        let response = self.client.post(&endpoint)
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let res_json: serde_json::Value = response.json().await?;
            // Extract the text from the Gemini response structure
            if let Some(text) = res_json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                return Ok(text.to_string());
            }
        }

        Err("Failed to parse AI response or hit API limits".into())
    }
}
