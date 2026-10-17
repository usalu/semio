/// 🧭️ Interaction sync and focus: the selection replacement, the hover and kind, the declared granularity, the focus on points and lines and the tile presence.

#[test]
fn sync_interaction_replaces_previous_selection_within_active_granularity() {
    let mut host = overlay_host();
    host.sync_interaction("pin", &["p1".to_string(), "p2".to_string()], None);
    assert_eq!(host.selected_ids("pin").count(), 2);
    host.sync_interaction("path", &["r1".to_string()], None);
    assert_eq!(host.selected_ids("pin").count(), 0, "switching to path clears pins");
    assert_eq!(host.selected_ids("path").collect::<Vec<_>>(), ["r1"]);
    host.sync_interaction("path", &[], None);
    assert_eq!(host.selected_ids("path").count(), 0);
}

#[test]
fn sync_interaction_none_clears_hover_and_some_sets_kind_and_id() {
    let mut host = overlay_host();
    host.sync_interaction("pin", &[], Some("p1"));
    assert_eq!(host.hovered_kind(), Some("pin"));
    assert_eq!(host.hovered_id(), Some("p1"));
    host.sync_interaction("pin", &[], None);
    assert_eq!(host.hovered_kind(), None);
    assert_eq!(host.hovered_id(), None);
}

#[test]
fn sync_interaction_json_resolves_the_declared_granularity_like_the_react_twin() {
    let mut host = overlay_host();
    host.sync_interaction_json(r#"{"pins":["pin-1"],"paths":[]}"#, "null");
    assert_eq!(host.selected_ids("pin").collect::<Vec<_>>(), ["pin-1"]);
    host.sync_interaction_json(r#"{"pins":["pin-1"],"paths":["path-2"]}"#, r#"{"kind":"path","id":"path-2"}"#);
    assert_eq!((host.selected_ids("path").collect::<Vec<_>>(), host.selected_ids("pin").count()), (vec!["path-2"], 0), "the first declared pickable layer wins");
    assert_eq!((host.hovered_kind(), host.hovered_id()), (Some("path"), Some("path-2")));
    host.sync_interaction_json(r#"{"pins":[],"paths":[]}"#, r#"{"kind":"pin","id":"pin-1"}"#);
    assert_eq!((host.hovered_kind(), host.hovered_id(), host.selected_ids("pin").count()), (Some("pin"), Some("pin-1"), 0));
    host.sync_interaction_json(r#"{"zones":["zone-1"],"pins":["pin-1"]}"#, r#"{"kind":"path","id":"path-2"}"#);
    assert_eq!((host.selected_ids("pin").collect::<Vec<_>>(), host.hovered_id()), (vec!["pin-1"], None), "a non-pickable layer never selects and a foreign hover never lands");
    host.sync_interaction_json("not json", "null");
    assert_eq!((host.selected_ids("pin").count(), host.hovered_id()), (0, None));
}

#[test]
fn focus_feature_point_moves_camera_and_missing_id_returns_false() {
    let mut host = overlay_host();
    host.sync_map_json(r#"{"overlays":{"pins":[{"id":"p1","coordinates":[[10.0,20.0]]}]}}"#).expect("sync");
    assert!(!host.focus_feature("pin", "missing"));
    assert!(host.focus_feature("pin", "p1"));
    let expected = lonlat_to_world(10.0, 20.0);
    assert!((host.camera.x - expected.x).abs() < 1e-9);
    assert!((host.camera.y - expected.y).abs() < 1e-9);
}

#[test]
fn focus_feature_line_requires_at_least_two_points_and_fits_bounds() {
    let mut host = overlay_host();
    host.sync_map_json(r#"{"overlays":{"paths":[{"id":"short","coordinates":[[0.0,0.0]]},{"id":"r1","coordinates":[[0.0,0.0],[5.0,5.0]]}]}}"#).expect("sync");
    assert!(!host.focus_feature("path", "short"));
    assert!(host.focus_feature("path", "r1"));
    assert!(!host.focus_feature("path", "missing"));
}

#[test]
fn focus_feature_unknown_kind_returns_false() {
    let mut host = super::MapHost::new();
    assert!(!host.focus_feature("bogus", "anything"));
}

#[test]
fn has_tile_and_has_vector_tile_reflect_uploads() {
    let mut host = super::MapHost::new();
    assert!(!host.has_tile("0/0/0"));
    host.upload_tile(0, 0, 0, &test_png_1x1()).expect("upload png");
    assert!(host.has_tile("0/0/0"));
    assert!(!host.has_tile("1/0/0"));
    assert!(!host.has_vector_tile("0/0/0"));
}
