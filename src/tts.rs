use std::fs;
use std::path::PathBuf;

use reqwest::Client;
use serde_json::json;

use crate::utility::hash_to_base64;

pub async fn tts(russian: &str) -> bool {
    let output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("cache")
        .join("mp3")
        .join(format!("{}.mp3", hash_to_base64(&russian.replace("'", ""))));

    if output_path.exists() {
        return true;
    }

    let client = Client::new();

    let response = client
        .post("http://localhost:8000/tts")
        .json(&json!({ "text": russian }))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let bytes = response.bytes().await.unwrap();
    fs::write(output_path, &bytes).unwrap();

    true
}
