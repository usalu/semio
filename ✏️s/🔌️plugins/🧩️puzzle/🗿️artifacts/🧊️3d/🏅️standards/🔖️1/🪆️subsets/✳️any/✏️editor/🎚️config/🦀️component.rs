//! 🎛️ Puzzle 3d play app — its `ArtifactApp::Config`: every piece of view state the app owns but the
//! document must never carry (selection, hover, per-window camera/grid/LOD/vortex/sun chrome, brush
//! and fill scratch, distribution weights, active utility/tool, locale/terminology, the live window
//! registry), plus the whole-snapshot `ConfigMutation` that patches it.
//!
//! 🪟️ The flat window-option fields are a scratch, currently-materialized-window working copy:
//! `load_window`/`save_window` swap them in and out of [`Puzzle3dRuntime::window_options`] around
//! every `render`/`window_measures`/`window_engagements`/`handle` call, so two window instances of
//! the same kind (e.g. split top/perspective panes) never share a preference. Fill count,
//! distribution weights and overlap budget deliberately stay flat and shared — they drive the one
//! document/precompute plan and split panes must never disagree about it.

use semio_framework_plugin::{WorldProjectionConfig, WorldSunConfig};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️Defaults
fn one_f64() -> f64 {
    1.0
}

fn default_true() -> bool {
    true
}

fn default_overlap_budget() -> f64 {
    0.02
}

fn default_manual_lod() -> f64 {
    100.0
}

fn default_grid_spacing() -> f64 {
    10.0
}

fn default_proximity_radius() -> f64 {
    0.75
}

fn default_chunk_size() -> f64 {
    256.0
}

fn default_voxel_dims() -> [u32; 3] {
    [1, 1, 1]
}

fn default_vortex_show() -> String {
    crate::editor::puzzle3d::PUZZLE3D_VORTEX_SHOW_SELECTED.into()
}

fn default_vortex_direction() -> String {
    crate::editor::puzzle3d::PUZZLE3D_VORTEX_DIRECTION_OUTWARDS.into()
}

fn default_terminology() -> String {
    "native".into()
}

fn default_locale() -> String {
    "en-US".into()
}

fn default_window_ids() -> Vec<String> {
    vec![crate::editor::puzzle3d::modes::edit::windows::main::WINDOW_KIND_ID.to_string()]
}
//#endregion 🔖️Defaults

//#region 🔖️Camera
/// 🎥️ Session-only per-window viewport camera — never a document field (see `setCamera`'s
/// `ActionKind::View`): orbiting one window instance must never move a sibling's camera and must
/// never create a VCS edit.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dCamera {
    #[serde(default)]
    pub position: [f64; 3],
    #[serde(default)]
    pub target: [f64; 3],
    #[serde(default = "one_f64")]
    pub zoom: f64,
    #[serde(default)]
    pub up: Option<[f64; 3]>,
    #[serde(default)]
    pub projection: WorldProjectionConfig,
}

/// 📐️ Distance from `camera.position` to `camera.target`, defaulting to the historic 30-unit orbit radius when degenerate.
pub fn puzzle3d_camera_distance(camera: &Puzzle3dCamera) -> f64 {
    let [dx, dy, dz] = [camera.position[0] - camera.target[0], camera.position[1] - camera.target[1], camera.position[2] - camera.target[2]];
    let distance = (dx * dx + dy * dy + dz * dz).sqrt();
    if distance > 1e-3 {
        distance
    } else {
        30.0
    }
}
//#endregion 🔖️Camera

//#region 🔖️Selection
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dSelectableKinds {
    #[serde(default = "default_true")]
    pub objects: bool,
    #[serde(default = "default_true")]
    pub vortices: bool,
    #[serde(default = "default_true")]
    pub attractions: bool,
}

impl Default for Puzzle3dSelectableKinds {
    fn default() -> Self {
        Self { objects: true, vortices: true, attractions: true }
    }
}

