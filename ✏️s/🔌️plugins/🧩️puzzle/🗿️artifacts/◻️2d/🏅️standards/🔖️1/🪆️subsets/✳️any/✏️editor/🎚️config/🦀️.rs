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
    pub brush_candidates: Vec<dsl::DslValue>,
    #[value(default)]
    pub brush_candidate_source_handle_id: String,
    /// 📄 Per-section page cursor of the virtualised panels (window transient, `setPanelPage`).
    #[value(default)]
    pub panel_pages: BTreeMap<String, u32>,
    #[value(default = "default_fill_count")]
    pub fill_count: u32,
    #[value(default)]
    pub grid_snap_enabled: bool,
    #[value(default = "default_grid_factor")]
    pub grid_factor: f64,
    #[value(default = "default_suggestion_offset")]
    pub suggestion_offset: f64,
    #[value(default)]
    pub node_kind_weights: BTreeMap<String, f64>,
    #[value(default)]
    pub handle_kind_weights: BTreeMap<String, f64>,
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
            panel_pages: BTreeMap::new(),
            fill_count: default_fill_count(),
            grid_snap_enabled: false,
            grid_factor: default_grid_factor(),
            suggestion_offset: default_suggestion_offset(),
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
}

impl Default for Puzzle2dConfig {
    fn default() -> Self {
        Self { node_kind_weights: BTreeMap::new(), handle_kind_weights: BTreeMap::new(), fill_count: default_fill_count() }
    }
}

impl store::ArtifactDsl for Puzzle2dConfig {
    const EXTENSION: &'static str = "puzzle2dcfg";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        dsl::json::to_string_pretty(&dsl::json::from_dsl_value(&dsl::ToValue::to_value(self)))
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum Puzzle2dConfigMutation {
    Snapshot { config: Puzzle2dConfig },
}

impl protocol::Mutation<Puzzle2dConfig> for Puzzle2dConfigMutation {
    type Diff = Puzzle2dConfig;

    /// 🧷️ Hand-written (no `dsl::Mutations` derive on this enum). ⚠️ PROVISIONAL: neither
    /// `owner` leaf directory below exists on disk yet — these are metadata placeholders to
    /// satisfy `protocol::Mutation`, not real registrations.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📄snapshot",
        semantic_kind: "snapshot",
        display_name: "Snapshot",
        emoji: "📄",
        aggregate_variant: "Snapshot",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Puzzle2dConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
        }
    }

    fn diff(&self, _base: &Puzzle2dConfig) -> protocol::MutationOutcome<Puzzle2dConfig> {
        protocol::MutationOutcome::new(match self {
            Puzzle2dConfigMutation::Snapshot { config } => config.clone(),
        })
    }

    fn inverse(&self, base: &Puzzle2dConfig) -> Vec<Self> {
        vec![Puzzle2dConfigMutation::Snapshot { config: base.clone() }]
    }
}

impl protocol::OpBinary for Puzzle2dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(dsl::json::to_json_string(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

impl protocol::OpText for Puzzle2dConfigMutation {
    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️ConfigMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
