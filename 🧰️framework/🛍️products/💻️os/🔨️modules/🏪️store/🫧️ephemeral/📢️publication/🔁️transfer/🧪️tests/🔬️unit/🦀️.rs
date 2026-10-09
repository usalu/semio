//! 🧪️ Transfer retains payload identity under independently funded physical grants.

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

fn physical_grant() -> ArtifactStoreOneItemGrant {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let policy = &fixture["retirementGrant"];
    let n = |axis: &str| policy[axis].as_u64().unwrap() as usize;
    ArtifactStoreOneItemGrant { maximum_items: n("maximumItems"), maximum_copy_bytes: n("maximumCopyBytes"), maximum_capacity_bytes: n("maximumCapacityBytes"), maximum_release_bytes: n("maximumReleaseBytes"), maximum_depth: n("maximumDepth") }
}

fn close(preparation: &mut dyn ArtifactEphemeralOneItemPreparation<String, String>) {
    preparation.begin_close();
    let grant = physical_grant();
    for _ in 0..32_768 {
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| preparation.close_step(grant).unwrap());
        assert!(step.progress().fits(grant.retained_grant()));
        assert_eq!((allocated, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        if preparation.terminal_is_empty() { return; }
    }
    panic!("isolated transfer must close under its fixed physical grant");
}

#[test]
fn ephemeral_transfer_preparation_preserves_handed_off_and_aliased_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let payload = fixture["payload"]["text"].as_str().unwrap().repeat(fixture["payload"]["repeat"].as_u64().unwrap() as usize);
    let retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<String>> = Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<String>::default());
    let factory = ArtifactEphemeralTransferPreparationFactory::new(footprint, std::convert::identity, retirement.clone(), retirement.clone());
    for hand_off in [false, true] {
        let request = ArtifactEphemeralOneItemPreparationRequest {
            operation: semio_framework_job::OperationId(1),
            generation: semio_framework_job::Generation(1),
            base: ArtifactEphemeralBaseRead(super::super::ArtifactEphemeralBaseOwner::Transient(Arc::new(payload.clone()))),
            mutation: payload.clone(),
        };
        let mut preparation = factory.begin(request).unwrap_or_else(|_| panic!("admitted string transfer"));
        let grant = ArtifactStoreOneItemGrant { maximum_copy_bytes: 64, maximum_capacity_bytes: 256, ..physical_grant() };
        assert!(matches!(preparation.advance(grant).unwrap(), ArtifactStoreOneItemPreparationStep::Prepared(..)));
        let retained = if hand_off { preparation.take_prepared().unwrap().next_root } else { preparation.prepared().unwrap().next_root.clone() };
        if hand_off {
            assert!(matches!(preparation.advance(grant).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
        }
        close(preparation.as_mut());
        assert_eq!(serde_json::to_value(retained.as_ref()).unwrap(), serde_json::to_value(&payload).unwrap());
        assert_eq!(Arc::strong_count(&retained), 1);
        let mut cleanup = ReturnedSnapshotReadRetirement::new(retained, retirement.clone());
        for _ in 0..32_768 {
            let grant = physical_grant().retained_grant();
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| cleanup.close_step(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!((allocated, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            if cleanup.terminal_is_empty() { break; }
        }
        assert!(cleanup.terminal_is_empty());
    }
}

#[test]
fn ephemeral_transfer_preparation_returns_rejected_mutation_ownership_intact() {
    let retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<String>> = Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<String>::default());
    let factory = ArtifactEphemeralTransferPreparationFactory::new(footprint, std::convert::identity, retirement.clone(), retirement);
    let mut mutation = String::with_capacity(super::super::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES + 1);
    mutation.push_str("retained rejection");
    let pointer = mutation.as_ptr();
    let request = ArtifactEphemeralOneItemPreparationRequest {
        operation: semio_framework_job::OperationId(1),
        generation: semio_framework_job::Generation(1),
        base: ArtifactEphemeralBaseRead(super::super::ArtifactEphemeralBaseOwner::Transient(Arc::new(String::new()))),
        mutation,
    };
    let returned = match factory.begin(request) {
        Err(request) => request,
        Ok(_) => panic!("oversized retained capacity must reject"),
    };
    assert_eq!(returned.mutation.as_ptr(), pointer);
    assert_eq!(returned.mutation, "retained rejection");
}

#[test]
fn ephemeral_transfer_preparation_obeys_neutral_grants_and_bounded_retirement() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let state: Arc<dyn ArtifactOwnedValueRetirementFactory<String>> = Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<String>::default());
    let factory = ArtifactEphemeralTransferPreparationFactory::new(footprint, transfer, state.clone(), state.clone());
    for row in fixture["cases"].as_array().unwrap() {
        let mutation = fixture["payload"]["text"].as_str().unwrap().repeat(fixture["payload"]["repeat"].as_u64().unwrap() as usize);
        let expected = serde_json::to_value(&mutation).unwrap();
        assert_eq!(mutation.len(), fixture["payload"]["utf8Bytes"].as_u64().unwrap() as usize);
        let pointer = mutation.as_ptr();
        let request = ArtifactEphemeralOneItemPreparationRequest {
            operation: semio_framework_job::OperationId(1),
            generation: semio_framework_job::Generation(1),
            base: ArtifactEphemeralBaseRead(super::super::ArtifactEphemeralBaseOwner::Transient(Arc::new(String::new()))),
            mutation,
        };
        TRANSFERS.store(0, Ordering::Relaxed);
        let mut preparation = factory.begin(request).unwrap_or_else(|_| panic!("admitted transfer fixture"));
        if row["cancelBefore"].as_bool().unwrap() {
            preparation.cancel();
        }
        let grant = ArtifactStoreOneItemGrant { maximum_items: row["items"].as_u64().unwrap() as usize, maximum_copy_bytes: row["copyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: row["capacityBytes"].as_u64().unwrap() as usize, ..physical_grant() };
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| preparation.advance(grant).unwrap());
        let progress = step.ownership_progress();
        assert!(progress.fits(grant.retained_grant()));
        assert_eq!((allocated, released), (progress.retained_capacity_bytes, progress.released_bytes));
        assert_eq!(matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(..)), row["prepared"].as_bool().unwrap(), "{}", row["id"]);
        assert_eq!(TRANSFERS.load(Ordering::Relaxed), row["transferCalls"].as_u64().unwrap() as usize);
        if let Some(prepared) = preparation.prepared() {
            assert_eq!(prepared.next_root.as_str().as_ptr(), pointer);
            assert_eq!(serde_json::to_value(prepared.next_root.as_ref()).unwrap(), expected);
            assert_eq!(preparation.checkpoint().completed_bytes as usize, size_of::<String>() * 2);
            let metadata = &fixture["transferMetadata"][usize::BITS.to_string()];
            assert_eq!(progress.copied_bytes, metadata["copyBytes"].as_u64().unwrap() as usize);
            assert_eq!(progress.retained_capacity_bytes, metadata["arcBytes"].as_u64().unwrap() as usize);
            preparation.advance(grant).unwrap();
            assert_eq!(TRANSFERS.load(Ordering::Relaxed), 1);
        }
        preparation.cancel();
        preparation.begin_close();
        let denied = preparation.close_step(ArtifactStoreOneItemGrant { maximum_items: 0, ..physical_grant() }).unwrap();
        assert_eq!(denied.progress(), semio_framework_value::RetainedCloneProgress::default());
        assert!(!preparation.terminal_is_empty());
        close(preparation.as_mut());
        assert!(preparation.terminal_is_empty());
    }
}
