//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::editor::block3d::modes::edit::windows::world::transient::component::mutations::*;
use crate::editor::block3d::modes::edit::windows::world::transient::component::Block3dWorldWindowTransient;
use crate::editor::block3d::modes::edit::windows::world::transient::component::mutations::SetBrushPreview;

impl protocol::OpBinary for Block3dWorldWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
pub use mutations_codec::*;
