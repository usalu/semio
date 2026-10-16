//! 🧪️ Unit proofs of the tiled-map model: projection round trips, tile addressing, camera clamps, layer-state parsing and the map-policy slider surface.

use super::projection::{default_world_camera, lonlat_to_world, world_to_lonlat, WORLD_HALF};
use super::tiles::{self, visible_tiles};
use super::MapPolicy;
use super::MAX_VISIBLE_TILE_REQUESTS;
use canvas::camera::{Camera, Viewport};

#[test]
fn lonlat_world_round_trip() {
    let w = lonlat_to_world(8.5, 47.4);
    let (lon, lat) = world_to_lonlat(w.x, w.y);
    assert!((lon - 8.5).abs() < 0.01);
    assert!((lat - 47.4).abs() < 0.5);
}

#[test]
fn default_world_camera_fits_extent() {
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let cam = default_world_camera(&viewport);
    assert!(cam.zoom > 0.0);
    assert_eq!(cam.x, 0.0);
    assert_eq!(cam.y, 0.0);
}

#[test]
fn visible_tiles_at_world_view() {
    let camera = Camera { x: 0.0, y: 0.0, zoom: 200.0 };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let tiles = visible_tiles(&camera, &viewport, 2);
    assert!(!tiles.is_empty());
    assert!(tiles.len() < 256, "world view must not enumerate excessive tiles");
    let _ = WORLD_HALF;
}

#[test]
fn typed_visible_tile_cursor_matches_legacy_order_and_resumes_one_item_at_a_time() {
    let camera = Camera { x: 0.0, y: 0.0, zoom: 200.0 };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let legacy = visible_tiles(&camera, &viewport, 3);
    let mut cursor = tiles::visible_tile_cursor(&camera, &viewport, 3);
    assert_eq!(cursor.remaining(), legacy.len());
    let mut typed = Vec::new();
    while let Some(tile) = cursor.peek() {
        typed.push((tile.z, tile.x, tile.y));
        assert!(cursor.advance());
    }
    assert_eq!(typed, legacy);
    assert_eq!(cursor.remaining(), 0);
    assert!(!cursor.advance());
}

#[test]
fn map_camera_max_zoom_reaches_street_longitude_span() {
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let camera = Camera { x: 0.0, y: 0.0, zoom: super::MAP_CAMERA_ZOOM_MAX };
    let span = super::viewport_lon_span_degrees(&camera, &viewport);
    assert!(span < 0.01, "max camera zoom must reach sub-city scale (span={span:.6}°)");
}

#[test]
fn map_lod_picks_bounded_tile_zoom_at_world_fit() {
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let camera = default_world_camera(&viewport);
    let z = tiles::pick_zoom(&camera, &viewport, &super::DescribedMapPolicy::default(), None);
    let tiles = visible_tiles(&camera, &viewport, z);
    assert!(tiles.len() < MAX_VISIBLE_TILE_REQUESTS);
}

#[test]
fn ideal_tile_z_rises_when_viewport_span_shrinks() {
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let world = default_world_camera(&viewport);
    let zoomed = Camera { x: 0.0, y: 0.0, zoom: 1800.0 };
    assert!(super::ideal_tile_z_for_viewport(&zoomed, &viewport, 22) > super::ideal_tile_z_for_viewport(&world, &viewport, 22));
    assert_eq!(super::ideal_tile_z_for_viewport(&Camera { x: 0.0, y: 0.0, zoom: super::MAP_CAMERA_ZOOM_MAX }, &viewport, 7), 7, "the policy's raster pyramid caps the ideal tile z");
}

#[test]
fn map_wheel_screen_keeps_world_under_cursor_with_flipped_y() {
    let mut camera = Camera { x: 0.15, y: -0.25, zoom: 320.0 };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let sx = 220.0;
    let sy = 140.0;
    let before = super::map_viewport::screen_to_world(&camera, &viewport, super::Point::new(sx, sy));
    super::map_wheel_screen(&mut camera, &viewport, sx, sy, -80.0);
    let after = super::map_viewport::screen_to_world(&camera, &viewport, super::Point::new(sx, sy));
    assert!(camera.zoom > 320.0);
    assert!((after.x - before.x).abs() < 1e-9, "world x should stay under cursor");
    assert!((after.y - before.y).abs() < 1e-9, "world y should stay under cursor");
}

#[test]
fn map_render_mode_parse() {
    assert_eq!(super::MapTileMode::parse("image"), super::MapTileMode::Image);
    assert_eq!(super::MapTileMode::parse("vector"), super::MapTileMode::Vector);
    assert_eq!(super::MapTileMode::parse("combined"), super::MapTileMode::Combined);
}

#[test]
fn parse_tile_key_round_trips_tile_key() {
    let key = tiles::tile_key(3, 5, 7);
    assert_eq!(tiles::parse_tile_key(&key), Some((3, 5, 7)));
    assert_eq!(tiles::parse_tile_key("bad"), None);
    assert_eq!(tiles::parse_tile_key("1/2"), None);
}

