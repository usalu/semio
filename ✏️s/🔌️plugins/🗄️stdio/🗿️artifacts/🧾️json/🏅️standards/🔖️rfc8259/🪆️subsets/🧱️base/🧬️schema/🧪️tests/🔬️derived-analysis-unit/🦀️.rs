mod tests {
    use super::*;

    #[test]
    fn sniff_real_json_object_is_high() {
        let text = "{\"a\": 1, \"b\": [1, 2, 3]}";
        assert_eq!(JsonAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[test]
    fn sniff_malformed_json_is_not_high() {
        let text = "{\"a\": 1, \"b\": [1, 2, 3]";
        assert_ne!(JsonAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[test]
    fn sniff_unrelated_text_is_low() {
        assert_eq!(JsonAnalyzerAnalysis::sniff(&AnalyzeSource::Text("just a plain sentence.")), IoConfidence::Low);
    }
}
