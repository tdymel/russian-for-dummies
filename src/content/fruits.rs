use crate::{
    content::cache::get_noun,
    model::{Category, WordId},
};

pub async fn fruits() -> Category {
    Category::new("Früchte")
        .add(get_noun(WordId::Fruit).await)
        .add(get_noun(WordId::Apple).await)
        .add(get_noun(WordId::Banana).await)
        // .add(get_noun(WordId::Orange).await) // TODO: ERROR
        .add(get_noun(WordId::Strawberry).await)
        .add(get_noun(WordId::Blueberry).await)
        .add(get_noun(WordId::Raspberry).await)
        .add(get_noun(WordId::Blackberry).await)
}
