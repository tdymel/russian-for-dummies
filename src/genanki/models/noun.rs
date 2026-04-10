use std::sync::LazyLock;

use genanki_rs::{Field, Model, Template};

const CSS: &'static str = r#"
    body {
        margin: 0;
        padding: 0;
    }

    .rfd-container {
        font-size: 2rem;
        background: black;
        color: white;
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    }

    .rfd-header {
        display: flex;
        justify-content: center;
        align-items: center;
        background: #333333;
        padding: 1rem;
    }

    .rfd-body {
        font-size: 1.5rem;
        color: white;
        padding: 1rem;
        display: flex;
        flex-direction: column;
        gap: 2rem;
    }

    .rfd-body table {
        width: 100%;
        border: 1px solid #333333;
        border-radius: 0.25rem;
        border-collapse: separate;
        border-spacing: 0;
        font-size: 1.5rem;
        color: white;
        text-align: left;
    }

    .rfd-body td {
        padding: 1rem;
    }

    .rfd-body td:first-child, .rfd-body td:nth-child(2) {
        width: 1%;
        border-right: 1px solid #333333;
    }

    .rfd-body td:nth-child(2), .rfd-body td:nth-child(3) {
        width: auto;
    }

    .rfd-body tr:nth-child(even) {
        background-color: #333333;
    }

    .rfd-body tr:nth-child(even) td:first-child, .rfd-body tr:nth-child(even) td:nth-child(2) {
        border-right: 1px solid black;
    }

    .rfd-stressed {
        color: red !important;
    }

    .rfd-root {
        font-size: 2rem;
        align-self: center;
    }

    .rfd-example {
        font-size: 1.5rem;
        align-self: center;
    }
"#;

const TEMPLATE_QUESTION: &'static str = r#"
<div class="rfd-container">
    <div class="rfd-header">{{Question}}</div>
</div>
"#;

const TEMPLATE_ANSWER: &'static str = r#"
<div class="rfd-container">
    <div class="rfd-header">{{Question}}</div>

    <div class="rfd-body">
        <div class="rfd-root" onclick="document.getElementById('sn').play()">{{Root}} ({{Gender}})</div>

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

        <div class="rfd-example">{{Example}}</div>
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
</div>
"#;

pub static NOUN_MODEL: LazyLock<Model> = LazyLock::new(|| {
    Model::new(
        // Have to change the id every time, otherwise it wont refresh on the device
        1607392320 + 12,
        "Noun",
        vec![
            Field::new("Question"),
            Field::new("Root"),
            Field::new("Gender"),
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
        ],
        vec![
            Template::new("Noun")
                .qfmt(TEMPLATE_QUESTION)
                .afmt(TEMPLATE_ANSWER),
        ],
    )
    .css(CSS)
});
