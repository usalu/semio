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
    /// 🧵 Invalidates stale fill-materialization continuations after a newer slider request.
    #[serde(default)]
    pub fill_apply_generation: u64,
    /// 🪣️ Persisted prefix cursor, kept separate so each continuation need not reserialize the full plan.
    #[serde(default)]
    pub fill_applied_count: u32,
    /// 🧵 Worker-independent checkpoint for the bounded fill planner. This travels with the
    /// config snapshot so successive commands may execute on any shared-pool worker.
    #[serde(default)]
    pub fill_checkpoint: Vec<u8>,
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
            fill_apply_generation: 0,
            fill_applied_count: 0,
            fill_checkpoint: Vec::new(),
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
/// 🧮️ `Puzzle3dConfig` operations, with compact fill-cursor mutations for resumable hot paths.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Puzzle3dConfigMutation {
    Snapshot { config: Puzzle3dConfig },
    SetFillRequest { count: u32, generation: u64 },
    SetFillAppliedCount { count: u32 },
    SetOverlapBudget { value: f64 },
    SetObjectKindWeights { value: HashMap<String, f64> },
    SetVortexKindWeights { value: HashMap<String, f64> },
    SetWindowCamera { window_id: String, camera: Puzzle3dCamera },
    SetWindowSun { window_id: String, sun: WorldSunConfig },
    SetWindowLodAutomatic { window_id: String, value: bool },
    SetWindowLodDepthVariable { window_id: String, value: bool },
    SetWindowLodManual { window_id: String, value: f64 },
    SetWindowGridVisible { window_id: String, value: bool },
    SetWindowGridSnapEnabled { window_id: String, value: bool },
    SetWindowGridSpacing { window_id: String, value: f64 },
    SetWindowSelectableKinds { window_id: String, value: Puzzle3dSelectableKinds },
    SetWindowProximityRadius { window_id: String, value: f64 },
    SetWindowChunkSize { window_id: String, value: f64 },
    SetWindowVoxelDims { window_id: String, value: [u32; 3] },
    SetWindowTransformMove { window_id: String, value: bool },
    SetWindowTransformRotate { window_id: String, value: bool },
    SetWindowVortexShow { window_id: String, value: String },
    SetWindowVortexDirection { window_id: String, value: String },
    SetSuggestionMenu { value: Option<Puzzle3dSuggestionMenu> },
    SetBrushCandidateIndex { value: usize },
    SetWindowEngagementInput { window_id: String, value: String },
    SetActiveUtility { window_id: String, value: Option<String> },
    SetLocale { value: String },
    SetTerminology { value: String },
}

fn mutate_window_options(base: &Puzzle3dConfig, window_id: &str, mutate: impl FnOnce(&mut Puzzle3dWindowOptions)) -> Puzzle3dConfig {
    let mut next = base.clone();
    let mut options = base.window_options.get(window_id).cloned().unwrap_or_default();
    mutate(&mut options);
    next.apply_window_options(&options);
    next.window_options.insert(window_id.to_string(), options);
    next
}

fn window_options(base: &Puzzle3dConfig, window_id: &str) -> Puzzle3dWindowOptions {
    base.window_options.get(window_id).cloned().unwrap_or_default()
}

impl protocol::Mutation<Puzzle3dConfig> for Puzzle3dConfigMutation {
    type Diff = Puzzle3dConfig;

