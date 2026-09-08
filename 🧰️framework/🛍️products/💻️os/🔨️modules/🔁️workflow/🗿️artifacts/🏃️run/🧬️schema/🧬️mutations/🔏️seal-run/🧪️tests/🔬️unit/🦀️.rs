
use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_seal_identity() {
    assert_eq!(<SealRun as MutationLeaf>::DESCRIPTOR.semantic_kind, "seal-run");
}
