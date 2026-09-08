mod tests {
    use super::*;
    use crate::standards::v1_7::subsets::e::schema::PdfEBuilderConstruction as PdfEBuilder;
    use semio_framework_plugin::AnalyzeSource;
    use semio_framework_plugin::ArtifactBuilder as _;

    #[semio_framework_async_macros::async_test]
    async fn conforming_builder_snapshot_composes_and_stamps_e() {
        let snapshot = PdfEBuilder::new().add_page(crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage::new(100.0, 100.0)).build().unwrap();
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = PdfEComposerComposition::compose(&sources).expect("clean document must compose to e");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_runs_the_same_check() {
        let snapshot = PdfEBuilder::new().add_page(crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage::new(50.0, 50.0)).build().unwrap();
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let diagnostics = PdfEValidator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a builder-clean document: {diagnostics:?}");
    }
}
