use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
use protocol::MutationDiff;

#[test]
fn decoded_image_assets_preserve_every_field_without_a_carrier() {
    let corpus: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../🧫️fixtures/🔬️contract/🖼️decoded-image/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let image: SemioImageSnapshot = semio_framework_pack_json::from_json_str(&case["image"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(semio_s_artifact_raster_raster_model::raster_image_content_id("image", &image), case["contentId"].as_str().unwrap());
        let base = RasterSnapshot::default();
        let diff = semio_s_artifact_raster_raster_diff::diff_add_asset("image", image.clone());
        let after = protocol::apply_diff(&diff, &base).unwrap();
        assert_eq!(semio_s_artifact_raster_raster_model::raster_image(&after.assets, "image"), Some(image.clone()));
        let inverse = semio_s_artifact_raster_raster_mutations::inverse_raster_mutation(&after, &semio_s_artifact_raster_raster_mutations::RasterMutation::RemoveLayerAsset(semio_s_artifact_raster_raster_mutations_pixels::remove_layer_asset::mutation::RemoveLayerAsset { asset_id: "image".into() })).unwrap();
        assert!(matches!(&inverse[0], semio_s_artifact_raster_raster_mutations::RasterMutation::AddLayerAsset(payload) if payload.asset == image));
        for operation in inverse { protocol::Mutation::retire_cold(operation); }
        semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(after);
        semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(base);
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
