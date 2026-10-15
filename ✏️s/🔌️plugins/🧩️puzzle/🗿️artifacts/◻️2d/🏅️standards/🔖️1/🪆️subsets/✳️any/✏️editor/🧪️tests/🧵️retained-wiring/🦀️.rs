#[test]
fn retained_factory_validates_and_adopts_its_exact_checkpoint_owner() {
    let source = super::unit_tests::editor_source();
    assert!(!source.contains("impl semio_framework_plugin::ArtifactOwnedToolJobFactory for BoundedFirstStepCommandJobFactory"));
    assert!(!source.contains("registry.register(BoundedFirstStepCommandJobFactory"));
}
