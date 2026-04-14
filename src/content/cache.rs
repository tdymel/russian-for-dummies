use std::{fs::File, path::PathBuf};

use crate::{
    model::{Noun, Verb, WordId},
    open_russian::{fetch_noun, fetch_verb},
};

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

pub async fn get_noun(id: WordId) -> Noun {
    let path = noun_path(id);

    if let Ok(file) = File::open(path)
        && let Ok(noun) = serde_json::from_reader(file)
    {
        return noun;
    }

    let noun = fetch_noun(id).await.expect("Expected to fetch noun!");
    cache_noun(&noun);

    noun
}

pub async fn get_verb(id: WordId) -> Verb {
    let path = verb_path(id);

    if let Ok(file) = File::open(path)
        && let Ok(verb) = serde_json::from_reader(file)
    {
        return verb;
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