#[test]
fn clamp_map_layer_weight_handles_bounds_and_non_finite_values() {
    assert_eq!(super::clamp_map_layer_weight(super::MAP_LAYER_WEIGHT_MIN - 1.0), super::MAP_LAYER_WEIGHT_MIN);
    assert_eq!(super::clamp_map_layer_weight(super::MAP_LAYER_WEIGHT_MAX + 1.0), super::MAP_LAYER_WEIGHT_MAX);
    assert_eq!(super::clamp_map_layer_weight(f64::NAN), 1.0);
    assert_eq!(super::clamp_map_layer_weight(f64::INFINITY), 1.0);
}

#[test]
fn layer_switch_weights_are_sanitized_per_key() {
    let weights = super::MapLayerSwitches::sanitized_weights(std::collections::BTreeMap::from([("a".to_string(), 100.0), ("b".to_string(), -1.0), ("c".to_string(), f64::NAN)]));
    assert_eq!(weights["a"], super::MAP_LAYER_WEIGHT_MAX);
    assert_eq!(weights["b"], super::MAP_LAYER_WEIGHT_MIN);
    assert_eq!(weights["c"], 1.0);
}

#[test]
fn layer_switches_follow_the_policy_toggles() {
    let mut descriptor = super::MapPolicyDescriptor::neutral();
    descriptor.layer_toggles = vec![super::MapLayerToggle { key: "ground".into(), targets: vec![super::MapLayerTarget::Raster, super::MapLayerTarget::Role(super::MapLayerRole::LandCover)] }];
    let policy = super::DescribedMapPolicy::new(descriptor);
    let switches = super::MapLayerSwitches { visibility: std::collections::BTreeMap::from([("ground".to_string(), false)]), weights: std::collections::BTreeMap::from([("ground".to_string(), 2.0)]) };
    assert!(!switches.visible(&policy, &super::MapLayerTarget::Raster));
    assert!(!switches.visible(&policy, &super::MapLayerTarget::Role(super::MapLayerRole::LandCover)));
    assert_eq!(switches.weight(&policy, &super::MapLayerTarget::Role(super::MapLayerRole::LandCover)), 2.0);
    assert!(switches.visible(&policy, &super::MapLayerTarget::Role(super::MapLayerRole::Road)), "a target no toggle lists always paints");
    assert_eq!(switches.weight(&policy, &super::MapLayerTarget::Overlay("pins".into())), 1.0);
    assert!(super::MapLayerSwitches::default().visible(&policy, &super::MapLayerTarget::Raster), "an absent key is visible");
}

#[test]
fn map_camera_limits_json_min_never_exceeds_max() {
    let v: serde_json::Value = serde_json::from_str(&super::map_camera_limits_json()).expect("json");
    assert!(v["min"].as_f64().unwrap() <= v["max"].as_f64().unwrap());
}

#[test]
fn map_layer_weight_slider_keys_respect_render_mode() {
    let mut descriptor = super::MapPolicyDescriptor::neutral();
    descriptor.layer_toggles = vec![
        super::MapLayerToggle { key: "tiles".into(), targets: vec![super::MapLayerTarget::Raster] },
        super::MapLayerToggle { key: "streets".into(), targets: vec![super::MapLayerTarget::Role(super::MapLayerRole::Road)] },
        super::MapLayerToggle { key: "pins".into(), targets: vec![super::MapLayerTarget::Overlay("pins".into())] },
    ];
    let policy = super::DescribedMapPolicy::new(descriptor);
    let keys = |mode: &str| -> Vec<String> { serde_json::from_str(&super::map_layer_weight_slider_ids_json(&policy, "all", mode)).expect("slider ids") };
    assert_eq!(keys("image"), ["tiles", "pins"]);
    assert_eq!(keys("vector"), ["streets", "pins"], "every neutral layer gate is open");
    assert_eq!(keys("bogus-mode"), ["tiles", "streets", "pins"]);
    assert!(super::DescribedMapPolicy::default().slider_keys("all", super::MapTileMode::Combined).is_empty(), "the neutral policy declares no toggle");
}

#[test]
fn parse_tile_key_rejects_malformed_keys() {
    assert_eq!(tiles::parse_tile_key("1/2/3"), Some((1, 2, 3)));
    assert_eq!(tiles::parse_tile_key("1/2"), None);
    assert_eq!(tiles::parse_tile_key("1/2/3/4"), None);
    assert_eq!(tiles::parse_tile_key("a/b/c"), None);
}

#[test]
fn clamp_map_zoom_and_for_viewport_stay_within_bounds() {
    assert_eq!(super::clamp_map_zoom(super::MAP_CAMERA_ZOOM_MIN - 5.0), super::MAP_CAMERA_ZOOM_MIN);
    assert_eq!(super::clamp_map_zoom(super::MAP_CAMERA_ZOOM_MAX + 5.0), super::MAP_CAMERA_ZOOM_MAX);
    let viewport = Viewport { width: 4000, height: 4000, dpr: 1.0 };
    let cover = super::projection::cover_zoom_for_viewport(&viewport);
    assert!(super::clamp_map_zoom_for_viewport(0.0, &viewport) >= cover);
}
