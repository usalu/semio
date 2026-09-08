mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sniff_high_confidence_for_ifc2x3_envelope() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('IFC2X3'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
        assert_eq!(Ifc2x3AnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_medium_confidence_for_other_part21_schema() {
        let text = "ISO-10303-21;\nHEADER;\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n";
        assert_eq!(Ifc2x3AnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::Medium);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_low_confidence_for_non_part21_input() {
        assert_eq!(Ifc2x3AnalyzerAnalysis::sniff(&AnalyzeSource::Text("not a step file at all")), IoConfidence::Low);
        assert_eq!(Ifc2x3AnalyzerAnalysis::sniff(&AnalyzeSource::Binary(&[0xFF, 0xD8, 0xFF])), IoConfidence::Low);
    }
}
