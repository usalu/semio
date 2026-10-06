//! 🚧 scaffolded by W1b — binary representation marker for `stdio.mp3.diff`. Full field-layout
//! parse/print lands in W2/W3.
pub const BINARY_MAGIC: &str = "stdio.mp3.diff";

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::mpeg1_layer3::subsets::any::schema::diff::*;
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Frame, Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3FrameHeader, Mp3Snapshot};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

impl protocol::DiffBinary for Mp3Diff {
/// ⚡️ Binary = the text bytes verbatim (same simplification `DeflateDiff`/`GifDiff`'s
/// hand-rolled `DiffCodec` impls use).
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
