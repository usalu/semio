
use crate::binary::mutations::*;
use semio_s_artifact_block_3d_model::{BLOCK_3D_SCHEMA, Block3dSnapshot};
use store::{os_store::store_schema::ArtifactCommand, os_store::document_materialize::create_document_envelope};

#[semio_framework_async_macros::async_test]
async fn block3d_document_vcs_replays_granular_operations() {
    use semio_s_artifact_block_3d_mutations::{self as m,Block3dStore};

    let mut store = Block3dStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(create_document_envelope(BLOCK_3D_SCHEMA, "block3d", Block3dSnapshot::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid initial state");
    store.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<Block3dSnapshot, Block3dMutation>().expect("funded bounded document owners")).map_err(|(error, _)| error).expect("document owners install");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![m::rename_object_kind("o1".into())], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    let projection = store.snapshot().expect("snapshot");
    assert_eq!(projection.object_kind.name, "o1");
    store.close_owned_unscheduled().expect("block3d document store closes through its exact bounded owners");
}

#[semio_framework_async_macros::async_test]
async fn block3d_operation_binary_round_trips() {
    let operation = semio_s_artifact_block_3d_mut_vortices::delete_vortex("v0".into());
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}
