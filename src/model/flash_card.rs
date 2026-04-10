use crate::model::Noun;

#[derive(Debug, Clone)]
pub enum FlashCard {
    Noun(Noun),
}

impl From<Noun> for FlashCard {
    fn from(noun: Noun) -> Self {
        FlashCard::Noun(noun)
    }
}
