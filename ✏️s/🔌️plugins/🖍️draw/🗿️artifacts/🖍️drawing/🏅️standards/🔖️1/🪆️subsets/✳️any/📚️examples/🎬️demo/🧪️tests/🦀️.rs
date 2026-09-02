#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::drawing::schema::inferences::DrawingInference;
    use crate::artifacts::drawing::DrawingSnapshot;
    use protocol::Inference;
    let snapshot = DrawingSnapshot::default();
    assert_eq!(DrawingInference::infer(&snapshot), DrawingInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::drawing::schema::inferences::DrawingInference;
    use crate::artifacts::drawing::DrawingSnapshot;
    use protocol::Inference;
    assert_eq!(DrawingInference::infer(&DrawingSnapshot::default()), DrawingInference::default());
}
