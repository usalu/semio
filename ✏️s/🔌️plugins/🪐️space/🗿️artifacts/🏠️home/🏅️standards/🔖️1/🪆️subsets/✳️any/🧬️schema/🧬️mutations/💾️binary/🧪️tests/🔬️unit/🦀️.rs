
use super::*;
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
    let mut store: store::ArtifactStore<SHomeSnapshot, SHomeMutation> = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![change_catalog_generation(3)], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}
