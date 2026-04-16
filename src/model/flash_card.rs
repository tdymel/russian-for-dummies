use crate::model::{Csfr, Noun, Other, Pronoun, Verb};

#[derive(Debug, Clone)]
pub enum FlashCard {
    Noun(Noun),
    Verb(Verb),
    Pronoun(Pronoun),
    Other(Other),
}

impl FlashCard {
    pub fn csfr(&self) -> Csfr {
        match self {
            FlashCard::Noun(noun) => noun.csfr,
            FlashCard::Verb(verb) => verb.csfr,
            FlashCard::Other(other) => other.csfr,
            FlashCard::Pronoun(pronoun) => pronoun.csfr,
        }
    }
}

impl From<Pronoun> for FlashCard {
    fn from(value: Pronoun) -> Self {
        Self::Pronoun(value)
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
