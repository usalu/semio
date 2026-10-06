//! 📝️ Text representation codec surface for `stdio.avi` (mutations) — the real op text
//! codec is `protocol::OpText` in ../🦀️.rs.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1_0::subsets::any::schema::mutations::*;
use crate::standards::v1_0::subsets::any::schema::diff::{AviChunkDiff, AviDiff, AviStreamDiff, IndexedAdded, IndexedDiff, IndexedModified};
use crate::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader, RiffChunk};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

/// 🎙️ Handcrafted `OpText`/`OpBinary` — plain `pack::json` round-trip (see mp4's identical
/// module-doc rationale: f6-final-summary.md §4.4, no generic collection-diff `DslField` bridge).
impl OpText for AviMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let parsed = semio_framework_pack_json::parse(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| semio_framework_diagnostic::TextError::from_value_error(e.into_value_error(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|e| semio_framework_diagnostic::TextError::from_value_error(e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(self)))
    }
}
}
pub use mutations_codec::*;
