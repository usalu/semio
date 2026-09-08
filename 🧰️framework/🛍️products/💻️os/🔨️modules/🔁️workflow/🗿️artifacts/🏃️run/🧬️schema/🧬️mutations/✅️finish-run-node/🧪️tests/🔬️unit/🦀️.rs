
use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_finish_identity() {
    assert_eq!(<FinishRunNode as MutationLeaf>::DESCRIPTOR.semantic_kind, "finish-run-node");
}
