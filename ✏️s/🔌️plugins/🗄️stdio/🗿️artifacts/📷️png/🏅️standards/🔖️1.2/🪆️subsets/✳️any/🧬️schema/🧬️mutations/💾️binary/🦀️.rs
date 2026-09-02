//! 💾️ Framing and direct binary registry for PngMutation.
use crate::artifacts::png::schema::mutations::PngMutation;

//#region Registry
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
pub struct Entry {
    pub tag: u8,
    pub encode: fn(&PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>>,
    pub decode: fn(&[u8]) -> Result<PngMutation, protocol::ProtocolError>,
}
pub const REGISTRY: &[Entry] = &[
    crate::artifacts::png::schema::mutations::change_header::binary::CODEC,
    crate::artifacts::png::schema::mutations::replace_palette::binary::CODEC,
    crate::artifacts::png::schema::mutations::change_transparency::binary::CODEC,
    crate::artifacts::png::schema::mutations::change_gamma::binary::CODEC,
    crate::artifacts::png::schema::mutations::change_chromaticities::binary::CODEC,
    crate::artifacts::png::schema::mutations::change_srgb_intent::binary::CODEC,
    crate::artifacts::png::schema::mutations::change_physical_dims::binary::CODEC,
    crate::artifacts::png::schema::mutations::change_timestamp::binary::CODEC,
    crate::artifacts::png::schema::mutations::change_background::binary::CODEC,
    crate::artifacts::png::schema::mutations::insert_text_chunk::binary::CODEC,
    crate::artifacts::png::schema::mutations::remove_text_chunk::binary::CODEC,
    crate::artifacts::png::schema::mutations::replace_text_chunk::binary::CODEC,
    crate::artifacts::png::schema::mutations::replace_pixels::binary::CODEC,
    crate::artifacts::png::schema::mutations::insert_unknown_chunk::binary::CODEC,
    crate::artifacts::png::schema::mutations::remove_unknown_chunk::binary::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpBinary for PngMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let (tag, payload) = REGISTRY.iter().find_map(|entry| (entry.encode)(self).map(|result| (entry.tag, result))).expect("every aggregate variant has a direct binary owner");
        let mut result = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        result.extend(payload?);
        Ok(result)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        if bytes.len() < 2 || bytes[0] != store::pack_rt::OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "mutation frame", offset: 0, detail: "expected format byte and direct tag".into() });
        }
        let entry = REGISTRY.iter().find(|entry| entry.tag == bytes[1]).ok_or_else(|| protocol::ProtocolError::Malformed { what: "mutation tag", offset: 1, detail: format!("unknown tag {}", bytes[1]) })?;
        (entry.decode)(&bytes[2..])
    }
}
//#endregion Framing
