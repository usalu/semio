#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    let snapshot = <crate::artifacts::model::EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::model::standards::v1::subsets::any::schema::inferences::EnergyModelInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::model::standards::v1::subsets::any::schema::inferences::EnergyModelInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::model::standards::v1::subsets::any::schema::inferences::EnergyModelInference::infer(&crate::artifacts::model::EnergyModelSnapshot::default()),
        crate::artifacts::model::standards::v1::subsets::any::schema::inferences::EnergyModelInference::default(),
    );
}
//#endregion 🧪️InferenceLaws
