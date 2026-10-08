//! 🎛️ Puzzle 2D shared app preferences and the internal runtime assembled with one exact window.
//!
//! 🪟️ `Puzzle2dConfig` contains the shared generator weights and the fill tool's requested count (the
//! `ToolRunJobRequest` a fill run is built from carries it). Camera, LOD, grid, engagement input and
//! brush candidates belong to `🪟️window`; document cameras only seed a new
//! exact window and never receive live view updates.

use std::collections::BTreeMap;

//#region 🔖️Defaults
/// 📶️ Defines the artifact-local board suggestion offset without a styling dependency.
pub const PUZZLE2D_DEFAULT_SUGGESTION_OFFSET: f64 = 80.0;

/// 🚧️ World units a brush/fill placement footprint is GROWN by before it is tested against the head —
/// the 2d twin of puzzle3d's `contact_tolerance`, so a placement that merely grazes a neighbour is
/// refused instead of drawn overlapping by a hairline.
pub const PUZZLE2D_DEFAULT_CONTACT_TOLERANCE: f64 = 0.0;
/// 🫂️ World units of footprint overlap a brush/fill placement may SPEND before it counts as a
/// collision. It is the inverse of the tolerance and the two net out (`tolerance - budget`), so a
/// dense board can be packed deliberately without disabling collision testing.
pub const PUZZLE2D_DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET: f64 = 0.0;

/// 🧲️ Board units within which a dropped node's open handle auto-connects to a compatible open
/// handle — the 2d twin of puzzle3d's `proximity_radius`. Half a default node radius (24), so a drop
/// that visually touches snaps and a drop a node-width away does not.
pub const PUZZLE2D_DEFAULT_PROXIMITY_RADIUS: f64 = 12.0;

fn default_proximity_radius() -> f64 {
    PUZZLE2D_DEFAULT_PROXIMITY_RADIUS
}

fn default_contact_tolerance() -> f64 {
    PUZZLE2D_DEFAULT_CONTACT_TOLERANCE
}

fn default_brush_placement_overlap_budget() -> f64 {
    PUZZLE2D_DEFAULT_BRUSH_PLACEMENT_OVERLAP_BUDGET
}

fn default_grid_factor() -> f64 {
    1.0
}

/// 🖍️ The Area Brush paints a one-by-one grid cell until the utility's own steppers widen it — the
/// 2d twin of puzzle3d's `default_voxel_dims`.
pub const PUZZLE2D_DEFAULT_AREA_BRUSH_EXTENT: f64 = 1.0;

fn default_area_brush_extent() -> f64 {
    PUZZLE2D_DEFAULT_AREA_BRUSH_EXTENT
}

fn default_suggestion_offset() -> f64 {
    PUZZLE2D_DEFAULT_SUGGESTION_OFFSET
}

/// 📶️ Overview/selection default to automatic LOD; detail defaults to a fixed "detail" tier, matching the pre-migration triptych.
fn default_lod_mode_by_pane() -> BTreeMap<String, String> {
    use crate::editor::puzzle2d::modes::edit::windows::{detail, overview, selection};
    BTreeMap::from([
        (overview::WINDOW_KIND_ID.to_string(), crate::editor::puzzle2d::PUZZLE2D_LOD_MODE_AUTOMATIC.to_string()),
        (detail::WINDOW_KIND_ID.to_string(), "detail".to_string()),
        (selection::WINDOW_KIND_ID.to_string(), crate::editor::puzzle2d::PUZZLE2D_LOD_MODE_AUTOMATIC.to_string()),
    ])
}

fn default_camera_zoom() -> f64 {
    1.0
}

/// 🪣️ The fill count a fresh board offers — see `modes::edit::tools::fill`.
fn default_fill_count() -> u32 {
    crate::editor::puzzle2d::modes::edit::tools::fill::PUZZLE2D_DEFAULT_FILL_COUNT
}

