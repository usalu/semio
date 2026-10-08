
use super::*;
use protocol::MutationLeaf;
#[test]
fn metadata_has_the_canonical_retract_run_log_identity() {
    assert_eq!(<RetractRunLog as MutationLeaf>::DESCRIPTOR.semantic_kind, "retract-run-log");
}
