//! 🎛️ Puzzle 5D shared app preferences and the internal runtime assembled with one exact window.
//!
//! 🪟️ `Puzzle5dConfig` contains only shared overlap and distribution settings. Each board/world
//! camera and display setting belongs to that exact window instance, while engagement and candidate
//! scratch belongs to its transient owner.

use semio_framework_plugin::WorldSunConfig;
use std::collections::{BTreeMap, HashMap};

//#region 🔖️Defaults
fn one_f64() -> f64 {
    1.0
}

fn default_overlap_budget() -> f64 {
    0.02
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

//#endregion 🔖️Defaults

//#region 🔖️Cameras
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCamera2d {
    #[value(default)]
    pub x: f64,
    #[value(default)]
    pub y: f64,
    #[value(default = "one_f64")]
    pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dCamera3d {
    #[value(default)]
    pub position: [f64; 3],
    #[value(default)]
    pub target: [f64; 3],
    #[value(default = "one_f64")]
    pub zoom: f64,
}
//#endregion 🔖️Cameras

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
    #[value(default)]
    pub fill_count: u32,
    #[value(default)]
    pub brush_candidate_index: usize,
    #[value(default = "default_overlap_budget")]
    pub overlap_budget: f64,
    #[value(default = "default_lod_mode")]
    pub lod_mode: String,
    #[value(default = "default_suggestion_offset")]
    pub suggestion_offset: f64,
    #[value(default = "default_true")]
    pub grid_snap_enabled: bool,
    #[value(default = "one_f64")]
    pub grid_factor: f64,
    #[value(default)]
    pub engagement_input_by_window: BTreeMap<String, String>,
    #[value(default)]
    pub object_kind_weights: HashMap<String, f64>,
    #[value(default)]
    pub vortex_kind_weights: HashMap<String, f64>,
    #[value(default)]
    pub sun: WorldSunConfig,
}

/// ⚠️ Explicit impl (not `#[derive(Default)]`) so Rust construction matches the serde field defaults above.
impl Default for Puzzle5dRuntime {
    fn default() -> Self {
        Self {
            camera2d: Puzzle5dCamera2d { x: 0.0, y: 0.0, zoom: 1.0 },
            camera3d: Puzzle5dCamera3d { position: [8.0, -8.0, 8.0], target: [0.0, 0.0, 0.0], zoom: 1.0 },
            fill_count: 0,
            brush_candidate_index: 0,
            overlap_budget: default_overlap_budget(),
            lod_mode: default_lod_mode(),
            suggestion_offset: default_suggestion_offset(),
            grid_snap_enabled: true,
            grid_factor: 1.0,
            engagement_input_by_window: BTreeMap::new(),
            object_kind_weights: HashMap::new(),
            vortex_kind_weights: HashMap::new(),
            sun: WorldSunConfig::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dConfig {
    #[value(default = "default_overlap_budget")]
    pub overlap_budget: f64,
    #[value(default)]
    pub object_kind_weights: HashMap<String, f64>,
    #[value(default)]
    pub vortex_kind_weights: HashMap<String, f64>,
}

impl Default for Puzzle5dConfig {
    fn default() -> Self {
        Self { overlap_budget: default_overlap_budget(), object_kind_weights: HashMap::new(), vortex_kind_weights: HashMap::new() }
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
    SetOverlapBudget { value: f64 },
    SetObjectKindWeights { value: HashMap<String, f64> },
    SetVortexKindWeights { value: HashMap<String, f64> },
}

impl protocol::Mutation<Puzzle5dConfig> for Puzzle5dConfigMutation {
    type Diff = Puzzle5dConfig;

    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📄snapshot", semantic_kind: "snapshot", display_name: "Snapshot", emoji: "📄", aggregate_variant: "Snapshot", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️set-overlap-budget", semantic_kind: "set-overlap-budget", display_name: "Set Overlap Budget", emoji: "🖌️", aggregate_variant: "SetOverlapBudget", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️set-object-kind-weights", semantic_kind: "set-object-kind-weights", display_name: "Set Object Kind Weights", emoji: "🖌️", aggregate_variant: "SetObjectKindWeights", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️set-vortex-kind-weights", semantic_kind: "set-vortex-kind-weights", display_name: "Set Vortex Kind Weights", emoji: "🖌️", aggregate_variant: "SetVortexKindWeights", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Puzzle5dConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
            Puzzle5dConfigMutation::SetOverlapBudget { .. } => &Self::DESCRIPTORS[1],
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
            Puzzle5dConfigMutation::SetOverlapBudget { value } => next.overlap_budget = *value,
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
