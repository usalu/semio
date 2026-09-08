mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sniff_real_markdown_structure_is_high() {
        let text = "# Title\n\n- one\n- two\n";
        assert_eq!(MdAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_plain_paragraph_text_is_medium() {
        assert_eq!(MdAnalyzerAnalysis::sniff(&AnalyzeSource::Text("just a plain sentence.")), IoConfidence::Medium);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_empty_is_low() {
        assert_eq!(MdAnalyzerAnalysis::sniff(&AnalyzeSource::Text("")), IoConfidence::Low);
    }
}
