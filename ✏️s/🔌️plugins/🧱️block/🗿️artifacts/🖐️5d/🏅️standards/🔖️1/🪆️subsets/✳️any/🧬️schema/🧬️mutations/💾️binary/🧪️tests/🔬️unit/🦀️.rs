
use super::*;
use crate::{BLOCK_5D_SCHEMA, Block5dSnapshot};
use store::{ArtifactCommand, create_document_envelope};

#[semio_framework_async_macros::async_test]
async fn block5d_document_vcs_replays_granular_operations() {
    use crate::standards::v1::subsets::any::schema::mutations::{self as m, Block5dStore};

    let mut store = Block5dStore::new(create_document_envelope(BLOCK_5D_SCHEMA, "block5d", Block5dSnapshot::default(), None)).await.expect("valid initial state");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![m::rename_part_kind("p1".into())], description: None }).await.expect("apply");
    let projection = store.snapshot().expect("snapshot");
    assert_eq!(projection.part_kind.name, "p1");
}

#[semio_framework_async_macros::async_test]
async fn block5d_operation_binary_round_trips() {
    let operation = crate::standards::v1::subsets::any::schema::mutations::delete_grip("g0".into());
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}
