
use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_start_node_identity() {
    assert_eq!(<StartRunNode as MutationLeaf>::DESCRIPTOR.semantic_kind, "start-run-node");
}
