use reqwest::blocking::Client;

use crate::models::WordEntry;

const BASE_URL: &str = "https://api.dictionaryapi.dev/api/v2/entries";

/// Fetch dictionary entries for `word` in the specified `lang` (e.g. "en", "fr").
pub fn fetch_word(word: &str, lang: &str) -> Result<Vec<WordEntry>, String> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let url = format!("{}/{}/{}", BASE_URL, lang, word.to_lowercase());

    let resp = client
        .get(&url)
        .header("User-Agent", "define/0.1 (rust CLI)")
        .send()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let status = resp.status();
    let text = resp
        .text()
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    // The API returns a JSON object (not array) when a word is not found.
    if !status.is_success() || !text.trim_start().starts_with('[') {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(title) = val.get("title").and_then(|t| t.as_str()) {
                return Err(format!("{}: '{}'", title, word));
            }
        }
        return Err(format!(
            "No definitions found for '{}' (lang: {})",
            word, lang
        ));
    }

    let entries: Vec<WordEntry> =
        serde_json::from_str(&text).map_err(|e| format!("Failed to parse API response: {}", e))?;

    if entries.is_empty() {
        return Err(format!("No definitions found for '{}'", word));
    }

    Ok(entries)
}

/// Stream an audio URL into memory as a Vec<u8>.
pub fn fetch_audio(url: &str) -> Result<Vec<u8>, String> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let resp = client
        .get(url)
        .header("User-Agent", "define/0.1 (rust CLI)")
        .send()
        .map_err(|e| format!("Audio request failed: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Failed to fetch audio (status: {})", resp.status()));
    }

    let bytes = resp
        .bytes()
        .map_err(|e| format!("Failed to read audio bytes: {}", e))?;

    Ok(bytes.to_vec())
}