    /// 🧷️ Hand-written (not `#[derive(dsl::Mutations)]`: this enum predates the derive and its
    /// `diff`/`inverse` dispatch is a plain `match`, not the derive's per-leaf `MutationKind`
    /// shape). One entry per variant, in declaration order. ⚠️ PROVISIONAL: none of these variants
    /// has an authored leaf directory on disk yet — every `owner` below names a path that does not
    /// exist, matching stdio's own `⚠️ PROVISIONAL` precedent (e.g. `wav`'s `WavMutation`).
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📄set-snapshot", semantic_kind: "set-snapshot", display_name: "Set Snapshot", emoji: "📄", aggregate_variant: "Snapshot", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧮set-fill-request", semantic_kind: "set-fill-request", display_name: "Set Fill Request", emoji: "🧮", aggregate_variant: "SetFillRequest", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/✅set-fill-applied-count", semantic_kind: "set-fill-applied-count", display_name: "Set Fill Applied Count", emoji: "✅", aggregate_variant: "SetFillAppliedCount", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📊set-overlap-budget", semantic_kind: "set-overlap-budget", display_name: "Set Overlap Budget", emoji: "📊", aggregate_variant: "SetOverlapBudget", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚖️set-object-kind-weights", semantic_kind: "set-object-kind-weights", display_name: "Set Object Kind Weights", emoji: "⚖️", aggregate_variant: "SetObjectKindWeights", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🌀set-vortex-kind-weights", semantic_kind: "set-vortex-kind-weights", display_name: "Set Vortex Kind Weights", emoji: "🌀", aggregate_variant: "SetVortexKindWeights", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📷set-window-camera", semantic_kind: "set-window-camera", display_name: "Set Window Camera", emoji: "📷", aggregate_variant: "SetWindowCamera", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/☀️set-window-sun", semantic_kind: "set-window-sun", display_name: "Set Window Sun", emoji: "☀️", aggregate_variant: "SetWindowSun", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🔍set-window-lod-automatic", semantic_kind: "set-window-lod-automatic", display_name: "Set Window Lod Automatic", emoji: "🔍", aggregate_variant: "SetWindowLodAutomatic", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📐set-window-lod-depth-variable", semantic_kind: "set-window-lod-depth-variable", display_name: "Set Window Lod Depth Variable", emoji: "📐", aggregate_variant: "SetWindowLodDepthVariable", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🎚️set-window-lod-manual", semantic_kind: "set-window-lod-manual", display_name: "Set Window Lod Manual", emoji: "🎚️", aggregate_variant: "SetWindowLodManual", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🔲set-window-grid-visible", semantic_kind: "set-window-grid-visible", display_name: "Set Window Grid Visible", emoji: "🔲", aggregate_variant: "SetWindowGridVisible", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧲set-window-grid-snap-enabled", semantic_kind: "set-window-grid-snap-enabled", display_name: "Set Window Grid Snap Enabled", emoji: "🧲", aggregate_variant: "SetWindowGridSnapEnabled", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📏set-window-grid-spacing", semantic_kind: "set-window-grid-spacing", display_name: "Set Window Grid Spacing", emoji: "📏", aggregate_variant: "SetWindowGridSpacing", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🎯set-window-selectable-kinds", semantic_kind: "set-window-selectable-kinds", display_name: "Set Window Selectable Kinds", emoji: "🎯", aggregate_variant: "SetWindowSelectableKinds", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📡set-window-proximity-radius", semantic_kind: "set-window-proximity-radius", display_name: "Set Window Proximity Radius", emoji: "📡", aggregate_variant: "SetWindowProximityRadius", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧱set-window-chunk-size", semantic_kind: "set-window-chunk-size", display_name: "Set Window Chunk Size", emoji: "🧱", aggregate_variant: "SetWindowChunkSize", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧊set-window-voxel-dims", semantic_kind: "set-window-voxel-dims", display_name: "Set Window Voxel Dims", emoji: "🧊", aggregate_variant: "SetWindowVoxelDims", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/✋set-window-transform-move", semantic_kind: "set-window-transform-move", display_name: "Set Window Transform Move", emoji: "✋", aggregate_variant: "SetWindowTransformMove", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🔄set-window-transform-rotate", semantic_kind: "set-window-transform-rotate", display_name: "Set Window Transform Rotate", emoji: "🔄", aggregate_variant: "SetWindowTransformRotate", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/👁️set-window-vortex-show", semantic_kind: "set-window-vortex-show", display_name: "Set Window Vortex Show", emoji: "👁️", aggregate_variant: "SetWindowVortexShow", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧭set-window-vortex-direction", semantic_kind: "set-window-vortex-direction", display_name: "Set Window Vortex Direction", emoji: "🧭", aggregate_variant: "SetWindowVortexDirection", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/💡set-suggestion-menu", semantic_kind: "set-suggestion-menu", display_name: "Set Suggestion Menu", emoji: "💡", aggregate_variant: "SetSuggestionMenu", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️set-brush-candidate-index", semantic_kind: "set-brush-candidate-index", display_name: "Set Brush Candidate Index", emoji: "🖌️", aggregate_variant: "SetBrushCandidateIndex", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⌨️set-window-engagement-input", semantic_kind: "set-window-engagement-input", display_name: "Set Window Engagement Input", emoji: "⌨️", aggregate_variant: "SetWindowEngagementInput", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🛠️set-active-utility", semantic_kind: "set-active-utility", display_name: "Set Active Utility", emoji: "🛠️", aggregate_variant: "SetActiveUtility", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🌐set-locale", semantic_kind: "set-locale", display_name: "Set Locale", emoji: "🌐", aggregate_variant: "SetLocale", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📖set-terminology", semantic_kind: "set-terminology", display_name: "Set Terminology", emoji: "📖", aggregate_variant: "SetTerminology", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Puzzle3dConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
            Puzzle3dConfigMutation::SetFillRequest { .. } => &Self::DESCRIPTORS[1],
            Puzzle3dConfigMutation::SetFillAppliedCount { .. } => &Self::DESCRIPTORS[2],
            Puzzle3dConfigMutation::SetOverlapBudget { .. } => &Self::DESCRIPTORS[3],
            Puzzle3dConfigMutation::SetObjectKindWeights { .. } => &Self::DESCRIPTORS[4],
            Puzzle3dConfigMutation::SetVortexKindWeights { .. } => &Self::DESCRIPTORS[5],
            Puzzle3dConfigMutation::SetWindowCamera { .. } => &Self::DESCRIPTORS[6],
            Puzzle3dConfigMutation::SetWindowSun { .. } => &Self::DESCRIPTORS[7],
            Puzzle3dConfigMutation::SetWindowLodAutomatic { .. } => &Self::DESCRIPTORS[8],
            Puzzle3dConfigMutation::SetWindowLodDepthVariable { .. } => &Self::DESCRIPTORS[9],
            Puzzle3dConfigMutation::SetWindowLodManual { .. } => &Self::DESCRIPTORS[10],
            Puzzle3dConfigMutation::SetWindowGridVisible { .. } => &Self::DESCRIPTORS[11],
            Puzzle3dConfigMutation::SetWindowGridSnapEnabled { .. } => &Self::DESCRIPTORS[12],
            Puzzle3dConfigMutation::SetWindowGridSpacing { .. } => &Self::DESCRIPTORS[13],
            Puzzle3dConfigMutation::SetWindowSelectableKinds { .. } => &Self::DESCRIPTORS[14],
            Puzzle3dConfigMutation::SetWindowProximityRadius { .. } => &Self::DESCRIPTORS[15],
            Puzzle3dConfigMutation::SetWindowChunkSize { .. } => &Self::DESCRIPTORS[16],
            Puzzle3dConfigMutation::SetWindowVoxelDims { .. } => &Self::DESCRIPTORS[17],
            Puzzle3dConfigMutation::SetWindowTransformMove { .. } => &Self::DESCRIPTORS[18],
            Puzzle3dConfigMutation::SetWindowTransformRotate { .. } => &Self::DESCRIPTORS[19],
            Puzzle3dConfigMutation::SetWindowVortexShow { .. } => &Self::DESCRIPTORS[20],
            Puzzle3dConfigMutation::SetWindowVortexDirection { .. } => &Self::DESCRIPTORS[21],
            Puzzle3dConfigMutation::SetSuggestionMenu { .. } => &Self::DESCRIPTORS[22],
            Puzzle3dConfigMutation::SetBrushCandidateIndex { .. } => &Self::DESCRIPTORS[23],
            Puzzle3dConfigMutation::SetWindowEngagementInput { .. } => &Self::DESCRIPTORS[24],
            Puzzle3dConfigMutation::SetActiveUtility { .. } => &Self::DESCRIPTORS[25],
            Puzzle3dConfigMutation::SetLocale { .. } => &Self::DESCRIPTORS[26],
            Puzzle3dConfigMutation::SetTerminology { .. } => &Self::DESCRIPTORS[27],
        }
    }

    fn diff(&self, _base: &Puzzle3dConfig) -> protocol::MutationOutcome<Puzzle3dConfig> {
        protocol::MutationOutcome::new(match self {
            Puzzle3dConfigMutation::Snapshot { config } => config.clone(),
            Puzzle3dConfigMutation::SetFillRequest { count, generation } => {
                let mut next = _base.clone();
                next.fill_count = *count;
                next.fill_apply_generation = *generation;
                next
            }
            Puzzle3dConfigMutation::SetFillAppliedCount { count } => {
                let mut next = _base.clone();
                next.fill_applied_count = *count;
                next
            }
            Puzzle3dConfigMutation::SetOverlapBudget { value } => {
                let mut next = _base.clone();
                next.overlap_budget = *value;
                next
            }
            Puzzle3dConfigMutation::SetObjectKindWeights { value } => {
                let mut next = _base.clone();
                next.object_kind_weights = value.clone();
                next
            }
            Puzzle3dConfigMutation::SetVortexKindWeights { value } => {
                let mut next = _base.clone();
                next.vortex_kind_weights = value.clone();
                next
            }
            Puzzle3dConfigMutation::SetWindowCamera { window_id, camera } => mutate_window_options(_base, window_id, |options| options.camera = camera.clone()),
            Puzzle3dConfigMutation::SetWindowSun { window_id, sun } => mutate_window_options(_base, window_id, |options| options.sun = sun.clone()),
            Puzzle3dConfigMutation::SetWindowLodAutomatic { window_id, value } => mutate_window_options(_base, window_id, |options| options.lod_automatic = *value),
            Puzzle3dConfigMutation::SetWindowLodDepthVariable { window_id, value } => mutate_window_options(_base, window_id, |options| options.lod_depth_variable = *value),
            Puzzle3dConfigMutation::SetWindowLodManual { window_id, value } => mutate_window_options(_base, window_id, |options| options.lod_manual = *value),
            Puzzle3dConfigMutation::SetWindowGridVisible { window_id, value } => mutate_window_options(_base, window_id, |options| options.grid_visible = *value),
            Puzzle3dConfigMutation::SetWindowGridSnapEnabled { window_id, value } => mutate_window_options(_base, window_id, |options| options.grid_snap_enabled = *value),
            Puzzle3dConfigMutation::SetWindowGridSpacing { window_id, value } => mutate_window_options(_base, window_id, |options| options.grid_spacing = *value),
            Puzzle3dConfigMutation::SetWindowSelectableKinds { window_id, value } => mutate_window_options(_base, window_id, |options| options.selectable_kinds = value.clone()),
            Puzzle3dConfigMutation::SetWindowProximityRadius { window_id, value } => mutate_window_options(_base, window_id, |options| options.proximity_radius = *value),
            Puzzle3dConfigMutation::SetWindowChunkSize { window_id, value } => mutate_window_options(_base, window_id, |options| options.chunk_size = *value),
            Puzzle3dConfigMutation::SetWindowVoxelDims { window_id, value } => mutate_window_options(_base, window_id, |options| options.voxel_dims = *value),
            Puzzle3dConfigMutation::SetWindowTransformMove { window_id, value } => mutate_window_options(_base, window_id, |options| options.transform_move = *value),
            Puzzle3dConfigMutation::SetWindowTransformRotate { window_id, value } => mutate_window_options(_base, window_id, |options| options.transform_rotate = *value),
            Puzzle3dConfigMutation::SetWindowVortexShow { window_id, value } => mutate_window_options(_base, window_id, |options| options.vortex_show = value.clone()),
            Puzzle3dConfigMutation::SetWindowVortexDirection { window_id, value } => mutate_window_options(_base, window_id, |options| options.vortex_direction = value.clone()),
            Puzzle3dConfigMutation::SetSuggestionMenu { value } => {
                let mut next = _base.clone();
                next.suggestion_menu = value.clone();
                next
            }
            Puzzle3dConfigMutation::SetBrushCandidateIndex { value } => {
                let mut next = _base.clone();
                next.brush_candidate_index = *value;
                next
            }
            Puzzle3dConfigMutation::SetWindowEngagementInput { window_id, value } => mutate_window_options(_base, window_id, |options| options.engagement_input = value.clone()),
            Puzzle3dConfigMutation::SetActiveUtility { window_id, value } => {
                let mut next = _base.clone();
                if let Some(value) = value {
                    next.active_utility_by_window_id.insert(window_id.clone(), value.clone());
                } else {
                    next.active_utility_by_window_id.remove(window_id);
                }
                next
            }
            Puzzle3dConfigMutation::SetLocale { value } => {
                let mut next = _base.clone();
                next.locale = value.clone();
                next
            }
            Puzzle3dConfigMutation::SetTerminology { value } => {
                let mut next = _base.clone();
                next.terminology = value.clone();
                next
            }
        })
    }

    fn inverse(&self, base: &Puzzle3dConfig) -> Vec<Self> {
        vec![match self {
            Puzzle3dConfigMutation::Snapshot { .. } => Puzzle3dConfigMutation::Snapshot { config: base.clone() },
            Puzzle3dConfigMutation::SetFillRequest { .. } => Puzzle3dConfigMutation::SetFillRequest { count: base.fill_count, generation: base.fill_apply_generation },
            Puzzle3dConfigMutation::SetFillAppliedCount { .. } => Puzzle3dConfigMutation::SetFillAppliedCount { count: base.fill_applied_count },
            Puzzle3dConfigMutation::SetOverlapBudget { .. } => Puzzle3dConfigMutation::SetOverlapBudget { value: base.overlap_budget },
            Puzzle3dConfigMutation::SetObjectKindWeights { .. } => Puzzle3dConfigMutation::SetObjectKindWeights { value: base.object_kind_weights.clone() },
            Puzzle3dConfigMutation::SetVortexKindWeights { .. } => Puzzle3dConfigMutation::SetVortexKindWeights { value: base.vortex_kind_weights.clone() },
            Puzzle3dConfigMutation::SetWindowCamera { window_id, .. } => Puzzle3dConfigMutation::SetWindowCamera { window_id: window_id.clone(), camera: window_options(base, window_id).camera },
            Puzzle3dConfigMutation::SetWindowSun { window_id, .. } => Puzzle3dConfigMutation::SetWindowSun { window_id: window_id.clone(), sun: window_options(base, window_id).sun },
            Puzzle3dConfigMutation::SetWindowLodAutomatic { window_id, .. } => Puzzle3dConfigMutation::SetWindowLodAutomatic { window_id: window_id.clone(), value: window_options(base, window_id).lod_automatic },
            Puzzle3dConfigMutation::SetWindowLodDepthVariable { window_id, .. } => Puzzle3dConfigMutation::SetWindowLodDepthVariable { window_id: window_id.clone(), value: window_options(base, window_id).lod_depth_variable },
            Puzzle3dConfigMutation::SetWindowLodManual { window_id, .. } => Puzzle3dConfigMutation::SetWindowLodManual { window_id: window_id.clone(), value: window_options(base, window_id).lod_manual },
            Puzzle3dConfigMutation::SetWindowGridVisible { window_id, .. } => Puzzle3dConfigMutation::SetWindowGridVisible { window_id: window_id.clone(), value: window_options(base, window_id).grid_visible },
            Puzzle3dConfigMutation::SetWindowGridSnapEnabled { window_id, .. } => Puzzle3dConfigMutation::SetWindowGridSnapEnabled { window_id: window_id.clone(), value: window_options(base, window_id).grid_snap_enabled },
            Puzzle3dConfigMutation::SetWindowGridSpacing { window_id, .. } => Puzzle3dConfigMutation::SetWindowGridSpacing { window_id: window_id.clone(), value: window_options(base, window_id).grid_spacing },
            Puzzle3dConfigMutation::SetWindowSelectableKinds { window_id, .. } => Puzzle3dConfigMutation::SetWindowSelectableKinds { window_id: window_id.clone(), value: window_options(base, window_id).selectable_kinds },
            Puzzle3dConfigMutation::SetWindowProximityRadius { window_id, .. } => Puzzle3dConfigMutation::SetWindowProximityRadius { window_id: window_id.clone(), value: window_options(base, window_id).proximity_radius },
            Puzzle3dConfigMutation::SetWindowChunkSize { window_id, .. } => Puzzle3dConfigMutation::SetWindowChunkSize { window_id: window_id.clone(), value: window_options(base, window_id).chunk_size },
            Puzzle3dConfigMutation::SetWindowVoxelDims { window_id, .. } => Puzzle3dConfigMutation::SetWindowVoxelDims { window_id: window_id.clone(), value: window_options(base, window_id).voxel_dims },
            Puzzle3dConfigMutation::SetWindowTransformMove { window_id, .. } => Puzzle3dConfigMutation::SetWindowTransformMove { window_id: window_id.clone(), value: window_options(base, window_id).transform_move },
            Puzzle3dConfigMutation::SetWindowTransformRotate { window_id, .. } => Puzzle3dConfigMutation::SetWindowTransformRotate { window_id: window_id.clone(), value: window_options(base, window_id).transform_rotate },
            Puzzle3dConfigMutation::SetWindowVortexShow { window_id, .. } => Puzzle3dConfigMutation::SetWindowVortexShow { window_id: window_id.clone(), value: window_options(base, window_id).vortex_show },
            Puzzle3dConfigMutation::SetWindowVortexDirection { window_id, .. } => Puzzle3dConfigMutation::SetWindowVortexDirection { window_id: window_id.clone(), value: window_options(base, window_id).vortex_direction },
            Puzzle3dConfigMutation::SetSuggestionMenu { .. } => Puzzle3dConfigMutation::SetSuggestionMenu { value: base.suggestion_menu.clone() },
            Puzzle3dConfigMutation::SetBrushCandidateIndex { .. } => Puzzle3dConfigMutation::SetBrushCandidateIndex { value: base.brush_candidate_index },
            Puzzle3dConfigMutation::SetWindowEngagementInput { window_id, .. } => Puzzle3dConfigMutation::SetWindowEngagementInput { window_id: window_id.clone(), value: window_options(base, window_id).engagement_input },
            Puzzle3dConfigMutation::SetActiveUtility { window_id, .. } => Puzzle3dConfigMutation::SetActiveUtility { window_id: window_id.clone(), value: base.active_utility_by_window_id.get(window_id).cloned() },
            Puzzle3dConfigMutation::SetLocale { .. } => Puzzle3dConfigMutation::SetLocale { value: base.locale.clone() },
            Puzzle3dConfigMutation::SetTerminology { .. } => Puzzle3dConfigMutation::SetTerminology { value: base.terminology.clone() },
        }]
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_round_trip_has_no_process_identity() {
        let config = Puzzle3dConfig::default();
        let json = serde_json::to_string(&config).expect("config serializes");
        let restored: Puzzle3dConfig = serde_json::from_str(&json).expect("config deserializes");
        assert_eq!(config, restored);
        let mutation = Puzzle3dConfigMutation::Snapshot { config };
        let encoded = protocol::OpBinary::encode_op(&mutation).expect("mutation encodes");
        assert!(!String::from_utf8_lossy(&encoded).contains("runtimeSessionId"));
    }
}
//#endregion 🧪️Tests
