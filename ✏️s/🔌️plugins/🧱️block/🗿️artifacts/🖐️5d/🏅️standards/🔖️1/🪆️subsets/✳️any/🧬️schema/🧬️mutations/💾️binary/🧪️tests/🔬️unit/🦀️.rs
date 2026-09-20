
use super::*;
use crate::{BLOCK_5D_SCHEMA, Block5dSnapshot};
use store::{ArtifactCommand, create_document_envelope};

#[semio_framework_async_macros::async_test]
async fn block5d_document_vcs_replays_granular_operations() {
    use crate::standards::v1::subsets::any::schema::mutations::{self as m, Block5dStore};

    let mut store = Block5dStore::new(create_document_envelope(BLOCK_5D_SCHEMA, "block5d", Block5dSnapshot::default(), None)).await.expect("valid initial state");
    // 🏪️ A bare store carries no owner catalog and refuses its first edit
    // (`edit history insertion requires its exact mutation retirement factory`); install the
    // exact owners production installs through `Block5dPlayApp::build_document_store_owners`.
    store.install_document_store_owners_exact(crate::standards::v1::subsets::any::schema::retirement::document_store_owners());
    store.dispatch(ArtifactCommand::Apply { mutations: vec![m::rename_part_kind("p1".into())], description: None }).await.expect("apply");
    let projection = store.snapshot().expect("snapshot");
    assert_eq!(projection.part_kind.name, "p1");
    while !store.close_owned_terminal_is_empty() {
        store.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("block5d document store closes through its exact bounded owners");
    }
}

#[semio_framework_async_macros::async_test]
async fn block5d_operation_binary_round_trips() {
    let operation = crate::standards::v1::subsets::any::schema::mutations::delete_grip("g0".into());
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}
