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
async fn selecting_another_example_loads_it_without_an_edit() {
    let mut app = app_with_registry().await;
    let dispatched = dispatch(&mut app, RemodelingCommand::SetActiveExample(SetActiveExample { example_id: crate::examples::synthetic_orbit::ID.into() })).await;
    assert!(!dispatched.edited_document(), "an example switch is a load, never an edit: {:?}", dispatched.lanes);
    let loaded = app.snapshot().expect("snapshot after");
    assert_eq!(loaded.assets.len(), crate::examples::synthetic_orbit::FRAMES.len(), "every committed frame loads as asset content");
    assert!(!loaded.streams.is_empty(), "the example's streams load with it");
}
