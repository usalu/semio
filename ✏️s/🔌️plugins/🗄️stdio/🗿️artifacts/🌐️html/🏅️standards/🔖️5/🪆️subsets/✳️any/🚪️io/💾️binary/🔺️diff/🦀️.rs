//! binary rep for stdio.html 🔺️diff -- see the sibling `encode_diff`/`decode_diff` two levels up.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v5::subsets::any::schema::diff::*;
use crate::standards::v5::subsets::any::schema::snapshot::{HtmlAttr, HtmlNode, HtmlSnapshot, RawTextKind};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

impl protocol::DiffBinary for HtmlDiff {
/// ⚡️ Binary = the text bytes verbatim, same simplification `SvgDiff`/`JsonDiff` (and the
/// repo's only other hand-rolled `DiffCodec`s) use — satisfies every `DiffBinary,DiffCodec,DiffText` law without
/// inventing a second wire format.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    Ok(self.print_diff().into_bytes())
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: e.to_string() })?;
    Self::parse_diff(line).map_err(|e| protocol::ProtocolError::Malformed { what: "diff text", offset: 0, detail: e.to_string() })
}
}
}
pub use diff_codec::*;
