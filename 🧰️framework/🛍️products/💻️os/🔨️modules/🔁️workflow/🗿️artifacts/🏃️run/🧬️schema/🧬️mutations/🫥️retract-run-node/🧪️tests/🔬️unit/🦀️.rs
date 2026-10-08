
use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_retract_run_node_identity() {
    assert_eq!(<RetractRunNode as MutationLeaf>::DESCRIPTOR.semantic_kind, "retract-run-node");
}
