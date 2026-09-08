mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sniff_real_csv_table_is_high() {
        let text = "a,b,c\n1,2,3\n4,5,6\n";
        assert_eq!(CsvAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_unrelated_text_is_low() {
        assert_eq!(CsvAnalyzerAnalysis::sniff(&AnalyzeSource::Text("just a plain sentence.")), IoConfidence::Low);
    }
}
