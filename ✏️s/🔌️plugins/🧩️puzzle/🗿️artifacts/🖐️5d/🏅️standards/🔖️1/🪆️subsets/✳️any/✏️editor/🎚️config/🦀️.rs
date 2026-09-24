//! 🎛️ Puzzle 5D shared app preferences and the internal runtime assembled with one exact window.
//!
//! 🪟️ `Puzzle5dConfig` contains only shared overlap and distribution settings. Each board/world
//! camera and display setting belongs to that exact window instance, while engagement and candidate
//! scratch belongs to its transient owner.

use semio_framework_plugin::{WorldProjectionConfig, WorldSunConfig};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️Defaults
fn one_f64() -> f64 {
    1.0
}

/// 🧲️ How deep (m) one object's surface may reach into another before a placement collides: 5 mm admits the numeric
/// contact of two docked faces and nothing a viewer would see as objects cutting into each other.
fn default_contact_tolerance() -> f64 {
    0.005
}

fn default_lod_mode() -> String {
    crate::editor::puzzle5d::PUZZLE5D_LOD_MODE_AUTOMATIC.into()
}

fn default_suggestion_offset() -> f64 {
    crate::editor::puzzle5d::PUZZLE5D_DEFAULT_SUGGESTION_OFFSET
}

fn default_true() -> bool {
    true
}

/// 🌐️ World-pane grid pitch (m) — the same spacing the 3D grid draws and snapping rounds onto.
fn default_grid_spacing() -> f64 {
    10.0
}

/// 📡️ How near (m) two open grips must come before a drop auto-connects them.
fn default_proximity_radius() -> f64 {
    crate::editor::puzzle5d::PUZZLE5D_PROXIMITY_RADIUS
}

/// 🧱️ The edge length (m) of one broad-phase chunk the placement search buckets parts into.
fn default_chunk_size() -> f64 {
    4.0
}

/// 🔭️ Where the manual LOD slider sits before anyone drags it — the midpoint of the 0…1000 band
/// `setLodManual` clamps against, so the world pane starts at a legible detail level.
fn default_lod_manual() -> f64 {
    100.0
}

/// 🤏️ Grips are chrome, not content: the world pane only marks the ones the pointer or the selection
/// is actually on until the operator asks for all of them.
fn default_grip_show() -> String {
    crate::editor::puzzle5d::PUZZLE5D_GRIP_SHOW_SELECTED.into()
}

fn default_grip_direction() -> String {
    crate::editor::puzzle5d::PUZZLE5D_GRIP_DIRECTION_OUTWARDS.into()
}

/// 🪣️ The fill count a fresh 5d document offers — see `crate::editor::puzzle5d::PUZZLE5D_DEFAULT_FILL_COUNT`.
fn default_fill_count() -> u32 {
    crate::editor::puzzle5d::PUZZLE5D_DEFAULT_FILL_COUNT
}

/// 🧊️ The voxel extent one Alt+click of the Volume Brush paints before the operator moves a slider.
fn default_voxel_dims() -> [u32; 3] {
    crate::editor::puzzle5d::PUZZLE5D_DEFAULT_VOXEL_DIMS
}

//#endregion 🔖️Defaults

//#region 🔖️Cameras
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCamera2d {
    #[value(default)]
    pub x: f64,
    #[value(default)]
    pub y: f64,
    #[value(default = "one_f64")]
    pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCamera3d {
    #[value(default)]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[value(default)]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[value(default = "one_f64")]
    pub zoom: f64,
    /// 🧭️ Re-derived from `projection` whenever a projection change moves the pose (`setProjection`).
    #[value(default)]
    #[dsl(coord)]
    pub up: Option<[f64; 3]>,
    /// 🎥️ The full classical projection taxonomy this pane's camera renders through — the same
    /// framework state puzzle3d's `Puzzle3dCamera::projection` carries, driven by
    /// `setProjection`/`setProjectionParam`.
    #[value(default)]
    #[dsl(block)]
    pub projection: WorldProjectionConfig,
}

/// 📐️ Distance from `position` to `target`, defaulting to the 5d world pane's own 8-unit boot orbit
/// when degenerate — the radius `world3d_projection_pose` re-poses the camera on.
pub fn puzzle5d_camera3d_distance(camera: &Puzzle5dCamera3d) -> f64 {
    let [dx, dy, dz] = [camera.position[0] - camera.target[0], camera.position[1] - camera.target[1], camera.position[2] - camera.target[2]];
    let distance = (dx * dx + dy * dy + dz * dz).sqrt();
    if distance > 1e-3 {
        distance
    } else {
        PUZZLE5D_CAMERA3D_DEFAULT_DISTANCE
    }
}

/// 📐️ `[8, -8, 8]` away from the origin — the boot pose `Puzzle5dWindowConfig::default` sets.
pub const PUZZLE5D_CAMERA3D_DEFAULT_DISTANCE: f64 = 13.856_406_460_551_018;
//#endregion 🔖️Cameras

//#region 🔖️Selection
/// 🎯️ Which entity kinds a pick in either pane may even reach — the 5d twin of
/// `Puzzle3dSelectableKinds`, in this artifact's part/grip/fastener vocabulary.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dSelectableKinds {
    #[value(default = "default_true")]
    pub parts: bool,
    #[value(default = "default_true")]
    pub grips: bool,
    #[value(default = "default_true")]
    pub fasteners: bool,
}

