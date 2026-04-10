use crate::model::FlashCard;

#[derive(Debug, Clone)]
pub struct Category {
    #[allow(unused)]
    pub title: &'static str,
    pub description: Option<&'static str>,
    pub flash_cards: Vec<FlashCard>,
}

impl Category {
    pub fn new(title: &'static str) -> Self {
        Self {
            title,
            description: None,
            flash_cards: Vec::new(),
        }
    }

    #[allow(unused)]
    pub fn with_description(mut self, description: &'static str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn add(mut self, flash_card: impl Into<FlashCard>) -> Self {
        self.flash_cards.push(flash_card.into());
        self
    }
}
