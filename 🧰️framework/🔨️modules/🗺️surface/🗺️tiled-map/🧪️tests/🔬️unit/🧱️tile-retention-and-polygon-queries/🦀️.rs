/// 🧱️ Tile retention and polygon queries: the raster and vector ancestor retention, the tile rectangle intersection, the point and polyline geometry, the interaction plan, the polygon feature queries and the feature screen coverage.

#[test]
fn tile_retention_keeps_raster_ancestor_after_zoom_in() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    let png = test_png_1x1();
    host.upload_tile(0, 0, 0, &png).expect("upload");
    assert!(host.tiles.tile_images.contains_key("0/0/0"));
    host.set_camera(0.0, 0.0, 6000.0);
    let z_fine = host.pick_raster_tile_zoom();
    assert!(z_fine >= 4, "zoomed camera should request finer raster tiles (got {z_fine})");
    host.prepare_visible_tiles();
    assert!(host.tiles.tile_images.contains_key("0/0/0"), "ancestor tile must survive zoom-level change for pyramid fallback");
}

#[test]
fn tile_rect_intersects_viewport_for_on_and_off_screen_tiles() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 3200.0);
    let z = host.pick_raster_tile_zoom();
    let visible = visible_tiles(&host.camera, &host.viewport, z);
    let (tz, tx, ty) = visible[0];
    let on_screen = tile_world_rect(tz, tx, ty);
    assert!(host.tile_rect_intersects_viewport(on_screen));
    let off_screen = super::Rect::new(4.0, 4.0, 5.0, 5.0);
    assert!(!host.tile_rect_intersects_viewport(off_screen));
}

#[test]
fn tile_retention_keeps_vector_ancestor_after_zoom_in() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.upload_vector_tile(0, 0, 0, &[]).expect("upload");
    assert!(host.tiles.vector_tiles.contains_key("0/0/0"));
    host.set_camera(0.0, 0.0, 2500.0);
    let vz = host.pick_vector_tile_zoom();
    assert!(vz >= 1, "zoomed camera should request finer vector tiles (got {vz})");
    host.prepare_visible_tiles();
    assert!(host.tiles.vector_tiles.contains_key("0/0/0"), "ancestor vector tile must survive zoom-level change for pyramid fallback");
}

#[test]
fn map_point_segment_distance_zero_length_segment_uses_point_distance() {
    let d = super::map_point_segment_distance(3.0, 4.0, 0.0, 0.0, 0.0, 0.0);
    assert!((d - 5.0).abs() < 1e-9);
}

#[test]
fn interaction_plan_matches_direct_wheel_and_pan_semantics() {
    let mut direct = super::MapHost::new();
    let mut planned = super::MapHost::new();
    direct.set_size(800, 600, 1.0);
    planned.set_size(800, 600, 1.0);
    direct.wheel_screen(320.0, 240.0, -12.0);
    let wheel = planned.plan_interaction(super::MapInteractionIntent::Wheel { sx: 320.0, sy: 240.0, delta_y: -12.0 });
    assert!(planned.commit_interaction(wheel));
    assert!(super::map_camera_matches(&direct.camera, &planned.camera));

    for intent in [super::MapInteractionIntent::PointerDown { sx: 100.0, sy: 120.0, button: 1 }, super::MapInteractionIntent::PointerMove { sx: 130.0, sy: 150.0 }, super::MapInteractionIntent::PointerUp { sx: 130.0, sy: 150.0 }] {
        let plan = planned.plan_interaction(intent);
        assert!(planned.commit_interaction(plan));
    }
    direct.pointer_down_screen(100.0, 120.0, 1);
    direct.pointer_move_screen(130.0, 150.0);
    direct.pointer_up_screen(130.0, 150.0);
    assert!(super::map_camera_matches(&direct.camera, &planned.camera));
    assert_eq!(direct.drain_events_json(), planned.drain_events_json());
}

#[test]
fn stale_interaction_plan_never_mutates_replaced_camera() {
    let mut host = super::MapHost::new();
    let plan = host.plan_interaction(super::MapInteractionIntent::Wheel { sx: 10.0, sy: 20.0, delta_y: -1.0 });
    host.set_camera(42.0, -7.0, 2.0);
    let replacement = [host.camera.x, host.camera.y, host.camera.zoom];
    assert!(!host.commit_interaction(plan));
    assert_eq!([host.camera.x, host.camera.y, host.camera.zoom], replacement);
}

