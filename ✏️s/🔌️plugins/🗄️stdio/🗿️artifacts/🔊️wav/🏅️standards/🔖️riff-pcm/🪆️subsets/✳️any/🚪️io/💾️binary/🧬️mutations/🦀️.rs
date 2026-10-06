//! 🚧 scaffolded by W1b — binary representation marker for `stdio.wav.mutations`. Full field-layout
//! parse/print lands in W2/W3.
pub const BINARY_MAGIC: &str = "stdio.wav.mutations";

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::riff_pcm::subsets::any::schema::mutations::*;
use crate::standards::riff_pcm::subsets::any::schema::diff::{diff_set_data, diff_set_fmt, diff_set_other_chunks, diff_set_snapshot, WavDiff};
use crate::standards::riff_pcm::subsets::any::schema::snapshot::{validate_wav_serialization, RiffChunk, WavData, WavFmt, WavSnapshot};
use protocol::Mutation;
use protocol::{OpBinary, OpText};

impl OpBinary for WavMutation {
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
/// 🏷️ `WavMutation`'s wire protocol: its `record <kind> tag=<n>` lines are the only source of the op tags.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
//#endregion 🏷️WireTags
