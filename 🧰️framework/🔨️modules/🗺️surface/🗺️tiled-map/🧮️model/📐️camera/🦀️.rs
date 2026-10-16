//! 📐️ Camera maths of the tiled map: zoom bounds and clamps, the visible longitude span, the ideal tile z, the tile-revision hash and the flipped-y viewport transform.
//!
//! Pure functions over `canvas::camera`; nothing here holds host state or knows a domain.

use super::{projection, tiles, MapPolicy, MapViewFacts};
use canvas::{camera, Point};

// #region 🔖️MapLod
/// 🧮️ The most tiles one visible-tile enumeration may request — the budget `pick_raster_tile_zoom` and
/// `pick_vector_tile_zoom` keep every viewport under.
pub const MAX_VISIBLE_TILE_REQUESTS: usize = 256;

pub const REVISION_FNV_OFFSET: u64 = 0xcbf29ce484222325;
const REVISION_FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// 🔢️ Deterministic FNV-1a mixer for the allocation-free tile-visibility revision signal
/// (no external hasher crate, no `DefaultHasher` — value must be stable within and across runs).
pub fn fnv1a_mix(hash: u64, value: u64) -> u64 {
    let mut h = hash;
    for b in value.to_le_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(REVISION_FNV_PRIME);
    }
    h
}

/// 🔢️ Folds a byte string into an FNV-1a chain, for hashing the rare `Option<String>` cache-key input.
pub fn fnv1a_bytes(hash: u64, bytes: &[u8]) -> u64 {
    let mut h = hash;
    for b in bytes {
        h = fnv1a_mix(h, *b as u64);
    }
    h
}

/// 🔭️ Hashes a tile cursor's range fields (z + inclusive x/y bounds) into a revision that changes
/// if and only if the visible tile set changes, with no allocation.
pub fn tile_range_revision(cursor: &tiles::VisibleTileCursor) -> u64 {
    let (x_min, x_max, y_min, y_max) = cursor.bounds();
    let mut h = REVISION_FNV_OFFSET;
    h = fnv1a_mix(h, cursor.z() as u64);
    h = fnv1a_mix(h, x_min as u64);
    h = fnv1a_mix(h, x_max as u64);
    h = fnv1a_mix(h, y_min as u64);
    h = fnv1a_mix(h, y_max as u64);
    h
}

pub fn viewport_lon_span_degrees(camera: &camera::Camera, viewport: &camera::Viewport) -> f64 {
    let cy = viewport.height as f64 * 0.5;
    let left = map_viewport::screen_to_world(camera, viewport, Point::new(0.0, cy));
    let right = map_viewport::screen_to_world(camera, viewport, Point::new(viewport.width as f64, cy));
    let (lon0, _) = projection::world_to_lonlat(left.x, left.y);
    let (lon1, _) = projection::world_to_lonlat(right.x, right.y);
    let span = (lon1 - lon0).abs();
    if !span.is_finite() {
        return 360.0;
    }
    span.min(360.0)
}

pub fn ideal_tile_z_for_viewport(camera: &camera::Camera, viewport: &camera::Viewport, raster_tile_z_max: u32) -> u32 {
    let span = viewport_lon_span_degrees(camera, viewport).max(1e-6);
    let w = viewport.width.max(1) as f64;
    let z = ((360.0 / span) * (w / 256.0)).log2();
    z.round().clamp(0.0, f64::from(raster_tile_z_max)) as u32
}

/// 👁️ The viewport facts a [`MapPolicy`] tile-z decision reads, with the ideal tile z bounded by the policy's raster pyramid.
pub fn map_view_facts(camera: &camera::Camera, viewport: &camera::Viewport, policy: &dyn MapPolicy) -> MapViewFacts {
    MapViewFacts { span_deg: viewport_lon_span_degrees(camera, viewport), ideal_tile_z: ideal_tile_z_for_viewport(camera, viewport, policy.descriptor().raster_tile_z_max) }
}

