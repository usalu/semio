//! 💾️ Binary representation codec surface for `stdio.mp4` (mutations) — the real op binary
//! codec is `protocol::OpBinary` in ../🦀️.rs (`encode_op`/`decode_op`, shared tagged records).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::isobmff::subsets::any::schema::mutations::*;
use crate::standards::isobmff::subsets::any::schema::diff::{IndexedAdded, IndexedDiff, IndexedModified, Mp4Diff, Mp4SampleDiff, Mp4TrackDiff};
use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Snapshot, Mp4Track};
#[cfg(test)]
use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Movie, Mp4TrackMetadata};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

/// ⚡️ Structured operation binary through the shared tagged-record protocol.
impl OpBinary for Mp4Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(crate::standards::isobmff::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO, self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(crate::standards::isobmff::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO, bytes)
    }
}
}
pub use mutations_codec::*;
