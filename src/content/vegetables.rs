use crate::{
    content::cache::get_noun,
    model::{Category, WordId},
};

pub async fn vegetables() -> Category {
    Category::new("Gemüse").add(get_noun(WordId::Garlic).await)
    // .add(Noun::new(
    //     "Gemüse",
    //     word!("о"!, "вощ"),
    //     word!("о"!, "во", "щи"),
    //     Gender::Male,
    // ))
    // .add(Noun::new(
    //     "Tomate",
    //     word!("по", "ми", "дор"!),
    //     word!("по", "ми", "до"!, "ры"),
    //     Gender::Male,
    // ))
    // .add(Noun::new(
    //     "Brokkoli",
    //     word!("бро"!, "кко", "ли"),
    //     None,
    //     Gender::Male,
    // ))
    // .add(Noun::new(
    //     "Avocado",
    //     word!("а", "во", "ка"!, "до"),
    //     None,
    //     Gender::Neuter,
    // ))
    // .add(Noun::new(
    //     "Gurke",
    //     word!("о", "гу", "рец"!),
    //     word!("о", "гур", "цы"!),
    //     Gender::Male,
    // ))
    // .add(Noun::new(
    //     "Karotte/Möhre",
    //     word!("мор", "ковь"!),
    //     None,
    //     Gender::Female,
    // ))
    // .add(Noun::new("Zwiebel", word!("лук"!), None, Gender::Male))
    // .add(Noun::new(
    //     "Knoblauch",
    //     word!("чес", "нок"!),
    //     None,
    //     Gender::Male,
    // ))
    // .add(Noun::new(
    //     "Kartoffel",
    //     word!("кар", "то"!, "фель"),
    //     None,
    //     Gender::Male,
    // ))
    // .add(Noun::new(
    //     "Salat",
    //     word!("са", "лат"!),
    //     word!("са", "ла"!, "ты"),
    //     Gender::Male,
    // ))
    // .add(Noun::new(
    //     "Kohl / Weißkohl",
    //     word!("ка", "пус"!, "та"),
    //     word!("ка", "пус"!, "ты"),
    //     Gender::Female,
    // ))
    // .add(Noun::new(
    //     "Blumenkohl",
    //     vec![word!("цвет", "на"!, "я"), word!("ка", "пус"!, "та")],
    //     None,
    //     Gender::Female,
    // ))
    // .add(Noun::new(
    //     "Linsen",
    //     word!("че", "че", "ви"!, "ца"),
    //     None,
    //     Gender::Female,
    // ))
    // .add(Noun::new(
    //     "Spinat",
    //     word!("шпи", "нат"!),
    //     None,
    //     Gender::Male,
    // ))
    // .add(Noun::new(
    //     "Zucchini",
    //     word!("ка", "ба", "чок"!),
    //     word!("ка", "ба", "чки"!),
    //     Gender::Male,
    // ))
    // .add(Noun::new(
    //     "Aubergine",
    //     word!("ба", "кла", "жан"!),
    //     word!("ба", "кла", "жа"!, "ны"),
    //     Gender::Male,
    // ))
    // .add(Noun::new(
    //     "Kürbis",
    //     word!("тык"!,"ва"),
    //     word!("тык"!,"вы"),
    //     Gender::Female,
    // ))
    // .add(Noun::new(
    //     "Mais",
    //     word!("ку", "ку", "ру"!, "за"),
    //     word!("ку", "ку", "ру"!, "зы"),
    //     Gender::Female,
    // ))
    // .add(Noun::new("Erbse", word!("го", "рох"!), None, Gender::Male))
    // .add(Noun::new(
    //     "Bohne",
    //     word!("фа", "соль"!),
    //     word!("фа", "со"!, "ли"),
    //     Gender::Female,
    // ))
    // .add(Noun::new(
    //     "Radieschen",
    //     word!("ре", "дис"!),
    //     word!("ре", "ди"!, "сы"),
    //     Gender::Male,
    // ))
    // .add(Noun::new(
    //     "Rucola",
    //     word!("ру"!, "кко", "ла"),
    //     None,
    //     Gender::Female,
    // ))
    // .add(Noun::new(
    //     "Spargel",
    //     word!("спар"!, "жа"),
    //     word!("спар"!, "жи"),
    //     Gender::Female,
    // ))
    // .add(Noun::new(
    //     "Paprika",
    //     word!("па"!, "при", "ка"),
    //     word!("па"!, "при", "ки"),
    //     Gender::Female,
    // ))
    // .add(Noun::new(
    //     "Pilz",
    //     word!("гриб"!),
    //     word!("гри", "бы"!),
    //     Gender::Male,
    // ))
}
