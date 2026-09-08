mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn new_stamps_transitional_and_builds_clean() {
        let snapshot = XlsxTransitionalBuilderConstruction::new(XlsxWorkbook::default()).build().expect("conforming construction must build");
        assert!(check_transitional_conformance(&snapshot).iter().all(|d| d.severity != Severity::Error), "got {:?}", check_transitional_conformance(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = XlsxTransitionalBuilderConstruction::new(XlsxWorkbook::default()).build().unwrap();
        snapshot.opc.set_part(WORKBOOK_PART, WORKBOOK_CONTENT_TYPE, br#"<workbook xmlns="http://purl.oclc.org/ooxml/spreadsheetml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships" conformance="strict"/>"#.to_vec());
        let (mutated, _diff) = XlsxTransitionalBuilderConstruction::from_snapshot(XlsxSnapshot::default()).mutate(XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }));
        let err = mutated.build().expect_err("a Strict-declared workbook.xml must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v_ecma_376::subsets::transitional::schema::CODE_CONFORMANCE_ATTRIBUTE), "got {err:?}");
    }
}
