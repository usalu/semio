
use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_remove_run_log_identity() {
    assert_eq!(<RemoveRunLog as MutationLeaf>::DESCRIPTOR.semantic_kind, "remove-run-log");
}
