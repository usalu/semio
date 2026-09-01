//! 💾️ Framing and direct binary registry for JpgMutation.
use crate::artifacts::jpg::schema::mutations::JpgMutation;

//#region Registry
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
pub struct Entry {
    pub tag: u8,
    pub encode: fn(&JpgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>>,
    pub decode: fn(&[u8]) -> Result<JpgMutation, protocol::ProtocolError>,
}
pub const REGISTRY: &[Entry] = &[
    crate::artifacts::jpg::schema::mutations::change_jfif_header::binary::CODEC,
    crate::artifacts::jpg::schema::mutations::replace_quant_table::binary::CODEC,
    crate::artifacts::jpg::schema::mutations::remove_quant_table::binary::CODEC,
    crate::artifacts::jpg::schema::mutations::replace_huffman_table::binary::CODEC,
    crate::artifacts::jpg::schema::mutations::remove_huffman_table::binary::CODEC,
    crate::artifacts::jpg::schema::mutations::change_restart_interval::binary::CODEC,
    crate::artifacts::jpg::schema::mutations::insert_other_segment::binary::CODEC,
    crate::artifacts::jpg::schema::mutations::remove_other_segment::binary::CODEC,
    crate::artifacts::jpg::schema::mutations::replace_pixels::binary::CODEC,
    crate::artifacts::jpg::schema::mutations::change_re_encode_quality::binary::CODEC,
];
//#endregion Registry

//#region Framing
impl protocol::OpBinary for JpgMutation {
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
