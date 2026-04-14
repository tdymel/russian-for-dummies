use crate::model::{Csfr, Noun, Other, Verb};

#[derive(Debug, Clone)]
pub enum FlashCard {
    Noun(Noun),
    Verb(Verb),
    Other(Other)
}

impl FlashCard {
    pub fn csfr(&self) -> Csfr {
        match self {
            FlashCard::Noun(noun) => noun.csfr,
            FlashCard::Verb(verb) => verb.csfr,
            FlashCard::Other(other) => other.csfr
        }
    }
}

impl From<Noun> for FlashCard {
    fn from(noun: Noun) -> Self {
        FlashCard::Noun(noun)
    }
}

impl From<Verb> for FlashCard {
    fn from(value: Verb) -> Self {
        FlashCard::Verb(value)
    }
}

impl From<Other> for FlashCard {
    fn from(value: Other) -> Self {
        FlashCard::Other(value)
    }
}