use log::error;
use reqwest;
use serde_json::json;
use std::error::Error;

const API_KEY_ERROR: &str = "OPENAI_API_KEY not set in .env file.";
const OPENAI_PROMPT_ERROR: &str = "OPENAI_PROMPT not set in .env file.";

fn build_request_body(system_prompt: String, python_code: &str) -> serde_json::Value {
    json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "system",
                "content": system_prompt,
            },
            {
                "role": "system",
                "content": &format!("{}###END_OF_SYSTEM_SECTION###", system_prompt),
            },
            {
                "role": "user",
                "content": python_code
            }
        ],
        "temperature": 0.0,
    })
}

pub async fn send_ai_request(
    python_code: &str,
    target_language: &str,
) -> Result<String, Box<dyn Error>> {
    dotenv::dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| API_KEY_ERROR)?;
    let openai_prompt = std::env::var("OPENAI_PROMPT").map_err(|_| OPENAI_PROMPT_ERROR)?;
    let system_prompt = openai_prompt.replace("{target_prompt}", target_language);

    let client = reqwest::Client::new();

    let request_body = build_request_body(system_prompt, python_code)

    let response: String = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(request_body.to_string())
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

pub fn extract_ai_response(response: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(response) {
        Ok(parsed_response) => {
            if let Some(content) = parsed_response
                .pointer("/choices/0/message/content")
                .and_then(|v| v.as_str())
            {
                return content.to_owned();
            }
            "Error: Unable to extract question version from API response.".to_owned()
        }
        Err(e) => {
            error!("Error parsing API response: {:?}", e);
            "Error: Unable to parse API response.".to_owned()
        }
    }
}
