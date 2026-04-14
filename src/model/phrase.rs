use serde::{Deserialize, Serialize};

use crate::model::Word;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Phrase(pub Vec<Word>);

impl Phrase {
    pub fn keep_contains(mut self, pat: &str) -> Self {
        self.0.retain(|it| it.contains(pat));
        self
    }

    #[allow(unused)]
    pub fn keep(mut self, index: usize) -> Self {
        let kept = self.0.remove(index);
        self.0.clear();
        self.0.push(kept);
        self
    }

    pub fn accented(&self) -> String {
        self.0
            .iter()
            .map(|it| it.accented())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl From<&str> for Phrase {
    fn from(value: &str) -> Self {
        Phrase(
            value
                .split([',', ' '])
                .filter(|it| !it.is_empty())
                .map(Word::from_stressed)
                .collect(),
        )
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
