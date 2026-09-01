//! 🎛️ Puzzle 2d play app — its `ArtifactApp::Config`: every piece of view state the app owns but the
//! document must never carry (camera, selection, per-pane LOD/engagement input, brush scratch, grid
//! settings, active utility, locale/terminology), plus typed `ConfigMutation` authorities that
//! patch it.
//!
//! 🎥️ The camera lives here, not on the document: moving it is an `ActionKind::View` action and must
//! never create a VCS edit (see `setCamera`'s arm in `🎮️commands/🎥️set-camera`).

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

//#region 🔖️Defaults
/// 📶️ Mirrors `ui_styling::metrics::board::SUGGESTION_OFFSET`; kept local since the plugin crate has no styling dependency.
pub const PUZZLE2D_DEFAULT_SUGGESTION_OFFSET: f64 = 80.0;

fn default_grid_factor() -> f64 {
    1.0
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

fn default_locale() -> String {
    "en-US".into()
}

fn default_terminology() -> String {
    "native".into()
}
//#endregion 🔖️Defaults

//#region 🧵️FillLifecycle
/// 🧵️ Event-sourced public lifecycle for the transient mounted fill owner.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Puzzle2dFillLifecycle {
    #[default]
    Idle,
    Capturing,
    Queued,
    Running,
    CheckpointReady,
    Applying,
    AwaitingAdoption,
    Closing,
    Completed,
    Cancelled,
    Faulted,
    Discarded,
}

pub const PUZZLE2D_FILL_TEXT_CAPACITY: usize = 64;

/// 🏷️ Fixed backing for bounded fill progress and fault identifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Puzzle2dFillText {
    bytes: [u8; PUZZLE2D_FILL_TEXT_CAPACITY],
    len: u8,
}

impl Puzzle2dFillText {
    pub fn try_from_str(value: &str) -> Option<Self> {
        if value.len() > PUZZLE2D_FILL_TEXT_CAPACITY {
            return None;
        }
        let mut text = Self::default();
        text.bytes[..value.len()].copy_from_slice(value.as_bytes());
        text.len = u8::try_from(value.len()).ok()?;
        Some(text)
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.bytes[..usize::from(self.len)]) }
    }

    pub fn clear(&mut self) {
        self.bytes = [0; PUZZLE2D_FILL_TEXT_CAPACITY];
        self.len = 0;
    }
}

impl Default for Puzzle2dFillText {
    fn default() -> Self {
        Self { bytes: [0; PUZZLE2D_FILL_TEXT_CAPACITY], len: 0 }
    }
}

impl std::ops::Deref for Puzzle2dFillText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Serialize for Puzzle2dFillText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Puzzle2dFillText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FillTextVisitor;

        impl serde::de::Visitor<'_> for FillTextVisitor {
            type Value = Puzzle2dFillText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a fill identifier of at most 64 UTF-8 bytes")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Puzzle2dFillText::try_from_str(value).ok_or_else(|| E::custom("fill identifier capacity exceeded"))
            }
        }

        deserializer.deserialize_str(FillTextVisitor)
    }
}

/// 🧵️ Fixed scalar projection carried by fill-only event mutations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dFillRuntime {
    pub fill_count: u32,
    pub fill_job_operation: u64,
    pub fill_job_generation: u64,
    pub fill_job_seed: u64,
    pub fill_job_base_revision: u64,
    pub fill_job_checkpoint_sequence: u64,
    pub fill_job_accepted_count: u64,
    pub fill_job_search_count: u64,
    pub fill_job_stage: Puzzle2dFillText,
    pub fill_job_lifecycle: Puzzle2dFillLifecycle,
    pub fill_job_fault_code: Option<Puzzle2dFillText>,
}

impl Puzzle2dFillRuntime {
    pub fn from_config(config: &Puzzle2dConfig) -> Self {
        Self {
            fill_count: config.fill_count,
            fill_job_operation: config.fill_job_operation,
            fill_job_generation: config.fill_job_generation,
            fill_job_seed: config.fill_job_seed,
            fill_job_base_revision: config.fill_job_base_revision,
            fill_job_checkpoint_sequence: config.fill_job_checkpoint_sequence,
            fill_job_accepted_count: config.fill_job_accepted_count,
            fill_job_search_count: config.fill_job_search_count,
            fill_job_stage: config.fill_job_stage,
            fill_job_lifecycle: config.fill_job_lifecycle,
            fill_job_fault_code: config.fill_job_fault_code,
        }
    }

    pub fn apply_to(self, config: &mut Puzzle2dConfig) {
        config.fill_count = self.fill_count;
        config.fill_job_operation = self.fill_job_operation;
        config.fill_job_generation = self.fill_job_generation;
        config.fill_job_seed = self.fill_job_seed;
        config.fill_job_base_revision = self.fill_job_base_revision;
        config.fill_job_checkpoint_sequence = self.fill_job_checkpoint_sequence;
        config.fill_job_accepted_count = self.fill_job_accepted_count;
        config.fill_job_search_count = self.fill_job_search_count;
        config.fill_job_stage = self.fill_job_stage;
        config.fill_job_lifecycle = self.fill_job_lifecycle;
        config.fill_job_fault_code = self.fill_job_fault_code;
    }

