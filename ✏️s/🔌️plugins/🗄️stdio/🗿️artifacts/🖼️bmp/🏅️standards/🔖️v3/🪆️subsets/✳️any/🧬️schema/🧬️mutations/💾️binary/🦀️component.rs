//! 💾️ Framing and direct binary registry for BmpMutation.
use crate::artifacts::bmp::schema::mutations::BmpMutation;

//#region Registry
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
pub struct Entry {
    pub tag: u8,
    pub encode: fn(&BmpMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>>,
    pub decode: fn(&[u8]) -> Result<BmpMutation, protocol::ProtocolError>,
}
pub const REGISTRY: &[Entry] = &[
    crate::artifacts::bmp::schema::mutations::change_header_fields::binary::CODEC,
    crate::artifacts::bmp::schema::mutations::insert_palette_entry::binary::CODEC,
    crate::artifacts::bmp::schema::mutations::remove_palette_entry::binary::CODEC,
    crate::artifacts::bmp::schema::mutations::replace_palette_entry::binary::CODEC,
    crate::artifacts::bmp::schema::mutations::replace_pixel_data::binary::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpBinary for BmpMutation {
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
