//! 🧬️ Rewriting diff schema — sparse field delta over the artifact.

use semio_framework_graph::manifest::PropertyValue;

use crate::LayoutPoint;
use ::semio_framework_schema::ArtifactSchema;
use replication::MapDelta;

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
    pub parameter_bindings: Option<MapDelta<PropertyValue>>,
    #[state(artifact)]
    pub rule_layout: Option<MapDelta<LayoutPoint>>,
}
//#endregion 🔖️Diff

#[cfg(test)]
#[path = "🧪️tests/🗂️map-ownership/🦀️.rs"]
mod map_ownership_tests;
