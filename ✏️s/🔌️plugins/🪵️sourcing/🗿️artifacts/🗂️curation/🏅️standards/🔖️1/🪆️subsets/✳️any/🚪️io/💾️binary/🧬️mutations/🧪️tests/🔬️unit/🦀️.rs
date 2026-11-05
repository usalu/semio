use crate::schema::mutations::SourcingMutation;
#[allow(unused_imports)]
use protocol::OpBinary;

use crate::standards::v1::subsets::any::io::binary::mutations::*;
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
    let envelope = store::os_store::document_materialize::create_document_envelope(crate::SOURCING_CURATION_SCHEMA, "sourcing-curation-test", document, None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut doc_store = store::ArtifactStore::new::<::semio_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 16 << 20, maximum_capacity_bytes: 16 << 20, maximum_release_bytes: 16 << 20, maximum_depth: 512 };
    let owners = semio_os_kernel::os_store::document_store_owners::funded_bounded_artifact_store_owners::<CurationSnapshot, SourcingMutation>().expect("funded bounded document owners");
    doc_store.install_document_store_owners_exact(owners).unwrap_or_else(|(error, _)| panic!("exact document store owners install: {error:?}"));
    let object_id = crate::stock_of(&doc_store.snapshot().expect("snapshot"))[0].id.clone();
    let mutation = crate::schema::mutations::create_curated_item(crate::CuratedItem { object_id, count: 3 });
    doc_store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }, &mut semio_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&doc_store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&doc_store).await;
    for _ in 0..500_000 {
        if matches!(store::SpaceMember::close_owned_step(&mut doc_store, grant).expect("bounded store close"), semio_framework_value::RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(store::SpaceMember::close_owned_terminal_is_empty(&doc_store), "store must reach its terminal-empty shell before drop");
}
