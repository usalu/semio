#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifacts::raster::schema::inferences::RasterInference;
    use crate::artifacts::raster::RasterSnapshot;
    use protocol::Inference;
    let snapshot = RasterSnapshot::default();
    assert_eq!(RasterInference::infer(&snapshot), RasterInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifacts::raster::schema::inferences::RasterInference;
    use crate::artifacts::raster::RasterSnapshot;
    use protocol::Inference;
    assert_eq!(RasterInference::infer(&RasterSnapshot::default()), RasterInference::default());
}
