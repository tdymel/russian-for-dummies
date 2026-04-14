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
        <div class="rfd-root" onclick="document.getElementById('root').play()">{{Root}} ({{Perfective}})</div>

        <table>
            <tr>
                <td>я</td>
                <td onclick="document.getElementById('Present1').play()">{{PresentI}}</td>
                <td onclick="document.getElementById('Future1').play()">{{FutureI}}</td>
            </tr>
            <tr>
                <td>ты</td>
                <td onclick="document.getElementById('Present2').play()">{{PresentYou}}</td>
                <td onclick="document.getElementById('Future2').play()">{{FutureYou}}</td>
            </tr>
            <tr>
                <td>он/она/оно</td>
                <td onclick="document.getElementById('Present3').play()">{{PresentHeSheIt}}</td>
                <td onclick="document.getElementById('Future3').play()">{{FutureHeSheIt}}</td>
            </tr>
            <tr>
                <td>мы</td>
                <td onclick="document.getElementById('Present4').play()">{{PresentWe}}</td>
                <td onclick="document.getElementById('Future4').play()">{{FutureWe}}</td>
            </tr>
            <tr>
                <td>вы</td>
                <td onclick="document.getElementById('Present5').play()">{{PresentYouTheyFormal}}</td>
                <td onclick="document.getElementById('Future5').play()">{{FutureYouTheyFormal}}</td>
            </tr>
            <tr>
                <td>они</td>
                <td onclick="document.getElementById('Present6').play()">{{PresentThey}}</td>
                <td onclick="document.getElementById('Future6').play()">{{FutureThey}}</td>
            </tr>
        </table>

        <table>
            <tr>
                <td>ты</td>
                <td onclick="document.getElementById('Imp1').play()">{{ImpYou}}</td>
            </tr>
            <tr>
                <td>вы</td>
                <td onclick="document.getElementById('Imp2').play()">{{ImpYouTheyFormal}}</td>
            </tr>
        </table>

        <table>
            <tr>
                <td>Masculine</td>
                <td onclick="document.getElementById('Past1').play()">{{PastM}}</td>
            </tr>
            <tr>
                <td>Feminine</td>
                <td onclick="document.getElementById('Past2').play()">{{PastF}}</td>
            </tr>
            <tr>
                <td>Neuter</td>
                <td onclick="document.getElementById('Past3').play()">{{PastN}}</td>
            </tr>
            <tr>
                <td>Plural</td>
                <td onclick="document.getElementById('Past4').play()">{{PastP}}</td>
            </tr>
        </table>

        <table>
            <tr>
                <td>Active</td>
                <td onclick="document.getElementById('ActivePresent').play()">{{ActivePresent}}</td>
                <td onclick="document.getElementById('ActivePast').play()">{{ActivePast}}</td>
            </tr>
            <tr>
                <td>Passive</td>
                <td onclick="document.getElementById('PassivePresent').play()">{{PassivePresent}}</td>
                <td onclick="document.getElementById('PassivePast').play()">{{PassivePast}}</td>
            </tr>
            <tr>
                <td>Gerund</td>
                <td onclick="document.getElementById('GerundPresent').play()">{{GerundPresent}}</td>
                <td onclick="document.getElementById('GerundPast').play()">{{GerundPast}}</td>
            </tr>
        </table>

        <div class="rfd-example" onclick="document.getElementById('example').play()">{{Example}}</div>
    </div>
    {{AudioRoot}}
    {{AudioPresent1}}
    {{AudioPresent2}}
    {{AudioPresent3}}
    {{AudioPresent4}}
    {{AudioPresent5}}
    {{AudioPresent6}}
    {{AudioFuture1}}
    {{AudioFuture2}}
    {{AudioFuture3}}
    {{AudioFuture4}}
    {{AudioFuture5}}
    {{AudioFuture6}}
    {{AudioImp1}}
    {{AudioImp2}}
    {{AudioPast1}}
    {{AudioPast2}}
    {{AudioPast3}}
    {{AudioPast4}}
    {{AudioActivePresent}}
    {{AudioActivePast}}
    {{AudioPassivePresent}}
    {{AudioPassivePast}}
    {{AudioGerundPresent}}
    {{AudioGerundPast}}
    {{AudioExample}}
</div>
"#;

pub static VERB_MODEL: LazyLock<Model> = LazyLock::new(|| {
    Model::new(
        1607390320 + 1,
        "Verb",
        vec![
            Field::new("Question"),
            Field::new("Root"),
            Field::new("Perfective"),
            Field::new("PresentI"),
            Field::new("PresentYou"),
            Field::new("PresentHeSheIt"),
            Field::new("PresentWe"),
            Field::new("PresentYouTheyFormal"),
            Field::new("PresentThey"),
            Field::new("FutureI"),
            Field::new("FutureYou"),
            Field::new("FutureHeSheIt"),
            Field::new("FutureWe"),
            Field::new("FutureYouTheyFormal"),
            Field::new("FutureThey"),
            Field::new("ImpYou"),
            Field::new("ImpYouTheyFormal"),
            Field::new("PastM"),
            Field::new("PastF"),
            Field::new("PastN"),
            Field::new("PastP"),
            Field::new("ActivePresent"),
            Field::new("ActivePast"),
            Field::new("PassivePresent"),
            Field::new("PassivePast"),
            Field::new("GerundPresent"),
            Field::new("GerundPast"),
            Field::new("Example"),
            Field::new("AudioRoot"),
            Field::new("AudioPresent1"),
            Field::new("AudioPresent2"),
            Field::new("AudioPresent3"),
            Field::new("AudioPresent4"),
            Field::new("AudioPresent5"),
            Field::new("AudioPresent6"),
            Field::new("AudioFuture1"),
            Field::new("AudioFuture2"),
            Field::new("AudioFuture3"),
            Field::new("AudioFuture4"),
            Field::new("AudioFuture5"),
            Field::new("AudioFuture6"),
            Field::new("AudioImp1"),
            Field::new("AudioImp2"),
            Field::new("AudioPast1"),
            Field::new("AudioPast2"),
            Field::new("AudioPast3"),
            Field::new("AudioPast4"),
            Field::new("AudioActivePresent"),
            Field::new("AudioActivePast"),
            Field::new("AudioPassivePresent"),
            Field::new("AudioPassivePast"),
            Field::new("AudioGerundPresent"),
            Field::new("AudioGerundPast"),
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