//#endregion 🔖️Defaults

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dPlayRuntime {
    #[value(default)]
    pub camera_x: f64,
    #[value(default)]
    pub camera_y: f64,
    #[value(default = "default_camera_zoom")]
    pub camera_zoom: f64,
    #[value(default = "default_lod_mode_by_pane")]
    pub lod_mode_by_pane: BTreeMap<String, String>,
    #[value(default)]
    pub engagement_input_by_pane: BTreeMap<String, String>,
    #[value(default)]
    pub brush_candidate_index: usize,
    #[value(default)]
    pub brush_candidates: Vec<semio_framework_value::DslValue>,
    #[value(default)]
    pub brush_candidate_source_handle_id: String,
    #[value(default)]
    pub suggestion_menu: Option<Puzzle2dSuggestionMenu>,
    #[value(default = "default_fill_count")]
    pub fill_count: u32,
    #[value(default)]
    pub grid_snap_enabled: bool,
    #[value(default = "default_grid_factor")]
    pub grid_factor: f64,
    #[value(default = "default_suggestion_offset")]
    pub suggestion_offset: f64,
    #[value(default = "default_proximity_radius")]
    pub proximity_radius: f64,
    #[value(default)]
    pub grid_visible: bool,
    /// 🖍️ Area Brush extent in grid cells — the 2d twin of puzzle3d's `voxel_dims`, one stepper per
    /// board axis. Alt+click paints a region this many grid cells wide and high.
    #[value(default = "default_area_brush_extent")]
    pub area_brush_width: f64,
    #[value(default = "default_area_brush_extent")]
    pub area_brush_height: f64,
    /// 🕹️ Gumball move handle — the board's native node drag. Composed by `setTransformGumballFlag`.
    #[value(default = "transform_flag_default")]
    pub transform_move: bool,
    /// 🔄️ Gumball rotate ring. Scale is deliberately absent: a node's size comes from its kind catalog.
    #[value(default = "transform_flag_default")]
    pub transform_rotate: bool,
    #[value(default)]
    pub selectable_kinds: Puzzle2dSelectableKinds,
    #[value(default = "default_contact_tolerance")]
    pub contact_tolerance: f64,
    #[value(default = "default_brush_placement_overlap_budget")]
    pub brush_placement_overlap_budget: f64,
    #[value(default)]
    pub node_kind_weights: BTreeMap<String, f64>,
    #[value(default)]
    pub handle_kind_weights: BTreeMap<String, f64>,
}

/// 💡️ The one-shot handle-suggestions popup this window has open: where it hangs (viewport pixels),
/// which exact window instance owns it, and the handle whose candidate page it lists. `None` is the
/// closed state — the 2d twin of puzzle3d's `Puzzle3dSuggestionMenu`. Per-gesture scratch, so it
/// lives on the window transient and never on a persisted config.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dSuggestionMenu {
    #[value(default)]
    pub x: f64,
    #[value(default)]
    pub y: f64,
    #[value(default)]
    pub window_id: String,
    #[value(default)]
    pub handle_id: String,
}

/// 🎯️ Which granularity a pick may even reach — the 2d twin of puzzle3d's
/// `selectable_kinds{objects,vortices,attractions}`, one flag per `vortex`-domain granularity.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dSelectableKinds {
    #[value(default = "selectable_default")]
    pub nodes: bool,
    #[value(default = "selectable_default")]
    pub handles: bool,
    #[value(default = "selectable_default")]
    pub edges: bool,
}

fn selectable_default() -> bool {
    true
}

/// 🕹️ Both gumball handles start composed, matching puzzle-3d's `transform_move`/`transform_rotate`.
fn transform_flag_default() -> bool {
    true
}

impl Default for Puzzle2dSelectableKinds {
    fn default() -> Self {
        Self { nodes: true, handles: true, edges: true }
    }
}

