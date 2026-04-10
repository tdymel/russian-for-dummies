use crate::{
    content::cache::get_noun,
    model::{Category, WordId},
};

pub async fn fruits() -> Category {
    Category::new("Früchte").add(get_noun(WordId::Fruit).await)
}
