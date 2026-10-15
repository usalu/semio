//! 🎨️ GIS vector-tile style policy: OpenMapTiles class ranks, per-level-of-detail draw gates, stroke and label sizing.
//!
//! Wrong-direction content: a framework module encodes OSM/OpenMapTiles cartography. It is grouped under `gis_policy` so the GIS plugin can take the whole subtree; see the follow-up note in `RESULT-TILED-MAP.md`.
//!
//! Re-exported unchanged under `vector_tiles` so no public path moves.

use super::Color;
use super::vector_tiles::{property_class, MAP_VECTOR_TILE_MAX_Z};

// #region 🔖️VectorStyle
// #region 🔖️VectorTiles
pub fn layer_draw_rank(layer: &str) -> u8 {
    match layer {
        "water" => 0,
        "landcover" => 1,
        "landuse" => 2,
        "park" => 3,
        "transportation" => 4,
        "building" => 5,
        "boundary" => 6,
        "waterway" => 7,
        "aeroway" => 8,
        "countries" => 9,
        "geolines" => 10,
        "place" => 20,
        "transportation_name" => 21,
        "water_name" => 22,
        "poi" => 24,
        "centroids" => 25,
        _ => 15,
    }
}

/// 🔭️ Per-viewport vector draw gates (span + tile z) to match OSM raster LOD.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VectorDetailProfile {
    pub draw_water: bool,
    pub draw_land_backdrop: bool,
    pub draw_landcover: bool,
    pub draw_transportation: bool,
    pub draw_buildings: bool,
    pub draw_boundary: bool,
    pub draw_coastline: bool,
    pub max_admin_level: u64,
}

/// 🗺️ Vector tile z cap from viewport span (finer than raster LOD floor so oceans stay separated).
pub fn max_tile_z_for_span(span_deg: f64) -> u32 {
    let z = if span_deg > super::GIS_MAP_LOD_MAX_SPAN_DEG[0] {
        3
    } else if span_deg > super::GIS_MAP_LOD_MAX_SPAN_DEG[1] {
        5
    } else if span_deg > super::GIS_MAP_LOD_MAX_SPAN_DEG[2] {
        7
    } else if span_deg > super::GIS_MAP_LOD_MAX_SPAN_DEG[3] {
        6
    } else {
        MAP_VECTOR_TILE_MAX_Z
    };
    z.min(MAP_VECTOR_TILE_MAX_Z)
}

pub fn vector_detail_profile(span_deg: f64, tile_z: u32, forced_lod_id: Option<&str>) -> VectorDetailProfile {
    let lod_idx = super::resolve_detail_lod_index(span_deg, forced_lod_id);
    vector_detail_profile_for_lod(lod_idx, span_deg, tile_z)
}

fn vector_detail_profile_for_lod(lod_idx: usize, span_deg: f64, tile_z: u32) -> VectorDetailProfile {
    match lod_idx {
        0 | 1 => VectorDetailProfile { draw_water: true, draw_land_backdrop: false, draw_landcover: true, draw_transportation: false, draw_buildings: false, draw_boundary: false, draw_coastline: true, max_admin_level: 0 },
        2 => VectorDetailProfile { draw_water: true, draw_land_backdrop: true, draw_landcover: false, draw_transportation: false, draw_buildings: false, draw_boundary: true, draw_coastline: false, max_admin_level: 2 },
        3 => VectorDetailProfile { draw_water: true, draw_land_backdrop: true, draw_landcover: true, draw_transportation: true, draw_buildings: false, draw_boundary: true, draw_coastline: false, max_admin_level: 6 },
        4 => VectorDetailProfile { draw_water: true, draw_land_backdrop: true, draw_landcover: true, draw_transportation: true, draw_buildings: false, draw_boundary: true, draw_coastline: false, max_admin_level: 6 },
        5 | 6 => VectorDetailProfile { draw_water: true, draw_land_backdrop: true, draw_landcover: true, draw_transportation: true, draw_buildings: false, draw_boundary: true, draw_coastline: false, max_admin_level: 8 },
        _ => VectorDetailProfile {
            draw_water: true,
            draw_land_backdrop: true,
            draw_landcover: true,
            draw_transportation: true,
            draw_buildings: span_deg < super::GIS_MAP_LOD_MAX_SPAN_DEG[4] && tile_z >= 13,
            draw_boundary: true,
            draw_coastline: false,
            max_admin_level: 8,
        },
    }
}

fn boundary_visible_for_lod(admin_level: u64, lod_idx: usize) -> bool {
    match lod_idx {
        0 | 1 => false,
        2 => admin_level == 2,
        3 | 4 => (2..=6).contains(&admin_level),
        _ => (2..=8).contains(&admin_level),
    }
}

