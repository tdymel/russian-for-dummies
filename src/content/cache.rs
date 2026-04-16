use std::{fs::File, path::PathBuf};

use crate::{
    model::{Noun, Other, Pronoun, Verb, WordId},
    open_russian::{fetch_noun, fetch_other, fetch_pronoun, fetch_verb},
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

fn pronoun_path(id: WordId) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("cache")
        .join("pronouns")
        .join(format!("{id}.json"))
}

fn other_path(id: WordId, translation_index: usize) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("cache")
        .join("others")
        .join(format!("{id}_{translation_index}.json"))
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

pub async fn get_pronoun(id: WordId) -> Pronoun {
    let path = pronoun_path(id);

    if let Ok(file) = File::open(path)
        && let Ok(pronoun) = serde_json::from_reader(file)
    {
        return pronoun;
    }

    let pronoun = fetch_pronoun(id).await.expect("Expected to fetch pronoun!");
    cache_pronoun(&pronoun);

    pronoun
}

pub async fn get_other(id: WordId, translation_index: usize, english_source: bool) -> Other {
    let path = other_path(id, translation_index);

    if let Ok(file) = File::open(path)
        && let Ok(other) = serde_json::from_reader(file)
    {
        return other;
    }

    let other = fetch_other(id, translation_index, english_source)
        .await
        .expect("Expected to fetch other!");
    cache_other(&other, translation_index);

    other
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

fn cache_pronoun(pronoun: &Pronoun) {
    let path = pronoun_path(pronoun.id);

    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, pronoun).unwrap();
}

fn cache_other(other: &Other, translation_index: usize) {
    let path = other_path(other.id, translation_index);

    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, other).unwrap();
}
