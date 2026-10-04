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
    pub working_graph: Option<semio_s_artifact_trinity_jack::JackSnapshot>,
    #[state(artifact)]
    pub lhs: Option<crate::standards::v1::subsets::any::schema::Lhs>,
    #[state(artifact)]
    pub rhs: Option<crate::standards::v1::subsets::any::schema::Rhs>,
    #[state(artifact)]
    pub parameter_bindings: Option<MapDelta<PropertyValue>>,
    #[state(artifact)]
    pub rule_layout: Option<MapDelta<LayoutPoint>>,
}
//#endregion 🔖️Diff

#[cfg(test)]
#[path = "🧪️tests/🗂️map-ownership/🦀️.rs"]
mod map_ownership_tests;

/// 📤️ Renders the sparse typed delta using its declared word and property roles.
pub fn encode_rewriting_diff_json(value:&RewritingDiff)->Result<String,semio_framework_value::ValueError>{
 let value=crate::standards::v1::subsets::any::schema::snapshot::json::diff(semio_framework_value::ToValue::to_value(value),false)?;
 Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&value)))
}
/// 📥️ Binds the sparse typed delta from its closed declared JSON fields.
pub fn decode_rewriting_diff_json(text:&str)->Result<RewritingDiff,semio_framework_value::ValueError>{
 let parsed=semio_framework_pack_json::parse(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string()))?;
 let value=crate::standards::v1::subsets::any::schema::snapshot::json::diff(semio_framework_pack_json::to_dsl_value(&parsed),true)?;
 semio_framework_value::FromValue::from_value(value)
}
