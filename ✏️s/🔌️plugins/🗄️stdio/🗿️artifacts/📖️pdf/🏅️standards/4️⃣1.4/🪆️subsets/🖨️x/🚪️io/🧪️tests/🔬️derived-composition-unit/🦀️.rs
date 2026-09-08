mod tests {
    use super::*;
    use crate::standards::v1_4::subsets::x::schema::CODE_SCHEMA_GAP;
    use semio_framework_plugin::AnalyzeSource;

    #[semio_framework_async_macros::async_test]
    async fn compose_always_carries_the_schema_gap_diagnostic() {
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&PdfSnapshot::default());
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = PdfXComposerComposition::compose(&sources).expect("pass-through compose never fails on conformance grounds");
        assert!(composed.diagnostics.iter().any(|d| d.code.0 == CODE_SCHEMA_GAP), "got {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_reports_the_schema_gap_diagnostic() {
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&PdfSnapshot::default());
        let diagnostics = PdfXValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SCHEMA_GAP), "got {diagnostics:?}");
    }
}
