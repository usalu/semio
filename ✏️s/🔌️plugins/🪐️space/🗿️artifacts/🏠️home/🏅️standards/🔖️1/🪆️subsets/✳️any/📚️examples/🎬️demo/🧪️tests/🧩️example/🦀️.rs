#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

//#region 🧪️InferenceLaws
use crate::standards::v1::subsets::any::schema::inferences::SHomeInference;
use crate::SHomeSnapshot;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 42 };
    assert_eq!(SHomeInference::infer(&snapshot).expect("valid materialized inference fixture"), SHomeInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SHomeInference::infer(&SHomeSnapshot::default()).expect("valid materialized inference fixture"), SHomeInference::default());
}
//#endregion 🧪️InferenceLaws
