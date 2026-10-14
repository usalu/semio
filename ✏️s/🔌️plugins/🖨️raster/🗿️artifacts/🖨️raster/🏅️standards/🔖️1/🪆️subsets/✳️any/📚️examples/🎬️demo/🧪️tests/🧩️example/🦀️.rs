#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🎬️demo/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use semio_s_artifact_raster_raster_model::inferences::RasterInference;
    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
    use protocol::Inference;
    let snapshot = RasterSnapshot::default();
    assert_eq!(RasterInference::infer(&snapshot).expect("valid materialized inference fixture"), RasterInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use semio_s_artifact_raster_raster_model::inferences::RasterInference;
    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
    use protocol::Inference;
    assert_eq!(RasterInference::infer(&RasterSnapshot::default()).expect("valid materialized inference fixture"), RasterInference::default());
}
