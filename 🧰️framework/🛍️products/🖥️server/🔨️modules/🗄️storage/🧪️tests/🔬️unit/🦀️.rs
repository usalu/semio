
use crate::conformance;
use crate::gateway::{InstanceStores, ServerInstance};
use crate::storage::StorageProfile;
use crate::test_instance::{MemoryProjectionStore, MemorySessionStore, TestInstance};

/// 🧩️ The four stores of the reference instance, opened the way a server opens them — so the suite
/// below judges `TestInstance` itself and not four types that merely happen to live next to it.
async fn stores() -> InstanceStores<TestInstance> {
    TestInstance::open(&StorageProfile::Ephemeral).await.expect("the test instance opens its stores")
}

//#region 🔖️Profile
#[semio_framework_async_macros::async_test]
async fn a_profile_answers_for_its_own_directory_without_being_matched_on() {
    assert_eq!(StorageProfile::Ephemeral.data_dir(), None);
    assert!(!StorageProfile::Ephemeral.is_durable());
    let embedded = StorageProfile::Embedded { data_dir: "/tmp/semio-storage".to_string() };
    assert_eq!(embedded.data_dir(), Some("/tmp/semio-storage"));
    assert!(embedded.is_durable());
    assert_eq!(serde_json::to_value(&StorageProfile::Ephemeral).unwrap(), serde_json::json!({ "kind": "ephemeral" }));
    assert_eq!(serde_json::to_value(&embedded).unwrap(), serde_json::json!({ "kind": "embedded", "dataDir": "/tmp/semio-storage" }));
}
//#endregion 🔖️Profile

//#region 🔖️Authority
#[semio_framework_async_macros::async_test]
async fn receipt_round_trips_and_is_idempotent() {
    conformance::receipt_round_trips_and_is_idempotent(&mut stores().await.authority).await;
}

#[semio_framework_async_macros::async_test]
async fn append_events_rejects_a_sequence_gap_and_writes_nothing() {
    conformance::append_events_rejects_a_sequence_gap_and_writes_nothing(&mut stores().await.authority).await;
}

#[semio_framework_async_macros::async_test]
async fn events_since_returns_only_later_events_of_that_actor() {
    conformance::events_since_returns_only_later_events_of_that_actor(&mut stores().await.authority).await;
}

#[semio_framework_async_macros::async_test]
async fn snapshots_only_move_forward() {
    conformance::snapshots_only_move_forward(&mut stores().await.authority).await;
}

#[semio_framework_async_macros::async_test]
async fn outbox_delivers_each_entry_exactly_once() {
    conformance::outbox_delivers_each_entry_exactly_once(&mut stores().await.authority).await;
}

#[semio_framework_async_macros::async_test]
async fn lease_epoch_bump_fences_out_the_previous_holder() {
    conformance::lease_epoch_bump_fences_out_the_previous_holder(&mut stores().await.authority).await;
}

/// 🔁️ The reference instance keeps nothing, so its "restart" is a second `open` — and the law holds
/// for the same reason a durable backend's does: a queue that was drained hands the workflow
/// nothing a second time.
#[semio_framework_async_macros::async_test]
async fn a_saga_reacts_to_a_committed_event_exactly_once_across_a_restart() {
    conformance::a_saga_reacts_to_a_committed_event_exactly_once_across_a_restart(&mut stores().await.authority, || async { stores().await.authority }).await;
}
//#endregion 🔖️Authority

//#region 🔖️Projection
#[semio_framework_async_macros::async_test]
async fn projection_list_is_prefix_scoped_and_key_ordered() {
    conformance::projection_list_is_prefix_scoped_and_key_ordered(&mut stores().await.projections).await;
}

#[semio_framework_async_macros::async_test]
async fn clearing_a_projection_resets_it_for_rebuild() {
    conformance::clearing_a_projection_resets_it_for_rebuild(&mut stores().await.projections).await;
}

#[semio_framework_async_macros::async_test]
async fn a_projection_write_reports_a_failing_sink() {
    let mut store = MemoryProjectionStore::new();
    conformance::seed_projection_fault_fixture(&mut store).await;
    store.fail();
    conformance::a_projection_write_reports_a_failing_sink(&mut store).await;
}
//#endregion 🔖️Projection

//#region 🔖️Blob
#[semio_framework_async_macros::async_test]
async fn blob_put_get_and_has_are_content_addressed() {
    conformance::blob_put_get_and_has_are_content_addressed(&mut stores().await.blobs).await;
}
//#endregion 🔖️Blob

//#region 🔖️Session
#[semio_framework_async_macros::async_test]
async fn revoke_principal_removes_every_session_of_that_principal() {
    conformance::revoke_principal_removes_every_session_of_that_principal(&mut stores().await.sessions).await;
}

#[semio_framework_async_macros::async_test]
async fn a_session_write_reports_a_failing_sink() {
    let mut store = MemorySessionStore::new();
    conformance::seed_session_fault_fixture(&mut store).await;
    store.fail();
    conformance::a_session_write_reports_a_failing_sink(&mut store).await;
}
//#endregion 🔖️Session
