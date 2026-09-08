
use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_start_identity() {
    assert_eq!(<StartRun as MutationLeaf>::DESCRIPTOR.semantic_kind, "start-run");
}
