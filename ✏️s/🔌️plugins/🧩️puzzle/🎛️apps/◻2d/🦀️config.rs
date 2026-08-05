//! 🎛️ Puzzle 2d play app — its `DocumentApp::Config`: every piece of view state the app owns but the
//! document must never carry (camera, selection, per-pane LOD/engagement input, brush scratch, grid
//! settings, active utility, locale/terminology), plus the whole-snapshot `ConfigOperation` that
//! patches it.
//!
//! 🎥️ The camera lives here, not on the document: moving it is an `ActionKind::View` action and must
//! never create a VCS edit (see `setCamera`'s arm in `🎮️commands/🎥️camera`).

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

//#region 🔖️Defaults
/// 📶️ Mirrors `ui_styling::metrics::board::SUGGESTION_OFFSET`; kept local since the plugin crate has no styling dependency.
pub const PUZZLE2D_DEFAULT_SUGGESTION_OFFSET: f64 = 80.0;

fn default_selection_method() -> String {
    "rectangle".into()
}

fn default_grid_factor() -> f64 {
    1.0
}

fn default_suggestion_offset() -> f64 {
    PUZZLE2D_DEFAULT_SUGGESTION_OFFSET
}

/// 📶️ Overview/selection default to automatic LOD; detail defaults to a fixed "detail" tier, matching the pre-migration triptych.
fn default_lod_mode_by_pane() -> BTreeMap<String, String> {
    use crate::apps::puzzle2d::modes::edit::windows::{detail, overview, selection};
    BTreeMap::from([
        (overview::WINDOW_KIND_ID.to_string(), crate::apps::puzzle2d::PUZZLE2D_LOD_MODE_AUTOMATIC.to_string()),
        (detail::WINDOW_KIND_ID.to_string(), "detail".to_string()),
        (selection::WINDOW_KIND_ID.to_string(), crate::apps::puzzle2d::PUZZLE2D_LOD_MODE_AUTOMATIC.to_string()),
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

//#region 🔖️Config
/// 🧮️ B1: puzzle2d's real `DocumentApp::Config`. `Puzzle2dConfig` is an alias for it (not a new
/// type), mirroring `Puzzle3dConfig = Puzzle3dRuntime`, so every helper taking a
/// `&Puzzle2dPlayRuntime` keeps working unchanged; every read comes from `cfg.projection`, every
/// write flows out as a `Puzzle2dConfigOperation` in the returned `Emit`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dPlayRuntime {
    #[serde(default)]
    pub selected_ids: Vec<String>,
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
    #[serde(default = "default_selection_method")]
    pub selection_method: String,
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
    /// now the app itself persists it (see `🎮️commands/🧰️utility`, the only writer).
    #[serde(default)]
    pub active_utility_by_window_id: BTreeMap<String, String>,
    /// 🗣️ B1: BCP-47 locale tag — was host-pushed `view_state.locale` (read via the deleted
    /// `semio_framework_plugin::is_de_locale(&ViewState)`; see `🦀️terminology.rs`'s `is_de_locale`).
    #[serde(default = "default_locale")]
    pub locale: String,
    /// 🗣️ B1: terminology id ("native" default, or "reuse") — was host-pushed `view_state.terminology`.
    #[serde(default = "default_terminology")]
    pub terminology: String,
}

/// ⚠️ Explicit impl (not `#[derive(Default)]`) so Rust construction matches the serde field defaults above.
impl Default for Puzzle2dPlayRuntime {
    fn default() -> Self {
        Self {
            selected_ids: Vec::new(),
            camera_x: 0.0,
            camera_y: 0.0,
            camera_zoom: default_camera_zoom(),
            lod_mode_by_pane: default_lod_mode_by_pane(),
            engagement_input_by_pane: BTreeMap::new(),
            brush_candidate_index: 0,
            brush_candidates: Vec::new(),
            brush_candidate_source_handle_id: String::new(),
            fill_count: 0,
            selection_method: default_selection_method(),
            grid_snap_enabled: false,
            grid_factor: default_grid_factor(),
            suggestion_offset: default_suggestion_offset(),
            node_kind_weights: BTreeMap::new(),
            handle_kind_weights: BTreeMap::new(),
            active_utility_by_window_id: BTreeMap::new(),
            locale: default_locale(),
            terminology: default_terminology(),
        }
    }
}

pub type Puzzle2dConfig = Puzzle2dPlayRuntime;

impl store::DocumentDsl for Puzzle2dPlayRuntime {
    const EXTENSION: &'static str = "puzzle2dcfg";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}

impl store::DocumentPack for Puzzle2dPlayRuntime {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(self).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map_err(store::PackError::Schema)
    }
}

store::impl_whole_record_config!(Puzzle2dPlayRuntime);
//#endregion 🔖️Config

//#region 🔖️ConfigOperation
/// 🧮️ B1: `Puzzle2dConfig`'s operation enum. Mirrors `Puzzle3dConfigOperation`'s single-generic-
/// `Snapshot`-variant pattern exactly: every real config edit is captured as "the whole config after
/// this edit"; `backwards()` restores the whole-config snapshot from just before, regardless of what
/// changed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Puzzle2dConfigOperation {
    Snapshot { config: Puzzle2dConfig },
}

impl protocol::Operation<Puzzle2dConfig> for Puzzle2dConfigOperation {
    type Diff = Puzzle2dConfig;

    fn diff(&self, _base: &Puzzle2dConfig) -> Puzzle2dConfig {
        match self {
            Puzzle2dConfigOperation::Snapshot { config } => config.clone(),
        }
    }

    fn backwards(&self, base: &Puzzle2dConfig) -> Vec<Self> {
        vec![Puzzle2dConfigOperation::Snapshot { config: base.clone() }]
    }
}

impl protocol::OpBinary for Puzzle2dConfigOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

impl protocol::OpText for Puzzle2dConfigOperation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️ConfigOperation
