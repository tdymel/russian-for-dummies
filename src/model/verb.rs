use serde::{Deserialize, Serialize};

use crate::model::{Csfr, Phrase, Sentence, Word, WordId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verb {
    pub id: WordId,
    pub translation: Vec<String>,
    pub root: Phrase,
    pub csfr: Csfr,
    pub present: PresentConjugation,
    pub imperative: ImperativeConjugation,
    pub past: PastConjugation,
    pub participles: Participles,
    pub example: Option<Sentence>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PresentConjugation {
    pub i: Word,
    pub you: Word,
    pub you_they_formal: Word,
    pub he_she_it: Word,
    pub we: Word,
    pub they: Word,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImperativeConjugation {
    pub you: Word,
    pub you_they_formal: Word,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PastConjugation {
    pub masculine: Word,
    pub feminine: Word,
    pub neuter: Word,
    pub plural: Word,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Participles {
    pub active_present: Option<Participle>,
    pub active_past: Option<Participle>,
    pub passive_present: Option<Participle>,
    pub passive_past: Option<Participle>,
    pub gerund_present: Option<Participle>,
    pub gerund_past: Option<Participle>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Participle {
    pub translation: String,
    pub russian: Word,
}
