mod tests {
    use super::*;
    use crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx;
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxWorkbook;
    use crate::standards::v_ecma_376::subsets::strict::schema::stamp_strict_namespace;
    use crate::standards::v_ecma_376::subsets::transitional::schema::CODE_NAMESPACE_MISMATCH;
    use crate::standards::v_ecma_376::subsets::transitional::schema::XlsxTransitionalBuilderConstruction as XlsxTransitionalBuilder;
    use semio_framework_plugin::{AnalyzeSource, ArtifactBuilder as _};

    #[semio_framework_async_macros::async_test]
    async fn conforming_builder_snapshot_composes_and_stamps_transitional() {
        let snapshot = XlsxTransitionalBuilder::new(XlsxWorkbook::default()).build().expect("conforming transitional construction must build");
        let bytes = <XlsxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = XlsxTransitionalComposerComposition::compose(&sources).expect("clean document must compose to transitional");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        assert_eq!(composed.snapshot, snapshot, "the wire round trip keeps the stamped XML authority");
    }

    /// 🔤️ A Strict-stamped package arriving through the text (`.dsl.semio`) path is refused with the real
    /// namespace diagnostic — the stamp travels verbatim in the XML authority, no transport rewrites it.
    #[semio_framework_async_macros::async_test]
    async fn strict_shaped_document_fails_compose_with_real_diagnostic() {
        let strict_snapshot = stamp_strict_namespace(build_minimal_xlsx(XlsxWorkbook::default()));
        let text = store::ArtifactDsl::print_dsl(&strict_snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&text) }];
        let err = XlsxTransitionalComposerComposition::compose(&sources).expect_err("a Strict-shaped workbook.xml must not stamp transitional");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }
}
