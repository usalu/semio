
use super::*;
use crate::editor::remodeling::RemodelingCommand;
use crate::editor::remodeling::examples::REMODELING_EXAMPLES;
use crate::editor::remodeling::testkit::{app, dispatch};

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
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot before");
    dispatch(&mut app, RemodelingCommand::SetActiveExample(SetActiveExample { example_id: "nonsense".into() })).await;
    assert_eq!(before, app.snapshot().expect("snapshot after"));
}
