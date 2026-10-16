//! 🏛️ Host-level proofs of the map policy port: a `MapHost` honouring a policy installed in code, shipped in the map descriptor JSON or described by band floors.

use super::*;

const FIXTURE: &str = include_str!("../🧫️fixtures/🔣️.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE).expect("policy fixture")
}

fn sample_policy() -> DescribedMapPolicy {
    DescribedMapPolicy::new(serde_json::from_value(fixture()["samplePolicy"].clone()).expect("sample policy descriptor"))
}

fn host_with(policy: DescribedMapPolicy) -> MapHost {
    let mut host = MapHost::with_extension(Arc::new(DefaultMapExtension::with_policy(policy)));
    host.set_size(800, 600, 1.0);
    host
}

/// 🧷️ A code-level policy: always raster tile z 3 and a vector cap of 2, whatever the viewport.
struct FixedTileZoomPolicy {
    descriptor: MapPolicyDescriptor,
}

impl MapPolicy for FixedTileZoomPolicy {
    fn descriptor(&self) -> &MapPolicyDescriptor {
        &self.descriptor
    }

    fn tile_z_target(&self, _view: MapViewFacts, _pinned: Option<&str>) -> u32 {
        3
    }

    fn vector_zoom_cap(&self, _span_deg: f64) -> u32 {
        2
    }
}

struct FixedTileZoomExtension(FixedTileZoomPolicy);

impl CanvasExtension for FixedTileZoomExtension {
    fn extension_id(&self) -> &str {
        "test.map/fixed-tile-zoom"
    }
}

impl MapExtension for FixedTileZoomExtension {
    fn map_crs(&self) -> &str {
        "EPSG:3857"
    }

    fn policy(&self) -> &dyn MapPolicy {
        &self.0
    }
}

#[test]
fn a_non_default_policy_is_honoured_by_the_map_host() {
    let mut host = MapHost::with_extension(Arc::new(FixedTileZoomExtension(FixedTileZoomPolicy { descriptor: MapPolicyDescriptor::neutral() })));
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 6000.0);
    let mut neutral = MapHost::new();
    neutral.set_size(800, 600, 1.0);
    neutral.set_camera(0.0, 0.0, 6000.0);
    assert_eq!(host.extension().extension_id(), "test.map/fixed-tile-zoom");
    assert_eq!(host.pick_raster_tile_zoom(), 3);
    assert_eq!(host.pick_vector_tile_zoom(), 2);
    assert_ne!(neutral.pick_raster_tile_zoom(), 3, "the neutral policy picks the ideal tile z instead");
    let visible: Vec<serde_json::Value> = serde_json::from_str(&host.visible_tiles_json()).expect("visible tiles");
    assert!(visible.iter().all(|row| row["z"] == 3));
}

#[test]
fn a_policy_shipped_in_the_map_descriptor_replaces_the_neutral_policy() {
    let mut host = MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_lod_mode("near");
    let lod: serde_json::Value = serde_json::from_str(&host.current_lod_json()).expect("lod");
    assert_eq!(lod["mode"], MAP_LOD_MODE_AUTOMATIC, "no policy declares `near` yet");
    let descriptor = fixture()["sampleDescriptor"].to_string();
    host.sync_map_json(&descriptor).expect("descriptor with policy");
    assert_eq!(host.policy().id(), "test.map/two-bands");
    assert_eq!(host.extension().extension_id(), MAP_EXTENSION_DEFAULT_ID);
    let lod: serde_json::Value = serde_json::from_str(&host.current_lod_json()).expect("lod");
    assert_eq!((lod["id"].as_str(), lod["mode"].as_str()), (Some("near"), Some("near")), "the requested band pins once the policy declares it");
    let revision = host.interaction_revision();
    host.sync_map_json(&descriptor).expect("same policy again");
    assert_eq!(host.interaction_revision(), revision, "an unchanged policy is not reinstalled");
    assert!(host.overlay_feature("pins", "p1").is_some(), "overlay features are mirrored even before a policy declares their layer");
    assert_eq!(host.hit_test_feature_json(400.0, 300.0), "null", "an undeclared overlay layer is neither painted nor pickable");
    host.sync_map_json(r#"{"overlays":{}}"#).expect("descriptor without policy");
    assert_eq!(host.policy().id(), "test.map/two-bands", "a descriptor without a policy keeps the installed one");
    assert!(host.sync_map_json(r#"{"policy":{"id":"broken"}}"#).is_err(), "a malformed policy is a sync error");
}

#[test]
fn a_described_policy_honours_its_band_floors_in_the_host() {
    let mut host = host_with(sample_policy());
    host.fit_world_camera();
    let lod: serde_json::Value = serde_json::from_str(&host.current_lod_json()).expect("lod");
    assert_eq!(lod["id"], "far");
    assert_eq!(host.pick_raster_tile_zoom(), 1, "the coarse band fixes its raster tile z");
    host.set_lod_mode("near");
    assert!(host.pick_raster_tile_zoom() >= 1);
    let scale: Vec<serde_json::Value> = serde_json::from_str(&map_lod_scale_json(host.policy())).expect("lod scale");
    assert_eq!(scale.len(), 2);
}
