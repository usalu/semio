#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use protocol::Inference;
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    let snapshot = <crate::artifacts::remodeling::RemodelingSnapshot as store::ArtifactDsl>::parse_dsl(text).expect("demo fixture parses");
    let inference = crate::artifacts::remodeling::standards::v1::subsets::any::schema::inferences::RemodelingInference::infer(&snapshot);
    assert_eq!(inference, crate::artifacts::remodeling::standards::v1::subsets::any::schema::inferences::RemodelingInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use protocol::Inference;
    assert_eq!(
        crate::artifacts::remodeling::standards::v1::subsets::any::schema::inferences::RemodelingInference::infer(&crate::artifacts::remodeling::RemodelingSnapshot::default()),
        crate::artifacts::remodeling::standards::v1::subsets::any::schema::inferences::RemodelingInference::default(),
    );
}
//#endregion 🧪️InferenceLaws
