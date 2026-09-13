//! 🎛️ Puzzle 3D shared app preferences and the internal runtime assembled with one exact window.
//!
//! 🪟️ `Puzzle3dConfig` contains only shared fill, overlap, and distribution settings. Camera,
//! display, tool input, suggestion, and candidate state belong to `🪟️window`; fill progress and
//! checkpoints remain in the retained operation that owns their frozen input.

use semio_framework_plugin::{WorldProjectionConfig, WorldSunConfig};
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

fn default_selection_method() -> String {
    crate::editor::puzzle3d::PUZZLE3D_SELECTION_METHOD_PICK.into()
}

fn default_window_ids() -> Vec<String> {
    vec![crate::editor::puzzle3d::modes::edit::windows::main::WINDOW_KIND_ID.to_string()]
}

/// 🏷️ The example a freshly created document was seeded from. `ArtifactApp::initial_snapshot` builds
/// that document out of [`crate::editor::puzzle3d::default_fixture`] — the Concrete Forest example —
/// so the config lane has to say so from the FIRST render. Defaulting it to `""` made a boot document
/// claim it came from no example at all, which is `export_fixture`'s only input for the download name
/// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B30): Concrete Forest exported as the generic
/// `puzzle-3d.json` until the user switched examples at least once. `set_active_example("")` still
/// writes the empty id explicitly, so a deliberately blanked document keeps the generic name.
fn default_active_example_id() -> String {
    crate::editor::puzzle3d::PUZZLE3D_EXAMPLE_CONCRETE_FOREST.into()
}
//#endregion 🔖️Defaults

//#region 🔖️Camera
/// 🎥️ Session-only per-window viewport camera — never a document field (see `setCamera`'s
/// `ActionKind::View`): orbiting one window instance must never move a sibling's camera and must
/// never create a VCS edit.
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dCamera {
    #[value(default)]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[value(default)]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[value(default = "one_f64")]
    pub zoom: f64,
    #[value(default)]
    #[dsl(coord)]
    pub up: Option<[f64; 3]>,
    #[value(default)]
    #[dsl(block)]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dSelectableKinds {
    #[value(default = "default_true")]
    pub objects: bool,
    #[value(default = "default_true")]
    pub vortices: bool,
    #[value(default = "default_true")]
    pub attractions: bool,
}

impl Default for Puzzle3dSelectableKinds {
    fn default() -> Self {
        Self { objects: true, vortices: true, attractions: true }
    }
}

