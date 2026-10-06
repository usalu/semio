//! 📝️ Canonical JSON operation grammar for `WavMutation::print_op` and `parse_op`.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::riff_pcm::subsets::any::schema::mutations::*;
use crate::standards::riff_pcm::subsets::any::schema::diff::{diff_set_data, diff_set_fmt, diff_set_other_chunks, diff_set_snapshot, WavDiff};
use crate::standards::riff_pcm::subsets::any::schema::snapshot::{validate_wav_serialization, RiffChunk, WavData, WavFmt, WavSnapshot};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

/// 🎙️ Handcrafted `OpText`/`OpBinary` via `pack::json` (one line of compact JSON per op) —
/// deliberately NOT `#[derive(dsl::DslOps)]`: `WavData` is a data-carrying enum embedded in
/// `SetData`'s payload, the same shape `f6-final-summary.md` §4.4 documents as structurally
/// unbindable by the derive machinery today (no generic/enum-payload `DslField` bridge). This is
/// a SEPARATE wire format from the subset's own `ArtifactDsl`/`ArtifactPack` envelope (which
/// wraps real RIFF/WAVE bytes, see that file's doc comment) — an op is always plain JSON here.
impl OpText for WavMutation {
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
