use crate::{
    content::{a1::a1, fruits::fruits, vegetables::vegetables},
    model::Deck,
};

pub const DECK_NAME: &str = "Russisch für Dummies";

pub async fn create_deck() -> Deck {
    Deck::new(DECK_NAME)
        .add(a1().await)
        .add(vegetables().await)
        .add(fruits().await)
}
