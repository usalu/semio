//! 🪢️ Map descriptors and layer state: extension trait, overlay features, tile mode, policy-keyed layer switches, tile ledger.
//!
//! Linked to `MapHost`, which owns one [`MapExtension`], one [`MapTileLedger`], one [`MapOverlayTables`] and one [`MapLayerSwitches`].

use super::*;

// #region 🔖️MapExtension
/// 🆔️ Extension id of [`DefaultMapExtension`].
pub const MAP_EXTENSION_DEFAULT_ID: &str = "surface.tiled-map/default";

/// 🧩️ Domain extension of a tiled map: its coordinate reference system and the [`MapPolicy`] a `MapHost` renders by.
pub trait MapExtension: CanvasExtension {
    fn map_crs(&self) -> &str;
    fn policy(&self) -> &dyn MapPolicy;
}

/// 🫙️ Web Mercator extension carrying one [`DescribedMapPolicy`] — the neutral policy by default, a shipped descriptor once `MapHost::sync_map_json` receives one.
#[derive(Clone, Debug, Default)]
pub struct DefaultMapExtension {
    policy: DescribedMapPolicy,
}

impl DefaultMapExtension {
    /// 🪄️ The default extension carrying `policy`.
    pub fn with_policy(policy: DescribedMapPolicy) -> Self {
        Self { policy }
    }
}

impl CanvasExtension for DefaultMapExtension {
    fn extension_id(&self) -> &str {
        MAP_EXTENSION_DEFAULT_ID
    }
}

impl MapExtension for DefaultMapExtension {
    fn map_crs(&self) -> &str {
        "EPSG:3857"
    }

    fn policy(&self) -> &dyn MapPolicy {
        &self.policy
    }
}
// #endregion 🔖️MapExtension

// #region 🔖️MapContent
/// 📍️ One overlay feature of the map descriptor: its id, its lon/lat coordinates (one for a point, a polyline for a line, a ring for an area) and an optional caption, style key, line width, icon and link.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapOverlayFeature {
    pub id: String,
    pub coordinates: Vec<[f64; 2]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

/// 📨️ What `MapHost::sync_map_json` reads: the overlay features per overlay layer id, plus an optional [`MapPolicyDescriptor`] that replaces the host's policy when it differs.
#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapSyncJson {
    #[serde(default)]
    pub overlays: std::collections::BTreeMap<String, Vec<MapOverlayFeature>>,
    #[serde(default)]
    pub policy: Option<MapPolicyDescriptor>,
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

/// 🎚️ Minimum layer line/label weight multiplier from window sliders.
pub const MAP_LAYER_WEIGHT_MIN: f64 = ui_styling::metrics::map::LAYER_WEIGHT_MIN;

/// 🎚️ Maximum layer line/label weight multiplier from window sliders.
pub const MAP_LAYER_WEIGHT_MAX: f64 = ui_styling::metrics::map::LAYER_WEIGHT_MAX;

/// 🎚️ Clamps a layer weight slider value to the supported multiplier range.
pub fn clamp_map_layer_weight(value: f64) -> f64 {
    if value.is_finite() {
        value.clamp(MAP_LAYER_WEIGHT_MIN, MAP_LAYER_WEIGHT_MAX)
    } else {
        1.0
    }
}

/// 👁️ Per-toggle visibility and weight, keyed by the policy's [`MapLayerToggle`] keys: an absent key is visible at weight 1.0, a target no toggle lists is always visible at weight 1.0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MapLayerSwitches {
    pub visibility: std::collections::BTreeMap<String, bool>,
    pub weights: std::collections::BTreeMap<String, f64>,
}

impl MapLayerSwitches {
    /// 👀️ Whether `target` paints under `policy`'s toggles.
    pub fn visible(&self, policy: &dyn MapPolicy, target: &MapLayerTarget) -> bool {
        policy.toggle_key(target).and_then(|key| self.visibility.get(key)).copied().unwrap_or(true)
    }

    /// ⚖️ The weight multiplier `target` paints with under `policy`'s toggles.
    pub fn weight(&self, policy: &dyn MapPolicy, target: &MapLayerTarget) -> f64 {
        policy.toggle_key(target).and_then(|key| self.weights.get(key)).copied().unwrap_or(1.0)
    }

    /// 🧽️ `weights` with every value clamped to the slider range.
    pub fn sanitized_weights(weights: std::collections::BTreeMap<String, f64>) -> std::collections::BTreeMap<String, f64> {
        weights.into_iter().map(|(key, weight)| (key, clamp_map_layer_weight(weight))).collect()
    }
}

/// 🗺️ Overlay features mirrored from projection JSON, per overlay layer id and feature id — not authoritative document state (OS `ArtifactStore` owns packs).
#[derive(Default)]
pub struct MapOverlayTables {
    pub layers: std::collections::BTreeMap<String, std::collections::BTreeMap<String, MapOverlayFeature>>,
}

impl MapOverlayTables {
    /// 📚️ The features of overlay layer `layer`, by id.
    pub fn features(&self, layer: &str) -> impl Iterator<Item = &MapOverlayFeature> {
        self.layers.get(layer).into_iter().flat_map(|features| features.values())
    }

    /// 🔎️ One feature of overlay layer `layer`.
    pub fn feature(&self, layer: &str, id: &str) -> Option<&MapOverlayFeature> {
        self.layers.get(layer)?.get(id)
    }
}

#[derive(Default)]
pub struct MapTileLedger {
    pub tile_images: std::collections::BTreeMap<String, Arc<RasterImage>>,
    pub vector_tiles: std::collections::BTreeMap<String, vector_tiles::VectorTile>,
    /// 🕰️ Last-touched tick per raster tile key ("touched" = uploaded or drawn) — drives LRU eviction.
    pub raster_touch: RefCell<std::collections::BTreeMap<String, u64>>,
    /// 🕰️ Same as `raster_touch`, for the vector tile cache.
    pub vector_touch: RefCell<std::collections::BTreeMap<String, u64>>,
    pub touch_clock: std::cell::Cell<u64>,
}

impl MapTileLedger {
    pub fn touch_raster(&self, key: &str) {
        Self::touch_in(&self.touch_clock, &self.raster_touch, key);
    }

    pub fn touch_vector(&self, key: &str) {
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
