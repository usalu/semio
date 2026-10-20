use crate::standards::v1::subsets::any::io::binary::mutations::*;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::default_remodeling_scene;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let scene = default_remodeling_scene();
    let operation = semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_feature_params(scene.params.feature);
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

/// 📄️ Full `print_document_text`/`parse_document_text` round trip through a live `ArtifactStore`
/// with an applied edit, the ground-truth contract for replacing the JSON envelope with text files.
#[semio_framework_async_macros::async_test]
async fn store_roundtrips_through_document_text() {
    let initial = default_remodeling_scene();
    let envelope = store::os_store::document_materialize::create_document_envelope("test/v1", "test", initial, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = store::ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    store.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot, RemodelingMutation>().expect("funded bounded document owners")).map_err(|(error, _)| error).expect("document owners install");
    let mut feature_params = store.snapshot().expect("initial projection").params.feature;
    feature_params.target_count = 12345;
    store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![semio_s_artifact_remodel_remodeling_mut_params::standards::v1::subsets::any::schema::mutations::update_feature_params::update_feature_params(feature_params)], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    store.close_owned_unscheduled().expect("remodeling document store closes through its exact bounded owners");
}
