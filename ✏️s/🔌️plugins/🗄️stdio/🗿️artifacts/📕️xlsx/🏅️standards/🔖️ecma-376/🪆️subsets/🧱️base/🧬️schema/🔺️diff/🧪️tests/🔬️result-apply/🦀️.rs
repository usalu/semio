use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::diff::*;
#[cfg(test)] use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;

#[semio_framework_async_macros::async_test]
async fn rejects_missing_part_target_without_mutating_base() {
    let base = XlsxSnapshot::default();
    let diff = XlsxDiff { xml_parts: Some(XlsxXmlPartsDelta { modified: vec![XlsxXmlPartModification { id: "xl/worksheets/missing.xml".into(), patch: XlsxXmlPartDiff::default() }], ..Default::default() }), ..Default::default() };
    let result = protocol::apply_diff(&diff, &base);
    assert_eq!(result.unwrap_err().code, "mutation.apply.missing-target");
    assert_eq!(base, XlsxSnapshot::default());
}
