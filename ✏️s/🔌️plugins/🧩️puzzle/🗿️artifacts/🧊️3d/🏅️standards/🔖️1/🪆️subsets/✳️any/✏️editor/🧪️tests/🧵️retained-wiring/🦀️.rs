#[test]
fn retained_factory_validates_and_adopts_its_exact_checkpoint_owner() {
    let source: &str = &crate::context::editor_source();
    let framework = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🏭️factory/🦀️.rs")).expect("committed test data reads");
    assert!(source.contains("retained_command_factory_admission!(maximum_raw_bytes:"));
    assert!(source.contains("checkpoint_resume: true"));
    assert!(framework.contains("ArtifactRetainedCommandJob::from_wire_with_checkpoint(*payload,input,checkpoint)"));
    assert!(framework.contains("ArtifactRetainedCommandJob::from_wire(*payload,input)"));
    assert!(framework.contains("bytes>super::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES"));
    assert!(!framework.contains("if checkpoint.is_some() || input.declared_bytes()"));
}
