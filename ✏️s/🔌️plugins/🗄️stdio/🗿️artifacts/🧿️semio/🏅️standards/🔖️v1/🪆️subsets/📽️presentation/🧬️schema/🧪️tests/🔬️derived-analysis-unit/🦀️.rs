mod tests {
    use super::*;
    use crate::standards::v1::subsets::presentation::schema::snapshot::SlideMaster;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample() -> SemioPresentationSnapshot {
        SemioPresentationSnapshot { masters: vec![SlideMaster { id: "m1".into(), shapes: Vec::new() }], ..Default::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_reports_high_for_real_payloads_low_for_garbage() {
        let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&sample());
        assert_eq!(SemioPresentationAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(&bytes)), semio_framework_plugin::io::Confidence::High);
        assert_eq!(SemioPresentationAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(b"not a presentation")), semio_framework_plugin::io::Confidence::Low);

        let text = <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(&sample());
        assert_eq!(SemioPresentationAnalyzerAnalysis::sniff(&AnalyzeSource::Text(&text)), semio_framework_plugin::io::Confidence::High);
        assert_eq!(SemioPresentationAnalyzerAnalysis::sniff(&AnalyzeSource::Text("garbage")), semio_framework_plugin::io::Confidence::Low);
    }

    #[semio_framework_async_macros::async_test]
    async fn analyze_decodes_binary_and_text_sources() {
        let snap = sample();
        let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let analysis = SemioPresentationAnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(&bytes)]);
        assert_eq!(analysis.confidence, semio_framework_plugin::io::Confidence::High);
        assert_eq!(analysis.parts.snapshot, Some(snap.clone()));

        let text = <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let analysis2 = SemioPresentationAnalyzerAnalysis::analyze(&[AnalyzeSource::Text(&text)]);
        assert_eq!(analysis2.parts.snapshot, Some(snap));
    }

    #[semio_framework_async_macros::async_test]
    async fn analyze_flags_low_confidence_on_undecodable_source() {
        let analysis = SemioPresentationAnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(b"garbage")]);
        assert_eq!(analysis.confidence, semio_framework_plugin::io::Confidence::Low);
        assert!(!analysis.diagnostics.is_empty());
    }
}
