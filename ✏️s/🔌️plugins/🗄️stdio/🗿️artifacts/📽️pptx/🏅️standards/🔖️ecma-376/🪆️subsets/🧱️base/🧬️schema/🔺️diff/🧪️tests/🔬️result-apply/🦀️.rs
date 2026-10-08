use super::*;

#[test]
fn a_declared_schema_diff_changes_only_the_schema() {
    let before = demo_snapshot_a();
    let diff = PptxDiff { schema: Some("s.stdio.pptx.ecma-376.base.v2".into()), ..Default::default() };
    let mut expected = before.clone();
    expected.schema = "s.stdio.pptx.ecma-376.base.v2".into();
    assert_eq!(protocol::apply_diff(&diff, &before).unwrap(), expected);
}
