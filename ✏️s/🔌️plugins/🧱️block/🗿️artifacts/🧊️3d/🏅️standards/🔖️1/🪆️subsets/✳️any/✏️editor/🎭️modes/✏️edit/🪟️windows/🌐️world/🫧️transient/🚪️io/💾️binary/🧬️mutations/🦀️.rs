//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use crate::edit::windows::world::transient::io::binary::mutations::*;
use crate::edit::windows::world::transient::mutations::*;
use crate::edit::windows::world::transient::Block3dWorldWindowTransient;
use crate::edit::windows::world::transient::mutations::set_brush_preview::SetBrushPreview;

impl protocol::OpBinary for Block3dWorldWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
