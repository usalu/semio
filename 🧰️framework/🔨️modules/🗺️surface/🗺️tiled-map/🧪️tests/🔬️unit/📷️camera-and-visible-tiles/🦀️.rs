/// 📷️ Camera and visible tiles: viewport clamping, the camera zoom, the bounded visible tiles, the vector tile zoom pick, the overzoom, the layer visibility and stroke scale JSON and the visible tiles at the world zoom.

#[test]
fn map_set_size_reclamps_cover_zoom_when_viewport_grows() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let zoom = host.camera.zoom;
    host.set_size(1024, 768, 1.0);
    assert!(host.camera.zoom >= zoom - 1e-6);
    assert_viewport_corners_inside_world(&host);
}

#[test]
fn map_pan_and_zoom_stay_inside_world() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_camera(4.0, 4.0, super::MAP_CAMERA_ZOOM_MIN);
    assert_viewport_corners_inside_world(&host);
    host.wheel_screen(400.0, 300.0, -5000.0);
    assert_viewport_corners_inside_world(&host);
    host.pointer_down_screen(400.0, 300.0, 0);
    host.pointer_move_screen(40.0, 30.0);
    host.pointer_up_screen(40.0, 30.0);
    assert_viewport_corners_inside_world(&host);
}

fn assert_viewport_corners_inside_world(host: &super::MapHost) {
    let w = host.viewport.width as f64;
    let h = host.viewport.height as f64;
    for (sx, sy) in [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)] {
        let p = super::map_viewport::screen_to_world(&host.camera, &host.viewport, super::Point::new(sx, sy));
        assert!(p.x >= -WORLD_HALF - 1e-8 && p.x <= WORLD_HALF + 1e-8, "x out of world at ({sx},{sy}): {}", p.x);
        assert!(p.y >= -WORLD_HALF - 1e-8 && p.y <= WORLD_HALF + 1e-8, "y out of world at ({sx},{sy}): {}", p.y);
    }
}

#[test]
fn map_camera_zoom_not_clamped_to_puzzle_scale() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let fit_zoom = host.camera.zoom;
    assert!(fit_zoom > super::MAP_CAMERA_ZOOM_MIN);
    host.set_camera(0.0, 0.0, fit_zoom);
    assert!((host.camera.zoom - fit_zoom).abs() < 1e-6);
}

#[test]
fn visible_tiles_json_bounded_for_default_host() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_tiles_json()).expect("json");
    assert!(!raw.is_empty());
    assert!(raw.len() < 512);
}

#[test]
fn pick_vector_tile_zoom_clamped_to_the_policy_vector_pyramid() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, super::MAP_CAMERA_ZOOM_MAX);
    assert!(host.pick_vector_tile_zoom() <= host.policy().descriptor().vector_tile_z_max);
}

#[test]
fn visible_vector_tiles_overzoom_at_max_camera_zoom() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, super::MAP_CAMERA_ZOOM_MAX);
    assert!(super::vector_tiles_available_at_camera_zoom(super::MAP_CAMERA_ZOOM_MAX));
    let span = super::viewport_lon_span_degrees(&host.camera, &host.viewport);
    let z = host.pick_vector_tile_zoom();
    assert!(z <= host.policy().descriptor().vector_tile_z_max);
    assert!(z <= host.policy().vector_zoom_cap(span));
    assert!(z <= host.pick_raster_tile_zoom());
    let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_vector_tiles_json()).expect("json");
    assert!(!raw.is_empty());
}

#[test]
fn layer_visibility_json_round_trip() {
    let mut host = overlay_host();
    let pins = super::MapLayerTarget::Overlay("pins".into());
    assert!(host.layer_visible(&pins));
    host.set_layer_visibility_from_json(r#"{"pins":false,"paths":true}"#).expect("parse");
    assert!(!host.layer_visible(&pins));
    assert!(host.layer_visible(&super::MapLayerTarget::Overlay("paths".into())));
    let parsed: std::collections::BTreeMap<String, bool> = serde_json::from_str(&host.layer_visibility_json()).expect("serialize");
    assert_eq!(parsed.get("pins"), Some(&false));
}

#[test]
fn layer_stroke_scale_json_clamps_weights() {
    let mut host = overlay_host();
    host.set_layer_stroke_scale_from_json(r#"{"pins":9,"paths":0.1}"#).expect("parse");
    assert!((host.layer_weight(&super::MapLayerTarget::Overlay("pins".into())) - super::MAP_LAYER_WEIGHT_MAX).abs() < f64::EPSILON);
    assert!((host.layer_weight(&super::MapLayerTarget::Overlay("paths".into())) - super::MAP_LAYER_WEIGHT_MIN).abs() < f64::EPSILON);
}

#[test]
fn pick_vector_tile_zoom_tracks_viewport_not_camera_lod_steps() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let z_world = host.pick_vector_tile_zoom();
    host.set_camera(0.0, 0.0, 6000.0);
    let z_zoomed = host.pick_vector_tile_zoom();
    assert!(z_zoomed > z_world, "vector tile z must rise when zooming in (world={z_world}, zoomed={z_zoomed})");
}

#[test]
fn visible_vector_tiles_at_world_zoom() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    assert!(super::vector_tiles_available_at_camera_zoom(host.camera.zoom));
    let raw: Vec<serde_json::Value> = serde_json::from_str(&host.visible_vector_tiles_json()).expect("json");
    assert!(!raw.is_empty());
}
