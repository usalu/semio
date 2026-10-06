//! 💾️ Binary representation codec surface for `stdio.avi` (mutations) — the real op binary
//! codec is `protocol::OpBinary` in ../🦀️.rs.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1_0::subsets::any::schema::mutations::*;
use crate::standards::v1_0::subsets::any::schema::diff::{AviChunkDiff, AviDiff, AviStreamDiff, IndexedAdded, IndexedDiff, IndexedModified};
use crate::standards::v1_0::subsets::any::schema::snapshot::{AviChunk, AviMainHeader, AviSnapshot, AviStream, AviStreamFormat, AviStreamHeader, RiffChunk};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

impl OpBinary for AviMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::tagged_value_binary::encode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Field("mutation"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::tagged_value_binary::decode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Field("mutation"), bytes)
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ `AviMutation`'s wire protocol: its `record <kind> tag=<n>` lines are the only source of the op tags.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
//#endregion 🏷️WireTags
