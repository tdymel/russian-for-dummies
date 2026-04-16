use serde::{Deserialize, Serialize};

use crate::model::{Csfr, Declension, Phrase, Sentence, WordId};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pronoun {
    pub id: WordId,
    pub translation: Vec<String>,
    pub root: Phrase,
    pub csfr: Csfr,
    pub masculine: Declension,
    pub feminine: Declension,
    pub neuter: Declension,
    pub plural: Declension,
    pub example: Option<Sentence>,
}

impl Pronoun {
    pub fn decl_translation(mut self, declension: Declension) -> Self {
        self.feminine = declension;
        self
    }
}