use crate::{
    content::cache::get_noun,
    model::{Category, WordId},
};

pub async fn fruits() -> Category {
    Category::new("Früchte")
        .add(get_noun(WordId::Fruit).await)
        .add(get_noun(WordId::Apple).await)
        .add(get_noun(WordId::Banana).await)
        .add(get_noun(WordId::Orange).await)
        .add(get_noun(WordId::Strawberry).await)
        .add(get_noun(WordId::Blueberry).await)
        .add(get_noun(WordId::Raspberry).await)
        .add(get_noun(WordId::Blackberry).await)
        .add(get_noun(WordId::Pear).await)
        .add(get_noun(WordId::Peach).await)
        .add(get_noun(WordId::Plum).await)
        .add(get_noun(WordId::Cherry).await)
        .add(get_noun(WordId::Kiwi).await)
        .add(get_noun(WordId::Pineapple).await)
        .add(get_noun(WordId::Mango).await)
        .add(get_noun(WordId::Watermelon).await)
        .add(get_noun(WordId::Lemon).await)
        .add(get_noun(WordId::Lime).await)
        .add(get_noun(WordId::Pomegranate).await)
}
