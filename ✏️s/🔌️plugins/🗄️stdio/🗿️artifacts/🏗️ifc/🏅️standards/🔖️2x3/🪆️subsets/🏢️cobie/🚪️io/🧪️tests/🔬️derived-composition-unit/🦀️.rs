mod tests {
    use super::*;
    use crate::standards::v2x3::subsets::cobie::schema::CODE_VIEW_DEFINITION;
    use crate::standards::v2x3::subsets::cobie::schema::Ifc2x3CobieBuilderConstruction as Ifc2x3CobieBuilder;
    use semio_framework_plugin::AnalyzeSource;
    use semio_framework_plugin::ArtifactBuilder as _;

    #[semio_framework_async_macros::async_test]
    async fn conforming_builder_snapshot_composes_and_stamps_cobie() {
        let snapshot = Ifc2x3CobieBuilder::new().build().expect("clean COBie document must build");
        let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = Ifc2x3CobieComposerComposition::compose(&sources).expect("clean document must compose to cobie");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[semio_framework_async_macros::async_test]
    async fn wrong_view_definition_fails_compose_with_real_diagnostic() {
        let mut snapshot = Ifc2x3CobieBuilder::new().build().expect("build");
        snapshot.document.header.file_description[0] = semio_s_artifact_stdio_step::engine::part21::Part21Value::List(vec![semio_s_artifact_stdio_step::engine::part21::Part21Value::Str("ViewDefinition [CoordinationView]".into())]);
        let bytes = <Ifc2x3Snapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let err = Ifc2x3CobieComposerComposition::compose(&sources).expect_err("wrong ViewDefinition must not stamp cobie");
        assert!(err.diagnostics.iter().any(|d| d.code.0 == CODE_VIEW_DEFINITION && d.severity == Severity::Error), "got {:?}", err.diagnostics);
    }
}
