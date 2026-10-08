//! 💾️ Generic framing and descriptor roster for the transparent TxtMutation.
//#region 🔖️Registry
/// 📦 Encodes a recognized mutation payload or declines another variant.
pub type TxtMutationPayloadEncoder = fn(&TxtMutation) -> Option<Result<Vec<u8>, String>>;

use crate::schema::mutations::TxtMutation;
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
struct BinaryCodec {
    tag: u32,
    try_encode: TxtMutationPayloadEncoder,
    decode: fn(&[u8]) -> Result<TxtMutation, String>,
}
const BINARY_CODECS: &[BinaryCodec] = &[
    BinaryCodec { tag: set_trailing_newline::BINARY_TAG, try_encode: set_trailing_newline::try_encode, decode: set_trailing_newline::decode_mutation },
    BinaryCodec { tag: set_line_ending::BINARY_TAG, try_encode: set_line_ending::try_encode, decode: set_line_ending::decode_mutation },
    BinaryCodec { tag: insert_line::BINARY_TAG, try_encode: insert_line::try_encode, decode: insert_line::decode_mutation },
    BinaryCodec { tag: remove_line::BINARY_TAG, try_encode: remove_line::try_encode, decode: remove_line::decode_mutation },
    BinaryCodec { tag: set_line::BINARY_TAG, try_encode: set_line::try_encode, decode: set_line::decode_mutation },
];
pub const BINARY_TAGS: &[(&str, u32)] = &[
    (crate::standards::v_utf_8::subsets::any::io::text::mutations::set_trailing_newline::TEXT_OPCODE, set_trailing_newline::BINARY_TAG),
    (crate::standards::v_utf_8::subsets::any::io::text::mutations::set_line_ending::TEXT_OPCODE, set_line_ending::BINARY_TAG),
    (crate::standards::v_utf_8::subsets::any::io::text::mutations::insert_line::TEXT_OPCODE, insert_line::BINARY_TAG),
    (crate::standards::v_utf_8::subsets::any::io::text::mutations::remove_line::TEXT_OPCODE, remove_line::BINARY_TAG),
    (crate::standards::v_utf_8::subsets::any::io::text::mutations::set_line::TEXT_OPCODE, set_line::BINARY_TAG),
];
//#endregion 🔖️Registry

//#region 🔖️Framing
fn malformed(offset: usize, detail: impl Into<String>) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "txt mutation", offset: offset as u64, detail: detail.into() }
}
//#endregion 🔖️Framing

//#region ⚙️Codec
impl protocol::OpBinary for TxtMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let (tag, payload) = BINARY_CODECS.iter().find_map(|codec| (codec.try_encode)(self).map(|payload| (codec.tag, payload))).expect("txt mutation registry covers every variant");
        let mut frame = vec![tag as u8];
        frame.extend(payload.map_err(|cause| malformed(1, cause))?);
        Ok(frame)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let (tag, payload) = bytes.split_first().ok_or_else(|| malformed(0, "missing mutation tag"))?;
        let codec = BINARY_CODECS.iter().find(|codec| codec.tag == u32::from(*tag)).ok_or_else(|| malformed(0, "unknown txt mutation tag"))?;
        (codec.decode)(payload).map_err(|cause| malformed(1, cause))
    }
}
//#endregion ⚙️Codec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "📥️insert-line/🦀️.rs"]
pub mod insert_line;

#[path = "✏️set-line/🦀️.rs"]
pub mod set_line;

#[path = "🔚️set-line-ending/🦀️.rs"]
pub mod set_line_ending;

#[path = "🗑️remove-line/🦀️.rs"]
pub mod remove_line;

#[path = "↩️set-trailing-newline/🦀️.rs"]
pub mod set_trailing_newline;
