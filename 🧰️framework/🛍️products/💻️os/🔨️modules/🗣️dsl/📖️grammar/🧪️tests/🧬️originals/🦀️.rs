//! 🧬️ Verifies the complete immutable original grammar input cohort alongside the current tree sweep.
use super::*;

#[test]
fn all_original_503_shipped_grammar_inputs_parse_and_compile_with_explicit_os_composition() {
    let capture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("capture JSON");
    let inputs = capture["inputs"].as_array().expect("captured inputs");
    let rows = capture["shippedGrammarLawInputs"].as_array().expect("shipped grammar inputs");
    assert_eq!(rows.len(), 503);
    let registry = family_fragments().expect("explicit OS family fragments");
    let mut failures = Vec::new();
    for row in rows {
        let path = row["path"].as_str().expect("grammar path");
        let original = inputs.iter().find(|input| input["path"].as_str() == Some(path)).expect("frozen original source");
        assert_eq!(original["sha256"], row["sha256"], "{path}");
        match parse_grammar(original["source"].as_str().expect("grammar source")) {
            Err(error) => failures.push(format!("{path}: parse: {error:?}")),
            Ok(grammar) => {
                if grammar.dialect != SemioDialect::Grammar {
                    failures.push(format!("{path}: dialect {:?}", grammar.dialect));
                } else if let Err(error) = Recognizer::compile(&grammar, &registry, product_macros()) {
                    failures.push(format!("{path}: compile: {error:?}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{} of 503 original grammars failed:\n{}", failures.len(), failures.join("\n"));
    println!("[grammar-original-sweep] 503 original grammars parsed and compiled");
}
