//! 🗺️ Tiled map on the infinite canvas: Web Mercator raster and vector tiles plus policy-declared point, line and area overlays.
//!
//! 🔌️ Domain-neutral host. Level-of-detail bands, the tile pyramid, vector-tile paint roles and label rules come from the [`MapPolicy`] of the host's [`MapExtension`] (`semio-framework-surface-tiled-map-model`), installed with [`MapHost::with_extension`] or shipped as the `policy` member of the map descriptor JSON; without one the host renders by [`MapPolicyDescriptor::neutral`].
//!
//! 🧭️ Doctrine classification (ticket `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`,
//! W3b): [`MapHost`] owns **no tier-(a) authoritative state**. `overlays` mirrors the overlay features
//! of the document that owns them, refreshed wholesale via [`MapHost::sync_map_json`] by the plugin that
//! renders the map ("mirrored from projection JSON — not authoritative document state (OS `ArtifactStore`
//! owns packs)"). `render_mode`/`vector_style`/`forced_lod_id`/`layer_switches`
//! mirror that plugin's window configuration commands. No `🧬️mutations` vocabulary is authored here.

pub use canvas::{self, *};
pub use std::sync::Arc;

use std::cell::RefCell;

// #region 🔖️Model
use tiled_map_model::{
    clamp_camera_to_world_bounds, clamp_map_zoom, clamp_map_zoom_for_viewport, fnv1a_bytes, fnv1a_mix, ideal_tile_z_for_viewport, map_view_facts, map_viewport, map_wheel_screen, tile_range_revision, viewport_lon_span_degrees, MapTileLedger,
    REVISION_FNV_OFFSET,
};
pub use tiled_map_model::{
    clamp_map_layer_weight, map_camera_limits_json, map_camera_limits_json_for_viewport, map_layer_weight_slider_ids_json, map_lod_scale_json, projection, tiles, vector_tiles_available_at_camera_zoom, DefaultMapExtension, DescribedMapPolicy,
    MapClassRank, MapClassRanks, MapClassSpan, MapClassSpanRange, MapClassSpans, MapClassWidth, MapClassWidths, MapDampingCurve, MapExtension, MapFeatureFacts, MapFeatureKeys, MapLabelRules, MapLayerGates, MapLayerRole, MapLayerSwitches,
    MapLayerTarget, MapLayerToggle, MapLevelWidth, MapLevelWidths, MapLineDamping, MapLineScale, MapLodBand, MapOverlayFeature, MapOverlayGeometry, MapOverlayLayer, MapOverlayStyleFill, MapOverlayTables, MapPaintTechnique, MapPalette,
    MapPaletteSlot, MapPolicy, MapPolicyDescriptor, MapSourceLayer, MapSpanZoom, MapStylePreset, MapSyncJson, MapTileMode, MapViewFacts, MapWidth, MAP_CAMERA_ZOOM_MAX, MAP_CAMERA_ZOOM_MIN, MAP_EXTENSION_DEFAULT_ID, MAP_LAYER_WEIGHT_MAX,
    MAP_LAYER_WEIGHT_MIN, MAP_LOD_MODE_AUTOMATIC, MAP_POLICY_NEUTRAL_ID, MAP_STYLE_PRESET_NEUTRAL_ID, MAX_VISIBLE_TILE_REQUESTS,
};
// #endregion 🔖️Model

// #region 🔖️HostHelpers
/// 🌫️ `color` with its alpha replaced.
fn color_with_alpha(color: Color, alpha: u8) -> Color {
    let rgba = color.to_rgba8();
    Color::from_rgba8(rgba.r, rgba.g, rgba.b, alpha)
}

/// 🪣️ Opaque land/water base fill; the weight scales RGB only so tile composites do not seam.
fn weighted_opaque_fill(color: Color, weight: f64) -> Color {
    let w = clamp_map_layer_weight(weight).clamp(0.25, 1.0);
    let rgba = color.to_rgba8();
    let scale = |c: u8| (f64::from(c) * w).round() as u8;
    Color::from_rgba8(scale(rgba.r), scale(rgba.g), scale(rgba.b), 255)
}

const MAX_MAP_TILE_CACHE_ENTRIES: usize = 512;
// #endregion 🔖️HostHelpers

// #region 🔖️Children
#[path = "🏛️map-host/🦀️.rs"]
mod map_host;
#[path = "🧱️vector-tiles/🦀️.rs"]
pub mod vector_tiles;
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
#[path = "🎛️map-policy/🧪️tests/🏛️host/🦀️.rs"]
mod map_policy_host_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
