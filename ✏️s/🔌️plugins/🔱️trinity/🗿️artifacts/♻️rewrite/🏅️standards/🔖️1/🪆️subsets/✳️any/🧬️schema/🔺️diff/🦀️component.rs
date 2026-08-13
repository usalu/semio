//! 🧬️ Rewrite diff schema — sparse field delta over the artifact.

use crate::artifacts::jack::{Camera, PropertyValue};
use crate::artifacts::rewrite::LayoutPoint;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the rewrite artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.trinity.rewrite")]
pub struct RewriteDiff {
    #[state(artifact)] pub before_fixture_json: Option<String>,
    #[state(artifact)] pub lhs_json: Option<String>,
    #[state(artifact)] pub rhs_json: Option<String>,
    #[state(artifact)] pub parameter_bindings: Option<BTreeMap<String, Option<PropertyValue>>>,
    #[state(artifact)] pub rule_layout: Option<BTreeMap<String, Option<LayoutPoint>>>,
    #[state(presence)] pub selected_node_ids: Option<RewriteStringList>,
    #[state(presence)] pub active_hover_var: Option<String>,
    #[state(presence)] pub active_select_var: Option<String>,
    #[state(presence)] pub lod_mode_by_window: Option<BTreeMap<String, Option<String>>>,
    #[state(config)] pub before_pane_camera: Option<Camera>,
    #[state(config)] pub reorganize_epoch: Option<u64>,
    #[state(config)] pub hover_epoch: Option<u64>,
    #[state(config)] pub select_epoch: Option<u64>,
    #[state(config)] pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct RewriteStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
