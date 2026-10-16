//! 🎛️ Map policy port: the neutral extension point through which a domain decides level-of-detail bands, tile-z targets, vector paint gates, source-layer roles, draw order, labels and stroke widths.
//!
//! [`MapPolicy`] reads one [`MapPolicyDescriptor`]; every rule has a descriptor-driven default an implementation may override. `MapExtension::policy` hands it to `MapHost`, which falls back to [`MapPolicyDescriptor::neutral`] without a domain. The JSON surfaces below are parameterised by the policy instead of naming a domain.

use super::{vector_tiles, MapTileMode};

#[path = "📋️descriptor/🦀️.rs"]
mod descriptor;
pub use descriptor::*;

// #region 🔖️MapPolicy
/// 📶️ Window LOD select value: the viewport span picks the band.
pub const MAP_LOD_MODE_AUTOMATIC: &str = "automatic";

/// 🔭️ Viewport facts a tile-z decision reads: the visible longitude span and the ideal tile z for the viewport width.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapViewFacts {
    pub span_deg: f64,
    pub ideal_tile_z: u32,
}

/// 🔍️ The feature properties a role reads, resolved through the descriptor's feature keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapFeatureFacts<'a> {
    pub class: &'a str,
    pub admin_level: Option<u64>,
    pub maritime: bool,
}

/// 🎛️ Domain policy of a tiled map. Implementations supply [`MapPolicy::descriptor`] (at least one band and one style preset) and may override any rule.
pub trait MapPolicy: Send + Sync {
    fn descriptor(&self) -> &MapPolicyDescriptor;

    fn id(&self) -> &str {
        &self.descriptor().id
    }

    fn bands(&self) -> &[MapLodBand] {
        &self.descriptor().bands
    }

    fn band(&self, index: usize) -> &MapLodBand {
        let bands = self.bands();
        &bands[index.min(bands.len().saturating_sub(1))]
    }

    fn band_index(&self, id: &str) -> Option<usize> {
        self.bands().iter().position(|band| band.id == id)
    }

    fn band_for_span(&self, span_deg: f64, pinned: Option<&str>) -> usize {
        if let Some(index) = pinned.and_then(|id| self.band_index(id)) {
            return index;
        }
        let bands = self.bands();
        bands.iter().position(|band| span_deg > band.max_span_deg).unwrap_or(bands.len().saturating_sub(1))
    }

    fn tile_z_target(&self, view: MapViewFacts, pinned: Option<&str>) -> u32 {
        let band = self.band_for_span(view.span_deg, None);
        let coarse = band < self.descriptor().coarse_band_count;
        let band_tile_z = self.band(band).raster_tile_z;
        if let Some(id) = pinned {
            let Some(pin) = self.band_index(id) else {
                return view.ideal_tile_z;
            };
            let floor = if coarse { band_tile_z } else { self.band(pin).raster_tile_z };
            return floor.max(view.ideal_tile_z);
        }
        if coarse {
            return band_tile_z;
        }
        view.ideal_tile_z.min(self.vector_zoom_cap(view.span_deg))
    }

    fn vector_zoom_cap(&self, span_deg: f64) -> u32 {
        let descriptor = self.descriptor();
        descriptor.vector_zoom_steps.iter().find(|step| span_deg > step.above_span_deg).map_or(descriptor.vector_tile_z_max, |step| step.tile_z).min(descriptor.vector_tile_z_max)
    }

    fn vector_tile_z_target(&self, view: MapViewFacts, raster_tile_z: u32) -> u32 {
        let cap = self.vector_zoom_cap(view.span_deg);
        let z = if self.band_for_span(view.span_deg, None) < self.descriptor().coarse_band_count { cap } else { raster_tile_z.min(cap) };
        z.min(self.descriptor().vector_tile_z_max)
    }

    fn layer_gates(&self, band: usize, span_deg: f64, tile_z: u32) -> MapLayerGates {
        let row = self.band(band);
        let mut gates = row.gates;
        gates.buildings = gates.buildings && row.buildings_max_span_deg.is_none_or(|max| span_deg < max) && tile_z >= row.buildings_min_tile_z;
        gates
    }

    fn source_layer(&self, layer: &str) -> Option<&MapSourceLayer> {
        self.descriptor().layers.iter().find(|row| row.name == layer)
    }

