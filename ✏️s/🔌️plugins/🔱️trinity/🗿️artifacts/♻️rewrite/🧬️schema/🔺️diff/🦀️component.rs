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
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::rewrite::schema::RewriteArtifact>>,
    #[state(persistent)] pub before_fixture_json: Option<String>,
    #[state(persistent)] pub lhs_json: Option<String>,
    #[state(persistent)] pub rhs_json: Option<String>,
    #[state(persistent)] pub parameter_bindings: Option<BTreeMap<String, Option<PropertyValue>>>,
    #[state(persistent)] pub rule_layout: Option<BTreeMap<String, Option<LayoutPoint>>>,
    #[state(shared_ui)] pub selected_node_ids: Option<RewriteStringList>,
    #[state(shared_ui)] pub active_hover_var: Option<String>,
    #[state(shared_ui)] pub active_select_var: Option<String>,
    #[state(shared_ui)] pub lod_mode_by_window: Option<BTreeMap<String, Option<String>>>,
    #[state(local_ui)] pub before_pane_camera: Option<Camera>,
    #[state(local_ui)] pub reorganize_epoch: Option<u64>,
    #[state(local_ui)] pub hover_epoch: Option<u64>,
    #[state(local_ui)] pub select_epoch: Option<u64>,
    #[state(local_ui)] pub locale: Option<String>,
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
