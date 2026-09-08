
use super::*;
use crate::{BLOCK_3D_SCHEMA, Block3dSnapshot};
use store::{ArtifactCommand, create_document_envelope};

#[semio_framework_async_macros::async_test]
async fn block3d_document_vcs_replays_granular_operations() {
    use crate::standards::v1::subsets::any::schema::mutations::{self as m, Block3dStore};

    let mut store = Block3dStore::new(create_document_envelope(BLOCK_3D_SCHEMA, "block3d", Block3dSnapshot::default(), None)).await.expect("valid initial state");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![m::rename_object_kind("o1".into())], description: None }).await.expect("apply");
    let projection = store.snapshot().expect("snapshot");
    assert_eq!(projection.object_kind.name, "o1");
}

#[semio_framework_async_macros::async_test]
async fn block3d_operation_binary_round_trips() {
    let operation = crate::standards::v1::subsets::any::schema::mutations::delete_vortex("v0".into());
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}
