//! 🏛️ The tiled-map host: struct, field-wise retirement owner, pan interaction state and default construction.
//!
//! The two `impl MapHost` halves are nested children so the private fields stay visible to them: camera and picking, and the host scene.

use super::*;

#[path = "📷️camera-and-picking/🦀️.rs"]
mod camera_and_picking;
#[path = "🖼️host-scene/🦀️.rs"]
mod host_scene;

// #region 🔖️MapHost
// #region 🔖️MapContent
pub struct MapHost {
    /// 🖱️ (c) Preview/Effect — live viewport camera during pan/zoom, never dispatched at frame rate.
    pub camera: camera::Camera,
    /// 🖥️ (d) Render-session wiring — device viewport size/DPR, not document content.
    pub viewport: camera::Viewport,
    /// 🗺️ (a) elsewhere, not here — see the module docstring: mirrors the gis plugin's real
    /// `GismapSnapshot` positions/routes/regions, which already carries the full 12-triad mutation
    /// vocabulary. Refreshed wholesale by [`MapHost::sync_map_json`]; never mutated field-by-field here.
    pub features: MapFeatureTables,
    /// 🧱️ (d) ephemeral working representation — decoded raster/vector tile cache, evictable LRU,
    /// wholly rebuildable from tile coordinates + network fetch, analogous to `🏔️terrain`'s
    /// out-of-doctrine external-resource cache (network bytes, not derivable from any snapshot).
    pub(super) tiles: MapTileLedger,
    /// 🎛️ (c) Preview/Effect — mirrors the gis plugin's `Gis2dConfig::SetRenderMode` LOCAL_UI command.
    render_mode: MapTileMode,
    /// 🎨️ (c) Preview/Effect — mirrors `Gis2dConfig::SetVectorStyle`, plugin-owned.
    vector_style: MapVectorStyle,
    /// 🔍️ (c) Preview/Effect — pinned LOD override, mirrors `Gis2dConfig::set_lod_mode`.
    forced_lod_id: Option<String>,
    /// 👁️ (c) Preview/Effect — per-layer visibility toggles, mirrors `Gis2dConfig::ToggleLayerVisibility`.
    pub(super) layer_visibility: MapLayerVisibility,
    /// 📏️ (c) Preview/Effect — per-layer stroke-weight multipliers, mirrors `Gis2dConfig::SetLayerStrokeScale`.
    pub(super) layer_stroke_scale: MapLayerStrokeScale,
    /// 📡️ (c) Preview/Effect — interaction event log drained each frame; not persisted, not a document.
    events: Vec<serde_json::Value>,
    /// 🖐️ (c) Preview/Effect — current pan gesture state, discarded on release.
    interaction: MapInteraction,
    /// 🎨️ (d) runtime wiring — color palette derived from the app's UI theme.
    pub(super) theme: MapPalette,
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3c: (c) Preview/Effect —
    /// selection state for positions, read from the framework's `DomainSelection` (granularity
    /// `"position"`) via [`MapHost::sync_interaction`], not pushed by the app anymore.
    pub(super) selected_positions: std::collections::BTreeSet<String>,
    /// 🕹️ (c) Preview/Effect — selection state for routes, `DomainSelection` granularity `"route"`.
    pub(super) selected_routes: std::collections::BTreeSet<String>,
    /// 🕹️ (c) Preview/Effect — hover feedback kind (position/route), UI-only.
    pub(super) hovered_kind: Option<String>,
    /// 🕹️ (c) Preview/Effect — hover feedback entity id, UI-only.
    pub(super) hovered_id: Option<String>,
    interaction_revision: u64,
    /// 🧊️ (d) ephemeral frame-cache — last built vector-tile-layer `Scene`, reused via a screen-space
    /// translate on a pure pan; any other camera/style/data change invalidates via `vector_scene_cache_key`.
    vector_scene_cache: RefCell<Option<CachedVectorScene>>,
    /// 🔢️ (d) diagnostics — bumps once per real vector-scene rebuild (cache miss); tests assert on it.
    pub(super) vector_scene_rebuild_count: std::cell::Cell<u64>,
}

/// 🧊️ Last vector-tile-layer `Scene` this host built, plus the inputs it was built from — a hit on
/// `key` with an unchanged camera reuses `scene` as-is, a hit with only `camera.x`/`camera.y` moved
/// reuses it translated, anything else triggers a rebuild.
struct CachedVectorScene {
    scene: Scene,
    key: u64,
    anchor_x: f64,
    anchor_y: f64,
}

