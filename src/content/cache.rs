use std::{fs::File, path::PathBuf};

use crate::{
    model::{Noun, Verb, WordId},
    open_russian::{fetch_mp3, fetch_noun, fetch_verb},
};

pub async fn get_verb(id: WordId) -> Verb {
    let verb = fetch_verb_try_cached(id).await;

    if let Some(val) = &verb.example {
        fetch_mp3(&val.to_string()).await;
    }

    fetch_mp3(&verb.root.accented()).await;
    fetch_mp3(&verb.present.i.accented()).await;
    fetch_mp3(&verb.present.you.accented()).await;
    fetch_mp3(&verb.present.you_they_formal.accented()).await;
    fetch_mp3(&verb.present.he_she_it.accented()).await;
    fetch_mp3(&verb.present.we.accented()).await;
    fetch_mp3(&verb.present.they.accented()).await;

    fetch_mp3(&verb.imperative.you.accented()).await;
    fetch_mp3(&verb.imperative.you_they_formal.accented()).await;

    fetch_mp3(&verb.past.masculine.accented()).await;
    fetch_mp3(&verb.past.feminine.accented()).await;
    fetch_mp3(&verb.past.neuter.accented()).await;
    fetch_mp3(&verb.past.plural.accented()).await;

    if let Some(val) = &verb.participles.active_present {
        fetch_mp3(&val.russian.accented()).await;
    }
    if let Some(val) = &verb.participles.active_past {
        fetch_mp3(&val.russian.accented()).await;
    }
    if let Some(val) = &verb.participles.passive_present {
        fetch_mp3(&val.russian.accented()).await;
    }
    if let Some(val) = &verb.participles.passive_past {
        fetch_mp3(&val.russian.accented()).await;
    }
    if let Some(val) = &verb.participles.gerund_present {
        fetch_mp3(&val.russian.accented()).await;
    }
    if let Some(val) = &verb.participles.gerund_past {
        fetch_mp3(&val.russian.accented()).await;
    }

    verb
}

pub async fn get_noun(id: WordId) -> Noun {
    let noun = fetch_noun_try_cached(id).await;
    if let Some(val) = &noun.example {
        fetch_mp3(&val.to_string()).await;
    }
    if let Some(val) = &noun.singular.nominative {
        fetch_mp3(&val.accented()).await;
    }
    if let Some(val) = &noun.singular.genitive {
        fetch_mp3(&val.accented()).await;
    }
    if let Some(val) = &noun.singular.dative {
        fetch_mp3(&val.accented()).await;
    }
    if let Some(val) = &noun.singular.accusative {
        fetch_mp3(&val.accented()).await;
    }
    if let Some(val) = &noun.singular.instrumental {
        fetch_mp3(&val.accented()).await;
    }
    if let Some(val) = &noun.singular.prepositional {
        fetch_mp3(&val.accented()).await;
    }
    if let Some(val) = &noun.plural.nominative {
        fetch_mp3(&val.accented()).await;
    }
    if let Some(val) = &noun.plural.genitive {
        fetch_mp3(&val.accented()).await;
    }
    if let Some(val) = &noun.plural.dative {
        fetch_mp3(&val.accented()).await;
    }
    if let Some(val) = &noun.plural.accusative {
        fetch_mp3(&val.accented()).await;
    }
    if let Some(val) = &noun.plural.instrumental {
        fetch_mp3(&val.accented()).await;
    }
    if let Some(val) = &noun.plural.prepositional {
        fetch_mp3(&val.accented()).await;
    }

    noun
}

fn noun_path(id: WordId) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("cache")
        .join("nouns")
        .join(format!("{id}.json"))
}

fn verb_path(id: WordId) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("cache")
        .join("verbs")
        .join(format!("{id}.json"))
}

async fn fetch_noun_try_cached(id: WordId) -> Noun {
    let path = noun_path(id);

    if let Some(file) = File::open(path).ok() {
        if let Some(noun) = serde_json::from_reader(file).ok() {
            return noun;
        }
    }

    let noun = fetch_noun(id).await.expect("Expected to fetch noun!");
    cache_noun(&noun);

    noun
}

async fn fetch_verb_try_cached(id: WordId) -> Verb {
    let path = verb_path(id);

    if let Some(file) = File::open(path).ok() {
        if let Some(verb) = serde_json::from_reader(file).ok() {
            return verb;
        }
    }

    let verb = fetch_verb(id).await.expect("Expected to fetch verb!");
    cache_verb(&verb);

    verb
}

fn cache_noun(noun: &Noun) {
    let path = noun_path(noun.id);

    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, noun).unwrap();
}

fn cache_verb(verb: &Verb) {
    let path = verb_path(verb.id);

    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, verb).unwrap();
}
