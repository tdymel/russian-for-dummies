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
