use crate::{
    content::cache::get_noun,
    model::{Category, WordId},
};

pub async fn vegetables() -> Category {
    Category::new("Gemüse")
        .add(get_noun(WordId::Vegetable).await)
        .add(get_noun(WordId::Garlic).await)
        .add(get_noun(WordId::Tomato).await)
        .add(get_noun(WordId::Broccoli).await)
        .add(get_noun(WordId::Avocado).await)
        .add(get_noun(WordId::Cucumber).await)        
        .add(get_noun(WordId::Carrote).await)
        .add(get_noun(WordId::Onion).await)
        .add(get_noun(WordId::Potato).await)
        .add(get_noun(WordId::Salat).await)
        .add(get_noun(WordId::Cabbage).await)
        .add(get_noun(WordId::Lentil).await)
        .add(get_noun(WordId::Spinach).await)
        .add(get_noun(WordId::Courgette).await)
        .add(get_noun(WordId::Eggplant).await)
        .add(get_noun(WordId::Pumpkin).await)
        .add(get_noun(WordId::Corn).await)
        .add(get_noun(WordId::Pea).await)
        .add(get_noun(WordId::Bean).await)
        .add(get_noun(WordId::Radish).await)
        .add(get_noun(WordId::Argugula).await)
        .add(get_noun(WordId::Asparagus).await)
        .add(get_noun(WordId::Paprika).await)
        .add(get_noun(WordId::Mushroom).await)

    
}
