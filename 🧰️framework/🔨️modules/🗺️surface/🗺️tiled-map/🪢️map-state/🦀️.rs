//! 🪢️ Map descriptors and layer state: extension trait, position/route/region data, tile mode, layer weights and visibility, tile ledger.
//!
//! Linked to [`super::MapHost`], which owns one [`MapTileLedger`] and one set of layer tables.

use super::*;

// #region 🔖️MapExtension
pub trait MapExtension: CanvasExtension {
    fn map_crs(&self) -> &str;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultMapExtension;

impl CanvasExtension for DefaultMapExtension {
    fn extension_id(&self) -> &str {
        "gis.map/default"
    }
}

impl MapExtension for DefaultMapExtension {
    fn map_crs(&self) -> &str {
        "EPSG:3857"
    }
}
// #endregion 🔖️MapExtension

// #region 🔖️MapContent
// 🌱️ `source_url`'s `#[serde(alias = "sourceUrl")]` has NO `#[value(...)]` equivalent (derive has no
// alias support) — read-compat gap, not replicated in `FromValue` on purpose rather than faked.
// Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, serde::Deserialize, semio_framework_value::FromValue)]
pub struct PositionData {
    pub id: String,
    pub lon: f64,
    pub lat: f64,
    #[serde(default)]
    #[value(default)]
    pub label: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub name: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub kind: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub icon: Option<String>,
    #[serde(default, alias = "sourceUrl")]
    #[value(default)]
    pub source_url: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, semio_framework_value::FromValue)]
pub struct RouteData {
    pub id: String,
    pub points: Vec<[f64; 2]>,
    #[serde(default = "default_route_stroke")]
    #[value(default = "default_route_stroke")]
    pub stroke_width: f64,
}

fn default_route_stroke() -> f64 {
    ui_styling::strokes::MAP_ROUTE_DEFAULT
}

#[derive(Clone, Debug, serde::Deserialize, semio_framework_value::FromValue)]
pub struct RegionData {
    pub id: String,
    pub ring: Vec<[f64; 2]>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, semio_framework_value::FromValue)]
