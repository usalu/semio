//! zip rep for stdio.zip 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v2_0::subsets::base::schema::mutations::*;
use crate::schema::diff::{self, ZipDiff};
use crate::schema::snapshot::ZipEntry;
use crate::ZipSnapshot;

impl protocol::OpBinary for ZipMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(crate::standards::v2_0::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO, self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(crate::standards::v2_0::subsets::base::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO, bytes)
    }
}
}
pub use mutations_codec::*;
