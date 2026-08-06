//! 🧮️ Trinity Rewrite app — view-state config + config operations.

use crate::artifacts::jack::Camera;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 🧮️ Rewrite's `DocumentApp::Config` — node selection, the Before pane's live viewport camera
/// (seeded once from the initial before-fixture's seed-only `camera` field, then only ever written by
/// `nodeGraphViewport`), the reorganize epoch, the hover/select var focus + their epochs, the
/// per-window LOD mode, and the BCP-47 locale tag.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "rewritecfg")]
#[dsl(layout = "lines")]
pub struct RewriteConfig {
    pub selected_node_ids: Vec<String>,
    #[dsl(block)]
    pub before_pane_camera: Camera,
    pub reorganize_epoch: u64,
    pub active_hover_var: String,
    pub hover_epoch: u64,
    pub active_select_var: String,
    pub select_epoch: u64,
    pub lod_mode_by_window: BTreeMap<String, String>,
    pub locale: String,
}

impl Default for RewriteConfig {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
            before_pane_camera: Camera::default(),
            reorganize_epoch: 0,
            active_hover_var: String::new(),
            hover_epoch: 0,
            active_select_var: String::new(),
            select_epoch: 0,
            lod_mode_by_window: BTreeMap::new(),
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(RewriteConfig);

/// @emoji 🧮️ Rewrite's `RewriteConfig` operation enum — one variant per settled interaction, plus a
/// generic `Snapshot` every variant's `backwards()` returns. See `JackConfigOperation`'s doc comment
/// for why `Snapshot`'s size is allowed rather than boxed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[allow(clippy::large_enum_variant)]
pub enum RewriteConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: RewriteConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { node_ids: Vec<String> },
    #[dsl(key = "before-pane-camera")]
    SetBeforePaneCamera {
        #[dsl(block)]
        camera: Camera,
    },
    #[dsl(key = "reorganize-epoch")]
    SetReorganizeEpoch { value: u64 },
    #[dsl(key = "active-hover-var")]
    SetActiveHoverVar { value: String },
    #[dsl(key = "hover-epoch")]
    SetHoverEpoch { value: u64 },
    #[dsl(key = "active-select-var")]
    SetActiveSelectVar { value: String },
    #[dsl(key = "select-epoch")]
    SetSelectEpoch { value: u64 },
    #[dsl(key = "lod-mode")]
    SetLodMode { window_id: String, value: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl protocol::Operation<RewriteConfig> for RewriteConfigOperation {
    type Diff = RewriteConfig;

    fn diff(&self, base: &RewriteConfig) -> RewriteConfig {
        let mut next = base.clone();
        match self {
            RewriteConfigOperation::Snapshot { config } => return config.clone(),
            RewriteConfigOperation::SetSelection { node_ids } => next.selected_node_ids = node_ids.clone(),
            RewriteConfigOperation::SetBeforePaneCamera { camera } => next.before_pane_camera = camera.clone(),
            RewriteConfigOperation::SetReorganizeEpoch { value } => next.reorganize_epoch = *value,
            RewriteConfigOperation::SetActiveHoverVar { value } => next.active_hover_var = value.clone(),
            RewriteConfigOperation::SetHoverEpoch { value } => next.hover_epoch = *value,
            RewriteConfigOperation::SetActiveSelectVar { value } => next.active_select_var = value.clone(),
            RewriteConfigOperation::SetSelectEpoch { value } => next.select_epoch = *value,
            RewriteConfigOperation::SetLodMode { window_id, value } => {
                next.lod_mode_by_window.insert(window_id.clone(), value.clone());
            }
            RewriteConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &RewriteConfig) -> Vec<Self> {
        vec![RewriteConfigOperation::Snapshot { config: base.clone() }]
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrite_config_default_has_empty_selection_and_default_locale() {
        let config = RewriteConfig::default();
        assert!(config.selected_node_ids.is_empty());
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.before_pane_camera, Camera::default());
    }

    #[test]
    fn rewrite_config_dsl_round_trips() {
        let mut config = RewriteConfig { selected_node_ids: vec!["n1".into()], active_hover_var: "a".into(), ..RewriteConfig::default() };
        config.lod_mode_by_window.insert("trinity-rewrite-before".into(), "compact".into());
        store::test_support::assert_dsl_round_trip(&config);
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn rewrite_config_operation_backwards_restores_prior_snapshot() {
        let base = RewriteConfig::default();
        let operation = RewriteConfigOperation::SetSelection { node_ids: vec!["n1".into()] };
        let next = protocol::Operation::diff(&operation, &base);
        assert_eq!(next.selected_node_ids, vec!["n1".to_string()]);
        let backwards = protocol::Operation::backwards(&operation, &base);
        let restored = protocol::Operation::diff(&backwards[0], &next);
        assert_eq!(restored, base);
    }

    #[test]
    fn rewrite_config_operation_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&RewriteConfigOperation::SetLodMode { window_id: "trinity-rewrite-before".into(), value: "compact".into() });
        store::test_support::assert_op_line_round_trip(&RewriteConfigOperation::SetSelection { node_ids: vec!["a".into(), "b".into()] });
    }
}
//#endregion 🧪️Tests
