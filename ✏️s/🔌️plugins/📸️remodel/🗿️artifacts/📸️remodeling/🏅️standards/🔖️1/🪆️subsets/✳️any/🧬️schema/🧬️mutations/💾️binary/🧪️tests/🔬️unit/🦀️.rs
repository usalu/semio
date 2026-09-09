use super::*;
use crate::default_remodeling_scene;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let scene = default_remodeling_scene();
    let operation = crate::mutations::update_feature_params(scene.params.feature);
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

/// 📄️ Full `print_document_text`/`parse_document_text` round trip through a live `ArtifactStore`
/// with an applied edit, the ground-truth contract for replacing the JSON envelope with text files.
#[semio_framework_async_macros::async_test]
async fn store_roundtrips_through_document_text() {
    let initial = default_remodeling_scene();
    let envelope = store::create_document_envelope("test/v1", "test", initial, None);
    let mut store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    let mut feature_params = store.snapshot().expect("initial projection").params.feature;
    feature_params.target_count = 12345;
    store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::mutations::update_feature_params(feature_params)], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}
