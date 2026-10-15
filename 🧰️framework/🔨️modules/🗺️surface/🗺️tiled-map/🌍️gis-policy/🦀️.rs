//! 🌍️ GIS policy of the tiled map: level-of-detail band tables, `gis_map_*` JSON surfaces, tile-zoom picking and layer-weight slider gating.
//!
//! Wrong-direction content: the framework surface names a concrete plugin domain. Everything GIS-specific is grouped here (and in `vector_style` below) so a follow-up can relocate this subtree into the GIS plugin behind a neutral map-policy port; see `RESULT-TILED-MAP.md`.
//!
//! Items keep their former `tiled_map::` paths through the root re-export.

use canvas::lod::{Lod, LodScale};
use super::*;

#[path = "🎨️vector-style/🦀️.rs"]
pub(super) mod vector_style;

// #region 🔖️GisPolicy
// #region 🔖️MapLod
pub(super) const GIS_MAP_LODS: &[Lod] = &[
    Lod { id: "world", name: "World", description: "Entire planet; coarsest OSM tiles.", max_zoom: 360.0 },
    Lod { id: "continent", name: "Continent", description: "Multi-country overview.", max_zoom: 680.0 },
    Lod { id: "country", name: "Country", description: "National extent.", max_zoom: 1_280.0 },
    Lod { id: "region", name: "Region", description: "Regional detail.", max_zoom: 2_400.0 },
    Lod { id: "city", name: "City", description: "Metropolitan area.", max_zoom: 4_400.0 },
    Lod { id: "district", name: "District", description: "Neighbourhood streets.", max_zoom: 6_400.0 },
    Lod { id: "street", name: "Street", description: "Block-level detail.", max_zoom: 7_200.0 },
    Lod { id: "building", name: "Building", description: "Maximum map fidelity.", max_zoom: f64::INFINITY },
];

/// 🔭️ Raster tile z per LOD band — one entry per [`GIS_MAP_LODS`] band, indexed only by a
/// `resolve_map_lod_index_from_span`/`GIS_MAP_LOD_SCALE::index_of` result, so it must stay exactly
/// as long as `GIS_MAP_LODS`. It previously carried a ninth, unreachable `18`, which is what led the
/// oracle fixture to record a nonexistent band index 8 for `spanDeg = 0`.
pub(super) const GIS_MAP_LOD_TILE_Z: &[u32] = &[0, 1, 2, 3, 4, 5, 7, 10];

/// 🌐️ Upper visible longitude span (degrees) per band; coarser band when span exceeds threshold.
pub(super) const GIS_MAP_LOD_MAX_SPAN_DEG: &[f64] = &[100.0, 35.0, 12.0, 4.0, 1.2, 0.35, 0.1, 0.0];

pub(super) const GIS_MAP_LOD_SCALE: LodScale = LodScale { lods: GIS_MAP_LODS };

/// 📶️ Window LOD select value: camera zoom picks the tile band.
pub const GIS_MAP_LOD_MODE_AUTOMATIC: &str = "automatic";

pub fn gis_map_lod_scale_json() -> String {
    let rows: Vec<serde_json::Value> = GIS_MAP_LODS
        .iter()
        .enumerate()
        .map(|(i, lod)| {
            serde_json::json!({
                "id": lod.id,
                "name": lod.name,
                "description": lod.description,
                "maxZoom": lod.max_zoom,
                "maxSpanDeg": GIS_MAP_LOD_MAX_SPAN_DEG.get(i).copied().unwrap_or(0.0),
                "tileZ": GIS_MAP_LOD_TILE_Z.get(i).copied().unwrap_or(0),
            })
        })
        .collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

pub(super) fn resolve_map_lod_index_from_span(span_deg: f64) -> usize {
    for (i, threshold) in GIS_MAP_LOD_MAX_SPAN_DEG.iter().enumerate() {
        if span_deg > *threshold {
            return i;
        }
    }
    GIS_MAP_LODS.len().saturating_sub(1)
}

/// 📶️ The automatic LOD band a viewport longitude span (degrees) resolves to, and that band's raster tile z —
/// the pair the `🕸️web-mercator-tile-oracle` case's `lod-band-selection` specification vectors pin.
pub fn map_lod_band(span_deg: f64) -> (usize, u32) {
    let index = resolve_map_lod_index_from_span(span_deg);
    (index, GIS_MAP_LOD_TILE_Z.get(index).copied().unwrap_or(0))
}

pub(super) fn resolve_detail_lod_index(span_deg: f64, forced_lod_id: Option<&str>) -> usize {
    if let Some(id) = forced_lod_id {
        if let Some(idx) = GIS_MAP_LOD_SCALE.index_of(id) {
            return idx;
        }
    }
    resolve_map_lod_index_from_span(span_deg)
}

/// 🔭️ Representative viewport longitude span (degrees) for a pinned LOD band.
pub fn representative_viewport_span_for_lod(lod_idx: usize) -> f64 {
    match lod_idx {
        0 => 180.0,
        1 => 50.0,
        2 => 20.0,
        3 => 8.0,
        4 => 2.0,
        5 => 0.6,
        6 => 0.2,
        _ => 0.05,
    }
}

fn current_map_lod(camera: &camera::Camera, viewport: &camera::Viewport) -> &'static Lod {
    let idx = resolve_map_lod_index_from_span(viewport_lon_span_degrees(camera, viewport));
    &GIS_MAP_LODS[idx]
}