impl Default for Puzzle5dSelectableKinds {
    fn default() -> Self {
        Self { parts: true, grips: true, fasteners: true }
    }
}
//#endregion 🔖️Selection

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dRuntime {
    /// 📷️ Camera pose — session-only view state (`ActionKind::View`), never a VCS document field:
    /// see `setCamera`/`setCamera2d`/`setCamera3d` in `🎮️commands/🎥️set-camera`.
    #[value(default)]
    pub camera2d: Puzzle5dCamera2d,
    #[value(default)]
    pub camera3d: Puzzle5dCamera3d,
    #[value(default = "default_fill_count")]
    pub fill_count: u32,
    #[value(default)]
    pub brush_candidate_index: usize,
    /// 🎣️ The grip suggestion menu this window has open (window-transient, never persisted).
    #[value(default)]
    pub suggestion_menu: Option<crate::editor::puzzle5d::window::Puzzle5dSuggestionMenu>,
    #[value(default = "default_contact_tolerance")]
    pub contact_tolerance: f64,
    /// 📡️ How near (m) two open grips must come before a drop auto-connects them (`setProximityRadius`).
    #[value(default = "default_proximity_radius")]
    pub proximity_radius: f64,
    /// 🧱️ The broad-phase chunk edge (m) the placement search buckets parts into (`setChunkSize`).
    #[value(default = "default_chunk_size")]
    pub chunk_size: f64,
    #[value(default = "default_lod_mode")]
    pub lod_mode: String,
    #[value(default = "default_suggestion_offset")]
    pub suggestion_offset: f64,
    #[value(default = "default_true")]
    pub grid_snap_enabled: bool,
    #[value(default = "one_f64")]
    pub grid_factor: f64,
    /// 🌐️ Whether the pane draws its grid at all — board and world pane both own one.
    #[value(default = "default_true")]
    pub grid_visible: bool,
    /// 🌐️ World-pane grid pitch (m), the slider `setGridSpacing` writes.
    #[value(default = "default_grid_spacing")]
    pub grid_spacing: f64,
    /// 🔭️ World-pane LOD trio: zoom-driven, depth-varying, and the manual override the slider holds.
    #[value(default = "default_true")]
    pub lod_automatic: bool,
    #[value(default)]
    pub lod_depth_variable: bool,
    #[value(default = "default_lod_manual")]
    pub lod_manual: f64,
    /// 🎯️ Which entity kinds a pick may reach in this pane.
    #[value(default)]
    pub selectable_kinds: Puzzle5dSelectableKinds,
    /// 🤏️ When grip markers are emitted: `PUZZLE5D_GRIP_SHOW_ALWAYS` or `…_SELECTED`.
    #[value(default = "default_grip_show")]
    pub grip_show: String,
    /// 🧭️ How grip direction arrows point: `PUZZLE5D_GRIP_DIRECTION_OUTWARDS` or `…_INWARDS`.
    #[value(default = "default_grip_direction")]
    pub grip_direction: String,
    /// 🎛️ Whether the transform gumball exposes translate handles.
    #[value(default = "default_true")]
    pub transform_move: bool,
    /// 🎛️ Whether the transform gumball exposes rotate handles.
    #[value(default = "default_true")]
    pub transform_rotate: bool,
    #[value(default)]
    pub engagement_input_by_window: BTreeMap<String, String>,
    #[value(default)]
    pub object_kind_weights: HashMap<String, f64>,
    #[value(default)]
    pub vortex_kind_weights: HashMap<String, f64>,
    #[value(default)]
    pub sun: WorldSunConfig,
    /// 🧊️ Volume-Brush voxel extent in grid-spacing units, `[w, d, h]`, each clamped to `[1, 64]` —
    /// what one Alt+click paints. Per-window view state (`setVoxelDims`), never a document field.
    #[value(default = "default_voxel_dims")]
    pub voxel_dims: [u32; 3],
}

