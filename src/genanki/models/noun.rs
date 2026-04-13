use std::sync::LazyLock;

use genanki_rs::{Field, Model, Template};

use crate::genanki::models::css::CSS;

const TEMPLATE_QUESTION: &'static str = r#"
<div class="rfd-container">
    <div class="rfd-header">{{Question}}</div>
</div>
"#;

const TEMPLATE_ANSWER: &'static str = r#"
<div class="rfd-container">
    <div class="rfd-header">{{Question}}</div>

    <div class="rfd-body">
        <div class="rfd-root" onclick="document.getElementById('sn').play()">{{Root}} ({{DeclensionType}})</div>

        <table>
            <tr>
                <td>Nom</td>
                <td onclick="document.getElementById('sn').play()">{{Nominativ-Singular}}</td>
                <td onclick="document.getElementById('pn').play()">{{Nominativ-Plural}}</td>
            </tr>
            <tr>
                <td>Gen</td>
                <td onclick="document.getElementById('sg').play()">{{Genitiv-Singular}}</td>
                <td onclick="document.getElementById('pg').play()">{{Genitiv-Plural}}</td>
            </tr>
            <tr>
                <td>Dat</td>
                <td onclick="document.getElementById('sd').play()">{{Dativ-Singular}}</td>
                <td onclick="document.getElementById('pd').play()">{{Dativ-Plural}}</td>
            </tr>
            <tr>
                <td>Akk</td>
                <td onclick="document.getElementById('sa').play()">{{Akkusativ-Singular}}</td>
                <td onclick="document.getElementById('pa').play()">{{Akkusativ-Plural}}</td>
            </tr>
            <tr>
                <td>Ins</td>
                <td onclick="document.getElementById('si').play()">{{Instrumental-Singular}}</td>
                <td onclick="document.getElementById('pi').play()">{{Instrumental-Plural}}</td>
            </tr>
            <tr>
                <td>Prä</td>
                <td onclick="document.getElementById('sp').play()">{{Praepositiv-Singular}}</td>
                <td onclick="document.getElementById('pp').play()">{{Praepositiv-Plural}}</td>
            </tr>
        </table>

        <div class="rfd-example" onclick="document.getElementById('example').play()">{{Example}}</div>
    </div>
    {{AudioSn}}
    {{AudioPn}}
    {{AudioSi}}
    {{AudioPi}}
    {{AudioSd}}
    {{AudioPd}}
    {{AudioSa}}
    {{AudioPa}}
    {{AudioSp}}
    {{AudioPp}}
    {{AudioSg}}
    {{AudioPg}}
    {{AudioExample}}
</div>
"#;

pub static NOUN_MODEL: LazyLock<Model> = LazyLock::new(|| {
    Model::new(
        // Have to change the id every time, otherwise it wont refresh on the device
        1607392320 + 14,
        "Noun",
        vec![
            Field::new("Question"),
            Field::new("Root"),
            Field::new("DeclensionType"),
            Field::new("Nominativ-Singular"),
            Field::new("Nominativ-Plural"),
            Field::new("Genitiv-Singular"),
            Field::new("Genitiv-Plural"),
            Field::new("Dativ-Singular"),
            Field::new("Dativ-Plural"),
            Field::new("Akkusativ-Singular"),
            Field::new("Akkusativ-Plural"),
            Field::new("Instrumental-Singular"),
            Field::new("Instrumental-Plural"),
            Field::new("Praepositiv-Singular"),
            Field::new("Praepositiv-Plural"),
            Field::new("Example"),
            Field::new("AudioSn"),
            Field::new("AudioPn"),
            Field::new("AudioSi"),
            Field::new("AudioPi"),
            Field::new("AudioSd"),
            Field::new("AudioPd"),
            Field::new("AudioSg"),
            Field::new("AudioPg"),
            Field::new("AudioSa"),
            Field::new("AudioPa"),
            Field::new("AudioSp"),
            Field::new("AudioPp"),
            Field::new("AudioExample"),
        ],
        vec![
            Template::new("Noun")
                .qfmt(TEMPLATE_QUESTION)
                .afmt(TEMPLATE_ANSWER),
        ],
    )
    .css(CSS)
});
