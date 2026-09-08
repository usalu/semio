mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sniff_text_source_is_high() {
        assert_eq!(TxtAnalyzerAnalysis::sniff(&AnalyzeSource::Text("anything at all")), IoConfidence::High);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_binary_with_nul_bytes_is_low_or_medium_not_high() {
        let bytes: &[u8] = b"\x00\x01\x02binary garbage\x00";
        assert_ne!(TxtAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(bytes)), IoConfidence::High);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_invalid_utf8_binary_is_low() {
        let bytes: &[u8] = &[0xff, 0xfe, 0xfd];
        assert_eq!(TxtAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(bytes)), IoConfidence::Low);
    }
}
