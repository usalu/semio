
use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_set_run_header_identity() {
    assert_eq!(<SetRunHeader as MutationLeaf>::DESCRIPTOR.semantic_kind, "set-run-header");
}
