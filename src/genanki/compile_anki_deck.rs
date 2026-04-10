use crate::{
    genanki::models::NOUN_MODEL,
    model::{Deck, FlashCard, Gender, Sentence, Word},
};

use genanki_rs::{Deck as AnkiDeck, Note};

trait ToTemplate {
    fn to_template(&self) -> String;
}

impl ToTemplate for Gender {
    fn to_template(&self) -> String {
        match self {
            Gender::Male => "M".to_string(),
            Gender::Female => "F".to_string(),
            Gender::Neuter => "N".to_string(),
        }
    }
}

impl ToTemplate for Vec<Word> {
    fn to_template(&self) -> String {
        self.iter()
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

impl ToTemplate for Option<Vec<Word>> {
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

trait ToNote {
    fn to_note(self) -> Note;
}

impl ToNote for FlashCard {
    fn to_note(self) -> Note {
        match self {
            FlashCard::Noun(noun) => Note::new(
                NOUN_MODEL.clone(),
                vec![
                    &noun.translation,
                    &noun.root.to_template(),
                    &noun.gender.to_template(),
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

        for category in self.categories {
            for flash_card in category.flash_cards {
                deck.add_note(flash_card.to_note());
            }
        }

        deck.write_to_file("russian_for_dummies.apkg").unwrap();
    }
}

/*
WISDOM:
- JE/JA/JO/JU nur, wenn e/a/o/u nach Vokal und nach soft/hard signs
*/
