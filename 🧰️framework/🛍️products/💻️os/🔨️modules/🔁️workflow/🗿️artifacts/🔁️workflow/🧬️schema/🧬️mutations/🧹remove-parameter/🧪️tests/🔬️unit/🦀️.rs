
use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_identity() {
    assert_eq!(<RemoveParameter as MutationLeaf>::DESCRIPTOR.semantic_kind, "remove-parameter");
}
