mod tests {
    use super::*;
    use crate::standards::v1_7::subsets::h::schema::PdfHBuilderConstruction as PdfHBuilder;
    use semio_framework_plugin::AnalyzeSource;
    use semio_framework_plugin::ArtifactBuilder as _;

    #[semio_framework_async_macros::async_test]
    async fn compose_always_succeeds_even_with_zero_setup() {
        let snapshot = PdfSnapshot::default();
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = PdfHComposerComposition::compose(&sources).expect("PDF/H never hard-gates");
        assert!(composed.diagnostics.iter().any(|d| d.code.0 == crate::standards::v1_7::subsets::h::schema::CODE_INFO_TITLE_OR_AUTHOR));
    }

    #[semio_framework_async_macros::async_test]
    async fn conforming_builder_snapshot_composes_with_fewer_advisories() {
        let snapshot = PdfHBuilder::new()
            .add_page(crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage::new(100.0, 100.0))
            .set_info(crate::standards::v1_7::subsets::base::schema::snapshot::PdfInfo { title: Some("A Chart".into()), author: Some("Dr. X".into()), ..Default::default() })
            .build()
            .unwrap();
        let bytes = <PdfSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = PdfHComposerComposition::compose(&sources).expect("PDF/H never hard-gates");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error && d.severity != Severity::Fatal), "got {:?}", composed.diagnostics);
    }
}