impl Default for Puzzle2dPlayRuntime {
    fn default() -> Self {
        Self {
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: default_camera_zoom(),
            lod_mode_by_pane: default_lod_mode_by_pane(),
            engagement_input_by_pane: BTreeMap::new(),
            brush_candidate_index: 0,
            brush_candidates: Vec::new(),
            brush_candidate_source_handle_id: String::new(),
            suggestion_menu: None,
            fill_count: default_fill_count(),
            grid_snap_enabled: false,
            grid_factor: default_grid_factor(),
            suggestion_offset: default_suggestion_offset(),
            proximity_radius: default_proximity_radius(),
            area_brush_width: default_area_brush_extent(),
            area_brush_height: default_area_brush_extent(),
            grid_visible: true,
            transform_move: transform_flag_default(),
            transform_rotate: transform_flag_default(),
            selectable_kinds: Puzzle2dSelectableKinds::default(),
            contact_tolerance: default_contact_tolerance(),
            brush_placement_overlap_budget: default_brush_placement_overlap_budget(),
            node_kind_weights: BTreeMap::new(),
            handle_kind_weights: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dConfig {
    #[value(default)]
    pub node_kind_weights: BTreeMap<String, f64>,
    #[value(default)]
    pub handle_kind_weights: BTreeMap<String, f64>,
    #[value(default = "default_fill_count")]
    pub fill_count: u32,
    #[value(default = "default_contact_tolerance")]
    pub contact_tolerance: f64,
    #[value(default = "default_brush_placement_overlap_budget")]
    pub brush_placement_overlap_budget: f64,
}

impl Default for Puzzle2dConfig {
    fn default() -> Self {
        Self {
            node_kind_weights: BTreeMap::new(),
            handle_kind_weights: BTreeMap::new(),
            fill_count: default_fill_count(),
            contact_tolerance: default_contact_tolerance(),
            brush_placement_overlap_budget: default_brush_placement_overlap_budget(),
        }
    }
}

impl store::ArtifactDsl for Puzzle2dConfig {
    const EXTENSION: &'static str = "puzzle2dcfg";

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(self)))
    }
}

impl store::ArtifactPack for Puzzle2dConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        semio_framework_value::ToValue::to_value(self).encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = semio_framework_value::DslValue::decode_pack_with(bytes, options)?;
        semio_framework_value::FromValue::from_value(value).map_err(|error| store::PackError::from(error))
    }
}

impl store::ConfigRecord for Puzzle2dConfig {}

/// 🔺️ Sparse typed delta of the shared Puzzle 2D configuration: names only the fields a mutation changes.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dConfigDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub node_kind_weights: Option<BTreeMap<String, f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub handle_kind_weights: Option<BTreeMap<String, f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fill_count: Option<u32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub contact_tolerance: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub brush_placement_overlap_budget: Option<f64>,
}

impl Puzzle2dConfigDiff {
    /// 🎯️ Every field set to `state`'s value.
    pub fn of(state: &Puzzle2dConfig) -> Self {
        Self { node_kind_weights: Some(state.node_kind_weights.clone()), handle_kind_weights: Some(state.handle_kind_weights.clone()), fill_count: Some(state.fill_count), contact_tolerance: Some(state.contact_tolerance), brush_placement_overlap_budget: Some(state.brush_placement_overlap_budget) }
    }
    /// ✂️ The named fields that differ from `base`.
    pub fn changed(&self, base: &Puzzle2dConfig) -> Self {
        Self { node_kind_weights: self.node_kind_weights.as_ref().filter(|value| **value != base.node_kind_weights).cloned(), handle_kind_weights: self.handle_kind_weights.as_ref().filter(|value| **value != base.handle_kind_weights).cloned(), fill_count: self.fill_count.as_ref().filter(|value| **value != base.fill_count).cloned(), contact_tolerance: self.contact_tolerance.as_ref().filter(|value| **value != base.contact_tolerance).cloned(), brush_placement_overlap_budget: self.brush_placement_overlap_budget.as_ref().filter(|value| **value != base.brush_placement_overlap_budget).cloned() }
    }
    /// ↩️ The named fields at the values `base` holds.
    pub fn restoring(&self, base: &Puzzle2dConfig) -> Self {
        Self { node_kind_weights: self.node_kind_weights.as_ref().map(|_| base.node_kind_weights.clone()), handle_kind_weights: self.handle_kind_weights.as_ref().map(|_| base.handle_kind_weights.clone()), fill_count: self.fill_count.as_ref().map(|_| base.fill_count), contact_tolerance: self.contact_tolerance.as_ref().map(|_| base.contact_tolerance), brush_placement_overlap_budget: self.brush_placement_overlap_budget.as_ref().map(|_| base.brush_placement_overlap_budget) }
    }
}

