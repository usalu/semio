use super::*;

/// 🏗️ The same closed corpus covers genuine zero-field records and malformed structure.
#[semio_framework_async_macros::async_test]
async fn dwg_grammar_shape_empty_record_matches_language_agnostic_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🏗️grammar/🔣️.json")).unwrap();
    assert_eq!(corpus["record"]["fieldCount"], 0);
    let rows = corpus["cases"].as_array().unwrap();
    assert_eq!(rows.len(), 15);
    let grammar = dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).unwrap();
    let recognizer = dsl::Recognizer::compile(&grammar);
    for row in rows {
        assert_eq!(recognizer.recognize(row["text"].as_str().unwrap()).unwrap(), row["accepted"].as_bool().unwrap(), "{}", row["id"]);
    }
    println!("dwg:grammar-shape cases={} emptyRecordFields=0 native=Recognizer", rows.len());
}
