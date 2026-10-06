//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::jack::transient::component::results_window::mutations::*;
use crate::editor::jack::transient::component::results_window::JackResultsWindowTransient;
use replace_query_result::ReplaceQueryResult;

impl protocol::OpBinary for JackResultsWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}
}
pub use mutations_codec::*;
