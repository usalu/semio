
use super::*;
use crate::editor::sourcing::SourcingCurationCommand;
use crate::editor::sourcing::commands::{curation_remove, curation_set_count, drop_on_curated, drop_on_pool};
use crate::editor::sourcing::testkit::{dispatch, new_app};
use crate::schema::curated_count;

#[semio_framework_async_macros::async_test]
async fn curation_add_and_remove_round_trip_through_operations() {
    let mut app = new_app().await;
    let document = app.snapshot().expect("snapshot");
    // stock[2] isn't part of the fixture's pre-curated set, so a single add lands on count 1.
    let object_id = document.stock_extra[2].id.clone();
    dispatch(&mut app, SourcingCurationCommand::CurationAdd(CurationAdd { object_id: object_id.clone() })).await;
    assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 1);

    dispatch(&mut app, SourcingCurationCommand::CurationRemove(curation_remove::CurationRemove { object_id: object_id.clone() })).await;
    assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 0);
}

#[semio_framework_async_macros::async_test]
async fn curation_set_count_supports_both_delta_and_absolute_value() {
    let mut app = new_app().await;
    let object_id = app.snapshot().expect("snapshot").stock_extra[2].id.clone();
    dispatch(&mut app, SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: object_id.clone(), delta: Some(3.0), value: None })).await;
    assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 3);
    dispatch(&mut app, SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: object_id.clone(), delta: None, value: Some(2.0) })).await;
    assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 2);
}

#[semio_framework_async_macros::async_test]
async fn drop_on_curated_and_drop_on_pool_mirror_add_and_remove() {
    let mut app = new_app().await;
    let document = app.snapshot().expect("snapshot");
    // stock[2] isn't part of the fixture's pre-curated set, so a single drop lands on count 1.
    let object_id = document.stock_extra[2].id.clone();
    dispatch(&mut app, SourcingCurationCommand::DropOnCurated(drop_on_curated::DropOnCurated { object_id: object_id.clone() })).await;
    assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 1);

    dispatch(&mut app, SourcingCurationCommand::DropOnPool(drop_on_pool::DropOnPool { object_id: object_id.clone() })).await;
    assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 0);
}

/// 🧬️ A no-op adjustment (removing an object that was never curated) must emit NOTHING —
/// `SourcingMutation` has no whole-snapshot no-op sentinel to fall back on any more.
#[semio_framework_async_macros::async_test]
async fn curation_remove_on_an_uncurated_object_emits_no_mutation() {
    let mut app = new_app().await;
    let object_id = app.snapshot().expect("snapshot").stock_extra[2].id.clone();
    assert_eq!(curated_count(&app.snapshot().expect("snapshot"), &object_id), 0);
    let result = dispatch(&mut app, SourcingCurationCommand::CurationRemove(curation_remove::CurationRemove { object_id })).await;
    assert!(result.mutations.is_empty(), "removing an already-uncurated object is a no-op");
}
