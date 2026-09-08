#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

/// 🧪️ The committed asset must decode to exactly the model this example claims — the example and
/// the engine's own case catalogue cannot drift apart silently.
#[semio_framework_async_macros::async_test]
async fn asset_carries_the_registered_case_model() {
    let text = include_str!("../../🖼️assets/🗣️.dsl.semio");
    let snapshot = <crate::EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    assert_eq!(snapshot.model, super::model());
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../../🖼️assets/🗣️.dsl.semio");
    let snapshot = <crate::EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::standards::v1::subsets::any::schema::inferences::EnergyModelInference::infer(&snapshot);
    assert_eq!(inference, crate::standards::v1::subsets::any::schema::inferences::EnergyModelInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::standards::v1::subsets::any::schema::inferences::EnergyModelInference::infer(&crate::EnergyModelSnapshot::default()),
        crate::standards::v1::subsets::any::schema::inferences::EnergyModelInference::default(),
    );
}
//#endregion 🧪️InferenceLaws
