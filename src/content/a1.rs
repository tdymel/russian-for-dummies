use crate::{
    content::cache::get_verb,
    model::{Category, WordId},
};

pub async fn a1() -> Category {
    Category::new("A1").add(get_verb(WordId::Can_AbleTo).await)
}
