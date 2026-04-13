use std::{fs, path::PathBuf};

use crate::{
    genanki::models::{NOUN_MODEL, VERB_MODEL},
    model::{Deck, DeclensionType, FlashCard, Gender, Phrase, Sentence, Word},
    utility::hash_to_base64,
};

use genanki_rs::{Deck as AnkiDeck, Note, Package};

trait ToTemplate {
    fn to_template(&self) -> String;
}

impl ToTemplate for Gender {
    fn to_template(&self) -> String {
        match self {
            Gender::Male => "M".to_string(),
            Gender::Female => "F".to_string(),
            Gender::Neuter => "N".to_string(),
            Gender::Unknown => "?".to_string(),
        }
    }
}

impl ToTemplate for Phrase {
    fn to_template(&self) -> String {
        self.0
            .iter()
            .map(|it| it.to_template())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl ToTemplate for Word {
    fn to_template(&self) -> String {
        let mut cursor = 0;
        let stress_index = self.stress.unwrap_or(usize::MAX);
        self.syllables()
            .iter()
            .map(|it| {
                let num_chars = it.chars().count();
                if stress_index >= cursor && stress_index < cursor + num_chars {
                    let mut new_str = String::new();
                    for (index, char) in it.chars().enumerate() {
                        if index + cursor == stress_index {
                            new_str
                                .push_str(&format!("<span class=\"rfd-stressed\">{}</span>", char));
                        } else {
                            new_str.push(char);
                        }
                    }
                    cursor += num_chars;
                    new_str
                } else {
                    cursor += num_chars;
                    it.clone()
                }
            })
            .collect::<Vec<_>>()
            .join("-")
    }
}

impl ToTemplate for Option<Phrase> {
    fn to_template(&self) -> String {
        self.as_ref()
            .map(|it| it.to_template())
            .unwrap_or("-".to_string())
    }
}

impl ToTemplate for Option<Sentence> {
    fn to_template(&self) -> String {
        self.as_ref()
            .map(|it| it.russian.clone())
            .unwrap_or("".to_string())
    }
}

impl ToTemplate for DeclensionType {
    fn to_template(&self) -> String {
        match self {
            DeclensionType::Irregular => "I".to_string(),
            DeclensionType::F1 => "F1".to_string(),
            DeclensionType::F2 => "F2".to_string(),
            DeclensionType::F3 => "F3".to_string(),
            DeclensionType::M1 => "M1".to_string(),
            DeclensionType::M2 => "M2".to_string(),
            DeclensionType::M3 => "M3".to_string(),
            DeclensionType::N => "N".to_string(),
        }
    }
}

trait ToAudio {
    fn to_audio(&self, id: &str) -> String;
}

impl<T: ToAudio> ToAudio for Option<T> {
    fn to_audio(&self, id: &str) -> String {
        self.as_ref()
            .map(|it| it.to_audio(id))
            .unwrap_or("".to_string())
    }
}

impl ToAudio for Phrase {
    fn to_audio(&self, id: &str) -> String {
        format!(
            "<audio id=\"{}\" src=\"{}.mp3\"></audio>",
            id,
            hash_to_base64(&self.accented().replace("'", ""))
        )
    }
}

impl ToAudio for Sentence {
    fn to_audio(&self, id: &str) -> String {
        format!(
            "<audio id=\"{}\" src=\"{}.mp3\"></audio>",
            id,
            hash_to_base64(&self.to_string())
        )
    }
}

impl ToAudio for Word {
    fn to_audio(&self, id: &str) -> String {
        format!(
            "<audio id=\"{}\" src=\"{}.mp3\"></audio>",
            id,
            hash_to_base64(&self.accented().replace("'", ""))
        )
    }
}

trait ToNote {
    fn to_note(self) -> Note;
}

impl ToNote for FlashCard {
    fn to_note(self) -> Note {
        match self {
            FlashCard::Verb(verb) => Note::new(
                VERB_MODEL.clone(),
                vec![
                    &verb.translation.join(", "),
                    &verb.root.to_template(),
                    &verb.present.i.to_template(),
                    &verb.present.you.to_template(),
                    &verb.present.he_she_it.to_template(),
                    &verb.present.we.to_template(),
                    &verb.present.you_they_formal.to_template(),
                    &verb.present.they.to_template(),
                    &verb.imperative.you.to_template(),
                    &verb.imperative.you_they_formal.to_template(),
                    &verb.past.masculine.to_template(),
                    &verb.past.feminine.to_template(),
                    &verb.past.neuter.to_template(),
                    &verb.past.plural.to_template(),
                    &verb.example.to_template(),
                    &verb.root.to_audio("root"),
                    &verb.present.i.to_audio("Present1"),
                    &verb.present.you.to_audio("Present2"),
                    &verb.present.he_she_it.to_audio("Present3"),
                    &verb.present.we.to_audio("Present4"),
                    &verb.present.you_they_formal.to_audio("Present5"),
                    &verb.present.they.to_audio("Present6"),
                    &verb.imperative.you.to_audio("Imp1"),
                    &verb.imperative.you_they_formal.to_audio("Imp2"),
                    &verb.past.masculine.to_audio("Past1"),
                    &verb.past.feminine.to_audio("Past2"),
                    &verb.past.neuter.to_audio("Past3"),
                    &verb.past.plural.to_audio("Past4"),
                    &verb.example.to_audio("example"),
                ],
            )
            .unwrap(),
            FlashCard::Noun(noun) => Note::new(
                NOUN_MODEL.clone(),
                vec![
                    &noun.translation.join(", "),
                    &noun.root.to_template(),
                    &noun.declension_type().to_template(),
                    &noun.singular.nominative.to_template(),
                    &noun.plural.nominative.to_template(),
                    &noun.singular.genitive.to_template(),
                    &noun.plural.genitive.to_template(),
                    &noun.singular.dative.to_template(),
                    &noun.plural.dative.to_template(),
                    &noun.singular.accusative.to_template(),
                    &noun.plural.accusative.to_template(),
                    &noun.singular.instrumental.to_template(),
                    &noun.plural.instrumental.to_template(),
                    &noun.singular.prepositional.to_template(),
                    &noun.plural.prepositional.to_template(),
                    &noun.example.to_template(),
                    &noun.singular.nominative.to_audio("sn"),
                    &noun.plural.nominative.to_audio("pn"),
                    &noun.singular.instrumental.to_audio("si"),
                    &noun.plural.instrumental.to_audio("pi"),
                    &noun.singular.dative.to_audio("sd"),
                    &noun.plural.dative.to_audio("pd"),
                    &noun.singular.genitive.to_audio("sg"),
                    &noun.plural.genitive.to_audio("pg"),
                    &noun.singular.accusative.to_audio("sa"),
                    &noun.plural.accusative.to_audio("pa"),
                    &noun.singular.prepositional.to_audio("sp"),
                    &noun.plural.prepositional.to_audio("pp"),
                    &noun.example.to_audio("example"),
                ],
            )
            .unwrap(),
        }
    }
}

pub trait CompileAnkiDeck {
    fn compile(self);
}

impl CompileAnkiDeck for Deck {
    fn compile(self) {
        let mut deck = AnkiDeck::new(
            1234,
            self.title,
            self.description.unwrap_or("Potato; Potato"),
        );

        let mut flash_cards = self
            .categories
            .into_iter()
            .flat_map(|it| it.flash_cards)
            .collect::<Vec<_>>();

        flash_cards.sort_by(|l, r| l.csfr().cmp(&r.csfr()));

        for flash_card in flash_cards {
            deck.add_note(flash_card.to_note());
        }

        let media_files = fs::read_dir(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("cache")
                .join("mp3"),
        )
        .unwrap()
        .map(|it| it.unwrap().path().to_string_lossy().to_string())
        .collect::<Vec<_>>();

        let mut package = Package::new(
            vec![deck],
            media_files.iter().map(|it| it.as_str()).collect::<Vec<_>>(),
        )
        .unwrap();
        package.write_to_file("russian_for_dummies.apkg").unwrap();
    }
}

/*
WISDOM:
- JE/JA/JO/JU nur, wenn e/a/o/u nach Vokal und nach soft/hard signs
*/