fn forced_lod_tile_z(id: &str) -> Option<u32> {
    let idx = GIS_MAP_LOD_SCALE.index_of(id)?;
    GIS_MAP_LOD_TILE_Z.get(idx).copied()
}

/// 🧷️ Pinned LOD is a minimum tile-detail floor; world/continent automatic bands use fixed coarse tile z.
pub(super) fn pick_tile_z_target(camera: &camera::Camera, viewport: &camera::Viewport, forced_lod_id: Option<&str>) -> u32 {
    let ideal = ideal_tile_z_for_viewport(camera, viewport);
    let span = viewport_lon_span_degrees(camera, viewport);
    let lod_idx = resolve_map_lod_index_from_span(span);
    if let Some(id) = forced_lod_id {
        let Some(pin_floor) = forced_lod_tile_z(id) else {
            return ideal;
        };
        let coarse_floor = GIS_MAP_LOD_TILE_Z.get(lod_idx).copied().unwrap_or(0);
        let floor = if lod_idx <= 1 { coarse_floor } else { pin_floor };
        return floor.max(ideal);
    }
    if lod_idx <= 1 {
        return GIS_MAP_LOD_TILE_Z.get(lod_idx).copied().unwrap_or(0);
    }
    ideal.min(vector_tiles::max_tile_z_for_span(span))
}

pub(super) fn active_map_lod(forced_lod_id: Option<&str>, camera: &camera::Camera, viewport: &camera::Viewport) -> &'static Lod {
    if let Some(id) = forced_lod_id {
        if let Some(idx) = GIS_MAP_LOD_SCALE.index_of(id) {
            return &GIS_MAP_LODS[idx];
        }
    }
    current_map_lod(camera, viewport)
}

pub fn gis_map_camera_limits_json() -> String {
    gis_map_camera_limits_json_for_viewport(&camera::Viewport::default())
}

pub fn gis_map_camera_limits_json_for_viewport(viewport: &camera::Viewport) -> String {
    serde_json::json!({
        "min": projection::cover_zoom_for_viewport(viewport).max(MAP_CAMERA_ZOOM_MIN),
        "max": MAP_CAMERA_ZOOM_MAX,
    })
    .to_string()
}
// #endregion 🔖️MapLod

// #region 🔖️MapContent
/// 🎚️ Layer ids that accept a weight slider at the given LOD and tile render mode.
pub fn map_layer_weight_slider_keys_at_lod(lod_id: &str, render_mode: &str) -> Vec<&'static str> {
    let lod_idx = GIS_MAP_LOD_SCALE.index_of(lod_id).unwrap_or(0);
    let span = representative_viewport_span_for_lod(lod_idx);
    let mut tile_z = GIS_MAP_LOD_TILE_Z.get(lod_idx).copied().unwrap_or(0);
    if lod_idx + 1 >= GIS_MAP_LODS.len() {
        tile_z = tile_z.max(vector_tiles::MAP_VECTOR_TILE_MAX_Z);
    }
    let profile = vector_tiles::vector_detail_profile(span, tile_z, Some(lod_id));
    let mode = MapTileMode::parse(render_mode);
    let mut keys: Vec<&'static str> = Vec::new();
    if matches!(mode, MapTileMode::Image | MapTileMode::Combined) {
        keys.push("raster");
    }
    if matches!(mode, MapTileMode::Vector | MapTileMode::Combined) {
        if profile.draw_water || profile.draw_coastline {
            keys.push("water");
        }
        if profile.draw_landcover || profile.draw_land_backdrop {
            keys.push("land");
        }
        if profile.draw_transportation {
            keys.push("roads");
        }
        if profile.draw_buildings {
            keys.push("buildings");
        }
        if profile.draw_boundary {
            keys.push("borders");
        }
        let vector_paint = profile.draw_water || profile.draw_coastline || profile.draw_landcover || profile.draw_land_backdrop || profile.draw_transportation || profile.draw_buildings || profile.draw_boundary;
        if vector_paint {
            keys.push("labels");
        }
    }
    keys.push("positions");
    keys.push("positionLabels");
    keys.push("routes");
    keys.push("regions");
    keys
}

/// 🎚️ JSON array of layer ids with weight sliders for window options at a LOD.
pub fn gis_map_layer_weight_slider_ids_json(lod_id: &str, render_mode: &str) -> String {
    let keys = map_layer_weight_slider_keys_at_lod(lod_id, render_mode);
    serde_json::to_string(&keys).unwrap_or_else(|_| "[]".into())
}

/// OpenFreeMap OSM vector tiles (OpenMapTiles schema) track viewport zoom up to z14.
pub fn vector_tiles_available_at_camera_zoom(camera_zoom: f64) -> bool {
    camera_zoom > 0.0
}
// #endregion 🔖️MapContent
// #endregion 🔖️GisPolicy
