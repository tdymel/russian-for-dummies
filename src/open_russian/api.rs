use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::WordId;

pub async fn fetch_word(id: WordId, english_source: bool) -> Option<WordEntry> {
    let url = format!(
        "https://api.openrussian.org/api/words/{}?all=&lang={}",
        id.id(),
        if english_source { "en" } else { "de" }
    );

    Some(
        reqwest::Client::new()
            .get(url)
            .send()
            .await
            .map_err(|err| {
                println!("ERROR: {:#?}", err);
                err
            })
            .ok()?
            .json::<ApiResponse>()
            .await
            .map_err(|err| {
                println!("ERROR: {:#?}", err);
                err
            })
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
    pub verb: Option<VerbData>,
    pub pronoun: Option<PronounData>,
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
    pub gender: Option<String>,
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
pub struct VerbData {
    pub aspect: Option<String>,
    pub directionality: Option<String>,
    #[serde(rename = "directionalityPartnerWord")]
    pub directionality_partner_word: Option<Value>,
    #[serde(default)]
    pub partners: Vec<String>,
    #[serde(default)]
    pub partners2: Vec<Value>,
    #[serde(default)]
    pub imperatives: Vec<String>,
    #[serde(rename = "imperativesAudio")]
    pub imperatives_audio: Option<String>,
    #[serde(default)]
    pub pasts: Vec<String>,
    #[serde(rename = "pastsAudio")]
    pub pasts_audio: Option<String>,
    #[serde(default)]
    pub presfut: Vec<String>,
    #[serde(default)]
    pub present: Vec<String>,
    #[serde(default)]
    pub future: Vec<String>,
    #[serde(rename = "presfutAudio")]
    pub presfut_audio: Option<String>,
    #[serde(rename = "hasPresfutStressChange")]
    pub has_presfut_stress_change: Option<bool>,
    pub participles: Option<VerbParticiples>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PronounData {
    pub declension: PronounDeclensions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PronounDeclensions {
    pub m: PronounDeclension,
    pub f: Option<PronounDeclension>,
    pub n: Option<PronounDeclension>,
    pub pl: Option<PronounDeclension>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PronounDeclension {
    #[serde(default)]
    pub nom: Vec<String>,
    #[serde(rename = "gen", default)]
    pub r#gen: Vec<String>,
    #[serde(default)]
    pub dat: Vec<String>,
    #[serde(default)]
    pub acc: Vec<String>,
    #[serde(default)]
    pub inst: Vec<String>,
    #[serde(default)]
    pub prep: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerbParticiples {
    #[serde(rename = "activePresent", default)]
    pub active_present: Vec<VerbParticipleEntry>,
    #[serde(rename = "activePast", default)]
    pub active_past: Vec<VerbParticipleEntry>,
    #[serde(rename = "passivePresent", default)]
    pub passive_present: Vec<VerbParticipleEntry>,
    #[serde(rename = "passivePast", default)]
    pub passive_past: Vec<VerbParticipleEntry>,
    #[serde(rename = "gerundPresent", default)]
    pub gerund_present: Vec<VerbParticipleEntry>,
    #[serde(rename = "gerundPast", default)]
    pub gerund_past: Vec<VerbParticipleEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerbParticipleEntry {
    pub accented: String,
    pub id: Option<String>,
    #[serde(default)]
    pub translations: Vec<Translation>,
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
