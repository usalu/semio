//! 🎛️ Puzzle 5d play app — its `DocumentApp::Config`: every piece of view state the app owns but the
//! document must never carry (the two projections' cameras, selection, hover, brush/fill scratch,
//! distribution weights, per-window engagement input and active utility, sun, locale/terminology),
//! plus the whole-snapshot `ConfigMutation` that patches it.
//!
//! 🪟️ Unlike puzzle3d, puzzle5d's two window KINDS are each single-instance, so there is no
//! per-instance `load_window`/`save_window` swap here — every field is flat and shared across the
//! two windows except the explicitly window-keyed `engagement_input_by_window`/
//! `active_utility_by_window_id` maps.

use semio_framework_plugin::{SelectionSet, WorldSunConfig};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

//#region 🔖️Defaults
fn one_f64() -> f64 {
    1.0
}

fn default_selection_method() -> String {
    "rectangle".into()
}

fn default_overlap_budget() -> f64 {
    0.02
}

fn default_lod_mode() -> String {
    crate::apps::puzzle5d::PUZZLE5D_LOD_MODE_AUTOMATIC.into()
}

fn default_suggestion_offset() -> f64 {
    crate::apps::puzzle5d::PUZZLE5D_DEFAULT_SUGGESTION_OFFSET
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

//#region 🔖️Selection
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dSelection {
    #[serde(default)]
    pub part_ids: SelectionSet,
    #[serde(default)]
    pub grip_ids: SelectionSet,
    #[serde(default)]
    pub fastener_ids: SelectionSet,
}

/// 🧹️ Clears every selection bag.
pub fn puzzle5d_clear_selection(selection: &mut Puzzle5dSelection) {
    *selection = Puzzle5dSelection::default();
}

/// 🧹️ Clears every selection bag except part ids.
pub fn puzzle5d_clear_non_part_selection(selection: &mut Puzzle5dSelection) {
    selection.grip_ids.clear();
    selection.fastener_ids.clear();
}

/// 🧹️ Clears every selection bag except grip ids.
pub fn puzzle5d_clear_non_grip_selection(selection: &mut Puzzle5dSelection) {
    selection.part_ids.clear();
    selection.fastener_ids.clear();
}
//#endregion 🔖️Selection

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dRuntime {
    /// 📷️ Camera pose — session-only view state (`ActionKind::View`), never a VCS document field:
    /// see `setCamera`/`setCamera2d`/`setCamera3d` in `🎮️commands/🎥️camera`.
    #[serde(default)]
    pub camera2d: Puzzle5dCamera2d,
    #[serde(default)]
    pub camera3d: Puzzle5dCamera3d,
    #[serde(default)]
    pub selection: Puzzle5dSelection,
    #[serde(default = "default_selection_method")]
    pub selection_method: String,
    #[serde(default)]
    pub hovered_part_id: Option<String>,
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
    /// now real VCS'd config (see `SET_ACTIVE_UTILITY_ACTION_ID` in `🎮️commands/🧰️utility`).
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
impl Default for Puzzle5dRuntime {
    fn default() -> Self {
        Self {
            camera2d: Puzzle5dCamera2d { x: 0.0, y: 0.0, zoom: 1.0 },
            camera3d: Puzzle5dCamera3d { position: [8.0, -8.0, 8.0], target: [0.0, 0.0, 0.0], zoom: 1.0 },
            selection: Puzzle5dSelection::default(),
            selection_method: default_selection_method(),
            hovered_part_id: None,
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

/// 🧮️ B1: puzzle5d's real `DocumentApp::Config` — `Puzzle5dRuntime` itself doubles as the config
/// record (an alias, not a new type), mirroring `Puzzle3dConfig`'s identical recipe, so every helper
/// taking `&Puzzle5dRuntime`/`&mut Puzzle5dRuntime` keeps working unchanged; every read comes from
/// `cfg.projection`, every write flows out as a `Puzzle5dConfigMutation` in the returned `Emit`
/// instead of a silent `self` mutation.
pub type Puzzle5dConfig = Puzzle5dRuntime;

impl store::DocumentDsl for Puzzle5dRuntime {
    const EXTENSION: &'static str = "puzzle5dcfg";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}

impl store::DocumentPack for Puzzle5dRuntime {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(self).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map_err(store::PackError::Schema)
    }
}

store::impl_whole_record_config!(Puzzle5dRuntime);
//#endregion 🔖️Config

//#region 🔖️ConfigMutation
/// 🧮️ B1: `Puzzle5dConfig`'s operation enum. Every real config edit is captured as "the whole config
/// after this edit"; `backwards()` is the same one-liner regardless of what changed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Puzzle5dConfigMutation {
    Snapshot { config: Puzzle5dConfig },
}

impl protocol::Mutation<Puzzle5dConfig> for Puzzle5dConfigMutation {
    type Diff = Puzzle5dConfig;

    fn diff(&self, _base: &Puzzle5dConfig) -> Puzzle5dConfig {
        match self {
            Puzzle5dConfigMutation::Snapshot { config } => config.clone(),
        }
    }

    fn inverse(&self, base: &Puzzle5dConfig) -> Vec<Self> {
        vec![Puzzle5dConfigMutation::Snapshot { config: base.clone() }]
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
