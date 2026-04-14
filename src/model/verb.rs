use serde::{Deserialize, Serialize};

use crate::model::{Csfr, Phrase, Sentence, Word, WordId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verb {
    pub id: WordId,
    pub translation: Vec<String>,
    pub root: Phrase,
    pub csfr: Csfr,
    pub is_perfective: bool,
    pub present: Conjugation,
    pub future: Conjugation,
    pub imperative: ImperativeConjugation,
    pub past: PastConjugation,
    pub participles: Participles,
    pub example: Option<Sentence>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Conjugation {
    pub i: Phrase,
    pub you: Phrase,
    pub you_they_formal: Phrase,
    pub he_she_it: Phrase,
    pub we: Phrase,
    pub they: Phrase,
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
    pub russian: Phrase,
}