impl protocol::MutationDiff<Puzzle2dConfig> for Puzzle2dConfigDiff {
    fn apply(&self, base: &Puzzle2dConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle2dConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.node_kind_weights {
            next.node_kind_weights = value.clone();
        }
        if let Some(value) = &self.handle_kind_weights {
            next.handle_kind_weights = value.clone();
        }
        if let Some(value) = &self.fill_count {
            next.fill_count = *value;
        }
        if let Some(value) = &self.contact_tolerance {
            next.contact_tolerance = *value;
        }
        if let Some(value) = &self.brush_placement_overlap_budget {
            next.brush_placement_overlap_budget = *value;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.node_kind_weights.is_some() {
            self.node_kind_weights = other.node_kind_weights;
        }
        if other.handle_kind_weights.is_some() {
            self.handle_kind_weights = other.handle_kind_weights;
        }
        if other.fill_count.is_some() {
            self.fill_count = other.fill_count;
        }
        if other.contact_tolerance.is_some() {
            self.contact_tolerance = other.contact_tolerance;
        }
        if other.brush_placement_overlap_budget.is_some() {
            self.brush_placement_overlap_budget = other.brush_placement_overlap_budget;
        }
    }
}

impl protocol::DiffAlgebra<Puzzle2dConfig> for Puzzle2dConfigDiff {
    fn inverse(&self, base: &Puzzle2dConfig) -> Self {
        self.restoring(base)
    }
    fn is_empty(&self) -> bool {
        self.node_kind_weights.is_none() && self.handle_kind_weights.is_none() && self.fill_count.is_none() && self.contact_tolerance.is_none() && self.brush_placement_overlap_budget.is_none()
    }
}

impl Puzzle2dConfig {
    /// 🔁️ One field mutation per field `next` changes — the config's event vocabulary, never a whole-config restore.
    pub fn mutations_to(&self, next: &Puzzle2dConfig) -> Vec<Puzzle2dConfigMutation> {
        let mut mutations = Vec::new();
        if self.node_kind_weights != next.node_kind_weights {
            mutations.push(Puzzle2dConfigMutation::SetNodeKindWeights { value: next.node_kind_weights.clone() });
        }
        if self.handle_kind_weights != next.handle_kind_weights {
            mutations.push(Puzzle2dConfigMutation::SetHandleKindWeights { value: next.handle_kind_weights.clone() });
        }
        if self.fill_count != next.fill_count {
            mutations.push(Puzzle2dConfigMutation::SetFillCount { value: next.fill_count });
        }
        if self.contact_tolerance != next.contact_tolerance {
            mutations.push(Puzzle2dConfigMutation::SetContactTolerance { value: next.contact_tolerance });
        }
        if self.brush_placement_overlap_budget != next.brush_placement_overlap_budget {
            mutations.push(Puzzle2dConfigMutation::SetBrushPlacementOverlapBudget { value: next.brush_placement_overlap_budget });
        }
        mutations
    }
}

//#endregion 🔖️Config

//#region 🔖️ConfigMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle2dConfigMutation {
    SetNodeKindWeights { value: BTreeMap<String, f64> },
    SetHandleKindWeights { value: BTreeMap<String, f64> },
    SetFillCount { value: u32 },
    SetContactTolerance { value: f64 },
    SetBrushPlacementOverlapBudget { value: f64 },
}

