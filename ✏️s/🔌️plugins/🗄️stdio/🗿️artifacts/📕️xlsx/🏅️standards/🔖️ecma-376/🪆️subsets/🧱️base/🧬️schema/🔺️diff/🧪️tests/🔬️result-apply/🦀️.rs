use super::*;

#[semio_framework_async_macros::async_test]
async fn rejects_missing_part_target_without_mutating_base() {
    let base = XlsxSnapshot::default();
    let diff = XlsxDiff { xml_parts: Some(XlsxXmlPartsDiff { modified: vec![NamedModified { key: "xl/worksheets/missing.xml".into(), diff: XlsxXmlPartDiff::default() }], ..Default::default() }), ..Default::default() };
    let result = protocol::apply_diff(&diff, &base);
    assert_eq!(result.unwrap_err().code, "mutation.apply.missing-target");
    assert_eq!(base, XlsxSnapshot::default());
}
