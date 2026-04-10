use serde::{Deserialize, Serialize};

use crate::model::Word;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Declension {
    pub nominative: Option<Vec<Word>>,
    pub genitive: Option<Vec<Word>>,
    pub dative: Option<Vec<Word>>,
    pub accusative: Option<Vec<Word>>,
    pub instrumental: Option<Vec<Word>>,
    pub prepositional: Option<Vec<Word>>,
}