impl protocol::Mutation<Puzzle2dConfig> for Puzzle2dConfigMutation {
    type Diff = Puzzle2dConfigDiff;

    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-node-kind-weights", display_name: "Set Puzzle 2D Node Kind Weights", emoji: "🎚️", aggregate_variant: "SetNodeKindWeights", payload_schema: "puzzle.2dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-handle-kind-weights", display_name: "Set Puzzle 2D Handle Kind Weights", emoji: "🎚️", aggregate_variant: "SetHandleKindWeights", payload_schema: "puzzle.2dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-fill-count", display_name: "Set Puzzle 2D Fill Count", emoji: "🎚️", aggregate_variant: "SetFillCount", payload_schema: "puzzle.2dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-contact-tolerance", display_name: "Set Puzzle 2D Contact Tolerance", emoji: "🎚️", aggregate_variant: "SetContactTolerance", payload_schema: "puzzle.2dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config", semantic_kind: "set-brush-placement-overlap-budget", display_name: "Set Puzzle 2D Brush Placement Overlap Budget", emoji: "🎚️", aggregate_variant: "SetBrushPlacementOverlapBudget", payload_schema: "puzzle.2dconfig", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::SetNodeKindWeights { .. } => &Self::DESCRIPTORS[0],
            Self::SetHandleKindWeights { .. } => &Self::DESCRIPTORS[1],
            Self::SetFillCount { .. } => &Self::DESCRIPTORS[2],
            Self::SetContactTolerance { .. } => &Self::DESCRIPTORS[3],
            Self::SetBrushPlacementOverlapBudget { .. } => &Self::DESCRIPTORS[4],
        }
    }

    fn diff(&self, base: &Puzzle2dConfig) -> protocol::MutationOutcome<Puzzle2dConfigDiff> {
        let diff = match self {
            Self::SetNodeKindWeights { value } => Puzzle2dConfigDiff { node_kind_weights: (value != &base.node_kind_weights).then(|| value.clone()), ..Default::default() },
            Self::SetHandleKindWeights { value } => Puzzle2dConfigDiff { handle_kind_weights: (value != &base.handle_kind_weights).then(|| value.clone()), ..Default::default() },
            Self::SetFillCount { value } => Puzzle2dConfigDiff { fill_count: (value != &base.fill_count).then_some(*value), ..Default::default() },
            Self::SetContactTolerance { value } => Puzzle2dConfigDiff { contact_tolerance: (value != &base.contact_tolerance).then_some(*value), ..Default::default() },
            Self::SetBrushPlacementOverlapBudget { value } => Puzzle2dConfigDiff { brush_placement_overlap_budget: (value != &base.brush_placement_overlap_budget).then_some(*value), ..Default::default() },
        };
        if protocol::DiffAlgebra::<Puzzle2dConfig>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The configuration already holds this value.");
        }
        protocol::MutationOutcome::new(diff)
    }

    fn inverse(&self, base: &Puzzle2dConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetNodeKindWeights { .. } => Self::SetNodeKindWeights { value: base.node_kind_weights.clone() },
            Self::SetHandleKindWeights { .. } => Self::SetHandleKindWeights { value: base.handle_kind_weights.clone() },
            Self::SetFillCount { .. } => Self::SetFillCount { value: base.fill_count },
            Self::SetContactTolerance { .. } => Self::SetContactTolerance { value: base.contact_tolerance },
            Self::SetBrushPlacementOverlapBudget { .. } => Self::SetBrushPlacementOverlapBudget { value: base.brush_placement_overlap_budget },
        }])
    }
}

impl protocol::OpBinary for Puzzle2dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}

impl protocol::OpText for Puzzle2dConfigMutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️ConfigMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
