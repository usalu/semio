
use super::*;
use crate::RasterImageAsset;
use crate::standards::v1::subsets::any::schema::mutations::binary::test_support::retire_raster_snapshot;

#[test]
fn raster_asset_capacity_matches_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔣️asset-capacity.json")).expect("neutral capacity vectors");
    assert_eq!(vectors["capacity"].as_u64(), Some(crate::RASTER_OWNED_MAP_CAPACITY as u64));
    for vector in vectors["cases"].as_array().expect("cases") {
        let count = usize::try_from(vector["count"].as_u64().expect("count")).expect("bounded count");
        let oracle: serde_json::Map<String, serde_json::Value> = (0..count).map(|index| (format!("asset-{index:03}"), serde_json::json!({"mime":"application/octet-stream","data":""}))).collect();
        let delta: RasterAssetsDelta = dsl::os_pack::from_json_str(&serde_json::json!({"entries":oracle}).to_string()).expect("first-party input");
        let diff = RasterDiff { assets: Some(delta), ..Default::default() };
        for full_artifact in [false, true] {
            let base = RasterSnapshot::default();
            let result = if full_artifact {
                diff.apply_to_artifact(&RasterArtifact::default()).map(|artifact| {
                    let RasterArtifact { schema, id, title, layers, assets, .. } = artifact;
                    RasterSnapshot { schema, id, title, layers, assets }
                })
            } else {
                diff.apply(&base)
            };
            match result {
                Ok(snapshot) => {
                    let observed = snapshot.assets.keys().cloned().collect::<Vec<_>>();
                    retire_raster_snapshot(snapshot);
                    assert_eq!(vector["accepted"], true, "oversized input must fail before creating an owner");
                    assert_eq!(observed, oracle.keys().cloned().collect::<Vec<_>>());
                }
                Err(error) => {
                    assert_eq!(vector["accepted"], false, "{error}");
                    assert!(error.to_string().contains("raster-map.item-capacity"));
                }
            }
            assert!(base.assets.is_empty());
        }
        assert_eq!(diff.assets.as_ref().expect("delta").entries.len(), count);
    }
    let unsupported = RasterImageAsset { mime: "application/octet-stream".into(), data: Vec::new() };
    assert!(crate::io::semio_image_snapshot_from_raster_asset(&unsupported).is_err());
}
