mod tests {
    use super::*;
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx;
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxWorkbook;
    use crate::standards::v_ecma_376::subsets::strict::schema::XlsxStrictBuilderConstruction as XlsxStrictBuilder;
    use crate::standards::v_ecma_376::subsets::strict::schema::{CODE_CONFORMANCE_ATTRIBUTE, CODE_NAMESPACE_MISMATCH};
    use semio_framework_plugin::{AnalyzeSource, ArtifactBuilder as _};

    #[semio_framework_async_macros::async_test]
    async fn conforming_builder_snapshot_composes_and_stamps_strict() {
        let snapshot = XlsxStrictBuilder::new(XlsxWorkbook::default()).build().expect("conforming strict construction must build");
        let bytes = <XlsxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = XlsxStrictComposerComposition::compose(&sources).expect("clean document must compose to strict");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        assert_eq!(composed.snapshot, snapshot, "the wire round trip keeps the stamped XML authority");
    }

    #[semio_framework_async_macros::async_test]
    async fn transitional_shaped_document_fails_compose_with_real_diagnostic() {
        let snapshot = build_minimal_xlsx(XlsxWorkbook::default());
        let bytes = <XlsxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let err = XlsxStrictComposerComposition::compose(&sources).expect_err("a Transitional-shaped workbook.xml must not stamp strict");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    /// 🛡️ The validate-on-build hook re-checks the wire payload itself: the XML parts travel as authored, so the
    /// Strict builder's stamp survives the wire and a Transitional-shaped payload is flagged hard.
    #[semio_framework_async_macros::async_test]
    async fn subset_validator_accepts_the_strict_builders_wire_payload() {
        let snapshot = XlsxStrictBuilder::new(XlsxWorkbook::default()).build().expect("build");
        let bytes = <XlsxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let diagnostics = XlsxStrictValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_flags_hard_diagnostics_on_the_wire_payload() {
        let bytes = <XlsxSnapshot as store::ArtifactPack>::encode_pack(&build_minimal_xlsx(XlsxWorkbook::default()));
        let diagnostics = XlsxStrictValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTRIBUTE && d.severity == Severity::Error), "got {diagnostics:?}");
    }
}
