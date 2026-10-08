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
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCamera2d {
    #[value(default)]
    pub x: f64,
    #[value(default)]
    pub y: f64,
    #[value(default = "one_f64")]
    pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(self)))
    }
}

impl store::ArtifactPack for Puzzle5dConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        semio_framework_value::ToValue::to_value(self).encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = semio_framework_value::DslValue::decode_pack_with(bytes, options)?;
        semio_framework_value::FromValue::from_value(value).map_err(|error| store::PackError::from(error))
    }
}

impl store::ConfigRecord for Puzzle5dConfig {}

/// 🔺️ Sparse typed delta of the shared Puzzle 5D configuration: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dConfigDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fill_count: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub contact_tolerance: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub proximity_radius: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub chunk_size: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub object_kind_weights: Option<HashMap<String, f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vortex_kind_weights: Option<HashMap<String, f64>>,
}

impl Puzzle5dConfigDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle5dConfig) -> Self {
        Self { fill_count: Some(state.fill_count), contact_tolerance: Some(state.contact_tolerance), proximity_radius: Some(state.proximity_radius), chunk_size: Some(state.chunk_size), object_kind_weights: Some(state.object_kind_weights.clone()), vortex_kind_weights: Some(state.vortex_kind_weights.clone()) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle5dConfig) -> Self {
        Self { fill_count: self.fill_count.as_ref().filter(|value| **value != base.fill_count).cloned(), contact_tolerance: self.contact_tolerance.as_ref().filter(|value| **value != base.contact_tolerance).cloned(), proximity_radius: self.proximity_radius.as_ref().filter(|value| **value != base.proximity_radius).cloned(), chunk_size: self.chunk_size.as_ref().filter(|value| **value != base.chunk_size).cloned(), object_kind_weights: self.object_kind_weights.as_ref().filter(|value| **value != base.object_kind_weights).cloned(), vortex_kind_weights: self.vortex_kind_weights.as_ref().filter(|value| **value != base.vortex_kind_weights).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle5dConfig) -> Self {
        Self { fill_count: self.fill_count.as_ref().map(|_| base.fill_count), contact_tolerance: self.contact_tolerance.as_ref().map(|_| base.contact_tolerance), proximity_radius: self.proximity_radius.as_ref().map(|_| base.proximity_radius), chunk_size: self.chunk_size.as_ref().map(|_| base.chunk_size), object_kind_weights: self.object_kind_weights.as_ref().map(|_| base.object_kind_weights.clone()), vortex_kind_weights: self.vortex_kind_weights.as_ref().map(|_| base.vortex_kind_weights.clone()) }
    }
}

impl protocol::MutationDiff<Puzzle5dConfig> for Puzzle5dConfigDiff {
    fn apply(&self, base: &Puzzle5dConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle5dConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.fill_count {
            next.fill_count = *value;
        }
        if let Some(value) = &self.contact_tolerance {
            next.contact_tolerance = *value;
        }
        if let Some(value) = &self.proximity_radius {
            next.proximity_radius = *value;
        }
        if let Some(value) = &self.chunk_size {
            next.chunk_size = *value;
        }
        if let Some(value) = &self.object_kind_weights {
            next.object_kind_weights = value.clone();
        }
        if let Some(value) = &self.vortex_kind_weights {
            next.vortex_kind_weights = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.fill_count.is_some() {
            self.fill_count = other.fill_count;
        }
        if other.contact_tolerance.is_some() {
            self.contact_tolerance = other.contact_tolerance;
        }
        if other.proximity_radius.is_some() {
            self.proximity_radius = other.proximity_radius;
        }
        if other.chunk_size.is_some() {
            self.chunk_size = other.chunk_size;
        }
        if other.object_kind_weights.is_some() {
            self.object_kind_weights = other.object_kind_weights;
        }
        if other.vortex_kind_weights.is_some() {
            self.vortex_kind_weights = other.vortex_kind_weights;
        }
    }
}

impl protocol::DiffAlgebra<Puzzle5dConfig> for Puzzle5dConfigDiff {
    fn inverse(&self, base: &Puzzle5dConfig) -> Self {
        self.restoring(base)
    }
    fn between(base: &Puzzle5dConfig, other: &Puzzle5dConfig) -> Self {
        Self { fill_count: (base.fill_count != other.fill_count).then(|| other.fill_count), contact_tolerance: (base.contact_tolerance != other.contact_tolerance).then(|| other.contact_tolerance), proximity_radius: (base.proximity_radius != other.proximity_radius).then(|| other.proximity_radius), chunk_size: (base.chunk_size != other.chunk_size).then(|| other.chunk_size), object_kind_weights: (base.object_kind_weights != other.object_kind_weights).then(|| other.object_kind_weights.clone()), vortex_kind_weights: (base.vortex_kind_weights != other.vortex_kind_weights).then(|| other.vortex_kind_weights.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.fill_count.is_none() && self.contact_tolerance.is_none() && self.proximity_radius.is_none() && self.chunk_size.is_none() && self.object_kind_weights.is_none() && self.vortex_kind_weights.is_none()
    }
}

impl Puzzle5dConfig {
    /// 🔁️ One field mutation per field `next` changes — the config's event vocabulary, never a whole-config restore.
    pub fn mutations_to(&self, next: &Puzzle5dConfig) -> Vec<Puzzle5dConfigMutation> {
        let mut mutations = Vec::new();
        if self.fill_count != next.fill_count {
            mutations.push(Puzzle5dConfigMutation::SetFillCount { value: next.fill_count });
        }
        if self.contact_tolerance != next.contact_tolerance {
            mutations.push(Puzzle5dConfigMutation::SetContactTolerance { value: next.contact_tolerance });
        }
        if self.proximity_radius != next.proximity_radius {
            mutations.push(Puzzle5dConfigMutation::SetProximityRadius { value: next.proximity_radius });
        }
        if self.chunk_size != next.chunk_size {
            mutations.push(Puzzle5dConfigMutation::SetChunkSize { value: next.chunk_size });
        }
        if self.object_kind_weights != next.object_kind_weights {
            mutations.push(Puzzle5dConfigMutation::SetObjectKindWeights { value: next.object_kind_weights.clone() });
        }
        if self.vortex_kind_weights != next.vortex_kind_weights {
            mutations.push(Puzzle5dConfigMutation::SetVortexKindWeights { value: next.vortex_kind_weights.clone() });
        }
        mutations
    }
}

//#endregion 🔖️Config

//#region 🔖️ConfigMutation
/// 🧮️ B1: `Puzzle5dConfig`'s operation enum. Every real config edit is captured as "the whole config
/// after this edit"; `backwards()` is the same one-liner regardless of what changed.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle5dConfigMutation {
    SetFillCount { value: u32 },
    SetContactTolerance { value: f64 },
    SetProximityRadius { value: f64 },
    SetChunkSize { value: f64 },
    SetObjectKindWeights { value: HashMap<String, f64> },
    SetVortexKindWeights { value: HashMap<String, f64> },
}

impl protocol::Mutation<Puzzle5dConfig> for Puzzle5dConfigMutation {
    type Diff = Puzzle5dConfigDiff;

    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-fill-count", display_name: "Set Puzzle 5D Fill Count", emoji: "🎚️", aggregate_variant: "SetFillCount", payload_schema: "puzzle.5dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-contact-tolerance", display_name: "Set Puzzle 5D Contact Tolerance", emoji: "🎚️", aggregate_variant: "SetContactTolerance", payload_schema: "puzzle.5dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-proximity-radius", display_name: "Set Puzzle 5D Proximity Radius", emoji: "🎚️", aggregate_variant: "SetProximityRadius", payload_schema: "puzzle.5dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-chunk-size", display_name: "Set Puzzle 5D Chunk Size", emoji: "🎚️", aggregate_variant: "SetChunkSize", payload_schema: "puzzle.5dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-object-kind-weights", display_name: "Set Puzzle 5D Object Kind Weights", emoji: "🎚️", aggregate_variant: "SetObjectKindWeights", payload_schema: "puzzle.5dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-vortex-kind-weights", display_name: "Set Puzzle 5D Vortex Kind Weights", emoji: "🎚️", aggregate_variant: "SetVortexKindWeights", payload_schema: "puzzle.5dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::SetFillCount { .. } => &Self::DESCRIPTORS[0],
            Self::SetContactTolerance { .. } => &Self::DESCRIPTORS[1],
            Self::SetProximityRadius { .. } => &Self::DESCRIPTORS[2],
            Self::SetChunkSize { .. } => &Self::DESCRIPTORS[3],
            Self::SetObjectKindWeights { .. } => &Self::DESCRIPTORS[4],
            Self::SetVortexKindWeights { .. } => &Self::DESCRIPTORS[5],
        }
    }

