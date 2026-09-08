mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn sniff_bumps_to_high_when_bcf_version_entry_name_is_present() {
        let snap = BcfSnapshot { schema: "stdio.bcf".into(), version: "2.1".into(), topics: Vec::new(), parts: Vec::new() };
        let bytes = crate::io::encode_bcf(&snap).expect("encode");
        assert_eq!(BcfAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(&bytes)), IoConfidence::High);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_stays_medium_for_a_real_zip_without_bcf_version() {
        let zip_snap = semio_s_artifact_stdio_zip::ZipSnapshot {
            schema: semio_s_artifact_stdio_zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(),
            entries: vec![semio_s_artifact_stdio_zip::schema::snapshot::ZipEntry { name: "unrelated.txt".into(), data: b"not a bcf archive".to_vec(), ..Default::default() }],
            comment: String::new(),
        };
        let bytes = semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::encode_zip(&zip_snap).expect("encode plain zip");
        assert_eq!(BcfAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(&bytes)), IoConfidence::Medium);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_rejects_non_zip_garbage() {
        assert_eq!(BcfAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(b"not a zip at all")), IoConfidence::Low);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_treats_text_source_as_low() {
        assert_eq!(BcfAnalyzerAnalysis::sniff(&AnalyzeSource::Text("deadbeef")), IoConfidence::Low);
    }
}
