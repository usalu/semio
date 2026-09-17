use super::*;
use crate::editor::remodeling::examples::REMODELING_EXAMPLES;
use crate::editor::remodeling::unit_tests::context::{app_with_registry, dispatch};
use crate::editor::remodeling::RemodelingCommand;

#[semio_framework_async_macros::async_test]
async fn every_registered_example_id_resolves_to_committed_text() {
    assert!(!REMODELING_EXAMPLES.is_empty(), "the example registry must not be empty");
    for example in REMODELING_EXAMPLES {
        assert!(example_text(example.id).is_some(), "example '{}' must resolve", example.id);
        assert!(!example.text.is_empty(), "example '{}' must carry committed text", example.id);
    }
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_example_id_is_a_no_op() {
    let mut app = app_with_registry().await;
    let before = app.snapshot().expect("snapshot before");
    let dispatched = dispatch(&mut app, RemodelingCommand::SetActiveExample(SetActiveExample { example_id: "nonsense".into() })).await;
    assert!(!dispatched.edited_document(), "an unknown id writes nothing: {:?}", dispatched.lanes);
    assert_eq!(before, app.snapshot().expect("snapshot after"));
}

#[semio_framework_async_macros::async_test]
async fn re_selecting_the_boot_example_writes_no_edit() {
    let mut app = app_with_registry().await;
    let before = app.snapshot().expect("snapshot before");
    let dispatched = dispatch(&mut app, RemodelingCommand::SetActiveExample(SetActiveExample { example_id: crate::editor::remodeling::examples::REMODELING_EXAMPLE_BOOT_ID.into() })).await;
    assert!(!dispatched.edited_document(), "the boot document already equals the boot example: {:?}", dispatched.lanes);
    assert_eq!(before, app.snapshot().expect("snapshot after"));
}

#[semio_framework_async_macros::async_test]
async fn the_replace_set_is_the_declared_state_diff() {
    let current = crate::default_remodeling_scene();
    assert!(replace_document_operations(&current, &current).is_empty(), "an equal document diffs to nothing");
    let mut next = current.clone();
    next.params.ingest.max_frames += 1;
    let mutations = replace_document_operations(&current, &next);
    assert_eq!(mutations.len(), 1, "only the changed parameter group is rewritten: {mutations:?}");
}
