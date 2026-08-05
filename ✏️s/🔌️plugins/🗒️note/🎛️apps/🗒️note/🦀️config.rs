//! 🧮️ Note play app — view state (`NoteConfig`) and its operation enum (`NoteConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.note` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/camera/utility edits are VCS'd exactly like
//! document content.

use crate::artifacts::note::NoteCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ Note's real `DocumentApp::Config` — mirrors `shooting_engine::ShootingConfig`'s pilot shape.
/// Absorbs every field that used to live on the old ui crate's `NotePlayRuntime` (selection, hover, the
/// in-progress engagement-rename input, and the free/live canvas camera) plus the two `ViewState`
/// fields the note UI actually reads (`locale`/`active_utility_id`) — see
/// `crate::apps::note::NotePlayApp::render`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "notecfg")]
#[dsl(layout = "lines")]
pub struct NoteConfig {
    /// 👁️ Selected block ids — was `NotePlayRuntime::selected_ids`.
    pub selected_block_ids: Vec<String>,
    /// 👁️ Hovered block id — was `NotePlayRuntime::hovered_id`.
    pub hovered_block_id: Option<String>,
    /// ✏️ In-progress engagement-rename input — was `NotePlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 📷️ The free/live canvas camera — session-only, never a document field. Was
    /// `NotePlayRuntime::camera`.
    #[dsl(block)]
    pub camera: NoteCamera,
    /// 🧰️ The active canvas utility (select/pencil/eraser/…) — was read off
    /// `view_state.active_utility_id` (host-pushed `ViewState`, deleted by the pure-trait migration).
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

impl Default for NoteConfig {
    fn default() -> Self {
        Self { selected_block_ids: Vec::new(), hovered_block_id: None, engagement_input: String::new(), camera: NoteCamera::default(), active_utility_id: "selectDirect".into(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(NoteConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ `NoteConfig`'s operation enum — mirrors `shooting_op::ShootingConfigOperation`'s pilot shape
/// exactly: one variant per settled interaction (the pre-migration `NotePlayRuntime` field writes), plus
/// a generic `Snapshot` every variant's `backwards()` returns — since a config-only "View" dispatch is a
/// plain `Apply` (not an `AmendLast`), each tick is its own distinct, real config edit, and "undo this
/// tick" is exactly "restore the whole-config snapshot from just before it".
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum NoteConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: NoteConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { block_ids: Vec<String> },
    #[dsl(key = "hovered-block")]
    SetHoveredBlock { block_id: Option<String> },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: NoteCamera,
    },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<NoteConfig> for NoteConfigOperation {
    type Diff = NoteConfig;

    fn diff(&self, base: &NoteConfig) -> NoteConfig {
        let mut next = base.clone();
        match self {
            NoteConfigOperation::Snapshot { config } => return config.clone(),
            NoteConfigOperation::SetSelection { block_ids } => next.selected_block_ids = block_ids.clone(),
            NoteConfigOperation::SetHoveredBlock { block_id } => next.hovered_block_id = block_id.clone(),
            NoteConfigOperation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            NoteConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            NoteConfigOperation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            NoteConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &NoteConfig) -> Vec<Self> {
        vec![NoteConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_config_default_matches_the_pre_migration_runtime_defaults() {
        let config = NoteConfig::default();
        assert!(config.selected_block_ids.is_empty());
        assert!(config.hovered_block_id.is_none());
        assert_eq!(config.active_utility_id, "selectDirect");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.camera, NoteCamera::default());
    }

    /// 🧮️ B1 Config dsl/pack round-trip law (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE).
    #[test]
    fn note_config_dsl_pack_round_trips() {
        let config = NoteConfig {
            selected_block_ids: vec!["text-1".into(), "table-2".into()],
            hovered_block_id: Some("image-3".into()),
            engagement_input: "Renaming…".into(),
            camera: NoteCamera { x: 12.5, y: -4.0, zoom: 2.5 },
            active_utility_id: "pencil".into(),
            locale: "de-DE".into(),
        };
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn note_config_operation_text_and_binary_round_trip_every_variant() {
        let config = NoteConfig {
            selected_block_ids: vec!["text-1".into()],
            hovered_block_id: Some("image-2".into()),
            engagement_input: "Renaming…".into(),
            camera: NoteCamera { x: 3.0, y: -1.5, zoom: 1.75 },
            active_utility_id: "pencil".into(),
            locale: "de-DE".into(),
        };
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::Snapshot { config });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetSelection { block_ids: vec!["text-1".into(), "table-2".into()] });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetHoveredBlock { block_id: Some("image-2".into()) });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetHoveredBlock { block_id: None });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetEngagementInput { value: "Renaming…".into() });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetCamera { camera: NoteCamera { x: 4.0, y: 5.0, zoom: 2.0 } });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetActiveUtility { utility_id: "eraserStroke".into() });
        store::test_support::assert_op_text_binary_equivalence(&NoteConfigOperation::SetLocale { value: "de-DE".into() });
    }

    /// 🧮️ Every `NoteConfigOperation`'s `backwards()` is the whole-config snapshot from just before it —
    /// mirrors `shooting_op`'s analogous coverage.
    #[test]
    fn note_config_operation_backwards_is_always_a_snapshot_of_the_prior_config() {
        let base = NoteConfig::default();
        let operation = NoteConfigOperation::SetActiveUtility { utility_id: "pencil".into() };
        assert_eq!(operation.backwards(&base), vec![NoteConfigOperation::Snapshot { config: base.clone() }]);
        let next = operation.diff(&base);
        assert_eq!(next.active_utility_id, "pencil");
        let restored = NoteConfigOperation::Snapshot { config: base.clone() }.diff(&next);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
