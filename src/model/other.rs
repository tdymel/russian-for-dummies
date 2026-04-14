use serde::{Deserialize, Serialize};

use crate::model::{Csfr, Phrase, Sentence, WordId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Other {
    pub id: WordId,
    pub translation: Vec<String>,
    pub root: Phrase,
    pub csfr: Csfr,
    pub example: Option<Sentence>,
    pub usage: Option<String>
}

impl Other {
    pub fn translation(mut self, translation: &str) -> Self {
        self.translation = vec![translation.to_string()];
        self
    }

    pub fn usage(mut self, usage: &str) -> Self {
        self.usage = Some(usage.to_string());
        self
    }
}
