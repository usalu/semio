
use crate::standards::v1::subsets::any::io::binary::mutations::*;
use crate::{BLOCK_2D_SCHEMA, Block2dSnapshot};
use store::{ArtifactCommand, create_document_envelope};

#[semio_framework_async_macros::async_test]
async fn block2d_document_vcs_replays_granular_operations() {
    use crate::standards::v1::subsets::any::schema::mutations::{self as m,Block2dStore};


    let mut store = Block2dStore::new(create_document_envelope(BLOCK_2D_SCHEMA, "block2d", Block2dSnapshot::default(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid initial state");
    // 🏪️ A bare store carries no owner catalog and refuses its first edit
    // (`edit history insertion requires its exact mutation retirement factory`); install the
    // exact owners production installs through `Block2dPlayApp::build_document_store_owners`.
    store.install_document_store_owners_exact(store::funded_bounded_artifact_store_owners::<Block2dSnapshot, Block2dMutation>().expect("funded bounded document owners")).map_err(|(error, _)| error).expect("document owners install");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![m::rename_node_kind("n1".into())], transaction: None }).await.expect("apply");
    let projection = store.snapshot().expect("snapshot");
    assert_eq!(projection.node_kind.name, "n1");
    store.close_owned_unscheduled().expect("block2d document store closes through its exact bounded owners");
}

#[semio_framework_async_macros::async_test]
async fn block2d_operation_binary_round_trips() {
    let operation = crate::standards::v1::subsets::any::schema::mutations::delete_handle("h0".into());
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}
