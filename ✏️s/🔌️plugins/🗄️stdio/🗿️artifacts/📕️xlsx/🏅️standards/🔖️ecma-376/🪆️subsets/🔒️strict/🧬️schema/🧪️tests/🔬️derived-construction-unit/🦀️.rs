mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn new_stamps_strict_and_builds_clean() {
        let snapshot = XlsxStrictBuilderConstruction::new(XlsxWorkbook::default()).build().expect("conforming construction must build");
        assert!(check_strict_conformance(&snapshot).iter().all(|d| d.severity != Severity::Error), "got {:?}", check_strict_conformance(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = XlsxStrictBuilderConstruction::new(XlsxWorkbook::default()).build().unwrap();
        // Directly corrupt the stamped namespace, bypassing every typed constructor -- mirrors the
        // PDF/A pilot's raw-mutate escape-hatch test.
        snapshot.opc.set_part(WORKBOOK_PART, WORKBOOK_CONTENT_TYPE, b"<workbook xmlns=\"transitional\"/>".to_vec());
        let (mutated, _diff) = XlsxStrictBuilderConstruction::from_snapshot(XlsxSnapshot::default()).mutate(XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }));
        let err = mutated.build().expect_err("a non-Strict workbook.xml must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::standards::v_ecma_376::subsets::strict::schema::CODE_NAMESPACE_MISMATCH), "got {err:?}");
    }
}
