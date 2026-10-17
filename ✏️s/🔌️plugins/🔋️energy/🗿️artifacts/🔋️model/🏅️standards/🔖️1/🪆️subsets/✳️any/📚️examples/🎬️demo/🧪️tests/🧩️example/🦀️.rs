use semio_framework_test_fixtures::fixture_path;

#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = &std::fs::read_to_string(fixture_path!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio")).expect("committed example asset");
    assert!(text.len() > 8);
}

/// 🧪️ The committed asset must decode to exactly the model this example claims — the example and
/// the engine's own case catalogue cannot drift apart silently.
#[semio_framework_async_macros::async_test]
async fn asset_carries_the_registered_case_model() {
    let text = &std::fs::read_to_string(fixture_path!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio")).expect("committed example asset");
    let snapshot = <crate::snapshot::EnergyModelSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    assert_eq!(snapshot.model, super::model());
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = &std::fs::read_to_string(fixture_path!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio")).expect("committed example asset");
    let snapshot = <crate::snapshot::EnergyModelSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::inferences::EnergyModelInference::infer(&snapshot).expect("valid materialized inference fixture");
    assert_eq!(inference, crate::inferences::EnergyModelInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(crate::inferences::EnergyModelInference::infer(&crate::snapshot::EnergyModelSnapshot::default()).expect("valid materialized inference fixture"), crate::inferences::EnergyModelInference::default(),);
}
//#endregion 🧪️InferenceLaws
