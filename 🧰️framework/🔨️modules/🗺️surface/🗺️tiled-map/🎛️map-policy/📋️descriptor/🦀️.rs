//! 📋️ Map policy descriptor: the serialisable, domain-neutral data a host renders a tiled map by — level-of-detail bands, the tile pyramid, source-layer paint roles, class tables, label rules, line scaling, layer toggles, overlay layers and vector style presets.
//!
//! Schema: `../🧬️schema/🔣️.json`. A domain plugin builds one and ships it as the `policy` member of the map descriptor JSON, so every host (the wgpu engine surface, the browser session, plugin-side hit testing) renders by the same rules. [`MapPolicyDescriptor::neutral`] is the framework default: one band, every layer gate open, no source-layer roles.

// #region 🔖️MapPolicyDescriptor
/// 🆔️ Id of [`MapPolicyDescriptor::neutral`].
pub const MAP_POLICY_NEUTRAL_ID: &str = "surface.tiled-map/neutral";

/// 🏷️ Id of the neutral descriptor's only style preset.
pub const MAP_STYLE_PRESET_NEUTRAL_ID: &str = "default";

/// 📋️ Everything a [`MapPolicy`] reads, ordered from the tile pyramid to the paint and label rules.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapPolicyDescriptor {
    pub id: String,
    pub bands: Vec<MapLodBand>,
    pub coarse_band_count: usize,
    pub raster_tile_z_max: u32,
    pub vector_tile_z_max: u32,
    pub vector_zoom_steps: Vec<MapSpanZoom>,
    pub boundary_admin_min: u64,
    pub layers: Vec<MapSourceLayer>,
    pub default_draw_rank: u8,
    pub feature_keys: MapFeatureKeys,
    pub road_spans: MapClassSpans,
    pub road_widths: MapClassWidths,
    pub boundary_widths: MapLevelWidths,
    pub coastline_width: MapWidth,
    pub line_scale: MapLineScale,
    pub labels: MapLabelRules,
    pub layer_toggles: Vec<MapLayerToggle>,
    pub overlays: Vec<MapOverlayLayer>,
    pub style_presets: Vec<MapStylePreset>,
}

/// 📶️ One level-of-detail band, coarse to fine: identity, the span it covers, its raster tile z and its paint gates.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapLodBand {
    pub id: String,
    pub name: String,
    pub description: String,
    pub max_zoom: Option<f64>,
    pub max_span_deg: f64,
    pub raster_tile_z: u32,
    pub representative_span_deg: f64,
    pub label_px: f64,
    pub gates: MapLayerGates,
    pub buildings_max_span_deg: Option<f64>,
    pub buildings_min_tile_z: u32,
    pub road_spans: Option<MapClassSpans>,
    pub road_stroke_scale: f64,
    pub water_area_classes: Option<Vec<String>>,
    pub waterways: bool,
    pub admin_area_holes: bool,
    pub land_wash: bool,
    pub land_cover_over_admin_areas: bool,
}

/// 🚦️ Per-band vector paint gates; `buildings` is further narrowed by the band's span and tile-z limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapLayerGates {
    pub water: bool,
    pub land_backdrop: bool,
    pub land_cover: bool,
    pub roads: bool,
    pub buildings: bool,
    pub boundaries: bool,
    pub coastline: bool,
    pub max_admin_level: u64,
}

/// 🎭️ The paint or label role a host gives every feature of one vector-tile source layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MapLayerRole {
    WaterArea,
    LandCover,
    GreenArea,
    Building,
    Road,
    Boundary,
    Coastline,
    Waterway,
    AdminArea,
    RoadLabel,
    PointLabel,
    PlaceLabel,
}

/// 🧱️ One vector-tile source layer: its role (none paints nothing), its draw order and an optional fixed label rank.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapSourceLayer {
    pub name: String,
    pub role: Option<MapLayerRole>,
    pub draw_rank: u8,
    pub label_rank: Option<u16>,
}

