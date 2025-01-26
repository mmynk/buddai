use reqwest::Client;
use serde_json;

use crate::error::Error;

const API_KEY: &str = "GOOGLE_API_KEY";
const GEMINI_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent";

pub async fn ask_gemini(query: &str) -> Result<String, Error> {
    let api_key = std::env::var(API_KEY)
        .map_err(|_| Error::new("GOOGLE_API_KEY environment variable not set"))?;
    let url = format!("{}?key={}", GEMINI_URL, api_key);

    //let headers = {
    //    "Content-Type": "application/json",
    //};
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    //let body = {
    //    "contents": [{
    //        "parts": [{
    //            "text": query,
    //        }],
    //    }],
    //};
    let text = format!("{}\nBe concise and to the point. If the question can be answered in a single sentence, do so. Only give more information if the question asks for it.", query);
    let body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": text,
            }],
        }],
    });

    let client = Client::new();
    let response = client
        .post(&url)
        .headers(headers)
        .body(body.to_string())
        .send()
        .await
        .map_err(|e| Error::new(&format!("Failed to send request: {}", e)))?;

    let status = response.status();
    if !status.is_success() {
        return Err(Error::new(&format!(
            "Request failed with status code: {}",
            status
        )));
    }

    let text = response
        .text()
        .await
        .map_err(|e| Error::new(&format!("Failed to read response: {}", e)))?;
    let json: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| Error::new(&format!("Failed to parse response: {}", e)))?;

    let answer = json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| Error::new("Failed to extract answer from response"))?
        .to_string();
    return Ok(answer);
}
