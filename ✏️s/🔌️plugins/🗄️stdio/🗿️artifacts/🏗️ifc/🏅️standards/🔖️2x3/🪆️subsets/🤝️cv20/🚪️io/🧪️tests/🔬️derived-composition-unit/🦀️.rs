mod tests {
    use super::*;
    use crate::standards::v2x3::subsets::cv20::schema::CODE_VIEW_DEFINITION;
    use crate::standards::v2x3::subsets::cv20::schema::Ifc2x3Cv20BuilderConstruction as Ifc2x3Cv20Builder;
    use semio_framework_plugin::AnalyzeSource;
    use semio_framework_plugin::ArtifactBuilder as _;

    #[semio_framework_async_macros::async_test]
    async fn conforming_builder_snapshot_composes_and_stamps_cv20() {
        let snapshot = Ifc2x3Cv20Builder::new().build().expect("clean CV2.0 document must build");
        let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = Ifc2x3Cv20ComposerComposition::compose(&sources).expect("clean document must compose to cv20");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn wrong_view_definition_fails_compose_with_real_diagnostic() {
        let mut snapshot = Ifc2x3Cv20Builder::new().build().expect("build");
        snapshot.document.header.file_description[0] = semio_s_artifact_stdio_step::engine::part21::Part21Value::List(vec![semio_s_artifact_stdio_step::engine::part21::Part21Value::Str("ViewDefinition [StructuralAnalysisView]".into())]);
        let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let err = Ifc2x3Cv20ComposerComposition::compose(&sources).expect_err("wrong ViewDefinition must not stamp cv20");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_VIEW_DEFINITION && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn subset_validator_recheck_is_clean_for_a_conforming_document() {
        let snapshot = Ifc2x3Cv20Builder::new().build().expect("build");
        let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let diagnostics = Ifc2x3Cv20Validator::validate(&IoPayload::Binary(bytes)).await;
        assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "wire recheck must never report a hard violation for a builder-clean document: {diagnostics:?}");
    }
}
