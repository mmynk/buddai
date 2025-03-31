use crate::{curl, error::Error, Ask};
use serde_json;

const API_KEY: &str = "GOOGLE_API_KEY";
const GEMINI_MODEL: &str = "GEMINI_MODEL";
const DEFAULT_MODEL: &str = "gemini-2.0-flash";

pub struct Gemini;

impl Ask for Gemini {
    fn name() -> &'static str {
        "Gemini"
    }

    async fn ask(query: &str) -> Result<String, Error> {
        let api_key = Self::get_api_key(API_KEY)?;
        let url = get_url(&api_key)?;

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

fn get_url(api_key: &str) -> Result<String, Error> {
    let model = std::env::var(GEMINI_MODEL)
        .unwrap_or_else(|_| DEFAULT_MODEL.to_string());
    Ok(format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={api_key}"
    ))
}