#[test]
fn map_point_in_polygon_rejects_degenerate_polygon() {
    assert!(!super::map_point_in_polygon(0.0, 0.0, &[]));
    assert!(!super::map_point_in_polygon(0.0, 0.0, &[super::Point::new(0.0, 0.0), super::Point::new(1.0, 1.0)]));
}

#[test]
fn map_polyline_intersects_rect_detects_edge_crossing_without_endpoints_inside() {
    let line = [super::Point::new(-10.0, 5.0), super::Point::new(10.0, 5.0)];
    assert!(super::map_polyline_intersects_rect(&line, 0.0, 0.0, 4.0, 10.0));
    let miss = [super::Point::new(-10.0, 50.0), super::Point::new(10.0, 50.0)];
    assert!(!super::map_polyline_intersects_rect(&miss, 0.0, 0.0, 4.0, 10.0));
}

#[test]
fn map_polyline_intersects_polygon_detects_crossing_edge() {
    let square = [super::Point::new(0.0, 0.0), super::Point::new(10.0, 0.0), super::Point::new(10.0, 10.0), super::Point::new(0.0, 10.0)];
    let crossing = [super::Point::new(-5.0, 5.0), super::Point::new(15.0, 5.0)];
    assert!(super::map_polyline_intersects_polygon(&crossing, &square));
    let outside = [super::Point::new(-5.0, 50.0), super::Point::new(-1.0, 50.0)];
    assert!(!super::map_polyline_intersects_polygon(&outside, &square));
}

#[test]
fn features_in_polygon_json_invalid_json_returns_empty() {
    let host = overlay_host();
    let out = host.features_in_polygon_json("not json", true);
    let v: serde_json::Value = serde_json::from_str(&out).expect("json");
    assert!(v["pins"].as_array().unwrap().is_empty());
    assert!(v["paths"].as_array().unwrap().is_empty());
}

#[test]
fn features_in_polygon_json_requires_at_least_a_triangle() {
    let host = overlay_host();
    let out = host.features_in_polygon_json("[[0.0,0.0],[1.0,1.0]]", true);
    let v: serde_json::Value = serde_json::from_str(&out).expect("json");
    assert!(v["pins"].as_array().unwrap().is_empty());
}

#[test]
fn features_in_polygon_json_finds_an_enclosed_point() {
    let mut host = overlay_host();
    host.sync_map_json(r#"{"overlays":{"pins":[{"id":"p1","coordinates":[[0.0,0.0]]}]}}"#).expect("sync");
    let sv: serde_json::Value = serde_json::from_str(&host.feature_screen_json("pin", "p1")).expect("json");
    let (sx, sy) = (sv["x"].as_f64().unwrap(), sv["y"].as_f64().unwrap());
    let polygon = format!("[[{},{}],[{},{}],[{},{}]]", sx - 20.0, sy - 20.0, sx + 20.0, sy - 20.0, sx, sy + 20.0);
    let out = host.features_in_polygon_json(&polygon, true);
    let v: serde_json::Value = serde_json::from_str(&out).expect("json");
    assert_eq!(v["pins"].as_array().unwrap().first().and_then(|x| x.as_str()), Some("p1"));
}

#[test]
fn feature_screen_json_covers_lines_missing_and_unknown_kind() {
    let mut host = overlay_host();
    host.sync_map_json(r#"{"overlays":{"paths":[{"id":"r1","coordinates":[[0.0,0.0],[1.0,1.0]]}]}}"#).expect("sync");
    let line_json = host.feature_screen_json("path", "r1");
    assert_ne!(line_json, "null");
    assert!(serde_json::from_str::<serde_json::Value>(&line_json).expect("json")["polyline"].as_array().is_some_and(|rows| rows.len() == 2));
    assert_eq!(host.feature_screen_json("path", "missing"), "null");
    assert_eq!(host.feature_screen_json("bogus", "r1"), "null");
    host.sync_map_json(r#"{"overlays":{"paths":[{"id":"empty","coordinates":[]}]}}"#).expect("sync");
    assert_eq!(host.feature_screen_json("path", "empty"), "null");
}
