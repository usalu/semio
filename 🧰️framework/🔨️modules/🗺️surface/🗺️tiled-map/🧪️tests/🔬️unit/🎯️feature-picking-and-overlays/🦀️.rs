/// 🎯️ Feature picking and overlays: the raster affine, the overlay JSON, the feature screen projection, the hit tests, the rectangle and line queries, the interaction sync, the pointer up camera emission, the overlay paint order and the map theme JSON.

#[test]
fn tile_raster_affine_maps_image_north_up() {
    let mut host = super::MapHost::new();
    host.set_size(256, 256, 1.0);
    host.fit_world_camera();
    let rect = tile_world_rect(1, 0, 0);
    let nw = super::map_viewport::world_to_screen(&host.camera, &host.viewport, super::Point::new(rect.x0(), rect.y1()));
    let sw = super::map_viewport::world_to_screen(&host.camera, &host.viewport, super::Point::new(rect.x0(), rect.y0()));
    assert!(nw.y < sw.y, "north edge should be above south edge on screen");
    let aff = host.tile_raster_affine(rect, 256, 256);
    let coeffs = aff.as_coeffs();
    assert!(coeffs[0] > 0.0, "east should map to increasing screen x");
    assert!(coeffs[3] > 0.0, "image rows should map downward on screen");
}

#[test]
fn sync_map_json_keeps_overlay_captions() {
    let mut host = overlay_host();
    host.sync_map_json(ZURICH_PIN).expect("descriptor");
    assert_eq!(host.overlay_feature("pins", "zurich").and_then(|feature| feature.label.as_deref()), Some("Zürich"));
}

#[test]
fn sync_map_json_parses_rich_overlay_metadata() {
    let mut host = overlay_host();
    host.sync_map_json(r#"{"overlays":{"pins":[{"id":"site-1","coordinates":[[8.54,47.37]],"label":"Site","style":"accent","icon":"package","link":"https://example.test/site"}]}}"#).expect("descriptor");
    let feature = host.overlay_feature("pins", "site-1").expect("feature");
    assert_eq!(feature.style.as_deref(), Some("accent"));
    assert_eq!(feature.icon.as_deref(), Some("package"));
    assert_eq!(feature.link.as_deref(), Some("https://example.test/site"));
    host.sync_map_json(r#"{"overlays":{}}"#).expect("empty descriptor");
    assert!(host.overlay_feature("pins", "site-1").is_none(), "a sync replaces the overlay tables wholesale");
}

#[test]
fn feature_screen_json_projects_a_known_point() {
    let mut host = overlay_host();
    host.fit_world_camera();
    host.sync_map_json(ZURICH_PIN).expect("descriptor");
    let raw: serde_json::Value = serde_json::from_str(&host.feature_screen_json("pin", "zurich")).expect("json");
    assert!(raw.get("x").and_then(|v| v.as_f64()).is_some());
    assert!(raw.get("y").and_then(|v| v.as_f64()).is_some());
    assert_eq!(host.feature_screen_json("pin", "missing"), "null");
}

#[test]
fn hit_test_feature_prefers_the_topmost_closest_target() {
    let mut host = overlay_host();
    host.fit_world_camera();
    host.sync_map_json(ZURICH_PIN_AND_PATH).expect("descriptor");
    let screen: serde_json::Value = serde_json::from_str(&host.feature_screen_json("pin", "zurich")).expect("json");
    let sx = screen["x"].as_f64().expect("x");
    let sy = screen["y"].as_f64().expect("y");
    let hit: serde_json::Value = serde_json::from_str(&host.hit_test_feature_json(sx, sy)).expect("hit");
    assert_eq!(hit["kind"], "pin", "the path starts under the pin too; the later-declared layer wins the tie");
    assert_eq!(hit["id"], "zurich");
    assert_eq!(host.hit_test_feature_json(-1_000.0, -1_000.0), "null");
}

#[test]
fn features_in_rect_crossing_includes_intersecting_lines() {
    let mut host = overlay_host();
    host.fit_world_camera();
    host.sync_map_json(ZURICH_PIN_AND_PATH).expect("descriptor");
    let raw: serde_json::Value = serde_json::from_str(&host.features_in_rect_json(0.0, 0.0, 800.0, 600.0, true)).expect("json");
    assert!(raw["pins"].as_array().is_some_and(|rows| !rows.is_empty()));
    assert!(raw["paths"].as_array().is_some_and(|rows| !rows.is_empty()));
    assert!(raw.get("zones").is_none(), "a layer without a granularity is not pickable");
}

#[test]
fn sync_interaction_updates_host_state_scoped_to_active_granularity() {
    let mut host = overlay_host();
    host.sync_interaction("pin", &["a".to_string()], Some("a"));
    assert_eq!(host.selected_ids("pin").collect::<Vec<_>>(), ["a"]);
    assert_eq!(host.selected_ids("path").count(), 0);
    assert_eq!(host.hovered_kind(), Some("pin"));
    assert_eq!(host.hovered_id(), Some("a"));

    host.sync_interaction("path", &["b".to_string()], None);
    assert_eq!(host.selected_ids("pin").count(), 0, "switching granularity clears the other kind's ids");
    assert_eq!(host.selected_ids("path").collect::<Vec<_>>(), ["b"]);
    assert!(host.hovered_kind().is_none());
    assert!(host.hovered_id().is_none());
}

#[test]
fn pointer_up_emits_camera_after_middle_button_pan() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.pointer_down_screen(100.0, 100.0, 1);
    host.pointer_move_screen(180.0, 140.0);
    host.pointer_up_screen(180.0, 140.0);
    let events: Vec<serde_json::Value> = serde_json::from_str(&host.drain_events_json()).expect("events");
    assert!(events.iter().any(|row| row["type"] == "camera"));
}

#[test]
fn overlay_layers_paint_behind_their_toggles() {
    let mut host = overlay_host();
    host.fit_world_camera();
    host.sync_map_json(ZURICH_PIN_AND_PATH).expect("descriptor");
    let all = host.build_vector_scene().path_count();
    assert!(all > 0);
    host.set_layer_visibility_from_json(r#"{"pinLabels":false}"#).expect("captions off");
    let without_captions = host.build_vector_scene().path_count();
    assert!(without_captions < all, "the pin caption is its own toggle target");
    host.set_layer_visibility_from_json(r#"{"pins":false,"paths":false}"#).expect("overlays off");
    assert_eq!(host.build_vector_scene().path_count(), 0);
    let mut neutral = super::MapHost::new();
    neutral.set_size(800, 600, 1.0);
    neutral.fit_world_camera();
    neutral.sync_map_json(ZURICH_PIN_AND_PATH).expect("descriptor");
    assert_eq!(neutral.build_vector_scene().path_count(), 0, "the neutral policy declares no overlay layer");
}

#[test]
fn set_map_theme_from_json_updates_surface_clear() {
    let mut host = super::MapHost::new();
    let json = r#"{"surfaceClear":[1,2,3,255]}"#;
    host.set_map_theme_from_json(json).expect("theme json");
    assert_eq!(host.theme.surface_clear.to_rgba8(), super::Color::from_rgba8(1, 2, 3, 255).to_rgba8());
}

#[test]
fn set_map_theme_from_json_zeros_land_stroke_alpha() {
    let mut host = super::MapHost::new();
    host.set_map_theme_from_json(r#"{"landStroke":[51,64,65,107]}"#).expect("theme json");
    assert_eq!(host.theme.land_stroke.to_rgba8().a, 0);
}