pub struct MapDescriptorJson {
    #[serde(default)]
    #[value(default)]
    pub positions: Vec<PositionData>,
    #[serde(default)]
    #[value(default)]
    pub routes: Vec<RouteData>,
    #[serde(default)]
    #[value(default)]
    pub regions: Vec<RegionData>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MapTileMode {
    #[default]
    Combined,
    Image,
    Vector,
}

impl MapTileMode {
    pub fn parse(mode: &str) -> Self {
        match mode {
            "image" => Self::Image,
            "vector" => Self::Vector,
            _ => Self::Combined,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MapVectorStyle {
    #[default]
    Colored,
    FigureGround,
    InvertedFigure,
}

impl MapVectorStyle {
    pub fn parse(style: &str) -> Self {
        match style {
            "figureGround" => Self::FigureGround,
            "invertedFigure" => Self::InvertedFigure,
            _ => Self::Colored,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Colored => "colored",
            Self::FigureGround => "figureGround",
            Self::InvertedFigure => "invertedFigure",
        }
    }
}

fn map_layer_default_true() -> bool {
    true
}

/// 🎚️ Minimum layer line/label weight multiplier from window sliders.
pub const MAP_LAYER_WEIGHT_MIN: f64 = ui_styling::metrics::map::LAYER_WEIGHT_MIN;

/// 🎚️ Maximum layer line/label weight multiplier from window sliders.
pub const MAP_LAYER_WEIGHT_MAX: f64 = ui_styling::metrics::map::LAYER_WEIGHT_MAX;

fn map_layer_default_weight() -> f64 {
    1.0
}

/// 🎚️ Clamps a layer weight slider value to the supported multiplier range.
pub fn clamp_map_layer_weight(value: f64) -> f64 {
    if value.is_finite() {
        value.clamp(MAP_LAYER_WEIGHT_MIN, MAP_LAYER_WEIGHT_MAX)
    } else {
        1.0
    }
}

/// 🎚️ Per-layer line/label weight multipliers (1.0 = default cartography).
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
pub struct MapLayerStrokeScale {
    #[serde(default = "map_layer_default_weight")]
    #[value(default = "map_layer_default_weight")]
    pub raster: f64,
    #[serde(default = "map_layer_default_weight")]
    #[value(default = "map_layer_default_weight")]
    pub water: f64,
    #[serde(default = "map_layer_default_weight")]
    #[value(default = "map_layer_default_weight")]
    pub land: f64,
    #[serde(default = "map_layer_default_weight")]
    #[value(default = "map_layer_default_weight")]
    pub roads: f64,
    #[serde(default = "map_layer_default_weight")]
    #[value(default = "map_layer_default_weight")]
    pub buildings: f64,
    #[serde(default = "map_layer_default_weight")]
    #[value(default = "map_layer_default_weight")]
    pub borders: f64,
    #[serde(default = "map_layer_default_weight")]
    #[value(default = "map_layer_default_weight")]
    pub labels: f64,
    #[serde(default = "map_layer_default_weight")]
    #[value(default = "map_layer_default_weight")]
    pub positions: f64,
    #[serde(default = "map_layer_default_weight")]
    #[value(default = "map_layer_default_weight")]
    pub position_labels: f64,
    #[serde(default = "map_layer_default_weight")]
    #[value(default = "map_layer_default_weight")]
    pub routes: f64,
    #[serde(default = "map_layer_default_weight")]
    #[value(default = "map_layer_default_weight")]
    pub regions: f64,
}

impl Default for MapLayerStrokeScale {
    fn default() -> Self {
        Self { raster: 1.0, water: 1.0, land: 1.0, roads: 1.0, buildings: 1.0, borders: 1.0, labels: 1.0, positions: 1.0, position_labels: 1.0, routes: 1.0, regions: 1.0 }
    }
}

impl MapLayerStrokeScale {
    pub fn sanitized(self) -> Self {
        Self {
            raster: clamp_map_layer_weight(self.raster),
            water: clamp_map_layer_weight(self.water),
            land: clamp_map_layer_weight(self.land),
            roads: clamp_map_layer_weight(self.roads),
            buildings: clamp_map_layer_weight(self.buildings),
            borders: clamp_map_layer_weight(self.borders),
            labels: clamp_map_layer_weight(self.labels),
            positions: clamp_map_layer_weight(self.positions),
            position_labels: clamp_map_layer_weight(self.position_labels),
            routes: clamp_map_layer_weight(self.routes),
            regions: clamp_map_layer_weight(self.regions),
        }
    }
}

/// 👁️ Per-layer show/hide gates for base map vector paint and user overlays.
#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
pub struct MapLayerVisibility {
    #[serde(default = "map_layer_default_true")]
    #[value(default = "map_layer_default_true")]
    pub raster: bool,
    #[serde(default = "map_layer_default_true")]
    #[value(default = "map_layer_default_true")]
    pub water: bool,
    #[serde(default = "map_layer_default_true")]
    #[value(default = "map_layer_default_true")]
    pub land: bool,
    #[serde(default = "map_layer_default_true")]
    #[value(default = "map_layer_default_true")]
    pub roads: bool,
    #[serde(default = "map_layer_default_true")]
    #[value(default = "map_layer_default_true")]
    pub buildings: bool,
    #[serde(default = "map_layer_default_true")]
    #[value(default = "map_layer_default_true")]
    pub borders: bool,
    #[serde(default = "map_layer_default_true")]
    #[value(default = "map_layer_default_true")]
    pub labels: bool,
    #[serde(default = "map_layer_default_true")]
    #[value(default = "map_layer_default_true")]
    pub positions: bool,
    #[serde(default = "map_layer_default_true")]
    #[value(default = "map_layer_default_true")]
    pub position_labels: bool,
    #[serde(default = "map_layer_default_true")]
    #[value(default = "map_layer_default_true")]
    pub routes: bool,
    #[serde(default = "map_layer_default_true")]
    #[value(default = "map_layer_default_true")]
    pub regions: bool,
}

impl Default for MapLayerVisibility {
    fn default() -> Self {
        Self { raster: true, water: true, land: true, roads: true, buildings: true, borders: true, labels: true, positions: true, position_labels: true, routes: true, regions: true }
    }
}

/// 🗺️ GIS feature tables mirrored from projection JSON — not authoritative document state (OS `ArtifactStore` owns packs).
#[derive(Default)]
pub struct MapFeatureTables {
    pub positions: std::collections::BTreeMap<String, PositionData>,
    pub routes: std::collections::BTreeMap<String, RouteData>,
    pub regions: std::collections::BTreeMap<String, RegionData>,
}

#[derive(Default)]
pub(super) struct MapTileLedger {
    pub(super) tile_images: std::collections::BTreeMap<String, Arc<RasterImage>>,
    pub(super) vector_tiles: std::collections::BTreeMap<String, vector_tiles::VectorTile>,
    /// 🕰️ Last-touched tick per raster tile key ("touched" = uploaded or drawn) — drives LRU eviction.
    pub(super) raster_touch: RefCell<std::collections::BTreeMap<String, u64>>,
    /// 🕰️ Same as `raster_touch`, for the vector tile cache.
    pub(super) vector_touch: RefCell<std::collections::BTreeMap<String, u64>>,
    pub(super) touch_clock: std::cell::Cell<u64>,
}

impl MapTileLedger {
    pub(super) fn touch_raster(&self, key: &str) {
        Self::touch_in(&self.touch_clock, &self.raster_touch, key);
    }

    pub(super) fn touch_vector(&self, key: &str) {
        Self::touch_in(&self.touch_clock, &self.vector_touch, key);
    }

    /// 🕰️ Stamps `key` with the next tick. The already-present branch writes through `get_mut` so the
    /// per-drawn-tile call in `append_tiles`/`append_vector_tiles` allocates nothing on a hot frame —
    /// only a genuinely new key pays for the `String`.
    fn touch_in(clock: &std::cell::Cell<u64>, ledger: &RefCell<std::collections::BTreeMap<String, u64>>, key: &str) {
        let next = clock.get().wrapping_add(1);
        clock.set(next);
        let mut map = ledger.borrow_mut();
        match map.get_mut(key) {
            Some(slot) => *slot = next,
            None => {
                map.insert(key.to_string(), next);
            }
        }
    }
}
// #endregion 🔖️MapContent
