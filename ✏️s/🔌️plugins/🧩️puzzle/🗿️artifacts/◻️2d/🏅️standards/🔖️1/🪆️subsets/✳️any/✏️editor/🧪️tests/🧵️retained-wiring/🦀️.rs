#[test]
fn retained_factory_validates_and_adopts_its_exact_checkpoint_owner() {
    let source = include_str!("../../🦀️.rs");
    assert!(!source.contains("impl semio_framework_plugin::ArtifactOwnedToolJobFactory for BoundedFirstStepCommandJobFactory"));
    assert!(!source.contains("registry.register(BoundedFirstStepCommandJobFactory"));
}
