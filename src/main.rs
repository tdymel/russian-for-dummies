use clap::Parser;

use crate::{content::russian_for_dummies::create_deck, genanki::CompileAnkiDeck};

mod content;
mod genanki;
mod model;
mod open_russian;

/*
TODO:
* Some nouns dont have plurals, they dont need to be repeated
* Duplicate validation
* Find list of the 80% must used words and use them for validation and suggestions of what to add
* Find a way to publish and update it more easily
* Use AI to generate Audio and Pictures for each word
* Use AI to map words to this pronounciation format


Word scraping:
* Audio: Wiktionary/AccentHub: https://accenthero.com/native/russian-standard/f1/%D1%87%D0%B5%D1%81%D0%BD%D0%BE%D0%BA.mp3
* https://accenthero.com/native/russian-standard/f1/%D1%87%D0%B5%D1%81%D0%BD%D0%BE%D0%BA.json
    * Phonemes (IPA)
    * Language Level
    * Syllable splitting (by phonemes)
OpenRussian.org
    * DB Dump: https://en.openrussian.org/about / https://app.togetherdb.com/db/fwoedz5fvtwvq03v/openrussian_public/words
    * Find Id via: https://api.openrussian.org/suggestions?q=%D1%87%D0%B5%D1%81%D0%BD%D0%BE%D0%BA&dummy=1775731600493&skip-duplicate-verb-pairs&lang=en
    * https://api.openrussian.org/api/words/10770?all=&lang=en / https://api.openrussian.org/api/words?bare=%D0%BD%D0%B0&lang=en
        * Has sample sentences
        * Declenation tables
        * Categories
        * type: noun, verb
    * Audio Files via phonetic api: https://api.openrussian.org/read/ru/%D1%87%D0%B5%D1%81%D0%BD%D0%BE%D0%BA%D0%B5%27
* Declination type?
* Ranking of words: https://en.openrussian.org/list/all (Looks like until 5500 or so words are ranked)

* Library to do inflection: https://crates.io/crates/zaliznyak (But its poorly documented)

*/

#[derive(Parser, Debug)]
#[command(name = "myapp")]
struct Args {
    #[arg(short, long)]
    anki: bool,
    #[arg(short, long)]
    test: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.test {
        // let response = fetch_word("чеснок").await?;
        // println!("{:#?}", response);

        // println!("{:#?}", fetch_noun().await)
    }

    if args.anki {
        println!("Generating Anki-Deck!");
        create_deck().await.compile();
    }

    Ok(())
}
