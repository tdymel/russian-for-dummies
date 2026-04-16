use crate::{
    model::{Csfr, Declension, Gender, Noun, Phrase, Sentence, WordId},
    open_russian::api::fetch_word,
};

pub async fn fetch_noun(id: WordId) -> Option<Noun> {
    println!("Fetching {id}");
    let word_entry = fetch_word(id, false).await?;
    // println!("{:#?}", word_entry);

    Some(Noun {
        id: WordId::try_from(word_entry.id as usize).unwrap(),
        translation: vec![word_entry.translations[0].tls[0].clone()],
        // translation: word_entry
        //     .translations
        //     .iter()
        //     .map(|it| it.tls[0].clone())
        //     .collect(),
        root: Phrase::from(word_entry.accented.as_str()),
        csfr: word_entry.level.map(Csfr::from).unwrap_or(Csfr::C2Plus),
        gender: word_entry
            .noun
            .as_ref()
            .unwrap()
            .gender
            .as_ref()
            .map(|it| Gender::from(it.clone()))
            .unwrap_or(Gender::Unknown),
        singular: map_declension(&word_entry.noun.as_ref().unwrap().declension.sg),
        plural: map_declension(&word_entry.noun.as_ref().unwrap().declension.pl),
        example: word_entry.translations[0]
            .example_ru
            .clone()
            .map(|it| Sentence {
                translation: word_entry.translations[0]
                    .example_tl
                    .as_ref()
                    .unwrap()
                    .clone(),
                russian: it.clone(),
            })
            .or(word_entry.sentences.first().map(|it| Sentence {
                translation: it.tl.clone(),
                russian: it.ru.clone(),
            })),
    })
}

fn map_declension(declension: &crate::open_russian::api::DeclensionForms) -> Declension {
    Declension {
        nominative: map_declension_words(&declension.nom),
        genitive: map_declension_words(&declension.r#gen),
        dative: map_declension_words(&declension.dat),
        accusative: map_declension_words(&declension.acc),
        instrumental: map_declension_words(&declension.inst),
        prepositional: map_declension_words(&declension.prep),
    }
}

fn map_declension_words(phrase: &str) -> Option<Phrase> {
    if phrase.is_empty() {
        return None;
    }

    Some(Phrase::from(phrase))
}