/// 🔑️ Feature property keys the roles read: class, administrative level, the maritime flag and label candidates in priority order.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapFeatureKeys {
    pub class: String,
    pub admin_level: String,
    pub maritime: String,
    pub label: Vec<String>,
}

/// 🔭️ Above `above_span_deg` the vector tile z is capped at `tile_z`; steps are tried in order.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapSpanZoom {
    pub above_span_deg: f64,
    pub tile_z: u32,
}

/// 🛣️ Class-keyed visibility: a class shows while the span stays below its row's limit (no limit shows always).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapClassSpans {
    pub rows: Vec<MapClassSpan>,
    pub fallback_below_span_deg: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapClassSpan {
    pub classes: Vec<String>,
    pub below_span_deg: Option<f64>,
}

/// 📏️ Class-keyed base stroke width, multiplied by the line scale and bounded.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapClassWidths {
    pub rows: Vec<MapClassWidth>,
    pub fallback: f64,
    pub min: f64,
    pub max: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapClassWidth {
    pub classes: Vec<String>,
    pub width: f64,
}

/// 🪜️ Level-keyed base stroke width (inclusive level ranges), multiplied by the line scale and bounded.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapLevelWidths {
    pub rows: Vec<MapLevelWidth>,
    pub fallback: f64,
    pub min: f64,
    pub max: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapLevelWidth {
    pub min_level: u64,
    pub max_level: u64,
    pub width: f64,
}

/// 🖊️ One base stroke width, multiplied by the line scale and bounded.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapWidth {
    pub width: f64,
    pub min: f64,
    pub max: f64,
}

/// 📐️ Screen line scale from the viewport span: `coefficient / max(span, minimums)`, damped by the first segment whose `up_to_span_deg` the span does not exceed (`beyond` past the last), then bounded.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapLineScale {
    pub coefficient: f64,
    pub min_span_deg: f64,
    pub coefficient_min_span_deg: f64,
    pub min: f64,
    pub max: f64,
    pub segments: Vec<MapLineDamping>,
    pub beyond: f64,
}

/// 📉️ `base + slope * curve(clamp((span - from) / (up_to - from), 0, 1))`.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapLineDamping {
    pub up_to_span_deg: f64,
    pub from_span_deg: f64,
    pub base: f64,
    pub slope: f64,
    pub curve: MapDampingCurve,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MapDampingCurve {
    Linear,
    SquareRoot,
}

/// 🔤️ Label sizing, visibility and ranking for the three label roles.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapLabelRules {
    pub span_floor_deg: f64,
    pub min_span_deg: f64,
    pub road_label_spans: MapClassSpans,
    pub road_label_ranks: MapClassRanks,
    pub place_label_spans: Vec<MapClassSpanRange>,
    pub place_label_ranks: MapClassRanks,
    pub point_label_max_span_deg: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapClassRanks {
    pub rows: Vec<MapClassRank>,
    pub fallback: u16,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapClassRank {
    pub classes: Vec<String>,
    pub rank: u16,
}

/// 🏷️ A place class shows while `min_span_deg < span <= max_span_deg` (a missing bound is open); an unlisted class never shows.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapClassSpanRange {
    pub classes: Vec<String>,
    pub max_span_deg: Option<f64>,
    pub min_span_deg: Option<f64>,
}
/// 🎚️ One user-facing layer switch: its key in the visibility and weight maps and every paint target it controls.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapLayerToggle {
    pub key: String,
    pub targets: Vec<MapLayerTarget>,
}

/// 🎯️ What a layer toggle switches and weights: raster tiles, one vector paint role, one overlay layer or that overlay's captions.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MapLayerTarget {
    Raster,
    Role(MapLayerRole),
    Overlay(String),
    OverlayLabels(String),
}

/// 📐️ The geometry of an overlay layer's features: one coordinate, a polyline or a closed ring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MapOverlayGeometry {
    Point,
    Line,
    Area,
}

