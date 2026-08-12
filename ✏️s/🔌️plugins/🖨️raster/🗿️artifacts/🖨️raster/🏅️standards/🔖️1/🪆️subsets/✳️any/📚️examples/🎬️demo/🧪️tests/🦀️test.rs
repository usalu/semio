#[test]
fn primary_asset_is_nonempty() {
    let text = include_str!("../🖼️assets/🗣️example.dsl.semio");
    assert!(text.len() > 8);
}

#[test]
fn inference_determinism_law() {
    use crate::artifacts::raster::schema::inferences::RasterInference;
    use crate::artifacts::raster::RasterSnapshot;
    use protocol::Inference;
    let snapshot = RasterSnapshot::default();
    assert_eq!(RasterInference::infer(&snapshot), RasterInference::infer(&snapshot));
}

#[test]
fn inference_default_law() {
    use crate::artifacts::raster::schema::inferences::RasterInference;
    use crate::artifacts::raster::RasterSnapshot;
    use protocol::Inference;
    assert_eq!(RasterInference::infer(&RasterSnapshot::default()), RasterInference::default());
}