pub fn transportation_visible(class: &str, span_deg: f64, tile_z: u32, forced_lod_id: Option<&str>) -> bool {
    if !vector_detail_profile(span_deg, tile_z, forced_lod_id).draw_transportation {
        return false;
    }
    let lod_idx = super::resolve_detail_lod_index(span_deg, forced_lod_id);
    if lod_idx == 3 {
        return match class {
            "motorway" | "trunk" | "primary" => span_deg < 12.0,
            _ => false,
        };
    }
    match class {
        "motorway" | "trunk" => span_deg < 22.0,
        "primary" => span_deg < 12.0,
        "secondary" | "tertiary" => span_deg < 6.0,
        "minor" | "street" | "bus_guideway" => span_deg < 2.8,
        "residential" | "service" | "living_street" | "unclassified" | "pedestrian" => span_deg < 1.2,
        "path" | "track" | "footway" | "cycleway" | "steps" => span_deg < 0.45,
        _ => span_deg < 4.0,
    }
}

/// 📏️ Per-LOD road stroke multiplier (city 30% of prior default).
pub fn transportation_stroke_lod_scale(span_deg: f64, forced_lod_id: Option<&str>) -> f64 {
    match super::resolve_detail_lod_index(span_deg, forced_lod_id) {
        3 => ui_styling::strokes::MAP_ROAD_LOD_REGION,
        4 => ui_styling::strokes::MAP_ROAD_LOD_CITY,
        _ => 1.0,
    }
}

/// 📏️ Screen stroke scale from viewport longitude span; damped in region/city bands.
pub fn vector_line_scale(span_deg: f64) -> f64 {
    let cap = ui_styling::strokes::MAP_LINE_SCALE_CAP;
    let span = span_deg.max(0.08);
    let raw = ui_styling::strokes::MAP_LINE_SCALE_RAW / span.max(0.25);
    let country = super::GIS_MAP_LOD_MAX_SPAN_DEG[2];
    let region = super::GIS_MAP_LOD_MAX_SPAN_DEG[3];
    let city = super::GIS_MAP_LOD_MAX_SPAN_DEG[4];
    let district = super::GIS_MAP_LOD_MAX_SPAN_DEG[5];
    let street = super::GIS_MAP_LOD_MAX_SPAN_DEG[6];
    let damp = if span_deg <= street {
        let t = (span / street).sqrt();
        ui_styling::metrics::map::LINE_SCALE_DAMP_MIN + 0.1 * t
    } else if span_deg <= district {
        let u = ((span - street) / (district - street)).clamp(0.0, 1.0);
        0.5 + 0.1 * u
    } else if span_deg <= city {
        let u = ((span - district) / (city - district)).clamp(0.0, 1.0);
        0.56 + 0.1 * u
    } else if span_deg <= region {
        let u = ((span - city) / (region - city)).clamp(0.0, 1.0);
        0.66 + 0.1 * u
    } else if span_deg <= country {
        let u = ((span - region) / (country - region)).clamp(0.0, 1.0);
        0.8 + 0.08 * u
    } else {
        1.0
    };
    (raw * damp).clamp(0.5, cap)
}

pub fn transportation_stroke_width(class: &str, line_scale: f64) -> f64 {
    let base = match class {
        "motorway" => ui_styling::strokes::MAP_ROAD_MOTORWAY,
        "trunk" => 2.1,
        "primary" => 1.8,
        "secondary" => 1.45,
        "tertiary" => 1.2,
        "minor" | "street" => 0.95,
        "residential" | "service" | "living_street" => 0.78,
        "path" | "track" | "footway" | "cycleway" | "steps" => ui_styling::strokes::MAP_ROAD_PATH,
        _ => 0.9,
    };
    (base * line_scale).clamp(ui_styling::strokes::MAP_ROAD_CLAMP_MIN, ui_styling::strokes::MAP_ROAD_CLAMP_MAX)
}

pub fn boundary_visible(admin_level: u64, span_deg: f64, tile_z: u32, forced_lod_id: Option<&str>) -> bool {
    let profile = vector_detail_profile(span_deg, tile_z, forced_lod_id);
    if !profile.draw_boundary || admin_level > profile.max_admin_level {
        return false;
    }
    let lod_idx = super::resolve_detail_lod_index(span_deg, forced_lod_id);
    boundary_visible_for_lod(admin_level, lod_idx)
}

pub fn boundary_stroke_width(admin_level: u64, line_scale: f64) -> f64 {
    let base = match admin_level {
        2 => ui_styling::strokes::MAP_BOUNDARY_ADMIN2,
        3 | 4 => 1.3,
        5 | 6 => 1.0,
        _ => ui_styling::strokes::MAP_BOUNDARY_DEFAULT,
    };
    (base * line_scale).clamp(ui_styling::strokes::MAP_BOUNDARY_CLAMP_MIN, ui_styling::strokes::MAP_BOUNDARY_CLAMP_MAX)
}

pub fn coastline_stroke_width(line_scale: f64) -> f64 {
    (ui_styling::strokes::MAP_COASTLINE_MULT * line_scale).clamp(ui_styling::strokes::MAP_COASTLINE_CLAMP_MIN, ui_styling::strokes::MAP_COASTLINE_CLAMP_MAX)
}

