use clap::Parser;

use crate::{
    content::russian_for_dummies::create_deck,
    genanki::{CompileAnkiDeck, sync_deck},
    stats::print_stats,
};

mod content;
mod genanki;
mod model;
mod open_russian;
mod stats;
mod utility;
/*
TODO:
* Noun declensions can be comma separated. Example Person.
* Verb Usage
* Support: Ajectives, Pronouns, other from OpenRussian
* Custom FlashCards müssen WordId referenzieren
* Add Wisdom
* Only package USED media files
* Ausdrücke
* Kategorien von OpenRussian inspirieren lassen
* Support english language as well
* CI/CD + Publish on GitHub later


*/

#[derive(Parser, Debug)]
#[command(name = "rfd")]
struct Args {
    #[arg(short, long)]
    anki: bool,
    #[arg(short, long)]
    test: bool,
    #[arg(short, long)]
    stats: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.stats {
        print_stats();
    }

    if args.test {
        // let response = fetch_word("чеснок").await?;
        // println!("{:#?}", response);

        // println!("{:#?}", fetch_noun().await)
    }

    if args.anki {
        println!("Generating Anki-Deck!");
        create_deck().await.compile();
        sync_deck().await.unwrap();
    }

    Ok(())
}
