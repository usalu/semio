//! 🗺️ GIS map on the infinite canvas: Web Mercator tiles, positions, routes, regions.
//!
//! 🧭️ Doctrine classification (ticket `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`,
//! W3b): [`MapHost`] owns **no tier-(a) authoritative state**, traced field-by-field rather than
//! assumed from the struct's setter shape (full per-field table in `📓️wave3b-reports/surface-report.md`).
//! `features` is the field the wave's own recon flagged as real tier-(a) content — and it is real, but
//! its authoritative owner is `✏️s/🔌️plugins/🌍️gis`'s shipped `GismapSnapshot`
//! (`crate::artifacts::gismap::schema::{snapshot,diff,mutations}`, 12 real triads —
//! `create/delete/reorder-positions`, `replace-position-data`, `create/delete/reorder-routes`,
//! `replace-route-data`, `create/delete/reorder-regions`, `replace-region-data`), refreshed wholesale
//! via [`MapHost::sync_map_json`] by `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻️2d/🗺️maphost/🦀️.rs::map_host_from`
//! — the field's own pre-existing docstring already said as much ("mirrored from projection JSON —
//! not authoritative document state (OS `ArtifactStore` owns packs)"), confirmed rather than merely
//! trusted. `render_mode`/`vector_style`/`forced_lod_id`/`layer_visibility`/`layer_stroke_scale`
//! similarly mirror the plugin app's already-shipped `Gis2dConfig` commands (`SetRenderMode`,
//! `SetVectorStyle`, `ToggleLayerVisibility`, `SetLayerStrokeScale`, `set_lod_mode`). No new
//! `🧬️mutations` vocabulary is authored here — the owners already have it.

pub use canvas::{self, *};
pub use std::sync::Arc;

use std::cell::RefCell;
// 🌱️ `ToValue`/`FromValue` here is the first-party analog of `Serialize`/`Deserialize`, for ticket
// 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;

// #region 🔖️MapPalette
fn map_color(rgba: [f32; 4]) -> Color {
    Color::new(rgba)
}

#[derive(Clone, Copy, Debug)]
pub struct MapPalette {
    pub surface_clear: Color,
    pub land_fill: Color,
    pub land_stroke: Color,
    pub label_fill: Color,
    pub label_halo: Color,
    pub region_fill: Color,
    pub region_stroke: Color,
    pub route_stroke: Color,
    pub position_fill: Color,
    pub position_stroke: Color,
    pub selection_stroke: Color,
    pub hover_stroke: Color,
}

impl MapPalette {
    /// 🎨️ Builds a map palette from centralized theme tokens.
    pub fn from_map_palette(t: &ui_styling::MapPalette) -> Self {
        Self {
            surface_clear: map_color(t.surface_clear),
            land_fill: map_color(t.land_fill),
            land_stroke: map_color(t.land_stroke),
            label_fill: map_color(t.label_fill),
            label_halo: map_color(t.label_halo),
            region_fill: map_color(t.region_fill),
            region_stroke: map_color(t.region_stroke),
            route_stroke: map_color(t.route_stroke),
            position_fill: map_color(t.position_fill),
            position_stroke: map_color(t.position_stroke),
            selection_stroke: map_color(t.position_fill),
            hover_stroke: map_color(t.route_stroke),
        }
    }
}

impl Default for MapPalette {
    fn default() -> Self {
        Self::from_map_palette(&ui_styling::MAP_LIGHT)
    }
}
// #endregion 🔖️MapPalette

// #region 🔖️MapLod
/// 🔭️ OSM raster tiles for automatic viewport picking.
pub const MAP_RASTER_TILE_Z_MAX: u32 = 19;

const MAX_MAP_TILE_CACHE_ENTRIES: usize = 512;

/// 🧮️ The most tiles one visible-tile enumeration may request — the budget `pick_raster_tile_zoom` and
/// `pick_vector_tile_zoom` keep every viewport under.
pub const MAX_VISIBLE_TILE_REQUESTS: usize = 256;

const REVISION_FNV_OFFSET: u64 = 0xcbf29ce484222325;
const REVISION_FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// 🔢️ Deterministic FNV-1a mixer for the allocation-free tile-visibility revision signal
/// (no external hasher crate, no `DefaultHasher` — value must be stable within and across runs).
fn fnv1a_mix(hash: u64, value: u64) -> u64 {
    let mut h = hash;
    for b in value.to_le_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(REVISION_FNV_PRIME);
    }
    h
}

/// 🔢️ Folds a byte string into an FNV-1a chain, for hashing the rare `Option<String>` cache-key input.
fn fnv1a_bytes(hash: u64, bytes: &[u8]) -> u64 {
    let mut h = hash;
    for b in bytes {
        h = fnv1a_mix(h, *b as u64);
    }
    h
}

/// 🔭️ Hashes a tile cursor's range fields (z + inclusive x/y bounds) into a revision that changes
/// if and only if the visible tile set changes, with no allocation.
fn tile_range_revision(cursor: &tiles::VisibleTileCursor) -> u64 {
    let (x_min, x_max, y_min, y_max) = cursor.bounds();
    let mut h = REVISION_FNV_OFFSET;
    h = fnv1a_mix(h, cursor.z() as u64);
    h = fnv1a_mix(h, x_min as u64);
    h = fnv1a_mix(h, x_max as u64);
    h = fnv1a_mix(h, y_min as u64);
    h = fnv1a_mix(h, y_max as u64);
    h
}

