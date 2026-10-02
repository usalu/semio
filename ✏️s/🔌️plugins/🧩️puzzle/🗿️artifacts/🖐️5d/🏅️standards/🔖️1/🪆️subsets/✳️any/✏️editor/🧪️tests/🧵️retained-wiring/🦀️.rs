#[test]
fn retained_factory_validates_and_adopts_its_exact_checkpoint_owner() {
    let source = include_str!("../../🦀️.rs");
    assert!(source.contains("RetainedPuzzleCommandJob::validate_wire_checkpoint(operation, &payload, &input, &checkpoint)"));
    assert!(source.contains("RetainedPuzzleCommandJob::from_validated_wire_checkpoint(operation, payload, input, checkpoint)"));
    assert!(!source.contains("if checkpoint.is_some() || input.declared_bytes()"));
}
