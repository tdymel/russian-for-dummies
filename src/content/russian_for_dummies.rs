use crate::{
    content::{fruits::fruits, vegetables::vegetables},
    model::Deck,
};

pub const DECK_NAME: &'static str = "Russisch für Dummies";

pub async fn create_deck() -> Deck {
    Deck::new(DECK_NAME)
        .add(vegetables().await)
        .add(fruits().await)
}
