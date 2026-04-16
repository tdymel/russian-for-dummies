use crate::{
    model::{Csfr, Declension, Phrase, Pronoun, Sentence, WordId},
    open_russian::api::{fetch_word, PronounDeclension},
};

pub async fn fetch_pronoun(id: WordId) -> Option<Pronoun> {
    println!("Fetching {id}");
    let word_entry = fetch_word(id, false).await?;
    let pronoun = word_entry.pronoun.as_ref()?;

    Some(Pronoun {
        id: WordId::try_from(word_entry.id as usize).unwrap(),
        translation: vec![word_entry.translations[0].tls[0].clone()],
        root: Phrase::from(word_entry.accented.as_str()),
        csfr: word_entry.level.map(Csfr::from).unwrap_or(Csfr::C2Plus),
        masculine: map_declension(&pronoun.declension.m),
        feminine: map_declension(&pronoun.declension.f.as_ref().unwrap()),
        neuter: map_declension(&pronoun.declension.n.as_ref().unwrap()),
        plural: map_declension(&pronoun.declension.pl.as_ref().unwrap()),
        example: word_entry.translations[0]
            .example_ru
            .clone()
            .map(|it| Sentence {
                translation: word_entry.translations[0]
                    .example_tl
                    .as_ref()
                    .unwrap()
                    .clone(),
                russian: it,
            })
            .or(word_entry.sentences.first().map(|it| Sentence {
                translation: it.tl.clone(),
                russian: it.ru.clone(),
            })),
    })
}

fn map_declension(declension: &PronounDeclension) -> Declension {
    Declension {
        nominative: map_declension_words(&declension.nom),
        genitive: map_declension_words(&declension.r#gen),
        dative: map_declension_words(&declension.dat),
        accusative: map_declension_words(&declension.acc),
        instrumental: map_declension_words(&declension.inst),
        prepositional: map_declension_words(&declension.prep),
    }
}

fn map_declension_words(phrases: &[String]) -> Option<Phrase> {
    if phrases.is_empty() {
        return None;
    }

    Some(Phrase::from(phrases[0].as_str()))
}
