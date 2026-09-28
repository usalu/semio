use super::*;

#[test]
fn xml_part_order_round_trip_is_exact() {
    let base = snapshot_a();
    let mut reordered = base.clone();
    reordered.xml_parts.reverse();
    let diff = DocxDiff::between(&base, &reordered);
    assert!(!diff.is_empty());
    assert_eq!(diff.apply(&base).expect("order diff applies"), reordered);
    assert_eq!(diff.inverse(&base).apply(&reordered).expect("inverse applies"), base);
}

#[semio_framework_async_macros::async_test]
async fn rejects_missing_xml_part_target_without_mutating_base() {
    let base = DocxSnapshot::default();
    let diff = DocxDiff { xml_parts: Some(DocxXmlPartsDiff { modified: vec![NamedModified { key: "missing.xml".into(), diff: DocxXmlPartDiff::default() }], ..Default::default() }), ..Default::default() };
    let result = diff.apply(&base);
    assert_eq!(result.unwrap_err().code, "mutation.apply.missing-target");
    assert_eq!(base, DocxSnapshot::default());
}
