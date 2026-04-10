use serde::{Deserialize, Serialize};

use crate::model::{Csfr, Declension, Gender, Phrase, Sentence, WordId};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Noun {
    pub id: WordId,
    pub translation: Vec<String>,
    pub root: Phrase,
    pub csfr: Csfr,
    pub singular: Declension,
    pub plural: Declension,
    pub gender: Gender,
    pub example: Option<Sentence>,
}