    pub fn differs_from(&self, config: &Puzzle2dConfig) -> bool {
        self != &Self::from_config(config)
    }
}
//#endregion 🧵️FillLifecycle

//#region 🔖️Config
/// 🧮️ B1: puzzle2d's real `ArtifactApp::Config`. `Puzzle2dConfig` is an alias for it (not a new
/// type), mirroring `Puzzle3dConfig = Puzzle3dRuntime`, so every helper taking a
/// `&Puzzle2dPlayRuntime` keeps working unchanged; every read comes from `cfg.snapshot`, every
/// write flows out as a `Puzzle2dConfigMutation` in the returned `Emit`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dConfig {
    /// 🎥️ The canvas camera (pan/zoom) — session-only view state, never a document/fixture field
    /// (see `setCamera`'s `ActionKind::View`): moving the camera must never create a VCS edit.
    #[serde(default)]
    pub camera_x: f64,
    #[serde(default)]
    pub camera_y: f64,
    #[serde(default = "default_camera_zoom")]
    pub camera_zoom: f64,
    #[serde(default = "default_lod_mode_by_pane")]
    pub lod_mode_by_pane: BTreeMap<String, String>,
    #[serde(default)]
    pub engagement_input_by_pane: BTreeMap<String, String>,
    #[serde(default)]
    pub brush_candidate_index: usize,
    #[serde(default)]
    pub brush_candidates: Vec<Value>,
    #[serde(default)]
    pub brush_candidate_source_handle_id: String,
    #[serde(default)]
    pub fill_count: u32,
    #[serde(default)]
    pub fill_job_operation: u64,
    #[serde(default)]
    pub fill_job_generation: u64,
    #[serde(default)]
    pub fill_job_seed: u64,
    #[serde(default)]
    pub fill_job_base_revision: u64,
    #[serde(default)]
    pub fill_job_checkpoint_sequence: u64,
    #[serde(default)]
    pub fill_job_accepted_count: u64,
    #[serde(default)]
    pub fill_job_search_count: u64,
    #[serde(default)]
    pub fill_job_stage: Puzzle2dFillText,
    #[serde(default)]
    pub fill_job_lifecycle: Puzzle2dFillLifecycle,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fill_job_fault_code: Option<Puzzle2dFillText>,
    #[serde(default)]
    pub grid_snap_enabled: bool,
    #[serde(default = "default_grid_factor")]
    pub grid_factor: f64,
    #[serde(default = "default_suggestion_offset")]
    pub suggestion_offset: f64,
    #[serde(default)]
    pub node_kind_weights: BTreeMap<String, f64>,
    #[serde(default)]
    pub handle_kind_weights: BTreeMap<String, f64>,
    /// 🧰️ B1: host-owned active utility per pane — was host-pushed `view_state.active_utility_by_window_id`;
    /// now the app itself persists it (see `🎮️commands/🧰️set-active-utility`, the only writer).
    #[serde(default)]
    pub active_utility_by_window_id: BTreeMap<String, String>,
    /// 🗣️ B1: BCP-47 locale tag — was host-pushed `view_state.locale` (read via the deleted
    /// `semio_framework_plugin::is_de_locale(&ViewModel)`; see `🦀️terminology.rs`'s `is_de_locale`).
    #[serde(default = "default_locale")]
    pub locale: String,
    /// 🗣️ B1: terminology id ("native" default, or "reuse") — was host-pushed `view_state.terminology`.
    #[serde(default = "default_terminology")]
    pub terminology: String,
    #[serde(default)]
    pub example_load_generation: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example_load_id: Option<String>,
}

/// ⚠️ Explicit impl (not `#[derive(Default)]`) so Rust construction matches the serde field defaults above.
impl Default for Puzzle2dConfig {
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
            fill_count: 0,
            fill_job_operation: 0,
            fill_job_generation: 0,
            fill_job_seed: 1,
            fill_job_base_revision: 0,
            fill_job_checkpoint_sequence: 0,
            fill_job_accepted_count: 0,
            fill_job_search_count: 0,
            fill_job_stage: Puzzle2dFillText::default(),
            fill_job_lifecycle: Puzzle2dFillLifecycle::Idle,
            fill_job_fault_code: None,
            grid_snap_enabled: false,
            grid_factor: default_grid_factor(),
            suggestion_offset: default_suggestion_offset(),
            node_kind_weights: BTreeMap::new(),
            handle_kind_weights: BTreeMap::new(),
            active_utility_by_window_id: BTreeMap::new(),
            locale: default_locale(),
            terminology: default_terminology(),
            example_load_generation: 0,
            example_load_id: None,
        }
    }
}

/// 🏷️ Alias kept for call sites that still name the runtime.
pub type Puzzle2dPlayRuntime = Puzzle2dConfig;

