use crate::{content::vegetables, model::Deck};

pub async fn create_deck() -> Deck {
    Deck::new("Russisch für Dummies").add(vegetables().await)
}
