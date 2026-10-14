
use crate::binary::mutations::*;
use semio_s_artifact_block_2d_model::{BLOCK_2D_SCHEMA, Block2dSnapshot};
use store::{os_store::store_schema::ArtifactCommand, os_store::document_materialize::create_document_envelope};

#[semio_framework_async_macros::async_test]
async fn block2d_document_vcs_replays_granular_operations() {
    use semio_s_artifact_block_2d_mutations::{self as m,Block2dStore};

    let mut store = Block2dStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(create_document_envelope(BLOCK_2D_SCHEMA, "block2d", Block2dSnapshot::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid initial state");
    // 🏪️ A bare store carries no owner catalog and refuses its first edit
    // (`edit history insertion requires its exact mutation retirement factory`); install the
    // exact owners production installs through `Block2dPlayApp::build_document_store_owners`.
    store.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Block2dSnapshot, Block2dMutation>().expect("funded bounded document owners")).map_err(|(error, _)| error).expect("document owners install");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![m::rename_node_kind("n1".into())], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    let projection = store.snapshot().expect("snapshot");
    assert_eq!(projection.node_kind.name, "n1");
    store.close_owned_unscheduled().expect("block2d document store closes through its exact bounded owners");
}

#[semio_framework_async_macros::async_test]
async fn block2d_operation_binary_round_trips() {
    let operation = semio_s_artifact_block_2d_mut_handles::delete_handle("h0".into());
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}
