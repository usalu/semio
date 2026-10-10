//! 🧪️ Actual Store read leases, bounded byte output, cancellation, and return-maintenance ownership.
use super::*;
use crate::app::InteractionConfigMutation;
use store::{ArtifactStore, SpaceMember};

type InteractionStore = ArtifactStore<InteractionState, InteractionConfigMutation>;

async fn fixture() -> (InteractionStore, LocalInteractionCaptureCursor, Vec<u8>, store::NativeSnapshotBodyWallet) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧫️fixtures/🏠️local-interaction/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == "semantic-unicode-over-page").unwrap();
    let mut state = row["expected"].clone();
    state["hover"] = serde_json::json!({"private": {"channel": "pointer", "ids": ["hover-is-not-captured"]}});
    let state: InteractionState = serde_json::from_value(state).unwrap();
    let envelope = store::create_document_envelope::<InteractionState, InteractionConfigMutation>("framework.interaction", "local-capture-test", state, None);
    let mut store = InteractionStore::new(envelope, protocol::ActorId(store::os_spr::LOCAL_ACTOR_ID.into())).await.unwrap();
    let mut recipient=recipient();crate::local_interaction::query::tests::install_original_interaction_owners(&mut store,&mut recipient);
    let identity = LocalInteractionIdentity { app_instance_id: 7, generation: store.generation_now(), revision: store.content_revision_now(), document_revision: [2; 32], topology_revision: [3; 32] };
    let hex = |bytes: &[u8; 32]| bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let expected = serde_json::to_vec(&serde_json::json!({"identity": {
        "appInstanceId": identity.app_instance_id,
        "documentRevision": hex(&identity.document_revision),
        "generation": identity.generation.to_string(),
        "revision": hex(&identity.revision),
        "topologyRevision": hex(&identity.topology_revision),
    }, "state": row["expected"]}))
    .unwrap();
    let cursor = LocalInteractionCaptureCursor::new(store.snapshot_read().unwrap(), identity);
    (store, cursor, expected,recipient)
}

fn recipient()->store::NativeSnapshotBodyWallet {
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../📃️query/🧫️fixtures/📨️authority.json")).unwrap();
    store::NativeSnapshotBodyWallet::new(serde_json::from_value(law["wholeOperationGrant"].clone()).unwrap())
}
fn finish(cursor: &mut LocalInteractionCaptureCursor, bytes: usize, recipient:&mut store::NativeSnapshotBodyWallet) -> Vec<u8> {
    let mut result = Vec::new();
    for _ in 0..200_000 {
        let prior=cursor.completed_bytes();let mut output=[0;4096];let grant=ArtifactStoreOneItemGrant::from_retained(recipient.remaining_grant());
        let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.write_chunk(grant,&mut output[..bytes.min(256)]).unwrap());let progress=step.ownership.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));let count=step.written_bytes;
        assert!(count<=bytes.min(256));assert_eq!(cursor.completed_bytes()-prior,count as u64);result.extend_from_slice(&output[..count]);if cursor.complete(){return result}
    }
    panic!("capture failed to complete");
}
fn close(store: &mut InteractionStore, cursor: &mut LocalInteractionCaptureCursor, recipient:&mut store::NativeSnapshotBodyWallet) {
    cursor.begin_close();let mut retirement:Option<Box<dyn ErasedSnapshotRetirement>>=None;
    for _ in 0..1_000_000 {
        let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(recipient.remaining_grant()).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
        if retirement.is_none(){let((owner,progress),physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||store.take_returned_snapshot_read_retirement(recipient.remaining_grant()).unwrap());recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));retirement=owner;}
        if let Some(active)=retirement.as_mut(){let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||active.close_step(recipient.remaining_grant()).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){assert!(active.terminal_is_empty());let(_,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(retirement.take()));assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));}}
        if cursor.terminal_is_empty()&&retirement.is_none()&&store.snapshot_read_leases_terminal_is_empty(){break}
    }
    assert!(cursor.terminal_is_empty());assert!(retirement.is_none());assert!(store.snapshot_read_leases_terminal_is_empty());
    for _ in 0..1_000_000 {let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||store.close_owned_step(recipient.remaining_grant()).unwrap());let progress=step.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){assert!(store.close_owned_terminal_is_empty());return}}
    panic!("capture Store failed to close");
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_capture_actual_store_matches_canonical_fixture_at_small_grants() {
    for bytes in [1, 64, 4096] {
        let (mut store, mut cursor, expected,mut recipient) = fixture().await;let original=recipient.remaining_grant();
        assert_eq!(cursor.write_chunk(ArtifactStoreOneItemGrant::from_retained(RetainedCloneGrant{maximum_items:0,..original}),&mut [0;1]).unwrap().written_bytes,0);
        assert_eq!(cursor.write_chunk(ArtifactStoreOneItemGrant::from_retained(RetainedCloneGrant{maximum_copy_bytes:0,..original}),&mut [0;1]).unwrap().written_bytes,0);
        assert_eq!(cursor.completed_bytes(), 0);
        assert_eq!(finish(&mut cursor,bytes,&mut recipient), expected);
        close(&mut store,&mut cursor,&mut recipient);
    }
}

#[semio_framework_async_macros::async_test]
async fn local_interaction_capture_cancel_worker_transfer_and_exact_registry_return() {
    for prefix in [0, 17, 4097, usize::MAX] {
        let (mut store, mut cursor, _,mut recipient) = fixture().await;
        if prefix == usize::MAX {
            finish(&mut cursor,4096,&mut recipient);
        } else {
            for _ in 0..prefix {
                let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.write_chunk(ArtifactStoreOneItemGrant::from_retained(recipient.remaining_grant()),&mut [0;1]).unwrap());let progress=step.ownership.progress();recipient.record_progress(progress).unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
            }
        }
        cursor = std::thread::spawn(move || cursor).join().unwrap();
        cursor.cancel();
        let before = cursor.completed_bytes();
        assert_eq!(cursor.write_chunk(ArtifactStoreOneItemGrant::from_retained(recipient.remaining_grant()),&mut [0;4096]).unwrap().written_bytes,0);
        assert_eq!(cursor.completed_bytes(), before);
        assert!(!cursor.terminal_is_empty());
        close(&mut store,&mut cursor,&mut recipient);
    }
}