impl store::ArtifactDsl for Puzzle2dConfig {
    const EXTENSION: &'static str = "puzzle2dcfg";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}

impl store::ArtifactPack for Puzzle2dConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(self).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map_err(store::PackError::Schema)
    }
}

store::impl_whole_record_config!(Puzzle2dConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigMutation
/// 🧮️ Carries ordinary config snapshots or the fixed fill-only runtime projection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Puzzle2dConfigMutation {
    Snapshot { config: Puzzle2dConfig },
    Fill { runtime: Puzzle2dFillRuntime },
}

impl protocol::Mutation<Puzzle2dConfig> for Puzzle2dConfigMutation {
    type Diff = Puzzle2dConfig;

    /// 🧷️ Hand-written (no `dsl::Mutations` derive on this enum). ⚠️ PROVISIONAL: neither
    /// `owner` leaf directory below exists on disk yet — these are metadata placeholders to
    /// satisfy `protocol::Mutation`, not real registrations.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📄snapshot", semantic_kind: "snapshot", display_name: "Snapshot", emoji: "📄", aggregate_variant: "Snapshot", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧵fill", semantic_kind: "fill", display_name: "Fill", emoji: "🧵", aggregate_variant: "Fill", payload_schema: "🔣️payload.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Puzzle2dConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
            Puzzle2dConfigMutation::Fill { .. } => &Self::DESCRIPTORS[1],
        }
    }

    fn diff(&self, base: &Puzzle2dConfig) -> protocol::MutationOutcome<Puzzle2dConfig> {
        protocol::MutationOutcome::new(match self {
            Puzzle2dConfigMutation::Snapshot { config } => config.clone(),
            Puzzle2dConfigMutation::Fill { runtime } => {
                let mut config = base.clone();
                runtime.apply_to(&mut config);
                config
            }
        })
    }

    fn inverse(&self, base: &Puzzle2dConfig) -> Vec<Self> {
        match self {
            Puzzle2dConfigMutation::Snapshot { .. } => vec![Puzzle2dConfigMutation::Snapshot { config: base.clone() }],
            Puzzle2dConfigMutation::Fill { .. } => vec![Puzzle2dConfigMutation::Fill { runtime: Puzzle2dFillRuntime::from_config(base) }],
        }
    }
}

impl protocol::OpBinary for Puzzle2dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

impl protocol::OpText for Puzzle2dConfigMutation {
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

    fn fixed_fill_runtime_contract(source: &str) -> bool {
        let production = source.split("//#region 🧪️Tests").next().unwrap_or(source);
        let Some(start) = production.find("pub struct Puzzle2dFillText") else { return false };
        let Some(end_relative) = production[start..].find("//#endregion 🧵️FillLifecycle") else { return false };
        let runtime = &production[start..start + end_relative];
        let Some(mutation_start) = production.find("pub enum Puzzle2dConfigMutation") else { return false };
        let mutation = &production[mutation_start..];
        runtime.contains("bytes: [u8; PUZZLE2D_FILL_TEXT_CAPACITY]")
            && runtime.contains("pub struct Puzzle2dFillRuntime")
            && runtime.contains("pub fill_job_stage: Puzzle2dFillText")
            && runtime.contains("pub fill_job_fault_code: Option<Puzzle2dFillText>")
            && !runtime.contains("Vec<")
            && !runtime.contains("BTreeMap")
            && !runtime.contains("String")
            && mutation.contains("Fill { runtime: Puzzle2dFillRuntime }")
            && mutation.contains("Puzzle2dConfigMutation::Fill { runtime: Puzzle2dFillRuntime::from_config(base) }")
    }

    /// 🧱️ Fill text owns exactly its fixed admitted backing at MAX and rejects MAX+1.
    #[test]
    fn fill_text_capacity_is_exact() {
        let maximum = "x".repeat(PUZZLE2D_FILL_TEXT_CAPACITY);
        let over = "x".repeat(PUZZLE2D_FILL_TEXT_CAPACITY + 1);
        let Some(text) = Puzzle2dFillText::try_from_str(&maximum) else { panic!("MAX fill text must fit") };
        assert_eq!(text.as_str(), maximum);
        assert!(Puzzle2dFillText::try_from_str(&over).is_none());
    }

    /// 🧬️ Dynamic fill runtime backing or whole-snapshot fill inverse mutations fail the source law.
    #[test]
    fn dynamic_fill_runtime_mutations_are_rejected() {
        let source = include_str!("🦀️component.rs");
        assert!(fixed_fill_runtime_contract(source));
        let dynamic = source.replacen("pub fill_job_stage: Puzzle2dFillText,", "pub brush_candidates: Vec<Value>, pub fill_job_stage: String,", 1);
        assert!(!fixed_fill_runtime_contract(&dynamic));
        let snapshot = source.replacen("Puzzle2dConfigMutation::Fill { runtime: Puzzle2dFillRuntime::from_config(base) }", "Puzzle2dConfigMutation::Snapshot { config: base.clone() }", 1);
        assert!(!fixed_fill_runtime_contract(&snapshot));
    }
}
//#endregion 🧪️Tests
