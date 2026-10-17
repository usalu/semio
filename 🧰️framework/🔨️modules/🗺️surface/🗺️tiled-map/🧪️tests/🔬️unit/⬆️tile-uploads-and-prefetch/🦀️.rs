/// ⬆️ Tile uploads and prefetch: the raster and vector uploads and their idempotence, the visible tile revisions, the prefetch tiles and the pinned LRU eviction.

#[test]
fn upload_tile_invalid_png_bytes_returns_err() {
    let mut host = super::MapHost::new();
    assert!(host.upload_tile(0, 0, 0, b"not a png").is_err());
}

#[test]
fn upload_tile_png_decode_matches_image_crate_oracle_for_every_8_bit_color_type() {
    use image::codecs::png::PngEncoder;
    use image::{ColorType, ImageEncoder};
    let (width, height) = (19u32, 13u32);
    for (color, channels) in [(ColorType::Rgba8, 4usize), (ColorType::Rgb8, 3), (ColorType::L8, 1), (ColorType::La8, 2)] {
        let samples: Vec<u8> = (0..width as usize * height as usize * channels).map(|index| (index * 37 + index / 7 * 11) as u8).collect();
        let mut png = Vec::new();
        PngEncoder::new(&mut png).write_image(&samples, width, height, color.into()).expect("encode oracle png");
        let oracle = image::load_from_memory(&png).expect("oracle decode").to_rgba8();
        let owned = semio_framework_pixels::decode_png(&png).expect("owned decode");
        assert_eq!((owned.width, owned.height), oracle.dimensions(), "{color:?} extent");
        assert_eq!(owned.pixels, oracle.into_raw(), "{color:?} pixels");
        let mut host = super::MapHost::new();
        host.upload_tile(0, 0, 0, &png).expect("upload decodes through the owned codec");
        let stored = host.tiles.tile_images.get("0/0/0").expect("stored tile");
        assert_eq!((stored.width(), stored.height()), (width, height), "{color:?} stored extent");
    }
}

#[test]
fn upload_vector_tile_invalid_bytes_returns_err() {
    let mut host = super::MapHost::new();
    assert!(host.upload_vector_tile(0, 0, 0, &[0x80]).is_err());
}

#[test]
fn upload_tile_second_call_on_existing_key_is_idempotent_and_skips_decode() {
    let mut host = super::MapHost::new();
    host.upload_tile(0, 0, 0, &test_png_1x1()).expect("first upload decodes");
    let before = host.tiles.tile_images.get("0/0/0").cloned().expect("cached after first upload");
    let result = host.upload_tile(0, 0, 0, b"not a png");
    assert!(result.is_ok(), "re-uploading an already-cached key must short-circuit before decoding");
    let after = host.tiles.tile_images.get("0/0/0").cloned().expect("still cached after second upload");
    assert!(std::sync::Arc::ptr_eq(&before, &after), "cached image must be untouched by the skipped decode");
}

#[test]
fn upload_vector_tile_second_call_on_existing_key_is_idempotent_and_skips_decode() {
    let mut host = super::MapHost::new();
    host.upload_vector_tile(0, 0, 0, &[]).expect("first upload decodes");
    let result = host.upload_vector_tile(0, 0, 0, &[0x80]);
    assert!(result.is_ok(), "re-uploading an already-cached key must short-circuit before decoding");
    assert!(host.has_vector_tile("0/0/0"), "still cached after second upload");
}

#[test]
fn visible_tiles_revision_is_stable_then_changes_on_pan_and_zoom() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 6000.0);
    let r1 = host.visible_tiles_revision();
    let r2 = host.visible_tiles_revision();
    assert_eq!(r1, r2, "revision must be stable across calls with an unchanged camera");
    let z = host.pick_raster_tile_zoom();
    let tile_span = tile_world_rect(z, 0, 0).width();
    host.set_camera(tile_span * 4.0, 0.0, host.camera.zoom);
    let r3 = host.visible_tiles_revision();
    assert_ne!(r1, r3, "revision must change after a pan that moves the visible tile range");
    host.set_camera(0.0, 0.0, host.camera.zoom * 6.0);
    let r4 = host.visible_tiles_revision();
    assert_ne!(r1, r4, "revision must change after a zoom that changes the tile z");
}

#[test]
fn visible_vector_tiles_revision_matches_json_unavailable_sentinel() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.camera.zoom = 0.0;
    assert_eq!(host.visible_vector_tiles_json(), "[]");
    assert_eq!(host.visible_vector_tiles_revision(), 0, "unavailable sentinel must match the json [] case");
    host.camera.zoom = 6000.0;
    assert_ne!(host.visible_vector_tiles_json(), "[]");
}

#[test]
fn prefetch_tiles_are_disjoint_from_visible_share_z_and_respect_cap() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 6000.0);
    let visible: Vec<serde_json::Value> = serde_json::from_str(&host.visible_tiles_json()).expect("json");
    let prefetch: Vec<serde_json::Value> = serde_json::from_str(&host.prefetch_tiles_json()).expect("json");
    assert!(!prefetch.is_empty(), "expect a prefetch ring around a finite visible window");
    let visible_z = visible[0]["z"].as_u64().unwrap();
    let visible_keys: std::collections::BTreeSet<String> = visible.iter().map(|v| v["key"].as_str().unwrap().to_string()).collect();
    for row in &prefetch {
        let key = row["key"].as_str().unwrap();
        assert!(!visible_keys.contains(key), "prefetch row {key} must not duplicate a visible tile");
        assert_eq!(row["z"].as_u64().unwrap(), visible_z, "prefetch must share the visible z");
    }
    assert!(visible.len() + prefetch.len() <= MAX_VISIBLE_TILE_REQUESTS);
}

#[test]
fn prefetch_vector_tiles_json_is_empty_when_vector_tiles_unavailable() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.fit_world_camera();
    host.camera.zoom = 0.0;
    assert_eq!(host.prefetch_vector_tiles_json(), "[]");
}

#[test]
fn lru_eviction_never_evicts_a_pinned_visible_tile_and_takes_the_stalest_one_first() {
    let mut host = super::MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 6000.0);
    let vz = host.pick_raster_tile_zoom();
    assert!(vz >= 4, "expected a finer raster zoom for this camera (got {vz})");
    let visible_now = visible_tiles(&host.camera, &host.viewport, vz);
    let (_, vx, vy) = visible_now[0];
    let visible_key = tiles::tile_key(vz, vx, vy);

    let n = 1u32 << vz;
    let stale_x = (vx + n / 2) % n;
    let stale_key = tiles::tile_key(vz, stale_x, vy);

    host.upload_tile(vz, stale_x, vy, &test_png_1x1()).expect("seed stale tile");
    host.upload_tile(vz, vx, vy, &test_png_1x1()).expect("seed visible tile");
    host.prepare_visible_tiles();
    assert!(host.has_tile(&stale_key), "an off-screen tile must survive below the cache cap");
    assert!(host.has_tile(&visible_key));

    let filler_z = 999;
    for i in 0..(super::MAX_MAP_TILE_CACHE_ENTRIES as u32 + 8) {
        host.upload_tile(filler_z, i, 0, &test_png_1x1()).expect("filler upload");
    }
    host.prepare_visible_tiles();

    assert!(!host.has_tile(&stale_key), "least-recently-touched tile must be evicted first under cap pressure");
    assert!(host.has_tile(&visible_key), "a currently-visible (pinned) tile must never be evicted");
}
