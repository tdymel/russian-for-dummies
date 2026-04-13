use serde::{Deserialize, Serialize};

use crate::model::{Csfr, Declension, DeclensionType, Gender, Phrase, Sentence, WordId};

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

impl Noun {
    pub fn translation(mut self, trans: &str) -> Self {
        self.translation = vec![trans.to_string()];
        self
    }

    pub fn declension_type(&self) -> DeclensionType {
        match self.root.to_string().chars().last().unwrap() {
            'а' if self.gender == Gender::Female => DeclensionType::F1,
            'я' if self.gender == Gender::Female => DeclensionType::F2,
            'ь' if self.gender == Gender::Female => DeclensionType::F3,
            'а' if self.gender == Gender::Male => DeclensionType::M2,
            'я' if self.gender == Gender::Male => DeclensionType::M3,
            'й' | 'ь' if self.gender == Gender::Male => DeclensionType::M1,
            'о' | 'е' if self.gender == Gender::Neuter => DeclensionType::N,
            c if c.is_alphabetic() && self.gender == Gender::Male => DeclensionType::M1,
            _ => DeclensionType::Irregular,
        }
    }
}
