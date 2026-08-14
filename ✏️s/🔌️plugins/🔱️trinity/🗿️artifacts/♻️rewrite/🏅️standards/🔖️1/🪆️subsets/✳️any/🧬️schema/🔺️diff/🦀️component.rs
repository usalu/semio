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
    #[state(presence)] pub lod_mode_by_window: Option<BTreeMap<String, Option<String>>>,
    #[state(config)] pub before_pane_camera: Option<Camera>,
    #[state(config)] pub reorganize_epoch: Option<u64>,
    #[state(config)] pub locale: Option<String>,
}
//#endregion 🔖️Diff
