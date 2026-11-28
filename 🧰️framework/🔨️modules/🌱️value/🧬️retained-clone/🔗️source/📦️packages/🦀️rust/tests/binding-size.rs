use semio_framework_value_clone_source::RetainedCloneBinding;

#[test]
fn retained_clone_binding_holds_a_compact_close_without_a_frontier() {
    let size=size_of::<RetainedCloneBinding>();
    assert!(size<=160,"retained clone binding is {size} bytes");
}
