mod api;
mod fetch_mp3;
mod fetch_noun;
mod fetch_verb;
mod fetch_other;
mod fetch_pronoun;

#[allow(unused)]
pub use fetch_mp3::*;
pub use fetch_noun::fetch_noun;
pub use fetch_verb::*;
pub use fetch_other::*;
pub use fetch_pronoun::*;