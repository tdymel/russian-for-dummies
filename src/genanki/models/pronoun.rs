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
        <div class="rfd-root" onclick="document.getElementById('nom_masc').play()">{{NomMasc}}</div>

        <table>
            <tr>
                <td>Nom</td>
                <td onclick="document.getElementById('nom_masc').play()">{{NomMasc}}</td>
                <td onclick="document.getElementById('nom_fem').play()">{{NomFem}}</td>
            </tr>
            <tr>
                <td>Gen</td>
                <td onclick="document.getElementById('gen_masc').play()">{{GenMasc}}</td>
                <td onclick="document.getElementById('gen_fem').play()">{{GenFem}}</td>
            </tr>
            <tr>
                <td>Dat</td>
                <td onclick="document.getElementById('dat_masc').play()">{{DatMasc}}</td>
                <td onclick="document.getElementById('dat_fem').play()">{{DatFem}}</td>
            </tr>
            <tr>
                <td>Akk</td>
                <td onclick="document.getElementById('acc_masc').play()">{{AccMasc}}</td>
                <td onclick="document.getElementById('acc_fem').play()">{{AccFem}}</td>
            </tr>
            <tr>
                <td>Ins</td>
                <td onclick="document.getElementById('ins_masc').play()">{{InsMasc}}</td>
                <td onclick="document.getElementById('ins_fem').play()">{{InsFem}}</td>
            </tr>
            <tr>
                <td>Prä</td>
                <td onclick="document.getElementById('pre_masc').play()">{{PreMasc}}</td>
                <td onclick="document.getElementById('pre_fem').play()">{{PreFem}}</td>
            </tr>
        </table>

        <table>
            <tr>
                <td>Nom</td>
                <td onclick="document.getElementById('nom_neu').play()">{{NomNeu}}</td>
                <td onclick="document.getElementById('nom_plu').play()">{{NomPlu}}</td>
            </tr>
            <tr>
                <td>Gen</td>
                <td onclick="document.getElementById('gen_neu').play()">{{GenNeu}}</td>
                <td onclick="document.getElementById('gen_plu').play()">{{GenPlu}}</td>
            </tr>
            <tr>
                <td>Dat</td>
                <td onclick="document.getElementById('dat_neu').play()">{{DatNeu}}</td>
                <td onclick="document.getElementById('dat_plu').play()">{{DatPlu}}</td>
            </tr>
            <tr>
                <td>Akk</td>
                <td onclick="document.getElementById('acc_neu').play()">{{AccNeu}}</td>
                <td onclick="document.getElementById('acc_plu').play()">{{AccPlu}}</td>
            </tr>
            <tr>
                <td>Ins</td>
                <td onclick="document.getElementById('ins_neu').play()">{{InsNeu}}</td>
                <td onclick="document.getElementById('ins_plu').play()">{{InsPlu}}</td>
            </tr>
            <tr>
                <td>Prä</td>
                <td onclick="document.getElementById('pre_neu').play()">{{PreNeu}}</td>
                <td onclick="document.getElementById('pre_plu').play()">{{PrePlu}}</td>
            </tr>
        </table>

        <div class="rfd-example" onclick="document.getElementById('example').play()">{{Example}}</div>
    </div>
    {{AudioNomMasc}}
    {{AudioNomFem}}
    {{AudioNomNeu}}
    {{AudioNomPlu}}
    {{AudioGenMasc}}
    {{AudioGenFem}}
    {{AudioGenNeu}}
    {{AudioGenPlu}}
    {{AudioDatMasc}}
    {{AudioDatFem}}
    {{AudioDatNeu}}
    {{AudioDatPlu}}
    {{AudioAccMasc}}
    {{AudioAccFem}}
    {{AudioAccNeu}}
    {{AudioAccPlu}}
    {{AudioInsMasc}}
    {{AudioInsFem}}
    {{AudioInsNeu}}
    {{AudioInsPlu}}
    {{AudioPreMasc}}
    {{AudioPreFem}}
    {{AudioPreNeu}}
    {{AudioPrePlu}}
    {{AudioExample}}
</div>
"#;

pub static PRONOUN_MODEL: LazyLock<Model> = LazyLock::new(|| {
    Model::new(
        1607393320 + 0,
        "Pronoun",
        vec![
            Field::new("Question"),
            Field::new("NomMasc"),
            Field::new("NomFem"),
            Field::new("NomNeu"),
            Field::new("NomPlu"),
            Field::new("AudioNomMasc"),
            Field::new("AudioNomFem"),
            Field::new("AudioNomNeu"),
            Field::new("AudioNomPlu"),
            Field::new("GenMasc"),
            Field::new("GenFem"),
            Field::new("GenNeu"),
            Field::new("GenPlu"),
            Field::new("AudioGenMasc"),
            Field::new("AudioGenFem"),
            Field::new("AudioGenNeu"),
            Field::new("AudioGenPlu"),
            Field::new("DatMasc"),
            Field::new("DatFem"),
            Field::new("DatNeu"),
            Field::new("DatPlu"),
            Field::new("AudioDatMasc"),
            Field::new("AudioDatFem"),
            Field::new("AudioDatNeu"),
            Field::new("AudioDatPlu"),
            Field::new("AccMasc"),
            Field::new("AccFem"),
            Field::new("AccNeu"),
            Field::new("AccPlu"),
            Field::new("AudioAccMasc"),
            Field::new("AudioAccFem"),
            Field::new("AudioAccNeu"),
            Field::new("AudioAccPlu"),
            Field::new("InsMasc"),
            Field::new("InsFem"),
            Field::new("InsNeu"),
            Field::new("InsPlu"),
            Field::new("AudioInsMasc"),
            Field::new("AudioInsFem"),
            Field::new("AudioInsNeu"),
            Field::new("AudioInsPlu"),
            Field::new("PreMasc"),
            Field::new("PreFem"),
            Field::new("PreNeu"),
            Field::new("PrePlu"),
            Field::new("AudioPreMasc"),
            Field::new("AudioPreFem"),
            Field::new("AudioPreNeu"),
            Field::new("AudioPrePlu"),
            Field::new("Example"),
            Field::new("AudioExample"),
        ],
        vec![
            Template::new("Pronoun")
                .qfmt(TEMPLATE_QUESTION)
                .afmt(TEMPLATE_ANSWER),
        ],
    )
    .css(CSS)
});
