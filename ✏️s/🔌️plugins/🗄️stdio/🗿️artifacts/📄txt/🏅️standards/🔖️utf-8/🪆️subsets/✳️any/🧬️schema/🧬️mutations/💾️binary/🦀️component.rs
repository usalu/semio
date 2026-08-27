//! 💾️ Generic framing and descriptor roster for the transparent TxtMutation.
//#region 🔖️Registry
use crate::artifacts::txt::schema::mutations::{TxtMutation, insert_line, remove_line, set_line, set_line_ending, set_trailing_newline};
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
struct BinaryCodec {
    tag: u32,
    try_encode: fn(&TxtMutation) -> Option<Result<Vec<u8>, String>>,
    decode: fn(&[u8]) -> Result<TxtMutation, String>,
}
const BINARY_CODECS: &[BinaryCodec] = &[
    BinaryCodec { tag: set_trailing_newline::binary::BINARY_TAG, try_encode: set_trailing_newline::binary::try_encode, decode: set_trailing_newline::binary::decode_mutation },
    BinaryCodec { tag: set_line_ending::binary::BINARY_TAG, try_encode: set_line_ending::binary::try_encode, decode: set_line_ending::binary::decode_mutation },
    BinaryCodec { tag: insert_line::binary::BINARY_TAG, try_encode: insert_line::binary::try_encode, decode: insert_line::binary::decode_mutation },
    BinaryCodec { tag: remove_line::binary::BINARY_TAG, try_encode: remove_line::binary::try_encode, decode: remove_line::binary::decode_mutation },
    BinaryCodec { tag: set_line::binary::BINARY_TAG, try_encode: set_line::binary::try_encode, decode: set_line::binary::decode_mutation },
];
pub const BINARY_TAGS: &[(&str, u32)] = &[
    (set_trailing_newline::text::TEXT_OPCODE, set_trailing_newline::binary::BINARY_TAG),
    (set_line_ending::text::TEXT_OPCODE, set_line_ending::binary::BINARY_TAG),
    (insert_line::text::TEXT_OPCODE, insert_line::binary::BINARY_TAG),
    (remove_line::text::TEXT_OPCODE, remove_line::binary::BINARY_TAG),
    (set_line::text::TEXT_OPCODE, set_line::binary::BINARY_TAG),
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
mod tests {
    use super::*;
    use protocol::OpBinary;

    #[test]
    fn generic_framing_refuses_missing_or_unknown_tags() {
        for frame in [Vec::new(), vec![255]] {
            assert!(TxtMutation::decode_op(&frame).is_err(), "{frame:?}");
        }
    }
}
//#endregion 🧪️Tests
