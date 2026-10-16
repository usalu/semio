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
    /// 🧩️ (d) runtime wiring — the domain extension whose [`MapPolicy`] decides bands, tile z, paint roles and labels.
    extension: Arc<dyn MapExtension>,
    /// 🔢️ (d) bumps whenever `extension` is replaced, so the vector-scene cache never reuses a scene built by another policy.
    policy_revision: u64,
    /// 🗺️ (a) elsewhere, not here — see the module docstring: mirrors the owning document's overlay
    /// features, whose plugin carries the mutation vocabulary. Refreshed wholesale by
    /// [`MapHost::sync_map_json`]; never mutated field-by-field here.
    pub overlays: MapOverlayTables,
    /// 🧱️ (d) ephemeral working representation — decoded raster/vector tile cache, evictable LRU,
    /// wholly rebuildable from tile coordinates + network fetch, analogous to `🏔️terrain`'s
    /// out-of-doctrine external-resource cache (network bytes, not derivable from any snapshot).
    pub(super) tiles: MapTileLedger,
    /// 🎛️ (c) Preview/Effect — mirrors the owning plugin's render-mode window command.
    render_mode: MapTileMode,
    /// 🎨️ (c) Preview/Effect — requested vector style preset id, mirroring the owning plugin's vector-style window command; an id the policy does not declare paints its first preset.
    vector_style: String,
    /// 🔍️ (c) Preview/Effect — requested pinned band id; it pins only while the policy declares that band.
    forced_lod_id: Option<String>,
    /// 👁️ (c) Preview/Effect — per-toggle visibility and weight from the owning plugin's window configuration, keyed by the policy's layer toggles.
    pub(super) layer_switches: MapLayerSwitches,
    /// 📡️ (c) Preview/Effect — interaction event log drained each frame; not persisted, not a document.
    events: Vec<serde_json::Value>,
    /// 🖐️ (c) Preview/Effect — current pan gesture state, discarded on release.
    interaction: MapInteraction,
    /// 🎨️ (d) runtime wiring — color palette derived from the app's UI theme.
    pub(super) theme: MapPalette,
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3c: (c) Preview/Effect —
    /// selected feature ids per overlay granularity, read from the framework's `DomainSelection` via
    /// [`MapHost::sync_interaction`], not pushed by the app anymore.
    pub(super) selected: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    /// 🕹️ (c) Preview/Effect — hover feedback kind (an overlay granularity), UI-only.
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
    overlays: std::collections::BTreeMap<String, std::collections::BTreeMap<String, MapOverlayFeature>>,
    tile_images: std::collections::BTreeMap<String, Arc<RasterImage>>,
    vector_tiles: std::collections::BTreeMap<String, vector_tiles::VectorTile>,
    raster_touch: std::collections::BTreeMap<String, u64>,
    vector_touch: std::collections::BTreeMap<String, u64>,
    forced_lod_id: Option<String>,
    vector_style: String,
    layer_visibility: std::collections::BTreeMap<String, bool>,
    layer_weights: std::collections::BTreeMap<String, f64>,
    events: Vec<serde_json::Value>,
    selected: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    hovered_kind: Option<String>,
    hovered_id: Option<String>,
    released: bool,
}

impl MapHostRetirement {
    pub fn new(host: MapHost) -> Self {
        let MapHost {
            camera: _,
            viewport: _,
            extension: _,
            policy_revision: _,
            overlays: MapOverlayTables { layers: overlays },
            tiles: MapTileLedger { tile_images, vector_tiles, raster_touch, vector_touch, touch_clock: _ },
            render_mode: _,
            vector_style,
            forced_lod_id,
            layer_switches: MapLayerSwitches { visibility: layer_visibility, weights: layer_weights },
            events,
            interaction: _,
            theme: _,
            selected,
            hovered_kind,
            hovered_id,
            interaction_revision: _,
            vector_scene_cache: _,
            vector_scene_rebuild_count: _,
        } = host;
        Self { overlays, tile_images, vector_tiles, raster_touch: raster_touch.into_inner(), vector_touch: vector_touch.into_inner(), forced_lod_id, vector_style, layer_visibility, layer_weights, events, selected, hovered_kind, hovered_id, released: false }
    }

    pub fn close_step(&mut self) -> bool {
        if self.released {
            return true;
        }
        if let Some(mut layer) = self.overlays.first_entry() {
            if layer.get_mut().pop_first().is_none() {
                layer.remove();
            }
            return false;
        }
        if let Some(mut granularity) = self.selected.first_entry() {
            if granularity.get_mut().pop_first().is_none() {
                granularity.remove();
            }
            return false;
        }
        if self.tile_images.pop_first().is_some()
            || self.vector_tiles.pop_first().is_some()
            || self.raster_touch.pop_first().is_some()
            || self.vector_touch.pop_first().is_some()
            || self.events.pop().is_some()
            || self.layer_visibility.pop_first().is_some()
            || self.layer_weights.pop_first().is_some()
        {
            return false;
        }
        if self.forced_lod_id.as_mut().is_some_and(|value| value.pop().is_some()) || self.vector_style.pop().is_some() || self.hovered_kind.as_mut().is_some_and(|value| value.pop().is_some()) || self.hovered_id.as_mut().is_some_and(|value| value.pop().is_some()) {
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
            && self.overlays.is_empty()
            && self.tile_images.is_empty()
            && self.vector_tiles.is_empty()
            && self.raster_touch.is_empty()
            && self.vector_touch.is_empty()
            && self.forced_lod_id.is_none()
            && self.vector_style.is_empty()
            && self.layer_visibility.is_empty()
            && self.layer_weights.is_empty()
            && self.events.is_empty()
            && self.selected.is_empty()
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
            extension: Arc::new(DefaultMapExtension::default()),
            policy_revision: 0,
            overlays: MapOverlayTables::default(),
            tiles: MapTileLedger::default(),
            render_mode: MapTileMode::Combined,
            vector_style: String::new(),
            forced_lod_id: None,
            layer_switches: MapLayerSwitches::default(),
            events: Vec::new(),
            interaction: MapInteraction::None,
            theme: MapPalette::default(),
            selected: std::collections::BTreeMap::new(),
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
