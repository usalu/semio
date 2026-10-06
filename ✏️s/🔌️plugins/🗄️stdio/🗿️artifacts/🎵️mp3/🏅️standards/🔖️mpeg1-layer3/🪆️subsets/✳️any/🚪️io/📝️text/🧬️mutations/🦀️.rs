//! 🚧 scaffolded by W1b — text representation marker for `stdio.mp3.mutations`. Full grammar-backed
//! parse/print lands in W2/W3.
pub const TEXT_MARKER: &str = "stdio.mp3.mutations";

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::mpeg1_layer3::subsets::any::schema::mutations::*;
use crate::standards::mpeg1_layer3::subsets::any::schema::diff::{diff_set_frames, diff_set_id3v1, diff_set_id3v2, diff_set_snapshot, Mp3Diff};
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3Snapshot};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

/// 🎙️ Handcrafted `OpText`/`OpBinary` via `pack::json` (one line of compact JSON per op) —
/// deliberately NOT `#[derive(dsl::DslOps)]`: `Mp3Frame`/`Id3v2Tag` embed nested collections of
/// named structs, the same generic-collection-diff shape `f6-final-summary.md` §4.4 documents as
/// needing a hand-rolled bridge. This is a SEPARATE wire format from the subset's own
/// `ArtifactDsl`/`ArtifactPack` envelope (which wraps real MP3 bytes, see that file's doc
/// comment) — an op is always plain JSON here.
impl OpText for Mp3Mutation {
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
