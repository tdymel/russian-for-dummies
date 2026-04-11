use std::{fs::File, path::PathBuf};

use crate::{
    model::{Noun, WordId},
    open_russian::{fetch_mp3, fetch_noun},
};

pub async fn get_noun(id: WordId) -> Noun {
    if let Some(noun) = fetch_noun_from_cache(id) {
        return noun;
    }

    let noun = fetch_noun(id).await.expect("Expected to fetch noun!");
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

    cache_noun(&noun);

    noun
}

fn noun_path(id: WordId) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("cache")
        .join("nouns")
        .join(format!("{id}.json"))
}

fn fetch_noun_from_cache(id: WordId) -> Option<Noun> {
    let path = noun_path(id);

    let file = File::open(path).ok()?;
    serde_json::from_reader(file).ok()
}

fn cache_noun(noun: &Noun) {
    let path = noun_path(noun.id);

    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, noun).unwrap();
}