/// 📍️ One overlay layer the map descriptor fills with features (`overlays.<id>`), drawn in declaration order above the tiles. A layer with a `granularity` is pickable: hits, marquee rows and selection use that granularity as their kind and the layer id as their key.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapOverlayLayer {
    pub id: String,
    pub geometry: MapOverlayGeometry,
    pub granularity: Option<String>,
    pub hit_radius_px: f64,
    pub line_width: f64,
    pub point_radius: f64,
    pub point_stroke: f64,
    pub label_offset_px: f64,
    pub fill: MapPaletteSlot,
    pub stroke: MapPaletteSlot,
    pub style_fills: Vec<MapOverlayStyleFill>,
}

/// 🖍️ A feature whose `style` equals `style` fills with `fill` instead of the layer fill.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapOverlayStyleFill {
    pub style: String,
    pub fill: MapPaletteSlot,
}

/// 🎨️ One colour slot of the host's map palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MapPaletteSlot {
    SurfaceClear,
    LandFill,
    LandStroke,
    LabelFill,
    LabelHalo,
    RegionFill,
    RegionStroke,
    RouteStroke,
    PositionFill,
    PositionStroke,
    SelectionStroke,
    HoverStroke,
}

/// 🖼️ How a vector style preset paints the tiles: the full-colour roles, or a two-tone figure-ground silhouette.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MapPaintTechnique {
    Colored,
    FigureGround,
}

/// 🌈️ One selectable vector style: its id (the window's vector-style value), the technique, the figure-ground ink and paper, and the label fill and halo slots.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapStylePreset {
    pub id: String,
    pub technique: MapPaintTechnique,
    pub ink: MapPaletteSlot,
    pub paper: MapPaletteSlot,
    pub label_fill: MapPaletteSlot,
    pub label_halo: MapPaletteSlot,
}
// #endregion 🔖️MapPolicyDescriptor