/// 🎯️ Open per-vortex brush-candidate suggestion popup (context menu / Alt+right-click).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dSuggestionMenu {
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    pub window_id: String,
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the target vortex full id this
    /// popup was opened on — previously implicit via `runtime.selection.vortex_ids`/
    /// `hovered_vortex_full_id`, now stored directly since selection is framework-owned and cannot be
    /// read back from `render` (see `puzzle3d_brush_target_vortex`'s doc comment).
    #[serde(default)]
    pub vortex_full_id: String,
}
//#endregion 🔖️Selection

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dConfig {
    #[serde(default)]
    pub suggestion_menu: Option<Puzzle3dSuggestionMenu>,
    #[serde(default = "default_overlap_budget")]
    pub overlap_budget: f64,
    #[serde(default)]
    pub fill_count: u32,
    #[serde(default)]
    pub brush_candidate_index: usize,
    #[serde(default)]
    pub object_kind_weights: HashMap<String, f64>,
    #[serde(default)]
    pub vortex_kind_weights: HashMap<String, f64>,
    #[serde(default = "default_true")]
    pub lod_automatic: bool,
    #[serde(default)]
    pub lod_depth_variable: bool,
    #[serde(default = "default_true")]
    pub grid_visible: bool,
    #[serde(default = "default_manual_lod")]
    pub lod_manual: f64,
    #[serde(default)]
    pub grid_snap_enabled: bool,
    #[serde(default = "default_grid_spacing")]
    pub grid_spacing: f64,
    #[serde(default)]
    pub selectable_kinds: Puzzle3dSelectableKinds,
    #[serde(default)]
    pub engagement_input: String,
    #[serde(default = "default_proximity_radius")]
    pub proximity_radius: f64,
    #[serde(default = "default_chunk_size")]
    pub chunk_size: f64,
    #[serde(default = "default_voxel_dims")]
    pub voxel_dims: [u32; 3],
    /// 🎛️ Whether the transform gumball exposes translate (move axes + move planes).
    #[serde(default = "default_true")]
    pub transform_move: bool,
    /// 🎛️ Whether the transform gumball exposes rotate handles.
    #[serde(default = "default_true")]
    pub transform_rotate: bool,
    /// 🌀️ When to emit vortex markers: `PUZZLE3D_VORTEX_SHOW_ALWAYS` or `PUZZLE3D_VORTEX_SHOW_SELECTED`.
    #[serde(default = "default_vortex_show")]
    pub vortex_show: String,
    /// 🧭️ How vortex direction arrows are drawn: `PUZZLE3D_VORTEX_DIRECTION_OUTWARDS` or `…_INWARDS`.
    #[serde(default = "default_vortex_direction")]
    pub vortex_direction: String,
    #[serde(default)]
    pub sun: WorldSunConfig,
    /// 🎥️ Session-only viewport camera for the window instance currently materialized onto this
    /// runtime (via `load_window`/`save_window`) — never persisted to the document.
    #[serde(default)]
    pub camera: Puzzle3dCamera,
    /// 🪟️ Per-window-instance snapshot of view-local chrome options, keyed by window INSTANCE id.
    #[serde(default)]
    pub window_options: BTreeMap<String, Puzzle3dWindowOptions>,
    /// 🧰️ B1: per-window active transform-gumball/brush/fill utility — was host-pushed
    /// `view_state.active_utility_by_window_id`, now real VCS'd config.
    #[serde(default)]
    pub active_utility_by_window_id: BTreeMap<String, String>,
    /// 🛠️ B1: the mode-level active tool (e.g. `"fill"`) — was host-pushed `view_state.active_tool_id`.
    #[serde(default)]
    pub active_tool_id: Option<String>,
    /// 🗣️ B1: terminology overlay (native/reuse) — was host-pushed `view_state.terminology`.
    #[serde(default = "default_terminology")]
    pub terminology: String,
    /// 🗣️ B1: BCP-47 locale tag — was host-pushed `view_state.locale`.
    #[serde(default = "default_locale")]
    pub locale: String,
    /// 🪟️ B1: every window INSTANCE id currently open for this app — was host-pushed
    /// `view_state.window_instances`. Always contains at least the main window id (see `Default`
    /// below) so a freshly-loaded document still engages its one window.
    #[serde(default = "default_window_ids")]
    pub window_ids: Vec<String>,
}

