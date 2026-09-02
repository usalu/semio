//! 🎛️ Puzzle 5d play app — its `ArtifactApp::Config`: every piece of view state the app owns but the
//! document must never carry (the two projections' cameras, selection, hover, brush/fill scratch,
//! distribution weights, per-window engagement input and active utility, sun, locale/terminology),
//! plus the whole-snapshot `ConfigMutation` that patches it.
//!
//! 🪟️ Unlike puzzle3d, puzzle5d's two window KINDS are each single-instance, so there is no
//! per-instance `load_window`/`save_window` swap here — every field is flat and shared across the
//! two windows except the explicitly window-keyed `engagement_input_by_window`/
//! `active_utility_by_window_id` maps.

use semio_framework_plugin::WorldSunConfig;
use serde::{Deserialize, Serialize};
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

fn default_terminology() -> String {
    "native".into()
}

fn default_locale() -> String {
    "en-US".into()
}
//#endregion 🔖️Defaults

//#region 🔖️Cameras
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCamera2d {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "one_f64")]
    pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dCamera3d {
    #[serde(default)]
    pub position: [f64; 3],
    #[serde(default)]
    pub target: [f64; 3],
    #[serde(default = "one_f64")]
    pub zoom: f64,
}
//#endregion 🔖️Cameras

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dConfig {
    /// 📷️ Camera pose — session-only view state (`ActionKind::View`), never a VCS document field:
    /// see `setCamera`/`setCamera2d`/`setCamera3d` in `🎮️commands/🎥️set-camera`.
    #[serde(default)]
    pub camera2d: Puzzle5dCamera2d,
    #[serde(default)]
    pub camera3d: Puzzle5dCamera3d,
    #[serde(default)]
    pub fill_count: u32,
    #[serde(default)]
    pub brush_candidate_index: usize,
    #[serde(default = "default_overlap_budget")]
    pub overlap_budget: f64,
    #[serde(default = "default_lod_mode")]
    pub lod_mode: String,
    #[serde(default = "default_suggestion_offset")]
    pub suggestion_offset: f64,
    #[serde(default = "default_true")]
    pub grid_snap_enabled: bool,
    #[serde(default = "one_f64")]
    pub grid_factor: f64,
    #[serde(default)]
    pub engagement_input_by_window: BTreeMap<String, String>,
    #[serde(default)]
    pub object_kind_weights: HashMap<String, f64>,
    #[serde(default)]
    pub vortex_kind_weights: HashMap<String, f64>,
    #[serde(default)]
    pub sun: WorldSunConfig,
    /// 🧰️ B1: per-window (kind-keyed — puzzle5d's two window KINDS are each single-instance, see
    /// `window_instance_ids`) active utility — was host-pushed `view_state.active_utility_by_window_id`,
    /// now real VCS'd config (see `SET_ACTIVE_UTILITY_ACTION_ID` in `🎮️commands/🧰️set-active`).
    #[serde(default)]
    pub active_utility_by_window_id: BTreeMap<String, String>,
    /// 🗣️ B1: terminology overlay (native/reuse) — was host-pushed `view_state.terminology`.
    #[serde(default = "default_terminology")]
    pub terminology: String,
    /// 🗣️ B1: BCP-47 locale tag — was host-pushed `view_state.locale`.
    #[serde(default = "default_locale")]
    pub locale: String,
}

/// ⚠️ Explicit impl (not `#[derive(Default)]`) so Rust construction matches the serde field defaults above.
impl Default for Puzzle5dConfig {
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
            active_utility_by_window_id: BTreeMap::new(),
            terminology: default_terminology(),
            locale: default_locale(),
        }
    }
}

/// 🧮️ B1: puzzle5d's real `ArtifactApp::Config` — `Puzzle5dRuntime` itself doubles as the config
/// record (an alias, not a new type), mirroring `Puzzle3dConfig`'s identical recipe, so every helper
/// taking `&Puzzle5dRuntime`/`&mut Puzzle5dRuntime` keeps working unchanged; every read comes from
/// `cfg.snapshot`, every write flows out as a `Puzzle5dConfigMutation` in the returned `Emit`
/// instead of a silent `self` mutation.
/// 🏷️ Alias kept for call sites that still name the runtime.
pub type Puzzle5dRuntime = Puzzle5dConfig;

impl store::ArtifactDsl for Puzzle5dConfig {
    const EXTENSION: &'static str = "puzzle5dcfg";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Puzzle5dConfigMutation {
    Snapshot { config: Puzzle5dConfig },
    SetBrushCandidateIndex { index: usize },
    SetActiveUtility { window_id: String, value: Option<String> },
    SetCamera2d { camera: Puzzle5dCamera2d },
    SetCamera3d { camera: Puzzle5dCamera3d },
    SetEngagementInput { window_id: String, value: String },
    SetFillCount { count: u32 },
    SetGridFactor { value: f64 },
    SetGridSnapEnabled { enabled: bool },
    SetLodMode { mode: String },
    SetOverlapBudget { value: f64 },
    SetObjectKindWeights { value: HashMap<String, f64> },
    SetSuggestionOffset { distance: f64 },
    SetSun { sun: WorldSunConfig },
    SetVortexKindWeights { value: HashMap<String, f64> },
}

