use super::*;

#[semio_framework_async_macros::async_test]
async fn rejects_missing_sheet_target_without_mutating_base() {
    let base = XlsxSnapshot::default();
    let diff =
        XlsxDiff { workbook: Some(XlsxWorkbookDiff { sheets: Some(XlsxSheetsDiff { modified: vec![NamedModified { key: "missing".into(), diff: XlsxSheetDiff::default() }], ..Default::default() }), ..Default::default() }), ..Default::default() };
    let result = diff.apply(&base);
    assert_eq!(result.unwrap_err().code, "mutation.apply.missing-target");
    assert_eq!(base, XlsxSnapshot::default());
}
