mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sniff_real_ascii_stl_is_high() {
        let text = "solid mesh\n  facet normal 0 0 1\n    outer loop\n      vertex 0 0 0\n      vertex 1 0 0\n      vertex 0 1 0\n    endloop\n  endfacet\nendsolid mesh\n";
        assert_eq!(StlAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_unrelated_text_is_low() {
        assert_eq!(StlAnalyzerAnalysis::sniff(&AnalyzeSource::Text("not an stl file")), IoConfidence::Low);
    }
}
