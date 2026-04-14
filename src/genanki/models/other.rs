use std::sync::LazyLock;

use genanki_rs::{Field, Model, Template};

use crate::genanki::models::css::CSS;

const TEMPLATE_QUESTION: &str = r#"
<div class="rfd-container">
    <div class="rfd-header">{{Question}}</div>
</div>
"#;

const TEMPLATE_ANSWER: &str = r#"
<div class="rfd-container">
    <div class="rfd-header">{{Question}}</div>

    <div class="rfd-body">
        <div class="rfd-root" onclick="document.getElementById('root').play()">{{Root}} {{Usage}}</div>

        <div class="rfd-example" onclick="document.getElementById('example').play()">{{Example}}</div>
    </div>
    {{AudioRoot}}
    {{AudioExample}}
</div>
"#;

pub static OTHER_MODEL: LazyLock<Model> = LazyLock::new(|| {
    Model::new(
        1607391320 + 1,
        "Verb",
        vec![
            Field::new("Question"),
            Field::new("Root"),
            Field::new("Usage"),
            Field::new("Example"),
            Field::new("AudioRoot"),
            Field::new("AudioExample"),
        ],
        vec![
            Template::new("Verb")
                .qfmt(TEMPLATE_QUESTION)
                .afmt(TEMPLATE_ANSWER),
        ],
    )
    .css(CSS)
});