fn viewport_lon_span_degrees(camera: &camera::Camera, viewport: &camera::Viewport) -> f64 {
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

fn ideal_tile_z_for_viewport(camera: &camera::Camera, viewport: &camera::Viewport) -> u32 {
    let span = viewport_lon_span_degrees(camera, viewport).max(1e-6);
    let w = viewport.width.max(1) as f64;
    let z = ((360.0 / span) * (w / 256.0)).log2();
    z.round().clamp(0.0, f64::from(MAP_RASTER_TILE_Z_MAX)) as u32
}

/// 🔭️ Whole-world fit on an ~800px viewport is ~300; min allows shrinking the planet to a few pixels.
pub const MAP_CAMERA_ZOOM_MIN: f64 = 8.0;
/// 🔭️ ~100M yields ~1.4e-3° longitude across 800px (~150 m), i.e. street scale (see `viewport_lon_span_degrees`).
pub const MAP_CAMERA_ZOOM_MAX: f64 = 100_000_000.0;

fn clamp_map_zoom(zoom: f64) -> f64 {
    zoom.clamp(MAP_CAMERA_ZOOM_MIN, MAP_CAMERA_ZOOM_MAX)
}

fn clamp_map_zoom_for_viewport(zoom: f64, viewport: &camera::Viewport) -> f64 {
    zoom.max(projection::cover_zoom_for_viewport(viewport)).min(MAP_CAMERA_ZOOM_MAX)
}

/// 🧷️ Keeps the viewport filled by the world map with no empty margins or outscroll.
fn clamp_camera_to_world_bounds(camera: &mut camera::Camera, viewport: &camera::Viewport) {
    camera.zoom = clamp_map_zoom_for_viewport(camera.zoom, viewport);
    let half_w = viewport.width as f64 / (2.0 * camera.zoom);
    let half_h = viewport.height as f64 / (2.0 * camera.zoom);
    let lim_x = projection::WORLD_HALF - half_w;
    let lim_y = projection::WORLD_HALF - half_h;
    camera.x = if lim_x <= 0.0 { 0.0 } else { camera.x.clamp(-lim_x, lim_x) };
    camera.y = if lim_y <= 0.0 { 0.0 } else { camera.y.clamp(-lim_y, lim_y) };
}

mod map_viewport {
    use super::canvas::camera::{screen_to_world as canvas_screen_to_world, world_to_screen as canvas_world_to_screen, Camera, Viewport};
    use super::Point;

    pub fn world_to_screen(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        canvas_world_to_screen(camera, viewport, Point::new(p.x, -p.y))
    }

    pub fn screen_to_world(camera: &Camera, viewport: &Viewport, p: Point) -> Point {
        let w = canvas_screen_to_world(camera, viewport, p);
        Point::new(w.x, -w.y)
    }
}

fn map_wheel_screen(camera: &mut camera::Camera, viewport: &camera::Viewport, sx: f64, sy: f64, delta_y: f64) {
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

// #region 🔖️Children
#[path = "🧭️projection/🦀️.rs"]
pub mod projection;
#[path = "🧩️tiles/🦀️.rs"]
pub mod tiles;
#[path = "🧱️vector-tiles/🦀️.rs"]
pub mod vector_tiles;
#[path = "🌍️gis-policy/🦀️.rs"]
mod gis_policy;
pub use gis_policy::*;
#[path = "🪢️map-state/🦀️.rs"]
mod map_state;
pub use map_state::*;
#[path = "🏛️map-host/🦀️.rs"]
mod map_host;
pub use map_host::*;
#[path = "🖱️interaction-and-labels/🦀️.rs"]
mod interaction_and_labels;
pub use interaction_and_labels::*;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🌐️wasm-session/🦀️.rs"]
mod wasm_session;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use wasm_session::MapSession;
// #endregion 🔖️Children

// #region 🔖️MapContent
//#region ⚠️ Errors
/// ⚠️ Tiled-map host errors — JSON decode failures, raster tile decode failures, and vector-tile protobuf decode failures.
#[derive(Debug)]
pub enum FrameworkSurfaceTiledMapError {
    Json(serde_json::Error),
    Image(String),
    Mvt(vector_tiles::MvtDecodeError),
}

impl std::fmt::Display for FrameworkSurfaceTiledMapError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => error.fmt(formatter),
            Self::Image(message) => formatter.write_str(message),
            Self::Mvt(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for FrameworkSurfaceTiledMapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Mvt(error) => Some(error),
            Self::Image(_) => None,
        }
    }
}

impl From<serde_json::Error> for FrameworkSurfaceTiledMapError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<semio_framework_pixels::RasterError> for FrameworkSurfaceTiledMapError {
    fn from(error: semio_framework_pixels::RasterError) -> Self {
        Self::Image(error.to_string())
    }
}

impl From<vector_tiles::MvtDecodeError> for FrameworkSurfaceTiledMapError {
    fn from(error: vector_tiles::MvtDecodeError) -> Self {
        Self::Mvt(error)
    }
}
//#endregion ⚠️ Errors
// #endregion 🔖️MapContent

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
