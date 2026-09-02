//! 🗺️ Web-Mercator / slippy-tile oracle test for `tiled_map::projection` and `tiled_map::tiles`.
//!
//! Reads the SAME frozen fixture as the sibling Python oracle adapter
//! (`../../🗺️tiled-map/🧪️tests/web-mercator-tile-oracle/🧫️fixtures/🔣️.json`), whose `projection`,
//! `tileNumbering` and `tileBounds` arrays were computed independently by the `mercantile` third-party
//! library (pure Python, zero runtime deps) from the published EPSG:3857 / OSM slippy-tile spec. This
//! file asserts this repository's Rust implementation reproduces the same numbers, discharging
//! CLAUDE.md's "at least one language-agnostic test per feature, validated against a third-party
//! library" requirement for Web-Mercator projection and tile selection.
//!
//! `lodBands` are NOT checked here — no third-party reference exists for the repository-owned
//! `GIS_MAP_LOD_MAX_SPAN_DEG`/`GIS_MAP_LOD_TILE_Z` band scheme, and the functions that resolve them
//! (`active_map_lod`, `viewport_lon_span_degrees`) are crate-private, reachable only from
//! `tiled-map/🦀️.rs`'s own `mod tests` — see `.🧬semio/…/GIS-MAP-END-TO-END/📓️research/📝️map-math-oracle-tests.md` for the ready-to-apply diff covering those, plus the zoom/pan invariants
//! that need `MAX_VISIBLE_TILE_REQUESTS`.
//!
//! @see 🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🧪️tests/web-mercator-tile-oracle/🥒️.feature

use semio_framework_surface::tiled_map::canvas::camera::{screen_to_world, Camera, Viewport};
use semio_framework_surface::tiled_map::projection::{lonlat_to_world, tile_world_rect, world_to_lonlat, WORLD_HALF};
use semio_framework_surface::tiled_map::tiles::visible_tiles;
use semio_framework_surface::tiled_map::{MapHost, MAP_CAMERA_ZOOM_MIN};
use serde_json::Value;
use std::path::PathBuf;

const WORLD_TOL: f64 = 1e-9;
const DEG_TOL: f64 = 1e-6;

fn fixture() -> Value {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "..",
        "..",
        "🗺️tiled-map",
        "🧪️tests",
        "web-mercator-tile-oracle",
        "🧫️fixtures",
        "🔣️.json",
    ]
    .iter()
    .collect();
    let raw = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("fixture missing at {}: {e}", path.display()));
    serde_json::from_str(&raw).expect("fixture must be valid JSON")
}

fn f64_field(v: &Value, key: &str) -> f64 {
    v.get(key).and_then(Value::as_f64).unwrap_or_else(|| panic!("fixture entry missing numeric field {key}: {v}"))
}

fn u32_field(v: &Value, key: &str) -> u32 {
    v.get(key).and_then(Value::as_u64).unwrap_or_else(|| panic!("fixture entry missing integer field {key}: {v}")) as u32
}

#[test]
fn lonlat_to_world_matches_mercantile_epsg3857() {
    let fx = fixture();
    for entry in fx["projection"].as_array().expect("projection array") {
        let lon = f64_field(entry, "lon");
        let lat = f64_field(entry, "lat");
        let expected_x = f64_field(entry, "worldX");
        let expected_y = f64_field(entry, "worldY");
        let got = lonlat_to_world(lon, lat);
        let id = entry["id"].as_str().unwrap_or("?");
        assert!((got.x - expected_x).abs() < WORLD_TOL, "{id}: worldX got {} want {expected_x}", got.x);
        assert!((got.y - expected_y).abs() < WORLD_TOL, "{id}: worldY got {} want {expected_y}", got.y);
    }
}

#[test]
fn tile_numbering_matches_mercantile_xyz_scheme() {
    let fx = fixture();
    for entry in fx["tileNumbering"].as_array().expect("tileNumbering array") {
        let lon = f64_field(entry, "lon");
        let lat = f64_field(entry, "lat");
        let z = u32_field(entry, "z");
        let expected_x = u32_field(entry, "x");
        let expected_y = u32_field(entry, "y");
        let id = entry["id"].as_str().unwrap_or("?");

        // Point the map at the fixture's lon/lat with a sub-pixel viewport at an extreme zoom, so the
        // production visible-tile enumeration (tiles::visible_tiles, driven by the real
        // screen_to_world windowing) can only ever resolve the single tile that contains this point —
        // the same production code path MapHost uses to pick tiles for rendering.
        let world = lonlat_to_world(lon, lat);
        let camera = Camera { x: world.x, y: -world.y, zoom: 1e15 };
        let viewport = Viewport { width: 2, height: 2, dpr: 1.0 };
        let got = visible_tiles(&camera, &viewport, z);
        assert!(got.contains(&(z, expected_x, expected_y)), "{id}: mercantile's tile ({z},{expected_x},{expected_y}) missing from {got:?}");

        // A point sitting exactly on a tile edge is shared by the tiles on both sides of it, so a
        // viewport centred there genuinely overlaps 2 (edge) or 4 (corner) of them — `visible_tiles`
        // answers "which tiles does this rect touch", which cannot disambiguate a measure-zero point
        // the way mercantile's "which tile owns this point" can. Away from an edge the windowing must
        // still collapse to the single containing tile.
        let n = f64::from(1u32 << z);
        let step = (WORLD_HALF * 2.0) / n;
        let on_edge = |v: f64| (v - v.round()).abs() < 1e-9;
        let straddles_edge = on_edge((world.x + WORLD_HALF) / step) || on_edge((WORLD_HALF - world.y) / step);
        if straddles_edge {
            assert!(got.len() <= 4, "{id}: an edge point may touch at most 4 tiles, got {got:?}");
        } else {
            assert_eq!(got, vec![(z, expected_x, expected_y)], "{id}: interior point must resolve to exactly one tile at z={z}");
        }
    }
}

