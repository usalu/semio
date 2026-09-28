mod tests {
    use super::*;
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx;
    use crate::standards::v_ecma_376::subsets::strict::schema::stamp_strict_namespace;
    use crate::standards::v_ecma_376::subsets::transitional::schema::CODE_CONFORMANCE_ATTRIBUTE;

    #[semio_framework_async_macros::async_test]
    async fn new_stamps_transitional_and_builds_clean() {
        let snapshot = XlsxTransitionalBuilderConstruction::new(XlsxWorkbook::default()).build().expect("conforming construction must build");
        assert!(check_transitional_conformance(&snapshot).iter().all(|d| d.severity != Severity::Error), "got {:?}", check_transitional_conformance(&snapshot));
    }

    /// 🛡️ A raw `SetSnapshot` that really applies (a valid, projectable Strict-stamped package) bypasses every typed
    /// constructor — `build()` is still the gate that refuses the Strict-declared workbook.
    #[semio_framework_async_macros::async_test]
    async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let strict = stamp_strict_namespace(build_minimal_xlsx(XlsxWorkbook::default()));
        let (mutated, outcome) = XlsxTransitionalBuilderConstruction::new(XlsxWorkbook::default()).mutate(XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: strict }));
        assert!(outcome.messages().is_empty(), "the raw SetSnapshot applies: {:?}", outcome.messages());
        let err = mutated.build().expect_err("a Strict-declared workbook.xml must fail build()");
        assert!(err.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTRIBUTE), "got {err:?}");
    }
}
