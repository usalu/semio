//! 🚧 scaffolded by W1b — binary representation marker for `stdio.mp3.mutations`. Full field-layout
//! parse/print lands in W2/W3.
pub const BINARY_MAGIC: &str = "stdio.mp3.mutations";

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::mpeg1_layer3::subsets::any::schema::mutations::*;
use crate::standards::mpeg1_layer3::subsets::any::schema::diff::{diff_set_frames, diff_set_id3v1, diff_set_id3v2, diff_set_snapshot, Mp3Diff};
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3Snapshot};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

impl OpBinary for Mp3Mutation {
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
/// 🏷️ `Mp3Mutation`'s wire protocol: its `record <kind> tag=<n>` lines are the only source of the op tags.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
//#endregion 🏷️WireTags
