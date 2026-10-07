use crate::{OwnedJsonSchemaValidator, SchemaError, ValidationControl};
use semio_framework_value::DslValue;

#[test]
fn intrinsic_values_preserve_the_independent_validation_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let validator = OwnedJsonSchemaValidator::compile_intrinsic_with_documents(&DslValue::from(&case["schema"]), &[]).unwrap();
        assert_eq!(validator.validate_intrinsic(&DslValue::from(&case["value"])).is_ok(), case["valid"].as_bool().unwrap(), "{}", case["id"]);
    }
    println!("[DEBUG] intrinsic validation oracle=Ajv corpus={}", corpus["cases"].as_array().unwrap().len());
}

#[test]
fn intrinsic_validation_controls_admission_before_copying_values() {
    let schema = DslValue::from(serde_json::json!({"type":"object","properties":{"n":{"type":"integer"}}}));
    let (validator, progress) = OwnedJsonSchemaValidator::compile_intrinsic_with_documents_and_control(&schema, &[], &ValidationControl::default()).unwrap();
    assert!(progress.visited_nodes > 0);
    let value = DslValue::from(serde_json::json!({"n":2}));
    assert!(validator.validate_intrinsic_with_control(&value, &ValidationControl::default()).unwrap().visited_nodes > 0);
    assert_eq!(validator.validate_intrinsic_with_control(&value, &ValidationControl::new(1)), Err(SchemaError::LimitExceeded(1)));
    let cancelled = ValidationControl::default(); cancelled.cancel();
    assert_eq!(validator.validate_intrinsic_with_control(&value, &cancelled), Err(SchemaError::Cancelled));
    assert!(matches!(OwnedJsonSchemaValidator::compile_intrinsic_with_documents_and_control(&schema, &[], &cancelled), Err(SchemaError::Cancelled)));
    let finite = OwnedJsonSchemaValidator::compile_intrinsic_with_documents(&DslValue::from(serde_json::json!({"type":"number"})), &[]).unwrap();
    assert!(finite.validate_intrinsic(&DslValue::float(f64::NAN)).is_err());
    assert!(finite.validate_intrinsic(&DslValue::float(f64::INFINITY)).is_err());
    println!("[DEBUG] intrinsic validator admission, progress and cancellation preserved");
}

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
