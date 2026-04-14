use crate::{
    content::cache::{get_noun, get_verb},
    model::{Category, WordId},
};

pub async fn a1() -> Category {
    Category::new("A1")
        .add(get_verb(WordId::Can_AbleTo).await)
        .add(get_verb(WordId::ToUnderstand).await)
        .add(get_verb(WordId::ToLove).await)
        .add(get_verb(WordId::ToBuy).await)
        .add(get_verb(WordId::ToHelp).await)
        .add(get_verb(WordId::ToAsk).await)
        .add(get_verb(WordId::ToLive).await)
        .add(get_verb(WordId::ToSpeak).await)
        .add(get_verb(WordId::ToWant).await)
        .add(get_verb(WordId::ToCost).await)
        .add(get_noun(WordId::Person).await.decl_plural_keep("люд"))
        .add(get_noun(WordId::Beer).await)
        .add(get_noun(WordId::Coffee).await)
        .add(get_noun(WordId::Hotel).await)
        .add(get_noun(WordId::Taxi).await)
        .add(get_noun(WordId::Toilet).await)
        .add(get_noun(WordId::Sock).await)
        .add(get_noun(WordId::Bar).await)
        .add(get_noun(WordId::Supermarket).await)
        .add(get_noun(WordId::Tea).await)
        .add(get_noun(WordId::Wine).await)
        .add(get_noun(WordId::Restaurant).await)
        .add(get_noun(WordId::Bill).await)
        .add(get_noun(WordId::Ruble).await)
        .add(get_noun(WordId::October).await)
        .add(get_noun(WordId::House).await)
        .add(get_noun(WordId::Water).await)
        .add(get_noun(WordId::Russia).await)
}
