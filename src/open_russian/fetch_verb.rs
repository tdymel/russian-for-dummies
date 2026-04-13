use crate::{
    model::{
        Csfr, ImperativeConjugation, Participle, Participles, PastConjugation, Phrase,
        PresentConjugation, Sentence, Verb, Word, WordId,
    },
    open_russian::api::{VerbParticipleEntry, fetch_word},
};

pub async fn fetch_verb(id: WordId) -> Option<Verb> {
    println!("Fetching {id}");
    let word_entry = fetch_word(id).await?;
    let verb = word_entry.verb.as_ref()?;

    Some(Verb {
        id: WordId::try_from(word_entry.id as usize).unwrap(),
        translation: word_entry
            .translations
            .first()
            .map(|t| t.tls.clone())
            .unwrap_or_default(),
        root: from_accented(&word_entry.accented),
        csfr: word_entry.level.map(Csfr::from).unwrap_or(Csfr::C2Plus),
        present: PresentConjugation {
            i: map_word(verb.presfut.first()?)?,
            you: map_word(verb.presfut.get(1)?)?,
            he_she_it: map_word(verb.presfut.get(2)?)?,
            we: map_word(verb.presfut.get(3)?)?,
            you_they_formal: map_word(verb.presfut.get(4)?)?,
            they: map_word(verb.presfut.get(5)?)?,
        },
        imperative: ImperativeConjugation {
            you: map_word(verb.imperatives.first()?)?,
            you_they_formal: map_word(verb.imperatives.get(1)?)?,
        },
        past: PastConjugation {
            masculine: map_word(verb.pasts.first()?)?,
            feminine: map_word(verb.pasts.get(1)?)?,
            neuter: map_word(verb.pasts.get(2)?)?,
            plural: map_word(verb.pasts.get(3)?)?,
        },
        participles: map_participles(verb.participles.as_ref()),
        example: word_entry
            .translations
            .first()
            .and_then(|t| {
                t.example_ru.clone().map(|ru| Sentence {
                    translation: t.example_tl.clone().unwrap_or_default(),
                    russian: ru.replace("'", ""),
                })
            })
            .or(word_entry.sentences.first().map(|it| Sentence {
                translation: it.tl.clone(),
                russian: it.ru.clone().replace("'", ""),
            })),
    })
}

fn map_participles(participles: Option<&crate::open_russian::api::VerbParticiples>) -> Participles {
    Participles {
        active_present: participles.and_then(|p| map_participle(p.active_present.first())),
        active_past: participles.and_then(|p| map_participle(p.active_past.first())),
        passive_present: participles.and_then(|p| map_participle(p.passive_present.first())),
        passive_past: participles.and_then(|p| map_participle(p.passive_past.first())),
        gerund_present: participles.and_then(|p| map_participle(p.gerund_present.first())),
        gerund_past: participles.and_then(|p| map_participle(p.gerund_past.first())),
    }
}

fn map_participle(entry: Option<&VerbParticipleEntry>) -> Option<Participle> {
    let entry = entry?;

    Some(Participle {
        translation: entry
            .translations
            .first()
            .and_then(|t| t.tls.first())
            .cloned()
            .unwrap_or_default(),
        russian: Word::from_stressed(&entry.accented),
    })
}

fn map_word(word: &str) -> Option<Word> {
    if word.is_empty() {
        return None;
    }

    Some(Word::from_stressed(word))
}

fn from_accented(accented: &str) -> Phrase {
    Phrase(
        accented
            .split_whitespace()
            .map(Word::from_stressed)
            .collect(),
    )
}
