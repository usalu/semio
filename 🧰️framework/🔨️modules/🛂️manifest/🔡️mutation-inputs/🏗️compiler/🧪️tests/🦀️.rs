//! 🧪️ Canonical schema compilation preserves the independent neutral input corpus and exact source fingerprints.
use super::*;
use sha2::{Digest, Sha256};
const CORPUS: &str = include_str!("../../../🧪️tests/🧫️fixtures/🧫️mutation-inputs/🔣️.json");

fn normalized(mut value: serde_json::Value) -> serde_json::Value {
    match &mut value {
        serde_json::Value::Array(values) => for value in values { *value = normalized(value.take()); },
        serde_json::Value::Object(fields) => for value in fields.values_mut() { *value = normalized(value.take()); },
        serde_json::Value::Number(number) => if let Some(value) = number.as_f64().filter(|value| value.fract() == 0.0 && value.abs() <= 9_007_199_254_740_991.0) { *number = serde_json::Number::from(value as i64); },
        _ => {},
    }
    value
}
#[test]
fn compiled_input_metadata_matches_every_neutral_reader_case_and_sha256_oracle() {
    let corpus: serde_json::Value = serde_json::from_str(CORPUS).unwrap();
    let original = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(CORPUS, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let documents = original.get("documents").and_then(semio_framework_value::DslValue::as_object).unwrap().iter().map(|(_, value)| semio_framework_pack_json::to_json_string(value)).collect::<Vec<_>>();
    let borrowed = documents.iter().map(String::as_str).collect::<Vec<_>>();
    let mut ready = 0; let mut refused = 0;
    for (index, case) in corpus["cases"].as_array().unwrap().iter().enumerate() {
        let source = &original.get("cases").and_then(semio_framework_value::DslValue::as_array).unwrap()[index];
        let schema = semio_framework_pack_json::to_json_string(source.get("input").unwrap().get("leafSchema").unwrap());
        let compiled = compile_input_definitions(&schema, &borrowed);
        assert_eq!(compiled.schema_sha256.as_slice(), Sha256::digest(schema.as_bytes()).as_slice());
        assert_eq!(compiled.document_sha256.len(), borrowed.len());
        for (digest, document) in compiled.document_sha256.iter().zip(&borrowed) { assert_eq!(digest.as_slice(), Sha256::digest(document.as_bytes()).as_slice()); }
        let seed_result = compiled.seed_definitions_json;
        match (case.get("expectedInputs"), case.get("expectedError"), compiled.definitions_json) {
            (Some(expected), None, Ok(json)) => {
                ready += 1; let actual: serde_json::Value = serde_json::from_str(&json).unwrap();
                assert_eq!(normalized(actual), normalized(expected.clone()), "{}", case["name"]);
                let inputs: Vec<semio_framework_manifest_arg_schema::ActionArgDef> = serde_json::from_value(expected.clone()).unwrap();
                let seeds = semio_framework_manifest_arg_schema::action_arg_shape_seeds(&inputs);
                let actual: serde_json::Value = serde_json::from_str(&seed_result.unwrap()).unwrap();
                assert_eq!(normalized(actual), normalized(serde_json::to_value(seeds).unwrap()), "{}", case["name"]);
            },
            (None, Some(expected), Err(error)) => { refused += 1; use semio_framework_value::ToValue; assert_eq!(error.code.to_value().as_str(), expected["code"].as_str(), "{}", case["name"]); assert_eq!(error.pointer, expected["pointer"].as_str().unwrap(), "{}", case["name"]); assert_eq!(seed_result.unwrap_err(), error); },
            (_, _, result) => panic!("{} unexpected compile result: {result:?}", case["name"]),
        }
    }
    assert!(ready >= 18 && refused >= 8);
}
#[test]
fn changing_only_schema_bytes_changes_provenance_without_changing_input_meaning() {
    let schema = r#"{"type":"object","properties":{}}"#;
    let first = compile_input_definitions(schema, &[]);
    let second = compile_input_definitions(&format!("{schema}\n"), &[]);
    assert_ne!(first.schema_sha256, second.schema_sha256);
    assert_eq!(first.definitions_json.unwrap(), second.definitions_json.unwrap());
    assert_eq!(first.seed_definitions_json.unwrap(), second.seed_definitions_json.unwrap());
}
