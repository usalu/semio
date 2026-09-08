mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sniff_real_obj_text_is_high() {
        let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
        assert_eq!(ObjAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_unrelated_text_is_low() {
        let text = "{\"not\": \"an obj file at all\"}";
        assert_eq!(ObjAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::Low);
    }
}
