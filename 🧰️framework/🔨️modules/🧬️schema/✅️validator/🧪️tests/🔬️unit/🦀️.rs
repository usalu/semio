use crate::{OwnedJsonSchemaValidator, SchemaError, ValidationControl};
use semio_framework_value::DslValue;

#[test]
fn original_compiled_validator_retires_recursive_pattern_fields_under_full_grants(){
 use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};
 use semio_framework_trace::observe_heap_allocations_on_this_thread;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../♻️retirement/🧫️fixtures/🔣️.json")).unwrap();let schema=law["schema"].to_string();let (validator,heap)=observe_heap_allocations_on_this_thread(||OwnedJsonSchemaValidator::compile(&schema).unwrap());let held=heap.requested_bytes-heap.released_bytes;for value in law["valid"].as_array().unwrap(){assert!(validator.validate_json(&value.to_string()).is_ok())}for value in law["invalid"].as_array().unwrap(){assert!(validator.validate_json(&value.to_string()).is_err())}
 let (mut owner,heap)=observe_heap_allocations_on_this_thread(||ControlledRetirement::new(validator).unwrap_or_else(|_|panic!("original compiled schema ownership")));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8192,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:128};let pointer=owner.original().unwrap()as *const _;for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let (step,heap)=observe_heap_allocations_on_this_thread(||owner.step(denied));assert!(step.as_ref().map_or(true,|step|step.progress()==Default::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.original().unwrap()as *const _,pointer)}let mut turns=0;let mut births=0;let mut releases=0;for _ in 0..10000{if owner.terminal_is_empty(){break}let (step,heap)=observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=heap.requested_bytes;releases+=heap.released_bytes;turns+=1}assert!(owner.terminal_is_empty());assert_eq!(releases,held+births);let (_,heap)=observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original compiled schema recursive matcher retirement turns={turns} original={held} births={births} releases={releases} terminalDrop=0");
}

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
