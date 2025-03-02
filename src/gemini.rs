use crate::{curl, error::Error, Ask};
use serde_json;

const API_KEY: &str = "GOOGLE_API_KEY";
const GEMINI_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent";

pub struct Gemini;

impl Ask for Gemini {
    fn name() -> &'static str {
        "Gemini"
    }

    async fn ask(query: &str) -> Result<String, Error> {
        let api_key = std::env::var(API_KEY)
            .map_err(|_| Error::new(Self::error_message(API_KEY).as_str()))?;
        let url = format!("{}?key={}", GEMINI_URL, api_key);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let text = format!("{}\nBe concise and to the point. If the question can be answered in a single sentence, do so. Only give more information if the question asks for it.", query);
        let body = serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": text,
                }],
            }],
        });

        let json = curl::post(&url, headers, body.to_string()).await?;

        let answer = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| Error::new("Failed to extract answer from response"))?
            .to_string();
        return Ok(answer);
    }
}
