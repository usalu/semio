#[allow(unused_imports)] use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
#[allow(unused_imports)] use framework_schema::ArtifactSchema;
#[allow(unused_imports)] use semio_s_artifact_draw_drawing_model::inferences::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = DrawingSnapshot::default();
    assert_eq!(DrawingInference::infer(&snapshot).expect("valid materialized inference fixture"), DrawingInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(DrawingInference::infer(&DrawingSnapshot::default()).expect("valid materialized inference fixture"), DrawingInference::default());
}
