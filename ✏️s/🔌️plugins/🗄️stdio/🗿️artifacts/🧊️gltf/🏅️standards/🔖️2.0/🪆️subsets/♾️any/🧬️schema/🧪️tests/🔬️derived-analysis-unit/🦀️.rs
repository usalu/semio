mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sniff_recognizes_glb_magic() {
        let mut bytes = vec![b'g', b'l', b'T', b'F'];
        bytes.extend_from_slice(&2u32.to_le_bytes());
        bytes.extend_from_slice(&[0u8; 4]);
        assert_eq!(GltfAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(&bytes)), semio_framework_plugin::io::Confidence::High);
        assert_eq!(GltfAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(b"not a glb")), semio_framework_plugin::io::Confidence::Low);
    }

    #[semio_framework_async_macros::async_test]
    async fn sniff_recognizes_gltf_json() {
        assert_eq!(GltfAnalyzerAnalysis::sniff(&AnalyzeSource::Text(r#"{"asset":{"version":"2.0"}}"#)), semio_framework_plugin::io::Confidence::High);
        assert_eq!(GltfAnalyzerAnalysis::sniff(&AnalyzeSource::Text("not json")), semio_framework_plugin::io::Confidence::Medium);
    }

    #[semio_framework_async_macros::async_test]
    async fn analyze_decodes_real_gltf_json_text_directly() {
        let text = r#"{"asset":{"version":"2.0"},"scenes":[]}"#;
        let analysis = GltfAnalyzerAnalysis::analyze(&[AnalyzeSource::Text(text)]);
        assert_eq!(analysis.confidence, semio_framework_plugin::io::Confidence::High);
        let snap = analysis.parts.snapshot.expect("snapshot");
        assert_eq!(snap.document.asset.version, "2.0");
    }
}
