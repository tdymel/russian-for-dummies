use serde::{Deserialize, Serialize};

use crate::model::Phrase;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Declension {
    pub nominative: Option<Phrase>,
    pub genitive: Option<Phrase>,
    pub dative: Option<Phrase>,
    pub accusative: Option<Phrase>,
    pub instrumental: Option<Phrase>,
    pub prepositional: Option<Phrase>,
}

impl Declension {
    pub fn keep_contains(mut self, pat: &str) -> Self {
        self.nominative = self.nominative.map(|it| it.keep_contains(pat));
        self.genitive = self.genitive.map(|it| it.keep_contains(pat));
        self.dative = self.dative.map(|it| it.keep_contains(pat));
        self.accusative = self.accusative.map(|it| it.keep_contains(pat));
        self.instrumental = self.instrumental.map(|it| it.keep_contains(pat));
        self.prepositional = self.prepositional.map(|it| it.keep_contains(pat));
        self
    }
}
