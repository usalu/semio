//! 🧪️ Actual Store read leases, bounded byte output, cancellation, and return-maintenance ownership.
use super::*;
use crate::app::InteractionConfigMutation;
use crate::local_interaction::retirement::interaction_store_owners;
use store::{ArtifactStore, SpaceMember};

type InteractionStore = ArtifactStore<InteractionState, InteractionConfigMutation>;

async fn fixture() -> (InteractionStore, LocalInteractionCaptureCursor, Vec<u8>) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️fixtures/🔣️local-interaction.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == "semantic-unicode-over-page").unwrap();
    let mut state = row["expected"].clone();
    state["hover"] = serde_json::json!({"private": {"channel": "pointer", "ids": ["hover-is-not-captured"]}});
    let state: InteractionState = serde_json::from_value(state).unwrap();
    let envelope = store::create_document_envelope::<InteractionState, InteractionConfigMutation>("framework.interaction", "local-capture-test", state, None);
    let mut store = InteractionStore::new(envelope).await.unwrap();
    store.install_member_store_owners_exact(interaction_store_owners());
    let identity = LocalInteractionIdentity { app_instance_id: 7, generation: store.generation_now(), revision: store.content_revision_now(), document_revision: [2; 32], topology_revision: [3; 32] };
    let expected = serde_json::to_vec(&serde_json::json!({"identity": identity, "state": row["expected"]})).unwrap();
    let cursor = LocalInteractionCaptureCursor::new(store.snapshot_read().unwrap(), identity);
    (store, cursor, expected)
}

fn finish(cursor: &mut LocalInteractionCaptureCursor, bytes: usize) -> Vec<u8> {
    let mut result = Vec::new();
    for _ in 0..200_000 {
        let prior = cursor.completed_bytes();
        let mut output = [0; 4096];
        let count = cursor.write_chunk(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }, &mut output).unwrap();
        assert!(count <= bytes.min(256));
        assert_eq!(cursor.completed_bytes() - prior, count as u64);
        result.extend_from_slice(&output[..count]);
        if cursor.complete() { return result; }
    }
    panic!("capture failed to complete");
}

fn close(store: &mut InteractionStore, cursor: &mut LocalInteractionCaptureCursor, bytes: usize) {
    cursor.begin_close();
    let mut retirement: Option<Box<dyn ErasedSnapshotRetirement>> = None;
    for _ in 0..1_000_000 {
        if !cursor.terminal_is_empty() {
            match cursor.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap() {
                SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= bytes),
                SnapshotRetirementStep::Complete => assert!(cursor.terminal_is_empty()),
                SnapshotRetirementStep::Blocked => {},
            }
        }
        if retirement.is_none() { retirement = store.take_returned_snapshot_read_retirement().unwrap(); }
        if let Some(active) = retirement.as_mut() {
            match active.close_step(1, bytes).unwrap() {
                SnapshotRetirementStep::Complete => { assert!(active.terminal_is_empty()); retirement = None; },
                SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= bytes),
                SnapshotRetirementStep::Blocked => {},
            }
        }
        if cursor.terminal_is_empty() && retirement.is_none() && store.snapshot_read_leases_terminal_is_empty() { break; }
    }
    assert!(cursor.terminal_is_empty());
    assert!(retirement.is_none());
    assert!(store.snapshot_read_leases_terminal_is_empty());
    for _ in 0..1_000_000 {
        match store.close_owned_step(1, bytes).unwrap() {
            SnapshotRetirementStep::Complete => { assert!(store.close_owned_terminal_is_empty()); return; },
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= bytes),
            SnapshotRetirementStep::Blocked => {},
        }
    }
    panic!("capture Store failed to close");
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_capture_actual_store_matches_canonical_fixture_at_small_grants() {
    for bytes in [1, 64, 4096] {
        let (mut store, mut cursor, expected) = fixture().await;
        assert_eq!(cursor.write_chunk(ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: bytes }, &mut [0; 1]).unwrap(), 0);
        assert_eq!(cursor.write_chunk(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 0 }, &mut [0; 1]).unwrap(), 0);
        assert_eq!(cursor.completed_bytes(), 0);
        assert_eq!(finish(&mut cursor, bytes), expected);
        close(&mut store, &mut cursor, bytes);
    }
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_capture_cancel_worker_transfer_and_exact_registry_return() {
    for prefix in [0, 17, 4097, usize::MAX] {
        let (mut store, mut cursor, _) = fixture().await;
        if prefix == usize::MAX { finish(&mut cursor, 4096); }
        else { for _ in 0..prefix { cursor.write_chunk(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }, &mut [0; 1]).unwrap(); } }
        cursor = std::thread::spawn(move || cursor).join().unwrap();
        cursor.cancel();
        let before = cursor.completed_bytes();
        assert_eq!(cursor.write_chunk(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 }, &mut [0; 4096]).unwrap(), 0);
        assert_eq!(cursor.completed_bytes(), before);
        assert!(!cursor.terminal_is_empty());
        close(&mut store, &mut cursor, 1);
    }
}
