
use super::*;
use crate::CurationSnapshot;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = crate::schema::mutations::create_curated_item(crate::CuratedItem { object_id: "beam-glulam-gl24h".into(), count: 3 });
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn curation_document_text_round_trips_through_a_vcs_store() {
    let document = crate::curation_snapshot_from_stock(&crate::schema::demo_stock(), Vec::new());
    let envelope = store::create_document_envelope(crate::SOURCING_CURATION_SCHEMA, "sourcing-curation-test", document, None);
    let mut doc_store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    doc_store.install_member_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<CurationSnapshot, SourcingMutation>());
    let object_id = crate::stock_of(&doc_store.snapshot().expect("snapshot"))[0].id.clone();
    let mutation = crate::schema::mutations::create_curated_item(crate::CuratedItem { object_id, count: 3 });
    doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&doc_store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&doc_store).await;
    for _ in 0..500_000 {
        if store::SpaceMember::close_owned_step(&mut doc_store, 1, 4096).expect("bounded store close") == store::SnapshotRetirementStep::Complete {
            break;
        }
    }
    assert!(store::SpaceMember::close_owned_terminal_is_empty(&doc_store), "store must reach its terminal-empty shell before drop");
}
