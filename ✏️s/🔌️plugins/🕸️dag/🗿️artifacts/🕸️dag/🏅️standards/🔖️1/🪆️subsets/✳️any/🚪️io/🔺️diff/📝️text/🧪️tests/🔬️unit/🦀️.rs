
use super::*;
use crate::default_snapshot;
use crate::schema::mutations::delete_node;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn dag_diff_default_has_no_pending_writes() {
    let diff = DagDiff::default();
    assert!(diff.content.is_none());
}

#[semio_framework_async_macros::async_test]
async fn delete_node_diff_removes_the_node() {
    let base = default_snapshot();
    let id = base.nodes().first().expect("fixture has a node").id.clone();
    let mutation = delete_node(id.clone());
    let outcome = mutation.diff(&base);
    assert!(outcome.diff().apply(&base).expect("valid mutation diff").nodes().iter().all(|node| node.id != id));
}
