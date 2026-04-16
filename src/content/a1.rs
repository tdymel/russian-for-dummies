use crate::{
    content::cache::{get_noun, get_other, get_pronoun, get_verb},
    model::{Category, Declension, Phrase, Sentence, WordId},
};

pub async fn a1() -> Category {
    Category::new("A1")
        .add(get_other(WordId::And, 0, false).await)
        .add(get_other(WordId::Not, 0, false).await)
        .add(get_other(WordId::What_That, 1, false).await)
        .add(
            get_other(WordId::With_From_Since, 0, false)
                .await
                .usage("+ Obj-Inst"),
        )
        .add(
            get_other(WordId::With_From_Since, 1, false)
                .await
                .usage("+ Ort-Gen"),
        )
        .add(
            get_other(WordId::With_From_Since, 2, false)
                .await
                .usage("+ Zeit-Gen"),
        )
        .add(
            get_other(WordId::ThatIs, 0, false)
                .await
                .translation("Das (ist)"),
        )
        .add(get_other(WordId::Also_Too, 0, false).await)
        .add(
            get_other(WordId::Possession_By_At_With_From, 0, false)
                .await
                .translation("Besitz ausdrücken")
                .usage("+ Obj-Gen + есть"),
        )
        .add(
            get_other(WordId::Possession_By_At_With_From, 0, false)
                .await
                .translation("Kein Besitz ausdrücken")
                .usage("+ Obj-Gen + нет")
                .example(Sentence {
                    russian: "Но у меня нет де́нег.".to_string(),
                    translation: "Aber ich habe kein Geld.".to_string(),
                }),
        )
        .add(
            get_other(WordId::Possession_By_At_With_From, 1, true)
                .await
                .translation("bei, an, am (Ort)")
                .usage("+ Obj-Gen"),
        )
        .add(
            get_other(WordId::Possession_By_At_With_From, 3, true)
                .await
                .translation("von")
                .usage("+ Obj-Gen"),
        )
        .add(get_other(WordId::From, 0, false).await.usage("+ Ort-Gen"))
        .add(get_other(WordId::Still_Else_Yet, 0, false).await)
        .add(get_other(WordId::Only_Just, 0, false).await)
        .add(get_other(WordId::Only_Just, 1, false).await)
        .add(get_other(WordId::Yes, 0, false).await)
        .add(get_other(WordId::Or, 0, false).await.translation("oder"))
        .add(get_other(WordId::No_There_is_Not, 0, false).await)
        .add(
            get_other(WordId::No_There_is_Not, 1, false)
                .await
                .translation("nicht, kein, es gibt nicht")
                .usage("+ Obj-Gen"),
        )
        .add(get_other(WordId::Where, 0, false).await)
        .add(get_other(WordId::There_Is, 0, false).await)
        .add(get_other(WordId::There, 0, false).await)
        .add(
            get_other(WordId::Afterwards_Then, 0, false)
                .await
                .translation("nachher, später, dann, danach"),
        )
        .add(get_other(WordId::Very, 0, false).await)
        .add(
            get_other(WordId::Without, 0, true)
                .await
                .translation("ohne"),
        )
        .add(
            get_other(WordId::One_Can_May, 0, false)
                .await
                .translation("man kann/darf, es ist möglich/erlaubt"),
        )
        .add(get_other(WordId::Here, 0, false).await)
        .add(get_other(WordId::Good, 0, false).await.translation("gut"))
        .add(get_other(WordId::Today, 0, false).await)
        .add(get_other(WordId::Fast, 0, false).await)
        .add(get_other(WordId::Slowly, 0, false).await)
        .add(get_other(WordId::Difficulty, 0, false).await)
        .add(get_other(WordId::Thanks, 0, false).await)
        .add(get_other(WordId::Please_YouAreWelcome, 0, false).await)
        .add(
            get_other(WordId::Right_Correct, 0, true)
                .await
                .translation("richtig, korrekt"),
        )
        .add(get_other(WordId::Coldly, 0, false).await)
        .add(get_other(WordId::Beautifully, 0, false).await)
        .add(get_other(WordId::Hotly, 0, false).await)
        .add(
            get_other(WordId::Free_Fluently, 0, true)
                .await
                .translation("frei, fließend, ungezwungen"),
        )
        .add(get_other(WordId::Tasty, 0, false).await)
        .add(get_other(WordId::Russian, 0, false).await)
        .add(get_other(WordId::Hello, 0, false).await)
        .add(get_other(WordId::Sorry, 0, false).await)
        .add(get_other(WordId::GoodBye, 0, false).await)
        .add(get_other(WordId::ToBeCalled, 0, false).await)
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
        .add(get_noun(WordId::Work).await)
        .add(get_noun(WordId::House).await)
        .add(get_noun(WordId::Water).await)
        .add(get_noun(WordId::Russia).await)
        .add(get_noun(WordId::ATM).await)
        .add(
            get_noun(WordId::Girl_Girlfriend)
                .await
                .translation("Mädchen, Partnerin, junge Dame"),
        )
        .add(get_pronoun(WordId::I).await)
        .add(get_pronoun(WordId::He).await)
        .add(get_pronoun(WordId::She).await)
        .add(get_pronoun(WordId::It).await)
        .add(get_pronoun(WordId::We).await)
        .add(get_pronoun(WordId::You_They_Formal).await)
        .add(get_pronoun(WordId::They).await)
        .add(get_pronoun(WordId::What_That).await)
        .add(get_pronoun(WordId::Who).await)
        .add(get_pronoun(WordId::Other).await)
        .add(get_pronoun(WordId::HowMuch).await)
}