/// 🧹️ Retained tiled-map owner that releases one admitted feature, tile, event, or text scalar per grant.
pub struct MapHostRetirement {
    positions: std::collections::BTreeMap<String, PositionData>,
    routes: std::collections::BTreeMap<String, RouteData>,
    regions: std::collections::BTreeMap<String, RegionData>,
    tile_images: std::collections::BTreeMap<String, Arc<RasterImage>>,
    vector_tiles: std::collections::BTreeMap<String, vector_tiles::VectorTile>,
    raster_touch: std::collections::BTreeMap<String, u64>,
    vector_touch: std::collections::BTreeMap<String, u64>,
    forced_lod_id: Option<String>,
    events: Vec<serde_json::Value>,
    selected_positions: std::collections::BTreeSet<String>,
    selected_routes: std::collections::BTreeSet<String>,
    hovered_kind: Option<String>,
    hovered_id: Option<String>,
    released: bool,
}

impl MapHostRetirement {
    pub fn new(host: MapHost) -> Self {
        let MapHost {
            camera: _,
            viewport: _,
            features: MapFeatureTables { positions, routes, regions },
            tiles: MapTileLedger { tile_images, vector_tiles, raster_touch, vector_touch, touch_clock: _ },
            render_mode: _,
            vector_style: _,
            forced_lod_id,
            layer_visibility: _,
            layer_stroke_scale: _,
            events,
            interaction: _,
            theme: _,
            selected_positions,
            selected_routes,
            hovered_kind,
            hovered_id,
            interaction_revision: _,
            vector_scene_cache: _,
            vector_scene_rebuild_count: _,
        } = host;
        Self { positions, routes, regions, tile_images, vector_tiles, raster_touch: raster_touch.into_inner(), vector_touch: vector_touch.into_inner(), forced_lod_id, events, selected_positions, selected_routes, hovered_kind, hovered_id, released: false }
    }

    pub fn close_step(&mut self) -> bool {
        if self.released {
            return true;
        }
        if self.positions.pop_first().is_some()
            || self.routes.pop_first().is_some()
            || self.regions.pop_first().is_some()
            || self.tile_images.pop_first().is_some()
            || self.vector_tiles.pop_first().is_some()
            || self.raster_touch.pop_first().is_some()
            || self.vector_touch.pop_first().is_some()
            || self.events.pop().is_some()
            || self.selected_positions.pop_first().is_some()
            || self.selected_routes.pop_first().is_some()
        {
            return false;
        }
        if self.forced_lod_id.as_mut().is_some_and(|value| value.pop().is_some()) || self.hovered_kind.as_mut().is_some_and(|value| value.pop().is_some()) || self.hovered_id.as_mut().is_some_and(|value| value.pop().is_some()) {
            return false;
        }
        self.forced_lod_id = None;
        self.hovered_kind = None;
        self.hovered_id = None;
        self.released = true;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.released
            && self.positions.is_empty()
            && self.routes.is_empty()
            && self.regions.is_empty()
            && self.tile_images.is_empty()
            && self.vector_tiles.is_empty()
            && self.raster_touch.is_empty()
            && self.vector_touch.is_empty()
            && self.forced_lod_id.is_none()
            && self.events.is_empty()
            && self.selected_positions.is_empty()
            && self.selected_routes.is_empty()
            && self.hovered_kind.is_none()
            && self.hovered_id.is_none()
    }
}

impl Drop for MapHostRetirement {
    fn drop(&mut self) {
        debug_assert!(self.terminal_is_empty(), "MapHostRetirement must reach terminal-empty before release");
    }
}

#[derive(Clone, Debug, Default)]
pub(super) enum MapInteraction {
    #[default]
    None,
    Pan {
        origin: camera::Camera,
        start_screen: Point,
    },
}

impl Default for MapHost {
    fn default() -> Self {
        let viewport = camera::Viewport::default();
        Self {
            camera: projection::default_world_camera(&viewport),
            viewport,
            features: MapFeatureTables::default(),
            tiles: MapTileLedger::default(),
            render_mode: MapTileMode::Combined,
            vector_style: MapVectorStyle::Colored,
            forced_lod_id: None,
            layer_visibility: MapLayerVisibility::default(),
            layer_stroke_scale: MapLayerStrokeScale::default(),
            events: Vec::new(),
            interaction: MapInteraction::None,
            theme: MapPalette::default(),
            selected_positions: std::collections::BTreeSet::new(),
            selected_routes: std::collections::BTreeSet::new(),
            hovered_kind: None,
            hovered_id: None,
            interaction_revision: 0,
            vector_scene_cache: RefCell::new(None),
            vector_scene_rebuild_count: std::cell::Cell::new(0),
        }
    }
}
// #endregion 🔖️MapContent
// #endregion 🔖️MapHost
