use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::WordId;

pub async fn fetch_word(id: WordId) -> Option<WordEntry> {
    let url = format!(
        "https://api.openrussian.org/api/words/{}?all=&lang=de",
        id.id()
    );

    Some(
        reqwest::Client::new()
            .get(url)
            .send()
            .await
            .ok()?
            .json::<ApiResponse>()
            .await
            .ok()?
            .result,
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub result: WordEntry,
    pub error: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordEntry {
    pub id: i64,
    pub bare: String,
    pub accented: String,
    pub position: String,
    pub audio: Option<String>,
    pub rank: Option<i64>,
    #[serde(rename = "type")]
    pub word_type: String,
    pub level: Option<String>,
    pub usage: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub categories: Vec<Category>,
    #[serde(default)]
    pub translations: Vec<Translation>,
    #[serde(default)]
    pub scenarios: Vec<Value>,
    #[serde(default)]
    pub expressions: Vec<ExpressionEntry>,
    #[serde(default)]
    pub contributions: Vec<Contribution>,
    #[serde(rename = "externalLinks", default)]
    pub external_links: Vec<ExternalLink>,
    #[serde(default)]
    pub relateds: Vec<Related>,
    #[serde(rename = "derivedFromWord")]
    pub derived_from_word: Option<Value>,
    #[serde(rename = "isParticiple")]
    pub is_participle: Option<bool>,
    pub noun: Option<NounData>,
    #[serde(default)]
    pub sentences: Vec<Sentence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub tls: Vec<String>,
    #[serde(rename = "exampleRu")]
    pub example_ru: Option<String>,
    #[serde(rename = "exampleTl")]
    pub example_tl: Option<String>,
    pub info: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub uid: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionEntry {
    pub id: i64,
    pub bare: String,
    pub accented: String,
    pub position: String,
    pub audio: Option<String>,
    pub rank: Option<i64>,
    #[serde(rename = "type")]
    pub word_type: String,
    pub level: Option<String>,
    pub usage: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub categories: Vec<Category>,
    #[serde(default)]
    pub translations: Vec<Translation>,
    #[serde(default)]
    pub scenarios: Vec<Value>,
    pub expression: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contribution {
    pub handlers: String,
    #[serde(rename = "whenUtc")]
    pub when_utc: String,
    pub username: Option<String>,
    pub img: String,
    #[serde(default)]
    pub areas: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalLink {
    pub code: String,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Related {
    pub relation: String,
    pub word: RelatedWord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedWord {
    pub id: i64,
    pub bare: String,
    pub accented: String,
    #[serde(default)]
    pub translations: Vec<RelatedTranslation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedTranslation {
    pub tls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NounData {
    pub gender: String,
    pub partner: Option<Value>,
    pub animate: bool,
    #[serde(rename = "declensionMode")]
    pub declension_mode: String,
    pub declension: Declension,
    #[serde(rename = "hasStressChange")]
    pub has_stress_change: bool,
    #[serde(rename = "declSgAudio")]
    pub decl_sg_audio: String,
    #[serde(rename = "declPlAudio")]
    pub decl_pl_audio: String,
    #[serde(rename = "declFullAudio")]
    pub decl_full_audio: String,
    pub partner2: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Declension {
    pub sg: DeclensionForms,
    pub pl: DeclensionForms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclensionForms {
    pub nom: String,
    pub r#gen: String,
    pub dat: String,
    pub acc: String,
    pub inst: String,
    pub prep: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sentence {
    pub id: i64,
    pub ru: String,
    pub tl: String,
    #[serde(default)]
    pub links: Vec<SentenceLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceLink {
    pub start: i64,
    pub length: i64,
    pub word: LinkedWord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedWord {
    pub id: i64,
    pub word: String,
}
