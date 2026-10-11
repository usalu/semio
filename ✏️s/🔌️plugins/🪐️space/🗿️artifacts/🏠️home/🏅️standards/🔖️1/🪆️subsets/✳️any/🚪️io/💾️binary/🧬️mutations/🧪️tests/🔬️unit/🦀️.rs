
use crate::standards::v1::subsets::any::io::binary::mutations::*;
use crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = change_catalog_generation(7);
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn home_document_text_round_trips_through_the_store() {
    use crate::SHomeSnapshot;
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
    let envelope = store::create_document_envelope::<SHomeSnapshot, SHomeMutation>("s.home", "home", projection, None);
    let mut store: store::ArtifactStore<SHomeSnapshot, SHomeMutation> = store::ArtifactStore::new(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    store.install_document_store_owners_exact(store::funded_bounded_artifact_store_owners::<SHomeSnapshot, SHomeMutation>().expect("funded bounded Home owners")).unwrap_or_else(|(error, _)| panic!("Home bounded owners install: {error:?}"));
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![change_catalog_generation(3)], transaction: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    while !store.close_owned_terminal_is_empty() {
        let demand = store.close_owned_demands(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Home document Store quotes its exact bounded owners");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let step = store.close_owned_step(grant).expect("Home document Store closes through its exact bounded owners");
        assert!(step.progress().fits(grant) && step.progress().copied_items <= 1);
    }
}
