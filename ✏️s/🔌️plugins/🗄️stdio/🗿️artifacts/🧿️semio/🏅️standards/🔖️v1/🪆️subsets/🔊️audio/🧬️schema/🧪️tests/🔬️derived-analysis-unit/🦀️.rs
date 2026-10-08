mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sniff_recognizes_own_marker_and_rejects_foreign_text() {
        let snapshot = SemioAudioSnapshot { sample_rate: 8_000, ..SemioAudioSnapshot::default() };
        let text = <SemioAudioSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
        assert_eq!(SemioAudioAnalyzerAnalysis::sniff(&AnalyzeSource::Text(&text)), semio_framework_plugin::io::Confidence::High);
        assert_eq!(SemioAudioAnalyzerAnalysis::sniff(&AnalyzeSource::Text("not-audio-at-all")), semio_framework_plugin::io::Confidence::Low);
    }

    #[semio_framework_async_macros::async_test]
    async fn analyze_decodes_a_real_binary_source() {
        let snapshot = SemioAudioSnapshot { sample_rate: 16_000, ..SemioAudioSnapshot::default() };
        let bytes = <SemioAudioSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let analysis = SemioAudioAnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(&bytes)]);
        assert_eq!(analysis.confidence, semio_framework_plugin::io::Confidence::High);
        assert_eq!(analysis.parts.snapshot, Some(snapshot));
        assert!(analysis.diagnostics.is_empty());
    }
}
