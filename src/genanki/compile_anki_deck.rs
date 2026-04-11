use std::{collections::HashSet, fs, path::PathBuf};

use crate::{
    genanki::models::NOUN_MODEL,
    model::{Deck, DeclensionType, FlashCard, Gender, Phrase, Sentence, Word, WordId},
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
            DeclensionType::First => "D1".to_string(),
            DeclensionType::Second => "D2".to_string(),
            DeclensionType::Third => "D3".to_string(),
            DeclensionType::Irregular => "DI".to_string(),
        }
    }
}

trait ToNote {
    fn to_note(self) -> Note;
}

impl ToNote for FlashCard {
    fn to_note(self) -> Note {
        match self {
            FlashCard::Noun(noun) => Note::new(
                NOUN_MODEL.clone(),
                vec![
                    &noun.translation.join(", "),
                    &noun.root.to_template(),
                    &noun.gender.to_template(),
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
                    &format!(
                        "<audio id=\"sn\" src=\"{}.mp3\"></audio>",
                        noun.singular
                            .nominative
                            .map(|it| it.to_string())
                            .unwrap_or("none".to_string())
                    ),
                    &format!(
                        "<audio id=\"pn\" src=\"{}.mp3\"></audio>",
                        noun.plural
                            .nominative
                            .map(|it| it.to_string())
                            .unwrap_or("none".to_string())
                    ),
                    &format!(
                        "<audio id=\"si\" src=\"{}-si.mp3\"></audio>",
                        noun.singular
                            .instrumental
                            .map(|it| it.to_string())
                            .unwrap_or("none".to_string())
                    ),
                    &format!(
                        "<audio id=\"pi\" src=\"{}.mp3\"></audio>",
                        noun.plural
                            .instrumental
                            .map(|it| it.to_string())
                            .unwrap_or("none".to_string())
                    ),
                    &format!(
                        "<audio id=\"sd\" src=\"{}.mp3\"></audio>",
                        noun.singular
                            .dative
                            .map(|it| it.to_string())
                            .unwrap_or("none".to_string())
                    ),
                    &format!(
                        "<audio id=\"pd\" src=\"{}.mp3\"></audio>",
                        noun.plural
                            .dative
                            .map(|it| it.to_string())
                            .unwrap_or("none".to_string())
                    ),
                    &format!(
                        "<audio id=\"sg\" src=\"{}.mp3\"></audio>",
                        noun.singular
                            .genitive
                            .map(|it| it.to_string())
                            .unwrap_or("none".to_string())
                    ),
                    &format!(
                        "<audio id=\"pg\" src=\"{}.mp3\"></audio>",
                        noun.plural
                            .genitive
                            .map(|it| it.to_string())
                            .unwrap_or("none".to_string())
                    ),
                    &format!(
                        "<audio id=\"sa\" src=\"{}.mp3\"></audio>",
                        noun.singular
                            .accusative
                            .map(|it| it.to_string())
                            .unwrap_or("none".to_string())
                    ),
                    &format!(
                        "<audio id=\"pa\" src=\"{}.mp3\"></audio>",
                        noun.plural
                            .accusative
                            .map(|it| it.to_string())
                            .unwrap_or("none".to_string())
                    ),
                    &format!(
                        "<audio id=\"sp\" src=\"{}.mp3\"></audio>",
                        noun.singular
                            .prepositional
                            .map(|it| it.to_string())
                            .unwrap_or("none".to_string())
                    ),
                    &format!(
                        "<audio id=\"pp\" src=\"{}.mp3\"></audio>",
                        noun.plural
                            .prepositional
                            .map(|it| it.to_string())
                            .unwrap_or("none".to_string())
                    ),
                    &format!(
                        "<audio id=\"example\" src=\"{}.mp3\"></audio>",
                        noun.example
                            .map(|it| it.to_string())
                            .unwrap_or("none".to_string())
                    ),
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

        let mut words = HashSet::<WordId>::new();
        let mut flash_cards = self
            .categories
            .into_iter()
            .flat_map(|it| it.flash_cards)
            .collect::<Vec<_>>();

        flash_cards.sort_by(|l, r| match (l, r) {
            (FlashCard::Noun(nl), FlashCard::Noun(nr)) => nl.csfr.cmp(&nr.csfr),
        });

        for flash_card in flash_cards {
            let id = match &flash_card {
                FlashCard::Noun(noun) => noun.id,
            };
            if words.contains(&id) {
                panic!("Duplicate word: {}!", id);
            }
            words.insert(id);
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
