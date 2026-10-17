/// 🎨️ Render scene and vector scene cache: the device pixel ratio scene, the render mode, the vector style preset and the cache hits and invalidations of the vector scene.

#[test]
fn build_render_scene_scales_for_device_pixel_ratio() {
    let mut host = super::MapHost::new();
    host.set_size(400, 300, 2.0);
    host.fit_world_camera();
    let logical = host.build_vector_scene();
    let scaled = host.build_render_scene();
    assert!(scaled.path_count() >= logical.path_count());
}

#[test]
fn build_vector_scene_respects_render_mode() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_render_mode("vector");
    let _ = host.build_vector_scene();
    host.set_render_mode("image");
    let _ = host.build_vector_scene();
}

#[test]
fn vector_style_resolves_to_a_policy_preset() {
    let mut host = super::MapHost::with_extension(land_cover_extension());
    host.set_size(800, 600, 1.0);
    host.set_render_mode("vector");
    host.set_vector_style("silhouette");
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_style_str(), "silhouette");
    host.set_vector_style("unknown");
    assert_eq!(host.vector_style_str(), "plain", "an undeclared preset paints the first one");
    assert_eq!(super::MapHost::new().vector_style_str(), super::MAP_STYLE_PRESET_NEUTRAL_ID);
}

/// 🎣️ Seeds a vector host whose camera sits *off* a tile boundary. Straight from
/// `fit_world_camera` the viewport edge lands exactly on one, where `visible_tile_cursor`'s
/// `ceil - 1` correctly excludes the zero-overlap tile — so any nudge in either direction pulls a
/// whole extra row in (measured live: 48 → 56 tiles for a 0.2 px pan) and legitimately invalidates
/// the cache. A sub-tile pan only stays within one visible set away from that boundary.
fn vector_cache_host_off_tile_boundary() -> super::MapHost {
    let mut host = super::MapHost::with_extension(land_cover_extension());
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    host.camera.y -= 0.017;
    host
}

#[test]
fn vector_scene_cache_hit_reuses_translated_scene_on_pure_pan() {
    let mut host = vector_cache_host_off_tile_boundary();
    let first = host.build_vector_scene();
    assert!(!first.is_empty());
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    let anchor_x = host.camera.x;
    let anchor_y = host.camera.y;
    host.camera.x = anchor_x;
    host.camera.y = anchor_y - 0.002;
    let panned = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1, "a sub-tile pan must reuse the cache, not rebuild");
    assert_eq!(panned.path_count(), first.path_count());

    let mut rebuilt = vector_cache_host_off_tile_boundary();
    rebuilt.camera.x = anchor_x;
    rebuilt.camera.y = anchor_y - 0.002;
    let forced = rebuilt.build_vector_scene();
    assert_eq!(rebuilt.vector_scene_rebuild_count.get(), 1);
    assert_eq!(panned.path_count(), forced.path_count(), "cached-translate pan must match a forced rebuild at the same camera");
}

#[test]
fn vector_scene_cache_rebuilds_when_a_pan_changes_the_visible_tile_set() {
    let mut host = vector_cache_host_off_tile_boundary();
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);
    let before = host.visible_vector_tiles_revision();
    host.camera.y += 0.017;
    assert_ne!(host.visible_vector_tiles_revision(), before, "this pan must cross a tile boundary for the test to mean anything");
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a pan that changes the visible tile set must rebuild, never translate a stale set");
}

#[test]
fn vector_scene_cache_invalidates_on_zoom_change() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.camera.zoom *= 1.001;
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a zoom change must not take the translate path");
}

#[test]
fn vector_scene_cache_invalidates_on_viewport_resize() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.set_size(801, 600, 1.0);
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a viewport resize must invalidate the cache");
}

#[test]
fn vector_scene_cache_invalidates_on_new_tile_upload() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.upload_vector_tile(1, 0, 0, &[]).expect("upload");
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a newly uploaded vector tile must invalidate the cache");
}

#[test]
fn vector_scene_cache_invalidates_on_vector_style_change() {
    let mut host = super::MapHost::with_extension(land_cover_extension());
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.set_render_mode("vector");
    host.tiles.vector_tiles.insert("0/0/0".to_string(), synthetic_land_tile());
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.set_vector_style("silhouette");
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a vector-style change must invalidate the cache");
}

#[test]
fn vector_scene_cache_invalidates_on_policy_change() {
    let mut host = vector_cache_host_off_tile_boundary();
    let styled = host.build_vector_scene();
    assert!(!styled.is_empty(), "the land-cover role paints the synthetic tile");
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);
    host.set_extension(super::Arc::new(super::DefaultMapExtension::default()));
    let neutral = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 2, "a replaced policy must never reuse a scene built by the previous one");
    assert!(neutral.path_count() < styled.path_count(), "the neutral policy gives the landcover layer no role");
}

/// 🚫️ The vector-tile layer must NOT key on interaction. Selection and hover are painted by the
/// overlay layers, which are rebuilt every frame outside this cache, so keying on
/// `interaction_revision` bought nothing — and because `pointer_move_screen`/`set_camera` bump that
/// revision on every frame of a pan, it silently invalidated the cache once per frame and pinned a
/// drag at the uncached cost (measured live: 35 ms/frame with it, 2.7 ms without).
#[test]
fn vector_scene_cache_survives_interaction_change_because_selection_draws_outside_it() {
    let mut host = vector_cache_host_off_tile_boundary();
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);

    host.sync_interaction("pin", &["a".to_string()], Some("a"));
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1, "selection/hover must not rebuild the vector-tile cache");
}

/// 🖐️ The whole point of the cache: a drag must reuse it on every frame that does not cross a tile
/// boundary, even though each `pointer_move_screen` bumps `interaction_revision`.
#[test]
fn vector_scene_cache_survives_a_drag_that_stays_within_one_tile_set() {
    let mut host = vector_cache_host_off_tile_boundary();
    let _ = host.build_vector_scene();
    assert_eq!(host.vector_scene_rebuild_count.get(), 1);
    let revision = host.visible_vector_tiles_revision();

    host.pointer_down_screen(400.0, 300.0, 1);
    for step in 1..=8 {
        host.pointer_move_screen(400.0, 300.0 + f64::from(step));
        let _ = host.build_vector_scene();
    }
    host.pointer_up_screen(400.0, 308.0);

    assert_eq!(host.visible_vector_tiles_revision(), revision, "this drag must stay inside one tile set for the test to mean anything");
    assert_eq!(host.vector_scene_rebuild_count.get(), 1, "a drag within one tile set must never rebuild the vector scene");
}