// #region 🔖️Tables
fn bounded(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

fn lists(classes: &[String], class: &str) -> bool {
    classes.iter().any(|candidate| candidate == class)
}

impl MapClassSpans {
    /// ✅️ Whether `class` shows at `span_deg`.
    pub fn admits(&self, class: &str, span_deg: f64) -> bool {
        let limit = self.rows.iter().find(|row| lists(&row.classes, class)).map_or(self.fallback_below_span_deg, |row| row.below_span_deg);
        limit.is_none_or(|below| span_deg < below)
    }
}

impl MapClassWidths {
    /// 🖌️ The bounded stroke width of `class` at `line_scale`.
    pub fn at(&self, class: &str, line_scale: f64) -> f64 {
        let base = self.rows.iter().find(|row| lists(&row.classes, class)).map_or(self.fallback, |row| row.width);
        bounded(base * line_scale, self.min, self.max)
    }
}

impl MapLevelWidths {
    /// 🧮️ The bounded stroke width of `level` at `line_scale`.
    pub fn at(&self, level: u64, line_scale: f64) -> f64 {
        let base = self.rows.iter().find(|row| (row.min_level..=row.max_level).contains(&level)).map_or(self.fallback, |row| row.width);
        bounded(base * line_scale, self.min, self.max)
    }
}

impl MapWidth {
    /// ✒️ The bounded stroke width at `line_scale`.
    pub fn at(&self, line_scale: f64) -> f64 {
        bounded(self.width * line_scale, self.min, self.max)
    }
}

impl MapLineScale {
    /// 🎚️ The damped, bounded line scale at `span_deg`.
    pub fn at(&self, span_deg: f64) -> f64 {
        let span = span_deg.max(self.min_span_deg);
        let raw = self.coefficient / span.max(self.coefficient_min_span_deg);
        let damp = self.segments.iter().find(|segment| span_deg <= segment.up_to_span_deg).map_or(self.beyond, |segment| {
            let t = ((span - segment.from_span_deg) / (segment.up_to_span_deg - segment.from_span_deg)).clamp(0.0, 1.0);
            let t = match segment.curve {
                MapDampingCurve::Linear => t,
                MapDampingCurve::SquareRoot => t.sqrt(),
            };
            segment.base + segment.slope * t
        });
        bounded(raw * damp, self.min, self.max)
    }
}

impl MapClassRanks {
    /// 🥇️ The rank of `class`, lower first.
    pub fn rank(&self, class: &str) -> u16 {
        self.rows.iter().find(|row| lists(&row.classes, class)).map_or(self.fallback, |row| row.rank)
    }
}

impl MapLabelRules {
    /// 📍️ Whether a place label of `class` shows at `span_deg`.
    pub fn place_admits(&self, class: &str, span_deg: f64) -> bool {
        self.place_label_spans
            .iter()
            .find(|row| lists(&row.classes, class))
            .is_some_and(|row| row.max_span_deg.is_none_or(|max| span_deg <= max) && row.min_span_deg.is_none_or(|min| span_deg > min))
    }
}
// #endregion 🔖️Tables

// #region 🔖️Neutral
impl MapPolicyDescriptor {
    /// 🫥️ The framework default: one band covering every zoom, every layer gate open except the land backdrop, no source-layer roles, unit line scale, no toggles or overlay layers, one colored style preset.
    pub fn neutral() -> Self {
        let open = MapClassSpans { rows: Vec::new(), fallback_below_span_deg: None };
        let unit = MapClassRanks { rows: Vec::new(), fallback: 0 };
        Self {
            id: MAP_POLICY_NEUTRAL_ID.into(),
            bands: vec![MapLodBand {
                id: "all".into(),
                name: "All".into(),
                description: "One band for every zoom.".into(),
                max_zoom: None,
                max_span_deg: 0.0,
                raster_tile_z: 0,
                representative_span_deg: 360.0,
                label_px: 10.5,
                gates: MapLayerGates { water: true, land_backdrop: false, land_cover: true, roads: true, buildings: true, boundaries: true, coastline: true, max_admin_level: u64::MAX },
                buildings_max_span_deg: None,
                buildings_min_tile_z: 0,
                road_spans: None,
                road_stroke_scale: 1.0,
                water_area_classes: None,
                waterways: true,
                admin_area_holes: true,
                land_wash: false,
                land_cover_over_admin_areas: true,
            }],
            coarse_band_count: 0,
            raster_tile_z_max: 22,
            vector_tile_z_max: 22,
            vector_zoom_steps: Vec::new(),
            boundary_admin_min: 0,
            layers: Vec::new(),
            default_draw_rank: 0,
            feature_keys: MapFeatureKeys { class: String::new(), admin_level: String::new(), maritime: String::new(), label: Vec::new() },
            road_spans: open.clone(),
            road_widths: MapClassWidths { rows: Vec::new(), fallback: 1.0, min: 0.0, max: f64::MAX },
            boundary_widths: MapLevelWidths { rows: Vec::new(), fallback: 1.0, min: 0.0, max: f64::MAX },
            coastline_width: MapWidth { width: 1.0, min: 0.0, max: f64::MAX },
            line_scale: MapLineScale { coefficient: 1.0, min_span_deg: 0.0, coefficient_min_span_deg: 1.0, min: 1.0, max: 1.0, segments: Vec::new(), beyond: 1.0 },
            labels: MapLabelRules { span_floor_deg: 360.0, min_span_deg: 0.0, road_label_spans: open, road_label_ranks: unit.clone(), place_label_spans: Vec::new(), place_label_ranks: unit, point_label_max_span_deg: f64::MAX },
            layer_toggles: Vec::new(),
            overlays: Vec::new(),
            style_presets: vec![MapStylePreset { id: MAP_STYLE_PRESET_NEUTRAL_ID.into(), technique: MapPaintTechnique::Colored, ink: MapPaletteSlot::LabelFill, paper: MapPaletteSlot::SurfaceClear, label_fill: MapPaletteSlot::LabelFill, label_halo: MapPaletteSlot::LabelHalo }],
        }
    }
}
// #endregion 🔖️Neutral
