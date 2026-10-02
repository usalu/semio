use crate::{OwnedJsonSchemaValidator, SchemaError, ValidationControl};

#[test]
fn unchanged_subset_corpus_matches_the_independent_oracle() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let validator = OwnedJsonSchemaValidator::compile(&case["schema"].to_string()).unwrap();
        assert_eq!(validator.is_valid_json(&case["value"].to_string()), case["valid"].as_bool().unwrap(), "{}", case["id"]);
    }
    for case in corpus["comparisons"].as_array().unwrap() {
        let schema = serde_json::json!({"const": case["left"]});
        let validator = OwnedJsonSchemaValidator::compile(&schema.to_string()).unwrap();
        assert_eq!(validator.is_valid_json(&case["right"].to_string()), case["equal"].as_bool().unwrap(), "{}", case["id"]);
    }
    println!("[DEBUG] neutral validator preserved {} subset vectors", corpus["cases"].as_array().unwrap().len() + corpus["comparisons"].as_array().unwrap().len());
}

#[test]
fn validator_control_reports_progress_and_preserves_cancellation() {
    let validator = OwnedJsonSchemaValidator::compile(r#"{"type":"object","properties":{"n":{"type":"integer"}}}"#).unwrap();
    assert_eq!(validator.validate_json(r#"{"n":2}"#).unwrap().visited_nodes, 2);
    assert_eq!(validator.validate_json_with_control(r#"{"n":2}"#, &ValidationControl::new(1)), Err(SchemaError::LimitExceeded(1)));
    let cancelled = ValidationControl::default();
    cancelled.cancel();
    assert_eq!(validator.validate_json_with_control(r#"{"n":2}"#, &cancelled), Err(SchemaError::Cancelled));
    println!("[DEBUG] neutral validator progress, bounds and cancellation preserved");
}
