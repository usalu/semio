//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::procedure::config::component::mutations::*;
use crate::editor::procedure::config::component::*;
use crate::editor::procedure::config::component::mutations::ReplaceConfig;
use crate::editor::procedure::config::component::mutations::SetRunOutput;
use crate::editor::procedure::config::component::mutations::SetContributions;

impl protocol::OpBinary for ImperativeConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
pub use mutations_codec::*;
