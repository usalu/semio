use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_identity() {
    assert_eq!(<SetNodePositions as MutationLeaf>::DESCRIPTOR.semantic_kind, "set-node-positions");
}