pub fn place_label_visible(class: &str, span_deg: f64) -> bool {
    let caps = super::GIS_MAP_LOD_MAX_SPAN_DEG;
    match class {
        "" => span_deg <= caps[1] && span_deg > caps[2],
        "continent" => span_deg > caps[1],
        "country" => span_deg <= caps[1] && span_deg > caps[2],
        "state" | "province" => span_deg <= caps[2] && span_deg > caps[3],
        "city" => span_deg <= caps[3] && span_deg > caps[4],
        "town" => span_deg <= caps[4] && span_deg > caps[5],
        "village" | "hamlet" | "suburb" => span_deg <= caps[5] && span_deg > caps[6],
        "neighbourhood" | "quarter" | "isolated_dwelling" => span_deg <= caps[6],
        _ => false,
    }
}

/// 🛣️ Street-name labels use a stricter span gate than road geometry to avoid city-band overload.
pub fn transportation_name_visible(class: &str, span_deg: f64) -> bool {
    let caps = super::GIS_MAP_LOD_MAX_SPAN_DEG;
    match class {
        "motorway" | "trunk" => span_deg < caps[1],
        "primary" => span_deg < caps[3],
        "secondary" | "tertiary" => span_deg < caps[4],
        "minor" | "street" | "bus_guideway" => span_deg < caps[5],
        "residential" | "service" | "living_street" | "unclassified" | "pedestrian" => span_deg < caps[6],
        "path" | "track" | "footway" | "cycleway" | "steps" => span_deg < caps[6] * 0.85,
        _ => span_deg < caps[4],
    }
}

/// 📍️ POI captions only appear from district zoom inward.
pub fn poi_label_visible(span_deg: f64) -> bool {
    span_deg <= super::GIS_MAP_LOD_MAX_SPAN_DEG[5]
}

const GIS_MAP_LABEL_BAND_SCREEN_PX: &[f64] = ui_styling::metrics::map::LABEL_PX_BANDS;

fn map_lod_band_floor_span_deg(lod_idx: usize) -> f64 {
    if lod_idx == 0 {
        return 180.0;
    }
    super::GIS_MAP_LOD_MAX_SPAN_DEG.get(lod_idx.saturating_sub(1)).copied().unwrap_or(180.0).max(0.05)
}

/// 🔤️ Label screen px scaled with viewport span inside one map LOD band.
pub fn vector_label_px_for_lod(lod_idx: usize, span_deg: f64, weight: f64) -> f64 {
    let base = GIS_MAP_LABEL_BAND_SCREEN_PX.get(lod_idx).copied().unwrap_or(10.5);
    let span = span_deg.max(0.05);
    let floor = map_lod_band_floor_span_deg(lod_idx);
    base * floor / span * weight
}

/// 🔤️ Resolves the active map LOD band from viewport span, then returns its zoom-scaled label px.
pub fn vector_label_px(span_deg: f64, weight: f64) -> f64 {
    let lod_idx = super::resolve_map_lod_index_from_span(span_deg);
    vector_label_px_for_lod(lod_idx, span_deg, weight)
}

pub fn place_label_rank(class: &str, layer: &str) -> u16 {
    if layer == "poi" {
        return 48;
    }
    if layer == "water_name" {
        return 40;
    }
    if layer == "centroids" {
        return 8;
    }
    match class {
        "continent" => 0,
        "country" | "" => 4,
        "state" | "province" => 8,
        "city" => 12,
        "town" => 16,
        "village" | "hamlet" | "suburb" => 20,
        "neighbourhood" | "quarter" | "isolated_dwelling" => 24,
        _ => 28,
    }
}

pub fn transportation_name_rank(class: &str) -> u16 {
    match class {
        "motorway" => 30,
        "trunk" => 31,
        "primary" => 32,
        "secondary" => 34,
        "tertiary" => 36,
        _ => 44,
    }
}

pub fn color_with_alpha(color: Color, alpha: u8) -> Color {
    let rgba = color.to_rgba8();
    Color::from_rgba8(rgba.r, rgba.g, rgba.b, alpha)
}

/// 🎨️ Opaque land/water base fill; weight scales RGB only so tile composites do not seam.
pub fn weighted_opaque_fill(color: Color, weight: f64) -> Color {
    let w = super::clamp_map_layer_weight(weight).clamp(0.25, 1.0);
    let rgba = color.to_rgba8();
    let scale = |c: u8| (f64::from(c) * w).round() as u8;
    Color::from_rgba8(scale(rgba.r), scale(rgba.g), scale(rgba.b), 255)
}

fn water_class_is_open_sea(class: &str) -> bool {
    matches!(class, "ocean" | "sea" | "bay" | "strait" | "fjord" | "lagoon" | "sound" | "gulf")
}

/// 🌊️ Keep continent zoom free of inland water while preserving adjacent LODs.
pub fn water_polygon_visible_for_lod(lod_idx: usize, properties: &std::collections::BTreeMap<String, String>) -> bool {
    lod_idx != 1 || water_class_is_open_sea(property_class(properties))
}

pub fn waterway_visible_for_lod(lod_idx: usize) -> bool {
    lod_idx >= 2
}

/// 🗺️ Suppress country-polygon lake holes only at continent zoom.
pub fn country_polygon_holes_visible_for_lod(lod_idx: usize) -> bool {
    lod_idx != 1
}
// #endregion 🔖️VectorTiles
// #endregion 🔖️VectorStyle

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