    fn diff(&self, base: &Puzzle5dConfig) -> protocol::MutationOutcome<Puzzle5dConfigDiff> {
        let diff = match self {
            Self::SetFillCount { value } => Puzzle5dConfigDiff { fill_count: (value != &base.fill_count).then_some(*value), ..Default::default() },
            Self::SetContactTolerance { value } => Puzzle5dConfigDiff { contact_tolerance: (value != &base.contact_tolerance).then_some(*value), ..Default::default() },
            Self::SetProximityRadius { value } => Puzzle5dConfigDiff { proximity_radius: (value != &base.proximity_radius).then_some(*value), ..Default::default() },
            Self::SetChunkSize { value } => Puzzle5dConfigDiff { chunk_size: (value != &base.chunk_size).then_some(*value), ..Default::default() },
            Self::SetObjectKindWeights { value } => Puzzle5dConfigDiff { object_kind_weights: (value != &base.object_kind_weights).then(|| value.clone()), ..Default::default() },
            Self::SetVortexKindWeights { value } => Puzzle5dConfigDiff { vortex_kind_weights: (value != &base.vortex_kind_weights).then(|| value.clone()), ..Default::default() },
        };
        if protocol::DiffAlgebra::<Puzzle5dConfig>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The configuration already holds this value.");
        }
        protocol::MutationOutcome::new(diff)
    }

    fn inverse(&self, base: &Puzzle5dConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetFillCount { .. } => Self::SetFillCount { value: base.fill_count },
            Self::SetContactTolerance { .. } => Self::SetContactTolerance { value: base.contact_tolerance },
            Self::SetProximityRadius { .. } => Self::SetProximityRadius { value: base.proximity_radius },
            Self::SetChunkSize { .. } => Self::SetChunkSize { value: base.chunk_size },
            Self::SetObjectKindWeights { .. } => Self::SetObjectKindWeights { value: base.object_kind_weights.clone() },
            Self::SetVortexKindWeights { .. } => Self::SetVortexKindWeights { value: base.vortex_kind_weights.clone() },
        }])
    }
}

impl protocol::OpBinary for Puzzle5dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}

impl protocol::OpText for Puzzle5dConfigMutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️ConfigMutation
