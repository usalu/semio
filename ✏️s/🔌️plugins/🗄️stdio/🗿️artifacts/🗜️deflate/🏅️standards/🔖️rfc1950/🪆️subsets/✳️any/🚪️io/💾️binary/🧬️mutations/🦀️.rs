//! deflate rep for stdio.deflate 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_rfc1950::subsets::any::schema::mutations::*;
use crate::schema::diff::{diff_set_compression_params, diff_set_payload, diff_set_preset_dictionary, DeflateDiff};
use crate::schema::snapshot::DeflateLevelHint;
use crate::DeflateSnapshot;
use protocol::Mutation;
use protocol::{OpBinary, OpText};

/// ⚡️ Handcrafted `OpBinary` (P6) — pure forward to `dsl::variants_binary`.
impl OpBinary for DeflateMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(crate::standards::v_rfc1950::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO, self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(crate::standards::v_rfc1950::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO, bytes)
    }
}
}
pub use mutations_codec::*;
