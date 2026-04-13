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
            'а' | 'я' if self.gender == Gender::Female || self.gender == Gender::Male => {
                DeclensionType::First
            }

            'й' | 'ь' if self.gender == Gender::Male => DeclensionType::Second,
            'о' | 'е' if self.gender == Gender::Neuter => DeclensionType::Second,
            c if c.is_alphabetic() && self.gender == Gender::Male => DeclensionType::Second,

            'ь' if self.gender == Gender::Female => DeclensionType::Third,

            'м' if matches!(
                self.root.to_string().as_str(),
                "время"
                    | "имя"
                    | "пламя"
                    | "знамя"
                    | "семя"
                    | "темя"
                    | "стремя"
                    | "бремя"
                    | "вымя"
                    | "рамя"
            ) =>
            {
                DeclensionType::Irregular
            }

            _ => DeclensionType::Irregular,
        }
    }
}
