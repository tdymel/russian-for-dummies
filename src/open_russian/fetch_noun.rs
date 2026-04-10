use crate::{
    model::{Csfr, Declension, Gender, Noun, Phrase, Sentence, Word, WordId},
    open_russian::word_api::fetch_word,
};

pub async fn fetch_noun(id: WordId) -> Option<Noun> {
    println!("Fetching {id}");
    let word_entry = fetch_word(id).await?;
    // println!("{:#?}", word_entry);

    Some(Noun {
        id: WordId::try_from(word_entry.id as usize).unwrap(),
        // TODO
        translation: vec![word_entry.translations[0].tls[0].clone()],
        root: from_accented(&word_entry.accented),
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
        example: word_entry.sentences.get(0).map(|it| Sentence {
            translation: it.tl.clone(),
            russian: it.ru.clone().replace("'", ""),
        }),
    })
}

fn map_declension(declension: &crate::open_russian::word_api::DeclensionForms) -> Declension {
    Declension {
        nominative: map_declension_words(&declension.nom),
        genitive: map_declension_words(&declension.r#gen),
        dative: map_declension_words(&declension.dat),
        accusative: map_declension_words(&declension.acc),
        instrumental: map_declension_words(&declension.inst),
        prepositional: map_declension_words(&declension.prep),
    }
}

fn map_declension_words(word: &str) -> Option<Phrase> {
    if word.is_empty() {
        return None;
    }

    Some(from_accented(word))
}

fn from_accented(accented: &str) -> Phrase {
    Phrase(
        accented
            .split_whitespace()
            .map(Word::from_stressed)
            .collect(),
    )
}
