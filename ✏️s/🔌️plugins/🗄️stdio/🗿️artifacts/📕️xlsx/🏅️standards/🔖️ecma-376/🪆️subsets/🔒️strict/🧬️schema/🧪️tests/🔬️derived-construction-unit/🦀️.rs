mod tests {
    use super::*;
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx;
    use crate::standards::v_ecma_376::subsets::strict::schema::CODE_NAMESPACE_MISMATCH;

    #[semio_framework_async_macros::async_test]
    async fn new_stamps_strict_and_builds_clean() {
        let snapshot = XlsxStrictBuilderConstruction::new(XlsxWorkbook::default()).build().expect("conforming construction must build");
        assert!(check_strict_conformance(&snapshot).iter().all(|d| d.severity != Severity::Error), "got {:?}", check_strict_conformance(&snapshot));
    }

    /// 🛡️ A raw `SetSnapshot` that really applies (a valid, projectable Transitional package) bypasses every typed
    /// constructor — `build()` is still the gate that refuses the non-Strict workbook.
    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let transitional = build_minimal_xlsx(XlsxWorkbook::default());
        let (mutated, outcome) = XlsxStrictBuilderConstruction::new(XlsxWorkbook::default()).mutate(XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: transitional }));
        assert!(outcome.messages().is_empty(), "the raw SetSnapshot applies: {:?}", outcome.messages());
        let err = mutated.build().expect_err("a non-Strict workbook.xml must fail build()");
        assert!(err.iter().any(|d| d.code.0 == CODE_NAMESPACE_MISMATCH), "got {err:?}");
    }
}
