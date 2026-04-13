use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::error::Error;

use crate::content::russian_for_dummies::DECK_NAME;

const ANKI_CONNECT_URL: &str = "http://127.0.0.1:8765";
const APKG_PATH: &str = "/home/shino/Repos/russian_for_dummies/russian_for_dummies.apkg";

#[derive(Debug, Deserialize)]
struct AnkiResponse<T> {
    result: Option<T>,
    error: Option<String>,
}

async fn invoke(action: &str, params: Value) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();

    let body = json!({
        "action": action,
        "version": 6,
        "params": params
    });

    let response = client
        .post(ANKI_CONNECT_URL)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;

    let parsed: AnkiResponse<Value> = response.json().await?;

    if let Some(error) = parsed.error {
        return Err(format!("AnkiConnect error in '{}': {}", action, error).into());
    }

    Ok(parsed.result.unwrap_or(Value::Null))
}

async fn delete_deck() -> Result<(), Box<dyn Error>> {
    invoke(
        "deleteDecks",
        json!({
            "decks": [DECK_NAME],
            "cardsToo": true
        }),
    )
    .await?;

    Ok(())
}

async fn import_apkg() -> Result<(), Box<dyn Error>> {
    let result = invoke(
        "importPackage",
        json!({
            "path": APKG_PATH
        }),
    )
    .await?;

    match result.as_bool() {
        Some(true) => Ok(()),
        Some(false) => Err("importPackage returned false".into()),
        None => Err("importPackage returned unexpected result".into()),
    }
}

async fn trigger_sync() -> Result<(), Box<dyn Error>> {
    invoke("sync", json!({})).await?;
    Ok(())
}

pub async fn sync_deck() -> Result<(), Box<dyn Error>> {
    delete_deck().await?;
    import_apkg().await?;
    trigger_sync().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    sync_deck().await?;
    println!("Deck synced successfully.");
    Ok(())
}
