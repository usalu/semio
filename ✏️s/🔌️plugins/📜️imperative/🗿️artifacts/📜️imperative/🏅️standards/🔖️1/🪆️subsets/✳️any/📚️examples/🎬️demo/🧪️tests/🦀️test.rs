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
    let snapshot = <crate::artifacts::imperative::ImperativeSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::imperative::standards::v1::subsets::any::schema::inferences::ImperativeInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::imperative::standards::v1::subsets::any::schema::inferences::ImperativeInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::imperative::standards::v1::subsets::any::schema::inferences::ImperativeInference::infer(&crate::artifacts::imperative::ImperativeSnapshot::default()),
        crate::artifacts::imperative::standards::v1::subsets::any::schema::inferences::ImperativeInference::default(),
    );
}
//#endregion 🧪️InferenceLaws
