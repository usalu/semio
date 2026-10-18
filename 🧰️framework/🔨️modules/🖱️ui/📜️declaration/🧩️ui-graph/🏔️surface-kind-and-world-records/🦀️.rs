//! 🏔️ Surface kind and world records: the `SurfaceKind` and the world mesh lod, lod, chunking and camera records with their json builders. See [`super`].

use super::*;

// #region ui
//#region 🔖️ComponentScenes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
pub enum SurfaceKind {
    #[serde(rename = "canvas-2d")]
    #[value(rename = "canvas-2d")]
    Canvas2d,
    #[serde(rename = "world-3d")]
    #[value(rename = "world-3d")]
    World3d,
    #[serde(rename = "node-graph")]
    #[value(rename = "node-graph")]
    NodeGraph,
    #[serde(rename = "text-editor")]
    #[value(rename = "text-editor")]
    TextEditor,
    #[serde(rename = "table")]
    #[value(rename = "table")]
    Table,
    #[serde(rename = "paint-2d")]
    #[value(rename = "paint-2d")]
    Paint2d,
    #[serde(rename = "virtual-file-system")]
    #[value(rename = "virtual-file-system")]
    VirtualFileSystem,
    #[serde(rename = "tiled-map")]
    #[value(rename = "tiled-map")]
    TiledMap,
    #[serde(rename = "board-2d")]
    #[value(rename = "board-2d")]
    Board2d,
    #[serde(rename = "icon-render")]
    #[value(rename = "icon-render")]
    IconRender,
    #[serde(rename = "ink-canvas")]
    #[value(rename = "ink-canvas")]
    InkCanvas,
    #[serde(rename = "graph-timeline")]
    #[value(rename = "graph-timeline")]
    GraphTimeline,
    #[serde(rename = "block-list")]
    #[value(rename = "block-list")]
    BlockList,
    #[serde(rename = "diff-view")]
    #[value(rename = "diff-view")]
    DiffView,
    #[serde(rename = "event-feed")]
    #[value(rename = "event-feed")]
    EventFeed,
}

impl SurfaceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canvas2d => "canvas-2d",
            Self::World3d => "world-3d",
            Self::NodeGraph => "node-graph",
            Self::TextEditor => "text-editor",
            Self::Table => "table",
            Self::Paint2d => "paint-2d",
            Self::VirtualFileSystem => "virtual-file-system",
            Self::TiledMap => "tiled-map",
            Self::Board2d => "board-2d",
            Self::IconRender => "icon-render",
            Self::InkCanvas => "ink-canvas",
            Self::GraphTimeline => "graph-timeline",
            Self::BlockList => "block-list",
            Self::DiffView => "diff-view",
            Self::EventFeed => "event-feed",
        }
    }

    pub fn is_viewport(self) -> bool {
        matches!(self, Self::World3d | Self::NodeGraph | Self::Canvas2d | Self::Board2d | Self::InkCanvas)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldMeshLodEntry {
    pub lod: f64,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldLodRecord {
    #[serde(default = "default_true")]
    #[value(default = "default_true")]
    pub automatic: bool,
    #[serde(default = "default_manual_lod")]
    #[value(default = "default_manual_lod")]
    pub manual: f64,
    #[serde(default = "default_distance_reference")]
    #[value(default = "default_distance_reference")]
    pub distance_reference: f64,
    #[serde(default)]
    #[value(default)]
    pub depth_variable: bool,
    #[serde(default = "default_grid_factor")]
    #[value(default = "default_grid_factor")]
    pub grid_factor: f64,
    #[serde(default)]
    #[value(default)]
    pub grid_snap_enabled: bool,
    #[serde(default = "default_true")]
    #[value(default = "default_true")]
    pub show_grid: bool,
    #[serde(default)]
    #[value(default)]
    pub grid_datum: Option<[f64; 3]>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WorldChunkingRecord {
    pub chunk_size: f64,
    pub max_distance: f64,
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
fn default_manual_lod() -> f64 {
    100.0
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
fn default_distance_reference() -> f64 {
    100.0
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
fn default_grid_factor() -> f64 {
    10.0
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
fn default_true() -> bool {
    true
}

pub fn world3d_default_lod_json() -> String {
    serde_json::json!({
        "automatic": true,
        "manual": 100.0,
        "distanceReference": 100.0,
        "depthVariable": false,
        "gridFactor": 10.0,
        "gridSnapEnabled": false,
        "showGrid": true,
        "gridDatum": [0.0, 0.0, 0.0],
    })
    .to_string()
}

pub fn world3d_chunking_json(chunk_size: f64, max_distance: f64) -> String {
    serde_json::json!({
        "chunkSize": chunk_size,
        "maxDistance": max_distance,
    })
    .to_string()
}

/// 🎯️ The world-3d `fit` lane: ask the render host to frame the scene's own bounds ONCE per
/// `revision`, preserving the current view direction.
///
/// `revision` is a producer-owned document identity, not a frame counter: `WorldAutoFit`
/// (`📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost`) keys its one-shot on
/// `${revision}:${meshes}` and never refits again while that key holds, so a producer that
/// derives the revision from what the DOCUMENT is (its catalogs, its domain) refits exactly on a
/// document swap, while a producer that derived it from the geometry would yank the camera on
/// every edit. Without a fit lane the camera after a document swap is whatever the previous
/// document left behind — a fixture centred elsewhere is simply off-screen (ticket 26/09/02 W-P5
/// §7, W-S2).
/// 📦️ `bounds` is the producer's OWN delivered extent, published rather than rediscovered: a
/// render host that measures its scene graph instead can only frame what it has already
/// uploaded, and the wgpu surface has no scene graph to measure at all — so without this lane
/// the two renderers frame differently by construction
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
pub fn world3d_fit_json(revision: u32, padding: f64, bounds: Option<([f64; 3], [f64; 3])>) -> String {
    match bounds {
        Some((minimum, maximum)) => serde_json::json!({ "enabled": true, "revision": revision, "padding": padding, "boundsMin": minimum, "boundsMax": maximum }).to_string(),
        None => serde_json::json!({ "enabled": true, "revision": revision, "padding": padding }).to_string(),
    }
}

// 🎬️ `world3d_default_selection_json`/`world3d_default_meshes_json` moved with `World3dScene`
// itself into `semio-framework-ui-scene` (they back its `#[serde(default = ...)]` fields) — no
// longer defined here; nothing else in this file called them directly.

pub fn world3d_camera_json(position: [f64; 3], target: [f64; 3], fov: f64) -> String {
    serde_json::json!({
        "position": position,
        "target": target,
        "up": [0.0, 0.0, 1.0],
        "fov": fov,
    })
    .to_string()
}
//#endregion 🔖️ComponentScenes
// #endregion ui
