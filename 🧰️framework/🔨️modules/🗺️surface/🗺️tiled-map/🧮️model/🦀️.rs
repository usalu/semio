//! 🧮️ Tiled-map model: Web Mercator projection, tile addressing, camera maths, map descriptors and layer state, and the [`MapPolicy`] port a domain extension implements.
//!
//! Pure and target-neutral: no GPU, no host state. The host crate `semio-framework-surface-tiled-map` renders from it; the vector-tile decoder lives in `semio-framework-surface-tiled-map-mvt`.

use canvas::{CanvasExtension, Color, Point, RasterImage, Rect};
use std::cell::RefCell;
use std::sync::Arc;
use tiled_map_mvt as vector_tiles;

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

impl MapPalette {
    /// 🎯️ The colour of one palette slot.
    pub fn slot(&self, slot: MapPaletteSlot) -> Color {
        match slot {
            MapPaletteSlot::SurfaceClear => self.surface_clear,
            MapPaletteSlot::LandFill => self.land_fill,
            MapPaletteSlot::LandStroke => self.land_stroke,
            MapPaletteSlot::LabelFill => self.label_fill,
            MapPaletteSlot::LabelHalo => self.label_halo,
            MapPaletteSlot::RegionFill => self.region_fill,
            MapPaletteSlot::RegionStroke => self.region_stroke,
            MapPaletteSlot::RouteStroke => self.route_stroke,
            MapPaletteSlot::PositionFill => self.position_fill,
            MapPaletteSlot::PositionStroke => self.position_stroke,
            MapPaletteSlot::SelectionStroke => self.selection_stroke,
            MapPaletteSlot::HoverStroke => self.hover_stroke,
        }
    }
}

impl Default for MapPalette {
    fn default() -> Self {
        Self::from_map_palette(&ui_styling::MAP_LIGHT)
    }
}
// #endregion 🔖️MapPalette

// #region 🔖️Children
#[path = "📐️camera/🦀️.rs"]
mod map_camera;
pub use map_camera::{
    clamp_camera_to_world_bounds, clamp_map_zoom, clamp_map_zoom_for_viewport, fnv1a_bytes, fnv1a_mix, ideal_tile_z_for_viewport, map_camera_limits_json, map_camera_limits_json_for_viewport, map_view_facts, map_viewport, map_wheel_screen,
    tile_range_revision, vector_tiles_available_at_camera_zoom, viewport_lon_span_degrees, MAP_CAMERA_ZOOM_MAX, MAP_CAMERA_ZOOM_MIN, MAX_VISIBLE_TILE_REQUESTS, REVISION_FNV_OFFSET,
};
#[path = "../🎛️map-policy/🦀️.rs"]
mod map_policy;
#[path = "../🧭️projection/🦀️.rs"]
pub mod projection;
#[path = "../🧩️tiles/🦀️.rs"]
pub mod tiles;
pub use map_policy::*;
#[path = "../🪢️map-state/🦀️.rs"]
mod map_state;
pub use map_state::*;
// #endregion 🔖️Children

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
