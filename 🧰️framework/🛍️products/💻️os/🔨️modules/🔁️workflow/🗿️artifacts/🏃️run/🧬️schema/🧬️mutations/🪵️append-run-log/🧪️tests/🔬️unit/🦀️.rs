
use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_append_identity() {
    assert_eq!(<AppendRunLog as MutationLeaf>::DESCRIPTOR.semantic_kind, "append-run-log");
}
