use crate::model::Category;

#[derive(Debug, Clone)]
pub struct Deck {
    pub title: &'static str,
    pub description: Option<&'static str>,
    pub categories: Vec<Category>,
}

impl Deck {
    pub fn new(title: &'static str) -> Self {
        Self {
            title,
            description: None,
            categories: Vec::new(),
        }
    }

    #[allow(unused)]
    pub fn with_description(mut self, description: &'static str) -> Self {
        self.description = Some(description);
        self
    }

    pub fn add(mut self, category: Category) -> Self {
        self.categories.push(category);
        self
    }
}