#[test]
fn tile_bounds_match_mercantile_bbox() {
    let fx = fixture();
    for entry in fx["tileBounds"].as_array().expect("tileBounds array") {
        let z = u32_field(entry, "z");
        let x = u32_field(entry, "x");
        let y = u32_field(entry, "y");
        let expected_west = f64_field(entry, "west");
        let expected_south = f64_field(entry, "south");
        let expected_east = f64_field(entry, "east");
        let expected_north = f64_field(entry, "north");
        let id = entry["id"].as_str().unwrap_or("?");

        let rect = tile_world_rect(z, x, y);
        let (west, south) = world_to_lonlat(rect.x0(), rect.y0());
        let (east, north) = world_to_lonlat(rect.x1(), rect.y1());
        assert!((west - expected_west).abs() < DEG_TOL, "{id}: west got {west} want {expected_west}");
        assert!((south - expected_south).abs() < DEG_TOL, "{id}: south got {south} want {expected_south}");
        assert!((east - expected_east).abs() < DEG_TOL, "{id}: east got {east} want {expected_east}");
        assert!((north - expected_north).abs() < DEG_TOL, "{id}: north got {north} want {expected_north}");
    }
}

#[test]
fn cursor_anchored_zoom_keeps_world_point_under_cursor_fixed() {
    let mut host = MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.02, -0.01, 5_000.0);
    let cursor = (300.0, 220.0);
    let before = screen_to_world(&host.camera, &host.viewport, semio_framework_surface::tiled_map::Point::new(cursor.0, cursor.1));
    host.wheel_screen(cursor.0, cursor.1, -100.0);
    let after = screen_to_world(&host.camera, &host.viewport, semio_framework_surface::tiled_map::Point::new(cursor.0, cursor.1));
    assert!((before.x - after.x).abs() < WORLD_TOL, "world x under cursor moved: {} -> {}", before.x, after.x);
    assert!((before.y - after.y).abs() < WORLD_TOL, "world y under cursor moved: {} -> {}", before.y, after.y);
}

#[test]
fn pan_by_delta_then_negative_delta_returns_exact_original_camera() {
    let mut host = MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 5_000.0);
    let origin = (host.camera.x, host.camera.y);
    let (sx, sy) = (400.0, 300.0);
    let (dx, dy) = (57.0, -33.0);

    host.pointer_down_screen(sx, sy, 1);
    host.pointer_move_screen(sx + dx, sy + dy);
    host.pointer_up_screen(sx + dx, sy + dy);
    assert!((host.camera.x - origin.0).abs() > 1e-6, "pan did not move the camera at all");

    host.pointer_down_screen(sx + dx, sy + dy, 1);
    host.pointer_move_screen(sx, sy);
    host.pointer_up_screen(sx, sy);

    assert!((host.camera.x - origin.0).abs() < WORLD_TOL, "camera.x did not round-trip: {} vs {}", host.camera.x, origin.0);
    assert!((host.camera.y - origin.1).abs() < WORLD_TOL, "camera.y did not round-trip: {} vs {}", host.camera.y, origin.1);
}

#[test]
fn zoom_in_then_out_by_one_wheel_step_returns_original_zoom() {
    let mut host = MapHost::new();
    host.set_size(800, 600, 1.0);
    host.set_camera(0.0, 0.0, 5_000.0);
    let original_zoom = host.camera.zoom;
    host.wheel_screen(400.0, 300.0, -100.0);
    assert!((host.camera.zoom - original_zoom).abs() > 1e-6, "zoom-in did not change zoom");
    host.wheel_screen(400.0, 300.0, 100.0);
    let relative_error = (host.camera.zoom - original_zoom).abs() / original_zoom;
    assert!(relative_error < 1e-9, "zoom did not round-trip: {} vs {} (rel err {relative_error})", host.camera.zoom, original_zoom);
}

#[test]
fn visible_tile_count_stays_within_the_request_budget() {
    // 256 mirrors the crate-private `MAX_VISIBLE_TILE_REQUESTS` at
    // 🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs:96 — not reachable from outside the
    // crate, so this bound is duplicated here and pinned again from inside `mod tests` in the diff.
    const EXPECTED_MAX_VISIBLE_TILE_REQUESTS: usize = 256;
    for zoom in [MAP_CAMERA_ZOOM_MIN, 5_000.0, 500_000.0, 100_000_000.0] {
        let mut host = MapHost::new();
        host.set_size(1920, 1080, 2.0);
        host.set_camera(0.0, 0.0, zoom);
        host.fit_world_camera();
        let raster_z = host.pick_raster_tile_zoom();
        let vector_z = host.pick_vector_tile_zoom();
        assert!(visible_tiles(&host.camera, &host.viewport, raster_z).len() <= EXPECTED_MAX_VISIBLE_TILE_REQUESTS);
        assert!(visible_tiles(&host.camera, &host.viewport, vector_z).len() <= EXPECTED_MAX_VISIBLE_TILE_REQUESTS);
    }
}
