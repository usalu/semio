use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_diff::*;
use semio_s_artifact_raster_raster_model::retire_raster_value_cold;
use semio_s_artifact_raster_raster_model::RasterImageAsset;

#[test]
fn raster_asset_capacity_matches_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../🚪️io/📝️text/🔺️diff/🧪️tests/🧫️fixtures/🔣️asset-capacity.json")).expect("neutral capacity vectors");
    assert_eq!(vectors["capacity"].as_u64(), Some(semio_s_artifact_raster_raster_model::RASTER_OWNED_MAP_CAPACITY as u64));
    for vector in vectors["cases"].as_array().expect("cases") {
        let count = usize::try_from(vector["count"].as_u64().expect("count")).expect("bounded count");
        let oracle: serde_json::Map<String, serde_json::Value> = (0..count).map(|index| (format!("asset-{index:03}"), serde_json::json!({"schema":"s.stdio.semio.image","width":0,"height":0,"colorspace":"rgb","bitDepth":0,"frames":[],"icc":null,"metadata":[]}))).collect();
        let delta: RasterAssetsDelta = semio_framework_pack_json::from_json_str(&serde_json::json!({"entries":oracle}).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("first-party input");
        let diff = RasterDiff { assets: Some(delta), ..Default::default() };
        let base = RasterSnapshot::default();
        match protocol::apply_diff_cold(&diff, &base) {
            Ok(snapshot) => {
                let observed = snapshot.assets.keys().cloned().collect::<Vec<_>>();
                retire_raster_value_cold(snapshot);
                assert_eq!(vector["accepted"], true, "oversized input must fail before creating an owner");
                assert_eq!(observed, oracle.keys().cloned().collect::<Vec<_>>());
            }
            Err(error) => {
                assert_eq!(vector["accepted"], false, "{error}");
                assert!(error.to_string().contains("raster-map.item-capacity"));
            }
        }
        assert!(base.assets.is_empty());
        assert_eq!(diff.assets.as_ref().expect("delta").entries.len(), count);
    }
    let unsupported = RasterImageAsset { mime: "application/octet-stream".into(), data: Vec::new() };
    assert!(semio_s_artifact_raster_raster_io::semio_image_snapshot_from_raster_asset(&unsupported).is_err());
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
