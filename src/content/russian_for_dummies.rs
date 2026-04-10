use crate::{
    content::{fruits::fruits, vegetables::vegetables},
    model::Deck,
};

pub async fn create_deck() -> Deck {
    Deck::new("Russisch für Dummies")
        .add(vegetables().await)
        .add(fruits().await)
}
