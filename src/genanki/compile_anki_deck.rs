use std::path::PathBuf;

use crate::{
    genanki::models::{NOUN_MODEL, OTHER_MODEL, PRONOUN_MODEL, VERB_MODEL},
    model::{Deck, DeclensionType, FlashCard, Gender, Participle, Phrase, Sentence, Word},
    tts::tts,
    utility::hash_to_base64,
};

use genanki_rs::{Deck as AnkiDeck, Note, Package};

trait ToTemplate {
    fn to_template(&self) -> String;
}

impl<T: ToTemplate> ToTemplate for Option<T> {
    fn to_template(&self) -> String {
        self.as_ref()
            .map(|it| it.to_template())
            .unwrap_or("-".to_string())
    }
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
            .join("<br />")
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

impl ToTemplate for Sentence {
    fn to_template(&self) -> String {
        self.to_string()
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
            DeclensionType::M4 => "M4".to_string(),
            DeclensionType::N => "N".to_string(),
        }
    }
}

impl ToTemplate for Participle {
    fn to_template(&self) -> String {
        self.russian.to_template()
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

impl ToAudio for Participle {
    fn to_audio(&self, id: &str) -> String {
        self.russian.to_audio(id)
    }
}

trait ToNote {
    fn to_note(self) -> Note;
}

impl ToNote for FlashCard {
    fn to_note(self) -> Note {
        match self {
            FlashCard::Pronoun(pronoun) => Note::new(
                PRONOUN_MODEL.clone(),
                vec![
                    &pronoun.translation.join(", "),
                    &pronoun.masculine.nominative.to_template(),
                    &pronoun.feminine.nominative.to_template(),
                    &pronoun.neuter.nominative.to_template(),
                    &pronoun.plural.nominative.to_template(),
                    &pronoun.masculine.nominative.to_audio("nom_masc"),
                    &pronoun.feminine.nominative.to_audio("nom_fem"),
                    &pronoun.neuter.nominative.to_audio("nom_neu"),
                    &pronoun.plural.nominative.to_audio("nom_plu"),
                    &pronoun.masculine.genitive.to_template(),
                    &pronoun.feminine.genitive.to_template(),
                    &pronoun.neuter.genitive.to_template(),
                    &pronoun.plural.genitive.to_template(),
                    &pronoun.masculine.genitive.to_audio("gen_masc"),
                    &pronoun.feminine.genitive.to_audio("gen_fem"),
                    &pronoun.neuter.genitive.to_audio("gen_neu"),
                    &pronoun.plural.genitive.to_audio("gen_plu"),
                    &pronoun.masculine.dative.to_template(),
                    &pronoun.feminine.dative.to_template(),
                    &pronoun.neuter.dative.to_template(),
                    &pronoun.plural.dative.to_template(),
                    &pronoun.masculine.dative.to_audio("dat_masc"),
                    &pronoun.feminine.dative.to_audio("dat_fem"),
                    &pronoun.neuter.dative.to_audio("dat_neu"),
                    &pronoun.plural.dative.to_audio("dat_plu"),
                    &pronoun.masculine.accusative.to_template(),
                    &pronoun.feminine.accusative.to_template(),
                    &pronoun.neuter.accusative.to_template(),
                    &pronoun.plural.accusative.to_template(),
                    &pronoun.masculine.accusative.to_audio("acc_masc"),
                    &pronoun.feminine.accusative.to_audio("acc_fem"),
                    &pronoun.neuter.accusative.to_audio("acc_neu"),
                    &pronoun.plural.accusative.to_audio("acc_plu"),
                    &pronoun.masculine.instrumental.to_template(),
                    &pronoun.feminine.instrumental.to_template(),
                    &pronoun.neuter.instrumental.to_template(),
                    &pronoun.plural.instrumental.to_template(),
                    &pronoun.masculine.instrumental.to_audio("ins_masc"),
                    &pronoun.feminine.instrumental.to_audio("ins_fem"),
                    &pronoun.neuter.instrumental.to_audio("ins_neu"),
                    &pronoun.plural.instrumental.to_audio("ins_plu"),
                    &pronoun.masculine.prepositional.to_template(),
                    &pronoun.feminine.prepositional.to_template(),
                    &pronoun.neuter.prepositional.to_template(),
                    &pronoun.plural.prepositional.to_template(),
                    &pronoun.masculine.prepositional.to_audio("pre_masc"),
                    &pronoun.feminine.prepositional.to_audio("pre_fem"),
                    &pronoun.neuter.prepositional.to_audio("pre_neu"),
                    &pronoun.plural.prepositional.to_audio("pre_plu"),
                    &pronoun.example.to_template(),
                    &pronoun.example.to_audio("example"),
                ],
            )
            .unwrap(),
            FlashCard::Other(other) => Note::new(
                OTHER_MODEL.clone(),
                vec![
                    &other.translation.join(", "),
                    &other.root.to_template(),
                    &other.usage.unwrap_or_default(),
                    &other.example.to_template(),
                    &other.root.to_audio("root"),
                    &other.example.to_audio("example"),
                ],
            )
            .unwrap(),
            FlashCard::Verb(verb) => Note::new(
                VERB_MODEL.clone(),
                vec![
                    &verb.translation.join(", "),
                    &verb.root.to_template(),
                    if verb.is_perfective {
                        "Completed"
                    } else {
                        "Ongoing"
                    },
                    &verb.present.i.to_template(),
                    &verb.present.you.to_template(),
                    &verb.present.he_she_it.to_template(),
                    &verb.present.we.to_template(),
                    &verb.present.you_they_formal.to_template(),
                    &verb.present.they.to_template(),
                    &verb.future.i.to_template(),
                    &verb.future.you.to_template(),
                    &verb.future.he_she_it.to_template(),
                    &verb.future.we.to_template(),
                    &verb.future.you_they_formal.to_template(),
                    &verb.future.they.to_template(),
                    &verb.imperative.you.to_template(),
                    &verb.imperative.you_they_formal.to_template(),
                    &verb.past.masculine.to_template(),
                    &verb.past.feminine.to_template(),
                    &verb.past.neuter.to_template(),
                    &verb.past.plural.to_template(),
                    &verb.participles.active_present.to_template(),
                    &verb.participles.active_past.to_template(),
                    &verb.participles.passive_present.to_template(),
                    &verb.participles.passive_past.to_template(),
                    &verb.participles.gerund_present.to_template(),
                    &verb.participles.gerund_past.to_template(),
                    &verb.example.to_template(),
                    &verb.root.to_audio("root"),
                    &verb.present.i.to_audio("Present1"),
                    &verb.present.you.to_audio("Present2"),
                    &verb.present.he_she_it.to_audio("Present3"),
                    &verb.present.we.to_audio("Present4"),
                    &verb.present.you_they_formal.to_audio("Present5"),
                    &verb.present.they.to_audio("Present6"),
                    &verb.future.i.to_audio("Future1"),
                    &verb.future.you.to_audio("Future2"),
                    &verb.future.he_she_it.to_audio("Future3"),
                    &verb.future.we.to_audio("Future4"),
                    &verb.future.you_they_formal.to_audio("Future5"),
                    &verb.future.they.to_audio("Future6"),
                    &verb.imperative.you.to_audio("Imp1"),
                    &verb.imperative.you_they_formal.to_audio("Imp2"),
                    &verb.past.masculine.to_audio("Past1"),
                    &verb.past.feminine.to_audio("Past2"),
                    &verb.past.neuter.to_audio("Past3"),
                    &verb.past.plural.to_audio("Past4"),
                    &verb.participles.active_present.to_audio("ActivePresent"),
                    &verb.participles.active_past.to_audio("ActivePast"),
                    &verb.participles.passive_present.to_audio("PassivePresent"),
                    &verb.participles.passive_past.to_audio("PassivePast"),
                    &verb.participles.gerund_present.to_audio("GerundPresent"),
                    &verb.participles.gerund_past.to_audio("GerundPast"),
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
    async fn compile(self);
}

impl CompileAnkiDeck for Deck {
    async fn compile(self) {
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

        flash_cards.sort_by_key(|l| l.csfr());

        let mut mp3_phrases = flash_cards
            .iter()
            .flat_map(|it| match it {
                FlashCard::Pronoun(pronoun) => {
                    vec![
                        pronoun
                            .masculine
                            .nominative
                            .as_ref()
                            .map(|it| it.accented()),
                        pronoun.feminine.nominative.as_ref().map(|it| it.accented()),
                        pronoun.neuter.nominative.as_ref().map(|it| it.accented()),
                        pronoun.plural.nominative.as_ref().map(|it| it.accented()),
                        pronoun.masculine.genitive.as_ref().map(|it| it.accented()),
                        pronoun.feminine.genitive.as_ref().map(|it| it.accented()),
                        pronoun.neuter.genitive.as_ref().map(|it| it.accented()),
                        pronoun.plural.genitive.as_ref().map(|it| it.accented()),
                        pronoun.masculine.dative.as_ref().map(|it| it.accented()),
                        pronoun.feminine.dative.as_ref().map(|it| it.accented()),
                        pronoun.neuter.dative.as_ref().map(|it| it.accented()),
                        pronoun.plural.dative.as_ref().map(|it| it.accented()),
                        pronoun
                            .masculine
                            .accusative
                            .as_ref()
                            .map(|it| it.accented()),
                        pronoun.feminine.accusative.as_ref().map(|it| it.accented()),
                        pronoun.neuter.accusative.as_ref().map(|it| it.accented()),
                        pronoun.plural.accusative.as_ref().map(|it| it.accented()),
                        pronoun
                            .masculine
                            .instrumental
                            .as_ref()
                            .map(|it| it.accented()),
                        pronoun
                            .feminine
                            .instrumental
                            .as_ref()
                            .map(|it| it.accented()),
                        pronoun.neuter.instrumental.as_ref().map(|it| it.accented()),
                        pronoun.plural.instrumental.as_ref().map(|it| it.accented()),
                        pronoun
                            .masculine
                            .prepositional
                            .as_ref()
                            .map(|it| it.accented()),
                        pronoun
                            .feminine
                            .prepositional
                            .as_ref()
                            .map(|it| it.accented()),
                        pronoun
                            .neuter
                            .prepositional
                            .as_ref()
                            .map(|it| it.accented()),
                        pronoun
                            .plural
                            .prepositional
                            .as_ref()
                            .map(|it| it.accented()),
                        pronoun.example.as_ref().map(|it| it.russian.clone()),
                    ]
                }
                FlashCard::Other(other) => vec![
                    Some(other.root.accented()),
                    other.example.as_ref().map(|it| it.russian.clone()),
                ],
                FlashCard::Noun(noun) => vec![
                    noun.singular.nominative.as_ref().map(|it| it.accented()),
                    noun.plural.nominative.as_ref().map(|it| it.accented()),
                    noun.singular.genitive.as_ref().map(|it| it.accented()),
                    noun.plural.genitive.as_ref().map(|it| it.accented()),
                    noun.plural.accusative.as_ref().map(|it| it.accented()),
                    noun.singular.accusative.as_ref().map(|it| it.accented()),
                    noun.plural.prepositional.as_ref().map(|it| it.accented()),
                    noun.singular.prepositional.as_ref().map(|it| it.accented()),
                    noun.plural.dative.as_ref().map(|it| it.accented()),
                    noun.singular.dative.as_ref().map(|it| it.accented()),
                    noun.plural.instrumental.as_ref().map(|it| it.accented()),
                    noun.plural.instrumental.as_ref().map(|it| it.accented()),
                    noun.example.as_ref().map(|it| it.russian.clone()),
                ],
                FlashCard::Verb(verb) => vec![
                    Some(verb.root.accented()),
                    Some(verb.present.i.accented()),
                    Some(verb.present.you.accented()),
                    Some(verb.present.he_she_it.accented()),
                    Some(verb.present.we.accented()),
                    Some(verb.present.you_they_formal.accented()),
                    Some(verb.present.they.accented()),
                    Some(verb.future.i.accented()),
                    Some(verb.future.you.accented()),
                    Some(verb.future.he_she_it.accented()),
                    Some(verb.future.we.accented()),
                    Some(verb.future.you_they_formal.accented()),
                    Some(verb.future.they.accented()),
                    Some(verb.past.masculine.accented()),
                    Some(verb.past.feminine.accented()),
                    Some(verb.past.neuter.accented()),
                    Some(verb.past.plural.accented()),
                    Some(verb.imperative.you.accented()),
                    Some(verb.imperative.you_they_formal.accented()),
                    verb.participles
                        .active_past
                        .as_ref()
                        .map(|it| it.russian.accented()),
                    verb.participles
                        .active_present
                        .as_ref()
                        .map(|it| it.russian.accented()),
                    verb.participles
                        .passive_past
                        .as_ref()
                        .map(|it| it.russian.accented()),
                    verb.participles
                        .passive_present
                        .as_ref()
                        .map(|it| it.russian.accented()),
                    verb.participles
                        .gerund_past
                        .as_ref()
                        .map(|it| it.russian.accented()),
                    verb.participles
                        .gerund_present
                        .as_ref()
                        .map(|it| it.russian.accented()),
                    verb.example.as_ref().map(|it| it.russian.clone()),
                ],
            })
            .flatten()
            .collect::<Vec<_>>();

        mp3_phrases.sort();
        mp3_phrases.dedup();

        for flash_card in flash_cards {
            deck.add_note(flash_card.to_note());
        }

        let mut paths = Vec::new();
        let mp3_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("cache")
            .join("mp3");
        for it in mp3_phrases {
            if tts(&it).await {
                let path = mp3_folder
                    .clone()
                    .join(hash_to_base64(&it.replace("'", "")) + ".mp3")
                    .to_string_lossy()
                    .to_string();
                tts(&it).await;
                paths.push(path);
            }
        }
        let media_files: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();

        let mut package = Package::new(vec![deck], media_files).unwrap();
        package.write_to_file("russian_for_dummies.apkg").unwrap();
    }
}

/*
WISDOM:
- JE/JA/JO/JU nur, wenn e/a/o/u nach Vokal und nach soft/hard signs
*/
