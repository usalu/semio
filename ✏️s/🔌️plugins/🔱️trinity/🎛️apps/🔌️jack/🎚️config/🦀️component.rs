//! 🧮️ Trinity Jack app — view-state config + config operations.

use crate::artifacts::jack::Camera;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 🎯️ Ephemeral editor selection range (offsets into the jack query text).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct JackEditorSelection {
    pub start: u64,
    pub end: u64,
}

/// 🧮️ Jack's `DocumentApp::Config` — node selection, the live node-graph viewport camera (seeded once
/// from the initial fixture's seed-only `camera` field, then only ever written by
/// `nodeGraphViewport`), the active fixture/example id, the jack query draft + its last result, the
/// three engagement-input drafts, the reorganize epoch, the editor's text selection, the per-window
/// LOD mode, a completion-request revision counter, and the BCP-47 locale tag.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "jackcfg")]
#[dsl(layout = "lines")]
pub struct JackConfig {
    pub selected_node_ids: Vec<String>,
    #[dsl(block)]
    pub camera: Camera,
    pub active_fixture_id: String,
    pub jack_query: String,
    pub jack_result_json: String,
    pub editor_engagement_input: String,
    pub graph_engagement_input: String,
    pub results_engagement_input: String,
    pub reorganize_epoch: u64,
    #[dsl(block)]
    pub editor_selection: Option<JackEditorSelection>,
    pub lod_mode_by_window: BTreeMap<String, String>,
    pub revision: u64,
    pub locale: String,
}

impl Default for JackConfig {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
            camera: Camera::default(),
            active_fixture_id: String::new(),
            jack_query: String::new(),
            jack_result_json: String::new(),
            editor_engagement_input: String::new(),
            graph_engagement_input: String::new(),
            results_engagement_input: String::new(),
            reorganize_epoch: 0,
            editor_selection: None,
            lod_mode_by_window: BTreeMap::new(),
            revision: 0,
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(JackConfig);

/// @emoji 🧮️ Jack's `JackConfig` operation enum — one variant per settled interaction, plus a generic
/// `Snapshot` every variant's `backwards()` returns. `Snapshot`'s whole-`JackConfig` payload is
/// inherent to the "backwards restores a full prior snapshot" design (mirrors `RewriteConfigOperation`
/// and `shooting_op::ShootingConfigOperation`) — boxing it would perturb the `#[dsl(block)]` wire
/// shape for no behavioral gain, so the size lint is silenced instead of restructuring the type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[allow(clippy::large_enum_variant)]
pub enum JackConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: JackConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { node_ids: Vec<String> },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: Camera,
    },
    #[dsl(key = "active-fixture")]
    SetActiveFixture { value: String },
    #[dsl(key = "query")]
    SetQuery { value: String },
    #[dsl(key = "result")]
    SetResult { value: String },
    #[dsl(key = "editor-input")]
    SetEditorEngagementInput { value: String },
    #[dsl(key = "graph-input")]
    SetGraphEngagementInput { value: String },
    #[dsl(key = "results-input")]
    SetResultsEngagementInput { value: String },
    #[dsl(key = "reorganize-epoch")]
    SetReorganizeEpoch { value: u64 },
    #[dsl(key = "editor-selection")]
    SetEditorSelection {
        #[dsl(block)]
        selection: Option<JackEditorSelection>,
    },
    #[dsl(key = "lod-mode")]
    SetLodMode { window_id: String, value: String },
    #[dsl(key = "revision")]
    SetRevision { value: u64 },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl protocol::Operation<JackConfig> for JackConfigOperation {
    type Diff = JackConfig;

    fn diff(&self, base: &JackConfig) -> JackConfig {
        let mut next = base.clone();
        match self {
            JackConfigOperation::Snapshot { config } => return config.clone(),
            JackConfigOperation::SetSelection { node_ids } => next.selected_node_ids = node_ids.clone(),
            JackConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            JackConfigOperation::SetActiveFixture { value } => next.active_fixture_id = value.clone(),
            JackConfigOperation::SetQuery { value } => next.jack_query = value.clone(),
            JackConfigOperation::SetResult { value } => next.jack_result_json = value.clone(),
            JackConfigOperation::SetEditorEngagementInput { value } => next.editor_engagement_input = value.clone(),
            JackConfigOperation::SetGraphEngagementInput { value } => next.graph_engagement_input = value.clone(),
            JackConfigOperation::SetResultsEngagementInput { value } => next.results_engagement_input = value.clone(),
            JackConfigOperation::SetReorganizeEpoch { value } => next.reorganize_epoch = *value,
            JackConfigOperation::SetEditorSelection { selection } => next.editor_selection = selection.clone(),
            JackConfigOperation::SetLodMode { window_id, value } => {
                next.lod_mode_by_window.insert(window_id.clone(), value.clone());
            }
            JackConfigOperation::SetRevision { value } => next.revision = *value,
            JackConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &JackConfig) -> Vec<Self> {
        vec![JackConfigOperation::Snapshot { config: base.clone() }]
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jack_config_default_has_empty_selection_and_default_locale() {
        let config = JackConfig::default();
        assert!(config.selected_node_ids.is_empty());
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.camera, Camera::default());
    }

    #[test]
    fn jack_config_dsl_round_trips() {
        let mut config = JackConfig {
            selected_node_ids: vec!["n1".into(), "n2".into()],
            jack_query: "MATCH (a:Piece) RETURN a".into(),
            editor_selection: Some(JackEditorSelection { start: 3, end: 9 }),
            ..JackConfig::default()
        };
        config.lod_mode_by_window.insert("trinity-jack-graph".into(), "compact".into());
        store::test_support::assert_dsl_round_trip(&config);
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn jack_config_operation_backwards_restores_prior_snapshot() {
        let base = JackConfig::default();
        let operation = JackConfigOperation::SetSelection { node_ids: vec!["n1".into()] };
        let next = protocol::Operation::diff(&operation, &base);
        assert_eq!(next.selected_node_ids, vec!["n1".to_string()]);
        let backwards = protocol::Operation::backwards(&operation, &base);
        let restored = protocol::Operation::diff(&backwards[0], &next);
        assert_eq!(restored, base);
    }

    #[test]
    fn jack_config_operation_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&JackConfigOperation::SetLodMode { window_id: "trinity-jack-graph".into(), value: "compact".into() });
        store::test_support::assert_op_line_round_trip(&JackConfigOperation::SetSelection { node_ids: vec!["a".into(), "b".into()] });
    }
}
//#endregion 🧪️Tests
