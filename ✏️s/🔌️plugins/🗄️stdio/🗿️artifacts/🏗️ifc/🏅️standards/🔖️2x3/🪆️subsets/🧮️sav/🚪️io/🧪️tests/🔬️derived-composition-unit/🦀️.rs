mod tests {
    use super::*;
    use crate::standards::v2x3::subsets::sav::schema::CODE_NO_ANALYSIS_MODEL;
    use crate::standards::v2x3::subsets::sav::schema::Ifc2x3SavBuilderConstruction as Ifc2x3SavBuilder;
    use semio_framework_plugin::AnalyzeSource;
    use semio_framework_plugin::ArtifactBuilder as _;

    #[semio_framework_async_macros::async_test]
    async fn conforming_builder_snapshot_composes_and_stamps_sav() {
        let snapshot = Ifc2x3SavBuilder::new().build().expect("clean SAV document must build");
        let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = Ifc2x3SavComposerComposition::compose(&sources).expect("clean document must compose to sav");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn no_analysis_model_fails_compose_with_real_diagnostic() {
        let mut snapshot = Ifc2x3SavBuilder::new().build().expect("build");
        snapshot.document.instances.retain(|i| !i.is_type("IFCSTRUCTURALANALYSISMODEL"));
        let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let err = Ifc2x3SavComposerComposition::compose(&sources).expect_err("a document with no analysis model must not stamp sav");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_NO_ANALYSIS_MODEL && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }
}