/// ⚠️ Explicit impl (not `#[derive(Default)]`) so Rust construction matches the serde field defaults above.
impl Default for Puzzle5dRuntime {
    fn default() -> Self {
        Self {
            camera2d: Puzzle5dCamera2d { x: 0.0, y: 0.0, zoom: 1.0 },
            camera3d: Puzzle5dCamera3d { position: [8.0, -8.0, 8.0], target: [0.0, 0.0, 0.0], zoom: 1.0, up: None, projection: WorldProjectionConfig::default() },
            fill_count: default_fill_count(),
            brush_candidate_index: 0,
            suggestion_menu: None,
            contact_tolerance: default_contact_tolerance(),
            proximity_radius: default_proximity_radius(),
            chunk_size: default_chunk_size(),
            lod_mode: default_lod_mode(),
            suggestion_offset: default_suggestion_offset(),
            grid_snap_enabled: true,
            grid_factor: 1.0,
            grid_visible: true,
            grid_spacing: default_grid_spacing(),
            lod_automatic: true,
            lod_depth_variable: false,
            lod_manual: default_lod_manual(),
            selectable_kinds: Puzzle5dSelectableKinds::default(),
            grip_show: default_grip_show(),
            grip_direction: default_grip_direction(),
            transform_move: true,
            transform_rotate: true,
            engagement_input_by_window: BTreeMap::new(),
            object_kind_weights: HashMap::new(),
            vortex_kind_weights: HashMap::new(),
            sun: WorldSunConfig::default(),
            voxel_dims: default_voxel_dims(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dConfig {
    #[value(default = "default_fill_count")]
    pub fill_count: u32,
    #[value(default = "default_contact_tolerance")]
    pub contact_tolerance: f64,
    /// 📡️ Session-wide placement settings — the ⚙️settings panel's steppers, shared by both panes
    /// because a drop and a fill mean the same distance whichever pane triggered them.
    #[value(default = "default_proximity_radius")]
    pub proximity_radius: f64,
    #[value(default = "default_chunk_size")]
    pub chunk_size: f64,
    #[value(default)]
    pub object_kind_weights: HashMap<String, f64>,
    #[value(default)]
    pub vortex_kind_weights: HashMap<String, f64>,
}

impl Default for Puzzle5dConfig {
    fn default() -> Self {
        Self {
            fill_count: default_fill_count(),
            contact_tolerance: default_contact_tolerance(),
            proximity_radius: default_proximity_radius(),
            chunk_size: default_chunk_size(),
            object_kind_weights: HashMap::new(),
            vortex_kind_weights: HashMap::new(),
        }
    }
}

impl store::ArtifactDsl for Puzzle5dConfig {
    const EXTENSION: &'static str = "puzzle5dcfg";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        dsl::json::to_string_pretty(&dsl::json::from_dsl_value(&dsl::ToValue::to_value(self)))
    }
}

impl store::ArtifactPack for Puzzle5dConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(self).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map_err(store::PackError::Schema)
    }
}

store::impl_whole_record_config!(Puzzle5dConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigMutation
/// 🧮️ B1: `Puzzle5dConfig`'s operation enum. Every real config edit is captured as "the whole config
/// after this edit"; `backwards()` is the same one-liner regardless of what changed.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle5dConfigMutation {
    Snapshot { config: Puzzle5dConfig },
    SetContactTolerance { value: f64 },
    SetObjectKindWeights { value: HashMap<String, f64> },
    SetVortexKindWeights { value: HashMap<String, f64> },
}

impl protocol::Mutation<Puzzle5dConfig> for Puzzle5dConfigMutation {
    type Diff = Puzzle5dConfig;

    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📄snapshot", semantic_kind: "snapshot", display_name: "Snapshot", emoji: "📄", aggregate_variant: "Snapshot", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️set-contact-tolerance", semantic_kind: "set-contact-tolerance", display_name: "Set Overlap Budget", emoji: "🖌️", aggregate_variant: "SetContactTolerance", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️set-object-kind-weights", semantic_kind: "set-object-kind-weights", display_name: "Set Object Kind Weights", emoji: "🖌️", aggregate_variant: "SetObjectKindWeights", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️set-vortex-kind-weights", semantic_kind: "set-vortex-kind-weights", display_name: "Set Vortex Kind Weights", emoji: "🖌️", aggregate_variant: "SetVortexKindWeights", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Puzzle5dConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
            Puzzle5dConfigMutation::SetContactTolerance { .. } => &Self::DESCRIPTORS[1],
            Puzzle5dConfigMutation::SetObjectKindWeights { .. } => &Self::DESCRIPTORS[2],
            Puzzle5dConfigMutation::SetVortexKindWeights { .. } => &Self::DESCRIPTORS[3],
        }
    }

    fn diff(&self, base: &Puzzle5dConfig) -> protocol::MutationOutcome<Puzzle5dConfig> {
        if let Puzzle5dConfigMutation::Snapshot { config } = self {
            return protocol::MutationOutcome::new(config.clone());
        }
        let mut next = base.clone();
        match self {
            Puzzle5dConfigMutation::Snapshot { .. } => {}
            Puzzle5dConfigMutation::SetContactTolerance { value } => next.contact_tolerance = *value,
            Puzzle5dConfigMutation::SetObjectKindWeights { value } => next.object_kind_weights = value.clone(),
            Puzzle5dConfigMutation::SetVortexKindWeights { value } => next.vortex_kind_weights = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Puzzle5dConfig) -> Vec<Self> {
        vec![Puzzle5dConfigMutation::Snapshot { config: base.clone() }]
    }
}

impl protocol::OpBinary for Puzzle5dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(dsl::json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

impl protocol::OpText for Puzzle5dConfigMutation {
    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️ConfigMutation
