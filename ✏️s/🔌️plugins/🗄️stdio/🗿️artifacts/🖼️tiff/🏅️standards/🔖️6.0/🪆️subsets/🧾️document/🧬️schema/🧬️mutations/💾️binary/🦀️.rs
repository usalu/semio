//! 💾️ Framing and direct binary registry for TiffMutation.
use crate::artifacts::tiff::schema::mutations::TiffMutation;

//#region Registry
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
pub struct Entry {
    pub tag: u8,
    pub encode: fn(&TiffMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>>,
    pub decode: fn(&[u8]) -> Result<TiffMutation, protocol::ProtocolError>,
}
pub const REGISTRY: &[Entry] = &[
    crate::artifacts::tiff::schema::mutations::change_byte_order::binary::CODEC,
    crate::artifacts::tiff::schema::mutations::insert_ifd::binary::CODEC,
    crate::artifacts::tiff::schema::mutations::remove_ifd::binary::CODEC,
    crate::artifacts::tiff::schema::mutations::replace_tag::binary::CODEC,
    crate::artifacts::tiff::schema::mutations::remove_tag::binary::CODEC,
    crate::artifacts::tiff::schema::mutations::replace_pixels::binary::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpBinary for TiffMutation {
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