/// 🛰️ Vector tiles are requested at every positive camera zoom; the tile z itself comes from the policy.
pub fn vector_tiles_available_at_camera_zoom(camera_zoom: f64) -> bool {
    camera_zoom > 0.0
}

/// 🔭️ Whole-world fit on an ~800px viewport is ~300; min allows shrinking the planet to a few pixels.
pub const MAP_CAMERA_ZOOM_MIN: f64 = 8.0;
/// 🔭️ ~100M yields ~1.4e-3° longitude across 800px (~150 m), i.e. street scale (see `viewport_lon_span_degrees`).
pub const MAP_CAMERA_ZOOM_MAX: f64 = 100_000_000.0;

pub fn clamp_map_zoom(zoom: f64) -> f64 {
    zoom.clamp(MAP_CAMERA_ZOOM_MIN, MAP_CAMERA_ZOOM_MAX)
}

pub fn clamp_map_zoom_for_viewport(zoom: f64, viewport: &camera::Viewport) -> f64 {
    zoom.max(projection::cover_zoom_for_viewport(viewport)).min(MAP_CAMERA_ZOOM_MAX)
}

/// 🔒️ Camera zoom limits for the default viewport as `{ "min", "max" }` JSON.
pub fn map_camera_limits_json() -> String {
    map_camera_limits_json_for_viewport(&camera::Viewport::default())
}

/// 📐️ Camera zoom limits for `viewport`: the cover zoom (never below [`MAP_CAMERA_ZOOM_MIN`]) up to [`MAP_CAMERA_ZOOM_MAX`].
pub fn map_camera_limits_json_for_viewport(viewport: &camera::Viewport) -> String {
    serde_json::json!({
        "min": projection::cover_zoom_for_viewport(viewport).max(MAP_CAMERA_ZOOM_MIN),
        "max": MAP_CAMERA_ZOOM_MAX,
    })
    .to_string()
}

/// 🧷️ Keeps the viewport filled by the world map with no empty margins or outscroll.
pub fn clamp_camera_to_world_bounds(camera: &mut camera::Camera, viewport: &camera::Viewport) {
    camera.zoom = clamp_map_zoom_for_viewport(camera.zoom, viewport);
    let half_w = viewport.width as f64 / (2.0 * camera.zoom);
    let half_h = viewport.height as f64 / (2.0 * camera.zoom);
    let lim_x = projection::WORLD_HALF - half_w;
    let lim_y = projection::WORLD_HALF - half_h;
    camera.x = if lim_x <= 0.0 { 0.0 } else { camera.x.clamp(-lim_x, lim_x) };
    camera.y = if lim_y <= 0.0 { 0.0 } else { camera.y.clamp(-lim_y, lim_y) };
}

pub mod map_viewport {
    use canvas::camera::{screen_to_world as canvas_screen_to_world, world_to_screen as canvas_world_to_screen, Camera, Viewport};
    use canvas::Point;

    pub fn world_to_screen(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        canvas_world_to_screen(camera, viewport, Point::new(p.x, -p.y))
    }

    pub fn screen_to_world(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        let w = canvas_screen_to_world(camera, viewport, p);
        Point::new(w.x, -w.y)
    }
}

pub fn map_wheel_screen(camera: &mut camera::Camera, viewport: &camera::Viewport, sx: f64, sy: f64, delta_y: f64) {
    let zoom_factor = if delta_y < 0.0 {
        1.12
    } else if delta_y > 0.0 {
        1.0 / 1.12
    } else {
        1.0
    };
    let next_zoom = clamp_map_zoom(camera.zoom * zoom_factor);
    let screen = Point::new(sx, sy);
    let world_before = map_viewport::screen_to_world(camera, viewport, screen);
    let half_w = viewport.width as f64 / 2.0;
    let half_h = viewport.height as f64 / 2.0;
    camera.x = world_before.x - (sx - half_w) / next_zoom;
    camera.y = -world_before.y - (sy - half_h) / next_zoom;
    camera.zoom = next_zoom;
}
// #endregion 🔖️MapLod
