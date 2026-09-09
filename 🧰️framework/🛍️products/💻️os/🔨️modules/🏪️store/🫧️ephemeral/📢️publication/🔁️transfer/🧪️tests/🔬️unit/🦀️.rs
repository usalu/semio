//! 🧪️ Transfer ownership is independent of payload size and closes with small grants.

use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static TRANSFERS: AtomicUsize = AtomicUsize::new(0);

fn transfer(value: String) -> String {
    TRANSFERS.fetch_add(1, Ordering::Relaxed);
    value
}

fn footprint(value: &String) -> Result<ArtifactStoreOneItemFootprint, String> {
    Ok(ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: value.capacity() })
}

fn close(preparation: &mut dyn ArtifactEphemeralOneItemPreparation<String, String>) {
    preparation.begin_close();
    for _ in 0..32_768 {
        match preparation.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 7 }).unwrap() {
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 7),
            SnapshotRetirementStep::Blocked => panic!("isolated transfer must close"),
        }
    }
    assert!(preparation.terminal_is_empty());
}

#[test]
fn ephemeral_transfer_preparation_preserves_handed_off_and_aliased_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let payload = fixture["payload"]["text"].as_str().unwrap().repeat(fixture["payload"]["repeat"].as_u64().unwrap() as usize);
    let retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<String>> = Arc::new(super::super::retirement::OwnedValueRetirementFactory::<String>::default());
    let factory = ArtifactEphemeralTransferPreparationFactory::new(footprint, std::convert::identity, retirement.clone(), retirement.clone());
    for hand_off in [false, true] {
        let request = ArtifactEphemeralOneItemPreparationRequest {
            operation: semio_framework_job::OperationId(1), generation: semio_framework_job::Generation(1),
            base: ArtifactEphemeralBaseRead(super::super::ArtifactEphemeralBaseOwner::Transient(Arc::new(payload.clone()))), mutation: payload.clone(),
        };
        let mut preparation = factory.begin(request).unwrap_or_else(|_| panic!("admitted string transfer"));
        let grant = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 0 };
        assert!(matches!(preparation.advance(grant).unwrap(), ArtifactStoreOneItemPreparationStep::Prepared(_)));
        let retained = if hand_off { preparation.take_prepared().unwrap().next_root } else { preparation.prepared().unwrap().next_root.clone() };
        if hand_off { assert!(matches!(preparation.advance(grant).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked)); }
        close(preparation.as_mut());
        assert_eq!(serde_json::to_value(retained.as_ref()).unwrap(), serde_json::to_value(&payload).unwrap());
        assert_eq!(Arc::strong_count(&retained), 1);
        let mut cleanup = ReturnedSnapshotReadRetirement::new(retained, retirement.clone());
        for _ in 0..32_768 {
            match cleanup.close_step(1, 7).unwrap() {
                SnapshotRetirementStep::Complete => break,
                SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 7),
                SnapshotRetirementStep::Blocked => panic!("isolated unique root must close"),
            }
        }
        assert!(cleanup.terminal_is_empty());
    }
    eprintln!("[DEBUG] ephemeral transfer: handed-off and aliased roots survive close; abandoned 12KiB raw base roots retire with seven-byte grants");
}

#[test]
fn ephemeral_transfer_preparation_returns_rejected_mutation_ownership_intact() {
    let retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<String>> = Arc::new(super::super::retirement::OwnedValueRetirementFactory::<String>::default());
    let factory = ArtifactEphemeralTransferPreparationFactory::new(footprint, std::convert::identity, retirement.clone(), retirement);
    let mut mutation = String::with_capacity(super::super::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES + 1);
    mutation.push_str("retained rejection");
    let pointer = mutation.as_ptr();
    let request = ArtifactEphemeralOneItemPreparationRequest {
        operation: semio_framework_job::OperationId(1), generation: semio_framework_job::Generation(1),
        base: ArtifactEphemeralBaseRead(super::super::ArtifactEphemeralBaseOwner::Transient(Arc::new(String::new()))), mutation,
    };
    let returned = match factory.begin(request) { Err(request) => request, Ok(_) => panic!("oversized retained capacity must reject") };
    assert_eq!(returned.mutation.as_ptr(), pointer);
    assert_eq!(returned.mutation, "retained rejection");
    eprintln!("[DEBUG] ephemeral transfer: admission returns oversized mutation allocation intact");
}

#[test]
fn ephemeral_transfer_preparation_obeys_neutral_grants_and_bounded_retirement() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let state: Arc<dyn ArtifactOwnedValueRetirementFactory<String>> = Arc::new(super::super::retirement::OwnedValueRetirementFactory::<String>::default());
    let factory = ArtifactEphemeralTransferPreparationFactory::new(footprint, transfer, state.clone(), state.clone());
    for row in fixture["cases"].as_array().unwrap() {
        let mutation = fixture["payload"]["text"].as_str().unwrap().repeat(fixture["payload"]["repeat"].as_u64().unwrap() as usize);
        let expected = serde_json::to_value(&mutation).unwrap();
        assert_eq!(mutation.len(), fixture["payload"]["utf8Bytes"].as_u64().unwrap() as usize);
        let pointer = mutation.as_ptr();
        let request = ArtifactEphemeralOneItemPreparationRequest {
            operation: semio_framework_job::OperationId(1), generation: semio_framework_job::Generation(1),
            base: ArtifactEphemeralBaseRead(super::super::ArtifactEphemeralBaseOwner::Transient(Arc::new(String::new()))), mutation,
        };
        TRANSFERS.store(0, Ordering::Relaxed);
        let mut preparation = factory.begin(request).unwrap_or_else(|_| panic!("admitted transfer fixture"));
        if row["cancelBefore"].as_bool().unwrap() { preparation.cancel(); }
        let grant = ArtifactStoreOneItemGrant { maximum_items: row["items"].as_u64().unwrap() as usize, maximum_bytes: row["bytes"].as_u64().unwrap() as usize };
        let step = preparation.advance(grant).unwrap();
        assert_eq!(matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(_)), row["prepared"].as_bool().unwrap(), "{}", row["id"]);
        assert_eq!(TRANSFERS.load(Ordering::Relaxed), row["transferCalls"].as_u64().unwrap() as usize);
        if let Some(prepared) = preparation.prepared() {
            assert_eq!(prepared.next_root.as_str().as_ptr(), pointer);
            assert_eq!(serde_json::to_value(prepared.next_root.as_ref()).unwrap(), expected);
            assert_eq!(preparation.checkpoint().completed_bytes, 0);
            preparation.advance(grant).unwrap();
            assert_eq!(TRANSFERS.load(Ordering::Relaxed), 1);
        }
        preparation.cancel();
        preparation.begin_close();
        assert_eq!(preparation.close_step(ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 4096 }).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        assert!(!preparation.terminal_is_empty());
        let grant = ArtifactStoreOneItemGrant { maximum_items: fixture["retirementGrant"]["maximumItems"].as_u64().unwrap() as usize, maximum_bytes: fixture["retirementGrant"]["maximumBytes"].as_u64().unwrap() as usize };
        for _ in 0..32_768 {
            match preparation.close_step(grant).unwrap() {
                SnapshotRetirementStep::Complete => break,
                SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 7),
                SnapshotRetirementStep::Blocked => panic!("isolated replacement has no blocked external owner"),
            }
        }
        assert!(preparation.terminal_is_empty());
    }
    eprintln!("[DEBUG] ephemeral transfer: four neutral grant/cancel cases preserve serde values and allocation identity; 12KiB owners retire under one-item/seven-byte grants");
}