/// 🎯️ Open per-vortex brush-candidate suggestion popup (context menu / Alt+right-click).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dSuggestionMenu {
    pub x: f64,
    pub y: f64,
    #[value(default)]
    pub window_id: String,
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the target vortex full id this
    /// popup was opened on — previously implicit via `runtime.selection.vortex_ids`/
    /// `hovered_vortex_full_id`, now stored directly since selection is framework-owned and cannot be
    /// read back from `render` (see `puzzle3d_brush_target_vortex`'s doc comment).
    #[value(default)]
    pub vortex_full_id: String,
}
//#endregion 🔖️Selection

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dRuntime {
    #[value(default)]
    pub suggestion_menu: Option<Puzzle3dSuggestionMenu>,
    #[value(default = "default_overlap_budget")]
    pub overlap_budget: f64,
    #[value(default)]
    pub fill_count: u32,
    #[value(default)]
    pub brush_candidate_index: usize,
    #[value(default)]
    pub object_kind_weights: HashMap<String, f64>,
    #[value(default)]
    pub vortex_kind_weights: HashMap<String, f64>,
    #[value(default = "default_true")]
    pub lod_automatic: bool,
    #[value(default)]
    pub lod_depth_variable: bool,
    #[value(default = "default_true")]
    pub grid_visible: bool,
    #[value(default = "default_manual_lod")]
    pub lod_manual: f64,
    #[value(default)]
    pub grid_snap_enabled: bool,
    #[value(default = "default_grid_spacing")]
    pub grid_spacing: f64,
    #[value(default)]
    pub selectable_kinds: Puzzle3dSelectableKinds,
    #[value(default)]
    pub engagement_input: String,
    /// 🖱️ How a viewport drag sweeps a selection: `PUZZLE3D_SELECTION_METHOD_PICK` (the default — a
    /// click picks, a drag sweeps an axis-aligned rectangle), `…_RECTANGLE` or `…_LASSO`, exactly the
    /// three the app's own `SelectionSpec` declares. Projected from [`Puzzle3dWindowConfig`], not from
    /// window transient scratch: the engagement bar's `pick`/`rectangle`/`lasso` verbs set it and it
    /// then OUTLIVES the activation that set it — `engagementAbort` deliberately leaves it alone, so
    /// pressing Escape never silently puts the marquee back to a shape the user did not ask for.
    /// `world_selection_json` hands it to `World3dHost.selection.method`.
    #[value(default = "default_selection_method")]
    pub selection_method: String,
    #[value(default = "default_proximity_radius")]
    pub proximity_radius: f64,
    #[value(default = "default_chunk_size")]
    pub chunk_size: f64,
    #[value(default = "default_voxel_dims")]
    pub voxel_dims: [u32; 3],
    /// 🎛️ Whether the transform gumball exposes translate (move axes + move planes).
    #[value(default = "default_true")]
    pub transform_move: bool,
    /// 🎛️ Whether the transform gumball exposes rotate handles.
    #[value(default = "default_true")]
    pub transform_rotate: bool,
    /// 🌀️ When to emit vortex markers: `PUZZLE3D_VORTEX_SHOW_ALWAYS` or `PUZZLE3D_VORTEX_SHOW_SELECTED`.
    #[value(default = "default_vortex_show")]
    pub vortex_show: String,
    /// 🧭️ How vortex direction arrows are drawn: `PUZZLE3D_VORTEX_DIRECTION_OUTWARDS` or `…_INWARDS`.
    #[value(default = "default_vortex_direction")]
    pub vortex_direction: String,
    #[value(default)]
    pub sun: WorldSunConfig,
    /// 🎥️ Session-only viewport camera composed from the exact window owner.
    #[value(default)]
    pub camera: Puzzle3dCamera,
    /// 🛠️ B1: the mode-level active tool (e.g. `"fill"`) — was host-pushed `view_state.active_tool_id`.
    #[value(default)]
    pub active_tool_id: Option<String>,
    /// 🪟️ B1: every window INSTANCE id currently open for this app — was host-pushed
    /// `view_state.window_instances`. Always contains at least the main window id (see `Default`
    /// below) so a freshly-loaded document still engages its one window.
    #[value(default = "default_window_ids")]
    pub window_ids: Vec<String>,
    #[value(default)]
    pub panel_pages: BTreeMap<String, u32>,
    /// 🏷️ Projected from [`Puzzle3dConfig::active_example_id`] — see its doc.
    #[value(default = "default_active_example_id")]
    pub active_example_id: String,
}

