use serde::{Deserialize, Serialize};

use crate::model::{Csfr, Declension, Gender, Sentence, Word, WordId};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Noun {
    pub id: WordId,
    pub translation: String,
    pub root: Vec<Word>,
    pub csfr: Csfr,
    pub singular: Declension,
    pub plural: Declension,
    pub gender: Gender,
    pub example: Option<Sentence>,
}
