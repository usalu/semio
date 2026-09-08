//! 🧬️ Rewriting diff schema — sparse field delta over the artifact.

use semio_s_artifact_trinity_jack::{Camera, PropertyValue};
use crate::LayoutPoint;
use ::semio_framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the rewriting artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.trinity.rewriting")]
pub struct RewritingDiff {
    #[state(artifact)]
    pub before_fixture_json: Option<String>,
    #[state(artifact)]
    pub lhs_json: Option<String>,
    #[state(artifact)]
    pub rhs_json: Option<String>,
    #[state(artifact)]
    pub parameter_bindings: Option<BTreeMap<String, Option<PropertyValue>>>,
    #[state(artifact)]
    pub rule_layout: Option<BTreeMap<String, Option<LayoutPoint>>>,
    #[state(config)]
    pub lod_mode_by_window: Option<BTreeMap<String, Option<String>>>,
    #[state(config)]
    pub before_pane_camera: Option<Camera>,
}
//#endregion 🔖️Diff
