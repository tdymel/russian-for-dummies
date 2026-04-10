use serde::{Deserialize, Serialize};

use crate::model::Word;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Phrase(pub Vec<Word>);

impl Phrase {
    pub fn accented(&self) -> String {
        self.0
            .iter()
            .map(|it| it.accented())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl std::fmt::Display for Phrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(|it| it.to_string())
            .collect::<Vec<_>>()
            .join(" ")
            .fmt(f)
    }
}