impl Default for Puzzle3dConfig {
    /// 🎛️ Mirrors every `#[serde(default = "...")]` above — `#[derive(Default)]` would silently ignore
    /// them and zero out fields like `overlap_budget`/`selection_method`/`lod_automatic` in Rust-constructed runtimes.
    fn default() -> Self {
        Self {
            suggestion_menu: None,
            overlap_budget: default_overlap_budget(),
            fill_count: 0,
            brush_candidate_index: 0,
            object_kind_weights: HashMap::new(),
            vortex_kind_weights: HashMap::new(),
            lod_automatic: default_true(),
            lod_depth_variable: false,
            grid_visible: default_true(),
            lod_manual: default_manual_lod(),
            grid_snap_enabled: false,
            grid_spacing: default_grid_spacing(),
            selectable_kinds: Puzzle3dSelectableKinds::default(),
            engagement_input: String::new(),
            proximity_radius: default_proximity_radius(),
            chunk_size: default_chunk_size(),
            voxel_dims: default_voxel_dims(),
            transform_move: default_true(),
            transform_rotate: default_true(),
            vortex_show: default_vortex_show(),
            vortex_direction: default_vortex_direction(),
            sun: WorldSunConfig::default(),
            camera: Puzzle3dCamera::default(),
            window_options: BTreeMap::new(),
            active_utility_by_window_id: BTreeMap::new(),
            active_tool_id: None,
            terminology: default_terminology(),
            locale: default_locale(),
            window_ids: default_window_ids(),
        }
    }
}

/// 🧮️ B1: puzzle3d's real `ArtifactApp::Config` — `Puzzle3dRuntime` itself doubles as the config
/// record (an alias, not a new type) so every helper taking `&Puzzle3dRuntime`/`&mut Puzzle3dRuntime`
/// keeps working unchanged; every read comes from `cfg.snapshot`, every write flows out as a
/// `Puzzle3dConfigMutation` in the returned `Emit` instead of a silent `self` mutation.
/// 🏷️ Alias kept for call sites that still name the runtime.
pub type Puzzle3dRuntime = Puzzle3dConfig;

impl store::ArtifactDsl for Puzzle3dConfig {
    const EXTENSION: &'static str = "puzzle3dcfg";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}

impl store::ArtifactPack for Puzzle3dConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(self).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map_err(store::PackError::Schema)
    }
}

store::impl_whole_record_config!(Puzzle3dConfig);
//#endregion 🔖️Config

//#region 🔖️WindowOptions
/// 🪟️ View-local chrome options a puzzle3d window exposes (grid, LOD, selection method/mode, vortex
/// display, sun, voxel steppers, camera) — stored per window INSTANCE in
/// [`Puzzle3dRuntime::window_options`]. Fill count, distribution weights and overlap budget are
/// intentionally absent: they drive the shared document/precompute plan.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle3dWindowOptions {
    pub lod_automatic: bool,
    pub lod_depth_variable: bool,
    pub grid_visible: bool,
    pub lod_manual: f64,
    pub grid_snap_enabled: bool,
    pub grid_spacing: f64,
    pub selectable_kinds: Puzzle3dSelectableKinds,
    pub engagement_input: String,
    pub proximity_radius: f64,
    pub chunk_size: f64,
    pub voxel_dims: [u32; 3],
    pub transform_move: bool,
    pub transform_rotate: bool,
    pub vortex_show: String,
    pub vortex_direction: String,
    pub sun: WorldSunConfig,
    pub camera: Puzzle3dCamera,
}

impl Default for Puzzle3dWindowOptions {
    fn default() -> Self {
        Self {
            lod_automatic: default_true(),
            lod_depth_variable: false,
            grid_visible: default_true(),
            lod_manual: default_manual_lod(),
            grid_snap_enabled: false,
            grid_spacing: default_grid_spacing(),
            selectable_kinds: Puzzle3dSelectableKinds::default(),
            engagement_input: String::new(),
            proximity_radius: default_proximity_radius(),
            chunk_size: default_chunk_size(),
            voxel_dims: default_voxel_dims(),
            transform_move: default_true(),
            transform_rotate: default_true(),
            vortex_show: default_vortex_show(),
            vortex_direction: default_vortex_direction(),
            sun: WorldSunConfig::default(),
            camera: Puzzle3dCamera::default(),
        }
    }
}

