//! gif rep for stdio.gif 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v89a::subsets::any::schema::mutations::*;
use crate::standards::v89a::subsets::any::schema::diff::{self, GifAppExtensionAdded, GifAppExtensionsDiff, GifCommentAdded, GifCommentsDiff, GifDiff, GifFrameAdded, GifFrameDiff, GifFrameModified, GifFramesDiff};
use crate::standards::v89a::subsets::any::schema::snapshot::{GifAppExtension, GifColorTable, GifDisposal, GifFrame, GifSnapshot};
use protocol::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};

/// ⚡️ Handcrafted `OpBinary` (P6) — pure forward to `dsl::variants_binary`.
impl OpBinary for GifMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(crate::standards::v89a::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO, self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(crate::standards::v89a::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO, bytes)
    }
}
}
pub use mutations_codec::*;
