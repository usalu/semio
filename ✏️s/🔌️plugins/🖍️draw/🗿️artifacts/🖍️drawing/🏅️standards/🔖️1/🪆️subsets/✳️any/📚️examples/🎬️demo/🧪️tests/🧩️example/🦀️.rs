#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use semio_s_artifact_draw_drawing_model::inferences::DrawingInference;
    use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
    use protocol::Inference;
    let snapshot = DrawingSnapshot::default();
    assert_eq!(DrawingInference::infer(&snapshot).expect("valid materialized inference fixture"), DrawingInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use semio_s_artifact_draw_drawing_model::inferences::DrawingInference;
    use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
    use protocol::Inference;
    assert_eq!(DrawingInference::infer(&DrawingSnapshot::default()).expect("valid materialized inference fixture"), DrawingInference::default());
}
