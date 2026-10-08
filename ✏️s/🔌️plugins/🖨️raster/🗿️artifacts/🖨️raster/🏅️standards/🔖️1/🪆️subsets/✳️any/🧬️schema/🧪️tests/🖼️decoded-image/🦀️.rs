use crate::{RasterSnapshot, SemioImageSnapshot};
use protocol::MutationDiff;

#[test]
fn decoded_image_assets_preserve_every_field_without_a_carrier() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖼️decoded-image/🔣️.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let image: SemioImageSnapshot = semio_framework_pack_json::from_json_str(&case["image"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(crate::raster_image_content_id("image", &image), case["contentId"].as_str().unwrap());
        let base = RasterSnapshot::default();
        let diff = crate::diff::diff_add_asset("image", image.clone());
        let after = protocol::apply_diff(&diff, &base).unwrap();
        assert_eq!(crate::raster_image(&after.assets, "image"), Some(image.clone()));
        let inverse = crate::mutations::inverse_raster_mutation(&after, &crate::mutations::RasterMutation::RemoveLayerAsset(crate::mutations::remove_layer_asset::RemoveLayerAsset { asset_id: "image".into() })).unwrap();
        assert!(matches!(&inverse[0], crate::mutations::RasterMutation::AddLayerAsset(payload) if payload.asset == image));
        eprintln!("[DEBUG] decoded image asset {} preserved", case["id"]);
        for operation in inverse { protocol::Mutation::retire_cold(operation); }
        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(after);
        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(base);
    }
}