impl protocol::Mutation<Puzzle5dConfig> for Puzzle5dConfigMutation {
    type Diff = Puzzle5dConfig;

    /// 🧷️ Hand-written (not `#[derive(dsl::Mutations)]`: this is a plain whole-config-record
    /// mutation enum, not a `dsl::Mutations`-eligible semantic-document vocabulary — see
    /// `🧬️schema/🧬️mutations/🦀️.rs`'s derive for the contrast). ⚠️ PROVISIONAL: none of
    /// the fifteen `owner` paths below name a directory that exists on disk — this enum has no
    /// `🧬️mutations/<slug>` leaf triads of its own (every field lives flat in `component.rs`), so
    /// every entry is a metadata placeholder to satisfy `protocol::Mutation`, matching stdio's
    /// `🔊️wav`/`🏗️ifc` precedent for enums in the same situation.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📄snapshot", semantic_kind: "snapshot", display_name: "Snapshot", emoji: "📄", aggregate_variant: "Snapshot", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️set-brush-candidate-index", semantic_kind: "set-brush-candidate-index", display_name: "Set Brush Candidate Index", emoji: "🖌️", aggregate_variant: "SetBrushCandidateIndex", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧰️set-active-utility", semantic_kind: "set-active-utility", display_name: "Set Active Utility", emoji: "🧰️", aggregate_variant: "SetActiveUtility", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🎥️set-camera2d", semantic_kind: "set-camera2d", display_name: "Set Camera2d", emoji: "🎥️", aggregate_variant: "SetCamera2d", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🎥️set-camera3d", semantic_kind: "set-camera3d", display_name: "Set Camera3d", emoji: "🎥️", aggregate_variant: "SetCamera3d", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🤝️set-engagement-input", semantic_kind: "set-engagement-input", display_name: "Set Engagement Input", emoji: "🤝️", aggregate_variant: "SetEngagementInput", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🪣️set-fill-count", semantic_kind: "set-fill-count", display_name: "Set Fill Count", emoji: "🪣️", aggregate_variant: "SetFillCount", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🌐️set-grid-factor", semantic_kind: "set-grid-factor", display_name: "Set Grid Factor", emoji: "🌐️", aggregate_variant: "SetGridFactor", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🌐️set-grid-snap-enabled", semantic_kind: "set-grid-snap-enabled", display_name: "Set Grid Snap Enabled", emoji: "🌐️", aggregate_variant: "SetGridSnapEnabled", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🔭️set-lod-mode", semantic_kind: "set-lod-mode", display_name: "Set Lod Mode", emoji: "🔭️", aggregate_variant: "SetLodMode", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️set-overlap-budget", semantic_kind: "set-overlap-budget", display_name: "Set Overlap Budget", emoji: "🖌️", aggregate_variant: "SetOverlapBudget", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️set-object-kind-weights", semantic_kind: "set-object-kind-weights", display_name: "Set Object Kind Weights", emoji: "🖌️", aggregate_variant: "SetObjectKindWeights", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️set-suggestion-offset", semantic_kind: "set-suggestion-offset", display_name: "Set Suggestion Offset", emoji: "🖌️", aggregate_variant: "SetSuggestionOffset", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/☀️set-sun", semantic_kind: "set-sun", display_name: "Set Sun", emoji: "☀️", aggregate_variant: "SetSun", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖌️set-vortex-kind-weights", semantic_kind: "set-vortex-kind-weights", display_name: "Set Vortex Kind Weights", emoji: "🖌️", aggregate_variant: "SetVortexKindWeights", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Puzzle5dConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
            Puzzle5dConfigMutation::SetBrushCandidateIndex { .. } => &Self::DESCRIPTORS[1],
            Puzzle5dConfigMutation::SetActiveUtility { .. } => &Self::DESCRIPTORS[2],
            Puzzle5dConfigMutation::SetCamera2d { .. } => &Self::DESCRIPTORS[3],
            Puzzle5dConfigMutation::SetCamera3d { .. } => &Self::DESCRIPTORS[4],
            Puzzle5dConfigMutation::SetEngagementInput { .. } => &Self::DESCRIPTORS[5],
            Puzzle5dConfigMutation::SetFillCount { .. } => &Self::DESCRIPTORS[6],
            Puzzle5dConfigMutation::SetGridFactor { .. } => &Self::DESCRIPTORS[7],
            Puzzle5dConfigMutation::SetGridSnapEnabled { .. } => &Self::DESCRIPTORS[8],
            Puzzle5dConfigMutation::SetLodMode { .. } => &Self::DESCRIPTORS[9],
            Puzzle5dConfigMutation::SetOverlapBudget { .. } => &Self::DESCRIPTORS[10],
            Puzzle5dConfigMutation::SetObjectKindWeights { .. } => &Self::DESCRIPTORS[11],
            Puzzle5dConfigMutation::SetSuggestionOffset { .. } => &Self::DESCRIPTORS[12],
            Puzzle5dConfigMutation::SetSun { .. } => &Self::DESCRIPTORS[13],
            Puzzle5dConfigMutation::SetVortexKindWeights { .. } => &Self::DESCRIPTORS[14],
        }
    }

    fn diff(&self, base: &Puzzle5dConfig) -> protocol::MutationOutcome<Puzzle5dConfig> {
        if let Puzzle5dConfigMutation::Snapshot { config } = self {
            return protocol::MutationOutcome::new(config.clone());
        }
        let mut next = base.clone();
        match self {
            Puzzle5dConfigMutation::Snapshot { .. } => {}
            Puzzle5dConfigMutation::SetBrushCandidateIndex { index } => next.brush_candidate_index = *index,
            Puzzle5dConfigMutation::SetActiveUtility { window_id, value } => {
                if let Some(value) = value {
                    next.active_utility_by_window_id.insert(window_id.clone(), value.clone());
                } else {
                    next.active_utility_by_window_id.remove(window_id);
                }
            }
            Puzzle5dConfigMutation::SetCamera2d { camera } => next.camera2d = camera.clone(),
            Puzzle5dConfigMutation::SetCamera3d { camera } => next.camera3d = camera.clone(),
            Puzzle5dConfigMutation::SetEngagementInput { window_id, value } => {
                next.engagement_input_by_window.insert(window_id.clone(), value.clone());
            }
            Puzzle5dConfigMutation::SetFillCount { count } => next.fill_count = *count,
            Puzzle5dConfigMutation::SetGridFactor { value } => next.grid_factor = *value,
            Puzzle5dConfigMutation::SetGridSnapEnabled { enabled } => next.grid_snap_enabled = *enabled,
            Puzzle5dConfigMutation::SetLodMode { mode } => next.lod_mode = mode.clone(),
            Puzzle5dConfigMutation::SetOverlapBudget { value } => next.overlap_budget = *value,
            Puzzle5dConfigMutation::SetObjectKindWeights { value } => next.object_kind_weights = value.clone(),
            Puzzle5dConfigMutation::SetSuggestionOffset { distance } => next.suggestion_offset = *distance,
            Puzzle5dConfigMutation::SetSun { sun } => next.sun = sun.clone(),
            Puzzle5dConfigMutation::SetVortexKindWeights { value } => next.vortex_kind_weights = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Puzzle5dConfig) -> Vec<Self> {
        vec![match self {
            Puzzle5dConfigMutation::Snapshot { .. } => Puzzle5dConfigMutation::Snapshot { config: base.clone() },
            Puzzle5dConfigMutation::SetBrushCandidateIndex { .. } => Puzzle5dConfigMutation::SetBrushCandidateIndex { index: base.brush_candidate_index },
            Puzzle5dConfigMutation::SetActiveUtility { window_id, .. } => Puzzle5dConfigMutation::SetActiveUtility {
                window_id: window_id.clone(),
                value: base.active_utility_by_window_id.get(window_id).cloned(),
            },
            Puzzle5dConfigMutation::SetCamera2d { .. } => Puzzle5dConfigMutation::SetCamera2d { camera: base.camera2d.clone() },
            Puzzle5dConfigMutation::SetCamera3d { .. } => Puzzle5dConfigMutation::SetCamera3d { camera: base.camera3d.clone() },
            Puzzle5dConfigMutation::SetEngagementInput { window_id, .. } => Puzzle5dConfigMutation::SetEngagementInput {
                window_id: window_id.clone(),
                value: base.engagement_input_by_window.get(window_id).cloned().unwrap_or_default(),
            },
            Puzzle5dConfigMutation::SetFillCount { .. } => Puzzle5dConfigMutation::SetFillCount { count: base.fill_count },
            Puzzle5dConfigMutation::SetGridFactor { .. } => Puzzle5dConfigMutation::SetGridFactor { value: base.grid_factor },
            Puzzle5dConfigMutation::SetGridSnapEnabled { .. } => Puzzle5dConfigMutation::SetGridSnapEnabled { enabled: base.grid_snap_enabled },
            Puzzle5dConfigMutation::SetLodMode { .. } => Puzzle5dConfigMutation::SetLodMode { mode: base.lod_mode.clone() },
            Puzzle5dConfigMutation::SetOverlapBudget { .. } => Puzzle5dConfigMutation::SetOverlapBudget { value: base.overlap_budget },
            Puzzle5dConfigMutation::SetObjectKindWeights { .. } => Puzzle5dConfigMutation::SetObjectKindWeights { value: base.object_kind_weights.clone() },
            Puzzle5dConfigMutation::SetSuggestionOffset { .. } => Puzzle5dConfigMutation::SetSuggestionOffset { distance: base.suggestion_offset },
            Puzzle5dConfigMutation::SetSun { .. } => Puzzle5dConfigMutation::SetSun { sun: base.sun.clone() },
            Puzzle5dConfigMutation::SetVortexKindWeights { .. } => Puzzle5dConfigMutation::SetVortexKindWeights { value: base.vortex_kind_weights.clone() },
        }]
    }
}

impl protocol::OpBinary for Puzzle5dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

impl protocol::OpText for Puzzle5dConfigMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️ConfigMutation
