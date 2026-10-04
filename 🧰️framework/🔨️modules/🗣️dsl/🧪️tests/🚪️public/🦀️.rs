//! 🧪️ External-client laws share language-neutral goldens with independent portable codecs and reference resolvers.
use semio_framework_dsl::escape_text;
use semio_framework_dsl::unescape_text;
use semio_framework_dsl::lex;
use semio_framework_diagnostic::Limits;
use semio_framework_dsl::TokenKind;
use semio_framework_dsl::FragmentRegistry;
use semio_framework_dsl::Recognizer;
use semio_framework_dsl::parse_grammar;
#[test]
fn explicit_fragment_selection_matches_the_independent_reference_corpus() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧩️selection/🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let mut registry = FragmentRegistry::new();
        for name in case["selectedFragments"].as_array().unwrap() {
            let name = name.as_str().unwrap();
            registry.insert(name, parse_grammar("grammar fragment\nstart doc\ndoc = IDENT\n").unwrap());
        }
        let mut missing = Vec::new();
        for name in case["uses"].as_array().unwrap() {
            let name = name.as_str().unwrap();
            let grammar = parse_grammar(&format!("grammar selected\nuse {name}\nstart doc\ndoc = IDENT\n")).unwrap();
            if let Err(error) = Recognizer::compile(&grammar, &registry, Vec::new()) {
                assert_eq!(error.message, format!("unregistered grammar fragment `{name}`"));
                missing.push(name);
            }
        }
        assert_eq!(serde_json::to_value(missing).unwrap(), case["expectedMissing"], "{}", case["id"]);
    }
}
#[test]
fn lexical_and_escape_outputs_match_independent_codec_goldens() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧱️ownership/🔣️.json")).unwrap();
    for case in cases["independentCases"].as_array().unwrap() {
        let text = case["text"].as_str().unwrap();
        assert_eq!(escape_text(text), case["escaped"].as_str().unwrap());
        assert_eq!(unescape_text(case["escaped"].as_str().unwrap(), false).unwrap(), text);
    }
    for case in cases["refusedEscapes"].as_array().unwrap() {
        assert!(unescape_text(case["escaped"].as_str().unwrap(), false).is_err(), "{}", case["id"]);
    }
    for case in cases["lexicalCases"].as_array().unwrap() {
        let actual = lex(case["text"].as_str().unwrap(), &Limits::default(), false).unwrap();
        let expected = case["tokens"].as_array().unwrap();
        let significant = actual.iter().filter(|token| !token.kind.is_trivia() && token.kind != TokenKind::Eof).collect::<Vec<_>>();
        assert_eq!(significant.len(), expected.len());
        for (token, expected) in significant.iter().zip(expected) {
            assert_eq!(format!("{:?}", token.kind), expected["kind"].as_str().unwrap());
            assert_eq!(token.text.as_str().as_ref(), expected["text"].as_str().unwrap());
            assert_eq!(token.span.column as usize - 1, expected["start"].as_u64().unwrap() as usize);
            assert_eq!(token.span.column as usize - 1 + token.span.length as usize, expected["end"].as_u64().unwrap() as usize);
        }
    }
}
