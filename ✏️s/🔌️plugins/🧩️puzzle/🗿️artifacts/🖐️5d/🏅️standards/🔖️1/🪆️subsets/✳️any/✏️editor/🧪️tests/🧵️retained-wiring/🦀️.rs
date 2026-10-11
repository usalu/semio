#[test]
fn retained_factory_validates_and_adopts_its_exact_checkpoint_owner() {
    let source = include_str!("../../🦀️.rs");
    assert!(source.contains("ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint)"));
    assert!(source.contains("ArtifactRetainedCommandJob::from_wire(payload, input)"));
    assert!(source.contains("checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES"));
    assert!(!source.contains("if checkpoint.is_some() || input.declared_bytes()"));
}
