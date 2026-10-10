//! 🧪️ Actual owned-state retirement laws at production and single-byte grants.
use super::*;

fn fixture() -> (serde_json::Value, serde_json::Value) {
    let cases = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/🏠️local-interaction/🔣️.json")).unwrap();
    let retirement = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/♻️retirement/🔣️.json")).unwrap();
    (cases, retirement)
}

fn close(cursor: &mut dyn ErasedSnapshotRetirement, bytes: usize) -> usize {
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📨️authority.json")).unwrap();let original:RetainedCloneGrant=serde_json::from_value(law["wholeOperationGrant"].clone()).unwrap();let mut recipient=store::NativeSnapshotBodyWallet::new(original);
    let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(RetainedCloneGrant{maximum_items:0,..recipient.remaining_grant()}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(serde_json::to_value([physical.requested_bytes,physical.released_bytes]).unwrap(),law["zeroItemPhysicalBytes"]);
    let(mut released,mut original_completed,mut physical_matches)=(0,false,true);
    for _ in 0..200_000 {
        let remaining=recipient.remaining_grant();let grant=RetainedCloneGrant{maximum_items:remaining.maximum_items.min(1),maximum_copy_bytes:remaining.maximum_copy_bytes.min(bytes),maximum_release_bytes:remaining.maximum_release_bytes.min(bytes),..remaining};
        let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(grant).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();physical_matches&=(physical.requested_bytes,physical.released_bytes)==(progress.retained_capacity_bytes,progress.released_bytes);assert!(progress.copied_items<=1);assert!(progress.copied_bytes<=bytes&&progress.released_bytes<=bytes);released+=progress.released_bytes;
        if matches!(step,RetainedCloneStep::Complete(_)){original_completed=true;break}if progress==Default::default(){break}
    }
    for _ in 0..200_000 {
        if cursor.terminal_is_empty(){break}let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(recipient.remaining_grant()).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();physical_matches&=(physical.requested_bytes,physical.released_bytes)==(progress.retained_capacity_bytes,progress.released_bytes);if matches!(step,RetainedCloneStep::Complete(_)){break}
    }
    eprintln!("[DEBUG] Original interaction retirement workLimit={bytes} originalCompleted={original_completed} actualPhysicalReceipts={physical_matches} releasedUnderOriginalLimit={released} cumulativeOriginalReceipt={:?} terminalEmpty={}",recipient.progress(),cursor.terminal_is_empty());
    assert!(cursor.terminal_is_empty(),"original retirement caller owner did not close");assert!(physical_matches,"retirement receipt differs from actual native allocator/release");assert!(original_completed,"original positive interaction retirement work byte limit {bytes} blocked");assert!(recipient.progress().fits(original));released
}

#[test]
fn local_interaction_retirement_matches_language_neutral_exact_bytes() {
    let (fixture, contract) = fixture();
    for grant in [1, 64, 4096] {
        for row in contract["cases"].as_array().unwrap() {
            let source = fixture["cases"].as_array().unwrap().iter().find(|case| case["id"] == row["sourceCase"]).unwrap();
            let mut value = source[row["sourceField"].as_str().unwrap()].clone();
            value["hover"] = serde_json::json!({});
            let state: InteractionState = serde_json::from_value(value).unwrap();
            let mut retirement = InteractionRetirement::owned(state);
            assert_eq!(close(&mut retirement, grant), row["expectedReleasedBytes"].as_u64().unwrap() as usize);
        }
    }
}

#[test]
fn local_interaction_retirement_shared_alias_and_final_owner_are_distinct() {
    let mut state = InteractionState::default();
    state.active_granularity.insert("domain".into(), "粒度🌊".repeat(1024));
    let expected = "domain".len() + "粒度🌊".len() * 1024;
    let root = Arc::new(state);
    let mut shared = InteractionRetirement::shared(root.clone());
    assert_eq!(close(&mut shared, 1), 0);
    assert_eq!(Arc::strong_count(&root), 1);
    let mut final_owner = InteractionRetirement::shared(root);
    assert_eq!(close(&mut final_owner, 1), expected);
}

#[test]
fn local_interaction_retirement_releases_empty_reserved_allocations() {
    let mut state = InteractionState::default();
    state.selection.insert("x".into(), DomainSelection { granularity: String::with_capacity(16384), ids: Vec::with_capacity(8192), anchor_id: Some(String::with_capacity(8192)) });
    state.hover.insert("y".into(), DomainHover { channel: String::with_capacity(16384), ids: Vec::with_capacity(8192) });
    let mut retirement = InteractionRetirement::owned(state);
    assert_eq!(close(&mut retirement, 1), 2);
    assert!(retirement.terminal_is_empty());
}

#[test]
fn local_interaction_retirement_live_drop_is_rejected() {
    assert!(std::panic::catch_unwind(|| {
        drop(InteractionRetirement::owned(InteractionState::default()));
    })
    .is_err());
}
