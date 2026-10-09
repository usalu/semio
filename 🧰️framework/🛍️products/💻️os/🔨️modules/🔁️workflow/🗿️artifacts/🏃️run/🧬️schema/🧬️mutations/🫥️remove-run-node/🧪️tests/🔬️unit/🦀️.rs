
use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_remove_run_node_identity() {
    assert_eq!(<RemoveRunNode as MutationLeaf>::DESCRIPTOR.semantic_kind, "remove-run-node");
}