    fn layer_role(&self, layer: &str) -> Option<MapLayerRole> {
        self.source_layer(layer).and_then(|row| row.role)
    }

    fn draw_rank(&self, layer: &str) -> u8 {
        self.source_layer(layer).map_or(self.descriptor().default_draw_rank, |row| row.draw_rank)
    }

    fn feature_facts<'a>(&self, properties: &'a std::collections::BTreeMap<String, String>) -> MapFeatureFacts<'a> {
        let keys = &self.descriptor().feature_keys;
        MapFeatureFacts { class: vector_tiles::property_str(properties, &keys.class).unwrap_or(""), admin_level: vector_tiles::property_u64(properties, &keys.admin_level), maritime: vector_tiles::property_flag(properties, &keys.maritime) }
    }

    fn feature_label(&self, properties: &std::collections::BTreeMap<String, String>) -> Option<String> {
        vector_tiles::feature_label(properties, &self.descriptor().feature_keys.label)
    }

    fn road_visible(&self, class: &str, span_deg: f64, band: usize) -> bool {
        let row = self.band(band);
        row.gates.roads && row.road_spans.as_ref().unwrap_or(&self.descriptor().road_spans).admits(class, span_deg)
    }

    fn boundary_visible(&self, admin_level: u64, gates: &MapLayerGates) -> bool {
        gates.boundaries && admin_level >= self.descriptor().boundary_admin_min && admin_level <= gates.max_admin_level
    }

    fn water_area_visible(&self, band: usize, class: &str) -> bool {
        self.band(band).water_area_classes.as_ref().is_none_or(|classes| classes.iter().any(|candidate| candidate == class))
    }

    fn waterway_visible(&self, band: usize) -> bool {
        self.band(band).waterways
    }

    fn admin_area_holes(&self, band: usize) -> bool {
        self.band(band).admin_area_holes
    }

    fn line_scale(&self, span_deg: f64) -> f64 {
        self.descriptor().line_scale.at(span_deg)
    }

    fn stroke_width(&self, role: MapLayerRole, feature: &MapFeatureFacts<'_>, line_scale: f64) -> f64 {
        let descriptor = self.descriptor();
        match role {
            MapLayerRole::Road => descriptor.road_widths.at(feature.class, line_scale),
            MapLayerRole::Boundary => descriptor.boundary_widths.at(feature.admin_level.unwrap_or(0), line_scale),
            MapLayerRole::Coastline => descriptor.coastline_width.at(line_scale),
            _ => line_scale,
        }
    }

    fn label_gate_and_rank(&self, layer: &str, class: &str, span_deg: f64) -> Option<u16> {
        let row = self.source_layer(layer)?;
        let labels = &self.descriptor().labels;
        let (visible, rank) = match row.role? {
            MapLayerRole::RoadLabel => (labels.road_label_spans.admits(class, span_deg), row.label_rank.unwrap_or_else(|| labels.road_label_ranks.rank(class))),
            MapLayerRole::PointLabel => (span_deg <= labels.point_label_max_span_deg, row.label_rank.unwrap_or_else(|| labels.place_label_ranks.rank(class))),
            MapLayerRole::PlaceLabel => (labels.place_admits(class, span_deg), row.label_rank.unwrap_or_else(|| labels.place_label_ranks.rank(class))),
            _ => return None,
        };
        visible.then_some(rank)
    }

    fn label_px(&self, band: usize, span_deg: f64, weight: f64) -> f64 {
        let labels = &self.descriptor().labels;
        let span = span_deg.max(labels.min_span_deg);
        let floor = if band == 0 { labels.span_floor_deg } else { self.band(band - 1).max_span_deg.max(labels.min_span_deg) };
        self.band(band).label_px * floor / span * weight
    }

    fn slider_keys(&self, band_id: &str, mode: MapTileMode) -> Vec<&str> {
        let index = self.band_index(band_id).unwrap_or(0);
        let row = self.band(index);
        let span = row.representative_span_deg;
        let mut tile_z = row.raster_tile_z;
        if index + 1 >= self.bands().len() {
            tile_z = tile_z.max(self.descriptor().vector_tile_z_max);
        }
        let gates = self.layer_gates(self.band_for_span(span, Some(band_id)), span, tile_z);
        let raster = matches!(mode, MapTileMode::Image | MapTileMode::Combined);
        let vector = matches!(mode, MapTileMode::Vector | MapTileMode::Combined);
        let painted = gates.water || gates.coastline || gates.land_cover || gates.land_backdrop || gates.roads || gates.buildings || gates.boundaries;
        let active = |target: &MapLayerTarget| match target {
            MapLayerTarget::Raster => raster,
            MapLayerTarget::Role(role) => {
                vector
                    && match role {
                        MapLayerRole::WaterArea | MapLayerRole::Waterway => gates.water || gates.coastline,
                        MapLayerRole::Coastline => gates.coastline || gates.water,
                        MapLayerRole::LandCover | MapLayerRole::GreenArea | MapLayerRole::AdminArea => gates.land_cover || gates.land_backdrop,
                        MapLayerRole::Road => gates.roads,
                        MapLayerRole::Building => gates.buildings,
                        MapLayerRole::Boundary => gates.boundaries,
                        MapLayerRole::RoadLabel | MapLayerRole::PointLabel | MapLayerRole::PlaceLabel => painted,
                    }
            }
            MapLayerTarget::Overlay(_) | MapLayerTarget::OverlayLabels(_) => true,
        };
        self.descriptor().layer_toggles.iter().filter(|toggle| toggle.targets.iter().any(active)).map(|toggle| toggle.key.as_str()).collect()
    }

    fn toggle_key(&self, target: &MapLayerTarget) -> Option<&str> {
        self.descriptor().layer_toggles.iter().find(|toggle| toggle.targets.contains(target)).map(|toggle| toggle.key.as_str())
    }

    fn overlay_layers(&self) -> &[MapOverlayLayer] {
        &self.descriptor().overlays
    }

    fn overlay_layer(&self, id: &str) -> Option<&MapOverlayLayer> {
        self.overlay_layers().iter().find(|layer| layer.id == id)
    }

    fn overlay_for_granularity(&self, granularity: &str) -> Option<&MapOverlayLayer> {
        self.overlay_layers().iter().find(|layer| layer.granularity.as_deref() == Some(granularity))
    }

    fn style_preset(&self, id: &str) -> &MapStylePreset {
        let presets = &self.descriptor().style_presets;
        presets.iter().find(|preset| preset.id == id).unwrap_or(&presets[0])
    }
}