impl Puzzle3dConfig {
    /// 🪟️ Snapshots this runtime's currently-materialized flat window-option fields into a
    /// [`Puzzle3dWindowOptions`] — the counterpart to `apply_window_options`.
    fn snapshot_window_options(&self) -> Puzzle3dWindowOptions {
        Puzzle3dWindowOptions {
            lod_automatic: self.lod_automatic,
            lod_depth_variable: self.lod_depth_variable,
            grid_visible: self.grid_visible,
            lod_manual: self.lod_manual,
            grid_snap_enabled: self.grid_snap_enabled,
            grid_spacing: self.grid_spacing,
            selectable_kinds: self.selectable_kinds.clone(),
            engagement_input: self.engagement_input.clone(),
            proximity_radius: self.proximity_radius,
            chunk_size: self.chunk_size,
            voxel_dims: self.voxel_dims,
            transform_move: self.transform_move,
            transform_rotate: self.transform_rotate,
            vortex_show: self.vortex_show.clone(),
            vortex_direction: self.vortex_direction.clone(),
            sun: self.sun.clone(),
            camera: self.camera.clone(),
        }
    }

    /// 🪟️ Materializes `options` onto this runtime's flat window-option fields — the counterpart to
    /// `snapshot_window_options`. Leaves fill count / distribution / overlap untouched so a pane
    /// switch cannot rewrite the shared fill scene.
    fn apply_window_options(&mut self, options: &Puzzle3dWindowOptions) {
        self.lod_automatic = options.lod_automatic;
        self.lod_depth_variable = options.lod_depth_variable;
        self.grid_visible = options.grid_visible;
        self.lod_manual = options.lod_manual;
        self.grid_snap_enabled = options.grid_snap_enabled;
        self.grid_spacing = options.grid_spacing;
        self.selectable_kinds = options.selectable_kinds.clone();
        self.engagement_input = options.engagement_input.clone();
        self.proximity_radius = options.proximity_radius;
        self.chunk_size = options.chunk_size;
        self.voxel_dims = options.voxel_dims;
        self.transform_move = options.transform_move;
        self.transform_rotate = options.transform_rotate;
        self.vortex_show = options.vortex_show.clone();
        self.vortex_direction = options.vortex_direction.clone();
        self.sun = options.sun.clone();
        self.camera = options.camera.clone();
    }

    /// 🪟️ Materializes `window_id`'s stored options (the type default, for a window never touched yet)
    /// onto this runtime's flat fields — call before building a `Puzzle3dScene` for that window, in
    /// every read (`render`/`window_engagements`/`window_measures`) and write (`handle`) path.
    pub fn load_window(&mut self, window_id: &str) {
        let options = self.window_options.get(window_id).cloned().unwrap_or_default();
        self.apply_window_options(&options);
    }

    /// 🪟️ Snapshots this runtime's current flat view-local option fields (as left by whatever action
    /// just ran) back into `window_id`'s stored entry. Other windows' entries are untouched, so a
    /// `setGridVisible` in one window instance never affects another's.
    pub fn save_window(&mut self, window_id: &str) {
        let options = self.snapshot_window_options();
        self.window_options.insert(window_id.to_string(), options);
    }
}
//#endregion 🔖️WindowOptions

//#region 🔖️ConfigMutation
/// 🧮️ B1: `Puzzle3dConfig`'s operation enum. Every real config edit is captured as "the whole config
/// after this edit"; `backwards()` is the same one-liner regardless of what changed ("restore the
/// whole-config snapshot from just before"), so no per-field inverse bookkeeping is needed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Puzzle3dConfigMutation {
    Snapshot { config: Puzzle3dConfig },
}

impl protocol::Mutation<Puzzle3dConfig> for Puzzle3dConfigMutation {
    type Diff = Puzzle3dConfig;

    fn diff(&self, _base: &Puzzle3dConfig) -> protocol::MutationOutcome<Puzzle3dConfig> {
        protocol::MutationOutcome::new(match self {
            Puzzle3dConfigMutation::Snapshot { config } => config.clone(),
        })
    }

    fn inverse(&self, base: &Puzzle3dConfig) -> Vec<Self> {
        vec![Puzzle3dConfigMutation::Snapshot { config: base.clone() }]
    }
}

impl protocol::OpBinary for Puzzle3dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

impl protocol::OpText for Puzzle3dConfigMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️ConfigMutation
