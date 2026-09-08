mod tests {
    use super::*;
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxWorkbook;
    use crate::standards::v_ecma_376::subsets::strict::schema::XlsxStrictBuilderConstruction as XlsxStrictBuilder;
    use crate::standards::v_ecma_376::subsets::strict::schema::{CODE_CONFORMANCE_ATTRIBUTE, CODE_NAMESPACE_MISMATCH};
    use semio_framework_plugin::{AnalyzeSource, ArtifactBuilder as _};

    /// 🩹 `encode_xlsx` (`⚙️engine/🦀️.rs`) always calls `regenerate_workbook_parts`,
    /// which REBUILDS `xl/workbook.xml` from `snap.workbook` (the typed model) on every encode --
    /// it doesn't know about Strict mode, so it would silently overwrite whatever
    /// `XlsxStrictBuilder::new(...)`'s `stamp_strict_namespace` post-processing wrote into `opc`.
    /// Encoding the OPC package directly (bypassing the typed-model regeneration entirely) is how
    /// this test genuinely exercises a workbook whose XML matches what the strict builder seeded —
    /// same fix as docx's sibling `🔒️strict` composer test.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn conforming_pack_bytes(snapshot: &XlsxSnapshot) -> Vec<u8> {
        let raw = semio_s_artifact_stdio_zip::opc::encode_opc(&snapshot.opc).expect("valid opc package encodes");
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<XlsxSnapshot as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).expect("valid envelope_id");
        store::semio_format::wrap_binary(&envelope, &raw)
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_builder_snapshot_composes_and_stamps_strict() {
        let snapshot = XlsxStrictBuilder::new(XlsxWorkbook::default()).build().expect("conforming strict construction must build");
        let bytes = conforming_pack_bytes(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = XlsxStrictComposerComposition::compose(&sources).expect("clean document must compose to strict");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn transitional_shaped_document_fails_compose_with_real_diagnostic() {
        let snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook::default());
        let bytes = <XlsxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let err = XlsxStrictComposerComposition::compose(&sources).expect_err("a Transitional-shaped workbook.xml must not stamp strict");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_NAMESPACE_MISMATCH && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_flags_hard_diagnostics_on_the_wire_payload() {
        // Documented writer scope cut (module doc comment): `encode_pack` -> `encode_xlsx` ->
        // `regenerate_workbook_parts` always re-emits Transitional-shaped bytes, so a round trip
        // honestly re-reports the Strict conformance-attribute violation -- not a false positive,
        // the wire bytes genuinely no longer declare Strict.
        let snapshot = XlsxStrictBuilder::new(XlsxWorkbook::default()).build().expect("build");
        let bytes = <XlsxSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let diagnostics = XlsxStrictValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_CONFORMANCE_ATTRIBUTE && d.severity == Severity::Error), "got {diagnostics:?}");
    }
}