/// 📜️ A [`MapPolicy`] that applies one descriptor as written; the neutral default when built from [`MapPolicyDescriptor::neutral`].
#[derive(Clone, Debug, PartialEq)]
pub struct DescribedMapPolicy {
    descriptor: MapPolicyDescriptor,
}

impl DescribedMapPolicy {
    /// 🧾️ Admits `descriptor`, substituting the neutral band table or style preset when it declares none.
    pub fn new(mut descriptor: MapPolicyDescriptor) -> Self {
        if descriptor.bands.is_empty() {
            descriptor.bands = MapPolicyDescriptor::neutral().bands;
        }
        if descriptor.style_presets.is_empty() {
            descriptor.style_presets = MapPolicyDescriptor::neutral().style_presets;
        }
        Self { descriptor }
    }
}

impl Default for DescribedMapPolicy {
    fn default() -> Self {
        Self::new(MapPolicyDescriptor::neutral())
    }
}

impl MapPolicy for DescribedMapPolicy {
    fn descriptor(&self) -> &MapPolicyDescriptor {
        &self.descriptor
    }
}
// #endregion 🔖️MapPolicy

// #region 🔖️JsonSurfaces
/// 📊️ JSON rows of the policy's bands for window LOD selects: id, name, description, max zoom, max span and raster tile z.
pub fn map_lod_scale_json(policy: &dyn MapPolicy) -> String {
    let rows: Vec<serde_json::Value> = policy
        .bands()
        .iter()
        .map(|band| {
            serde_json::json!({
                "id": band.id,
                "name": band.name,
                "description": band.description,
                "maxZoom": band.max_zoom,
                "maxSpanDeg": band.max_span_deg,
                "tileZ": band.raster_tile_z,
            })
        })
        .collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

/// 🎚️ JSON array of the layer ids that accept a weight slider at the band and tile render mode.
pub fn map_layer_weight_slider_ids_json(policy: &dyn MapPolicy, band_id: &str, render_mode: &str) -> String {
    serde_json::to_string(&policy.slider_keys(band_id, MapTileMode::parse(render_mode))).unwrap_or_else(|_| "[]".into())
}
// #endregion 🔖️JsonSurfaces

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
