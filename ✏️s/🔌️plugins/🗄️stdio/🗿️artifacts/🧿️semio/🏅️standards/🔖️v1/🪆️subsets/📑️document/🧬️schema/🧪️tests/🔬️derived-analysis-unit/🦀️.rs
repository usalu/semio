mod tests {
    use super::*;
    use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocStyle};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn rich_snapshot() -> SemioDocumentSnapshot {
        SemioDocumentSnapshot { schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(), styles: vec![DocStyle { id: "n".into(), name: "Normal".into(), based_on: None }], images: Vec::new(), blocks: vec![DocBlock::paragraph("hi")] }
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_detects_own_binary_and_text_payloads() {
        let snap = rich_snapshot();
        let bytes = store::ArtifactPack::encode_pack(&snap);
        assert_eq!(SemioDocumentAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(&bytes)), IoConfidence::High);
        let text = <SemioDocumentSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        assert_eq!(SemioDocumentAnalyzerAnalysis::sniff(&AnalyzeSource::Text(&text)), IoConfidence::High);
        assert_eq!(SemioDocumentAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(b"not a semio document at all")), IoConfidence::Low);
    }

    #[semio_framework_async_macros::async_test]
    async fn analyze_decodes_binary_source_into_snapshot() {
        let snap = rich_snapshot();
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let analysis = SemioDocumentAnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(&bytes)]);
        assert_eq!(analysis.confidence, IoConfidence::High);
        assert_eq!(analysis.parts.snapshot, Some(snap));
    }

    #[semio_framework_async_macros::async_test]
    async fn analyze_reports_low_confidence_on_malformed_text() {
        let analysis = SemioDocumentAnalyzerAnalysis::analyze(&[AnalyzeSource::Text("not valid semio document dsl")]);
        assert_eq!(analysis.confidence, IoConfidence::Low);
        assert!(!analysis.diagnostics.is_empty());
    }
}
