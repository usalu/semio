
use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_set_run_seal_identity() {
    assert_eq!(<SetRunSeal as MutationLeaf>::DESCRIPTOR.semantic_kind, "set-run-seal");
}