impl Default for Puzzle3dRuntime {
    /// 🎛️ Mirrors every `#[value(default = "...")]` above — `#[derive(Default)]` would silently ignore
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
            selection_method: default_selection_method(),
            proximity_radius: default_proximity_radius(),
            chunk_size: default_chunk_size(),
            voxel_dims: default_voxel_dims(),
            transform_move: default_true(),
            transform_rotate: default_true(),
            vortex_show: default_vortex_show(),
            vortex_direction: default_vortex_direction(),
            sun: WorldSunConfig::default(),
            camera: Puzzle3dCamera::default(),
            active_tool_id: None,
            window_ids: default_window_ids(),
            panel_pages: BTreeMap::new(),
            active_example_id: default_active_example_id(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dConfig {
    #[value(default)]
    pub fill_count: u32,
    #[value(default = "default_overlap_budget")]
    pub overlap_budget: f64,
    #[value(default)]
    pub object_kind_weights: HashMap<String, f64>,
    #[value(default)]
    pub vortex_kind_weights: HashMap<String, f64>,
    /// 🏷️ The example id the document was last loaded from — `concrete-forest`,
    /// `nakagin-capsule-tower`, or empty for a blank document. Document identity rather than a
    /// preference, and the only thing `export_fixture` can name its download after: a
    /// `Puzzle3dFixture` carries `schema`/`domain` only (both examples author the same pair), and
    /// `set_active_example` replaces the fixture wholesale, so nothing downstream of it remembers
    /// which example the user is looking at unless this field does. Defaults to
    /// [`default_active_example_id`] — the example `initial_snapshot` actually seeds the document
    /// from — never to the blank id.
    #[value(default = "default_active_example_id")]
    pub active_example_id: String,
}

impl Default for Puzzle3dConfig {
    fn default() -> Self {
        Self { fill_count: 0, overlap_budget: default_overlap_budget(), object_kind_weights: HashMap::new(), vortex_kind_weights: HashMap::new(), active_example_id: default_active_example_id() }
    }
}

impl store::ArtifactDsl for Puzzle3dConfig {
    const EXTENSION: &'static str = "puzzle3dcfg";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        dsl::json::to_string_pretty(&dsl::json::from_dsl_value(&dsl::ToValue::to_value(self)))
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

//#region 🔖️ConfigMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle3dConfigMutation {
    Snapshot { config: Puzzle3dConfig },
    SetFillCount { count: u32 },
    SetOverlapBudget { value: f64 },
    SetObjectKindWeights { value: HashMap<String, f64> },
    SetVortexKindWeights { value: HashMap<String, f64> },
}

impl protocol::Mutation<Puzzle3dConfig> for Puzzle3dConfigMutation {
    type Diff = Puzzle3dConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "snapshot", display_name: "Set Puzzle 3D Shared Configuration", emoji: "🎚️", aggregate_variant: "Snapshot", payload_schema: "puzzle.3dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-fill-count", display_name: "Set Puzzle 3D Fill Count", emoji: "🎚️", aggregate_variant: "SetFillCount", payload_schema: "puzzle.3dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-overlap-budget", display_name: "Set Puzzle 3D Overlap Budget", emoji: "🎚️", aggregate_variant: "SetOverlapBudget", payload_schema: "puzzle.3dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-object-kind-weights", display_name: "Set Puzzle 3D Object Kind Weights", emoji: "🎚️", aggregate_variant: "SetObjectKindWeights", payload_schema: "puzzle.3dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-vortex-kind-weights", display_name: "Set Puzzle 3D Vortex Kind Weights", emoji: "🎚️", aggregate_variant: "SetVortexKindWeights", payload_schema: "puzzle.3dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self { Self::Snapshot { .. } => &Self::DESCRIPTORS[0], Self::SetFillCount { .. } => &Self::DESCRIPTORS[1], Self::SetOverlapBudget { .. } => &Self::DESCRIPTORS[2], Self::SetObjectKindWeights { .. } => &Self::DESCRIPTORS[3], Self::SetVortexKindWeights { .. } => &Self::DESCRIPTORS[4] }
    }
    fn diff(&self, base: &Puzzle3dConfig) -> protocol::MutationOutcome<Self::Diff> {
        let mut next = base.clone();
        match self {
            Self::Snapshot { config } => next = config.clone(),
            Self::SetFillCount { count } => next.fill_count = *count,
            Self::SetOverlapBudget { value } => next.overlap_budget = *value,
            Self::SetObjectKindWeights { value } => next.object_kind_weights = value.clone(),
            Self::SetVortexKindWeights { value } => next.vortex_kind_weights = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &Puzzle3dConfig) -> Vec<Self> { vec![Self::Snapshot { config: base.clone() }] }
}

impl protocol::OpBinary for Puzzle3dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(dsl::json::to_json_string(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}
impl protocol::OpText for Puzzle3dConfigMutation {
    fn print_op(&self) -> String { dsl::json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, store::TextError> { dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1))) }
}
//#endregion 🔖️ConfigMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
