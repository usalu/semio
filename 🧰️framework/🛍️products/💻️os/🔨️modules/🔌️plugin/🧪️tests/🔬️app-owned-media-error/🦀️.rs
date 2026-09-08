mod owned_media_error_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn owned_errors_preserve_their_messages() {
        assert_eq!(MediaArtifactError::Payload("payload".into()).to_string(), "payload");
        assert_eq!(MediaArtifactError::SchemaMismatch { expected: "a".into(), found: "b".into() }.to_string(), "document schema mismatch: expected a, found b");
        assert_eq!(MediaArtifactError::NoImporter("raw".into()).to_string(), "no binary importer registered for format \"raw\"");
    }
}
