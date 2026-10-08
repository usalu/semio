use super::*;
use protocol::{command::DiffAlgebra, MutationDiff};

#[test]
fn stale_semantic_projection_is_not_a_diff_authority() {
    let before = demo_snapshot_a();
    let after = demo_snapshot_b();
    let diff = PptxDiff::between(&before, &after);
    assert!(diff.schema.is_none());
    assert!(diff.opc.is_some() || diff.xml_parts.is_some());
    assert_eq!(protocol::apply_diff(&diff, &before).unwrap(), after);
}
