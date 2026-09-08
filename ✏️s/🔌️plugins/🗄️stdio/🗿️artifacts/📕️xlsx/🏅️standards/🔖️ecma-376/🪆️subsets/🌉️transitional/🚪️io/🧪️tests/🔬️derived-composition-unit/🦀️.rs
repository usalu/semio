mod tests {
    use super::*;
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxWorkbook;
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
    }

    #[semio_framework_async_macros::async_test]
    async fn strict_shaped_document_fails_compose_with_real_diagnostic() {
        // ⚠️ `ArtifactPack::encode_pack` (-> `⚙️engine::encode_xlsx`) regenerates workbook.xml as
        // Transitional-shaped on every call (documented writer scope cut, see 🔒️strict's composer
        // module doc comment) -- so to genuinely exercise a Strict-shaped payload here, feed the
        // raw OPC bytes through the DSL text (hex) path instead, which routes straight to the real
        // `engine::decode_xlsx` without an intervening regenerate. Same technique the PDF/A 1.7
        // pilot's composer tests use for the analogous reason.
        use crate::standards::v_ecma_376::subsets::strict::schema::stamp_strict_namespace;
        let strict_snapshot = stamp_strict_namespace(crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook::default()));
        // 🩹 `engine::encode_xlsx` itself regenerates workbook.xml as Transitional-shaped (the
        // very thing this comment above warns about) -- encoding the OPC package directly, NOT
        // through `encode_xlsx`, is what actually avoids the regenerate.
        let opc_bytes = semio_s_artifact_stdio_zip::opc::encode_opc(&strict_snapshot.opc).expect("encode strict-shaped opc bytes");
        let hex: String = opc_bytes.iter().map(|b| format!("{b:02x}")).collect();
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Text(&hex) }];
        let err = XlsxTransitionalComposerComposition::compose(&sources).expect_err("a Strict-shaped workbook.xml must not stamp transitional");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }
}
