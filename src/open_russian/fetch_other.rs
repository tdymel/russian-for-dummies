use crate::{
    model::{Csfr, Other, Phrase, Sentence, WordId},
    open_russian::api::fetch_word,
};

pub async fn fetch_other(id: WordId, translation_index: usize) -> Option<Other> {
    println!("Fetching {id}");
    let word_entry = fetch_word(id).await?;
    let translation = word_entry.translations.get(translation_index);

    Some(Other {
        id: WordId::try_from(word_entry.id as usize).unwrap(),
        translation: translation.map(|t| t.tls.clone()).unwrap_or_default(),
        root: Phrase::from(word_entry.accented.as_str()),
        csfr: word_entry.level.map(Csfr::from).unwrap_or(Csfr::C2Plus),
        example: translation
            .and_then(|t| {
                t.example_ru.clone().map(|ru| Sentence {
                    translation: t.example_tl.clone().unwrap_or_default(),
                    russian: ru,
                })
            })
            .or(word_entry.sentences.first().map(|it| Sentence {
                translation: it.tl.clone(),
                russian: it.ru.clone(),
            })),
        usage: None,
    })
}
