//! 📝️ Physical text diff representation.

/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");

pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type RewritingDiffText = String;

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::diff::*;
use semio_framework_graph::manifest::PropertyValue;
use crate::LayoutPoint;
use ::semio_framework_schema::ArtifactSchema;
use replication::MapDelta;

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
}

pub use diff_codec::*;

semio_framework_os_kernel::diff_text!(crate::standards::v1::subsets::any::schema::diff::RewritingDiff);
