//! 💾️ Framing and direct binary registry for PngMutation.
use crate::schema::mutations::PngMutation;

pub type PngMutationPayloadEncoder = fn(&PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>>;
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

pub struct Entry {
    pub tag: u8,
    pub encode: PngMutationPayloadEncoder,
    pub decode: fn(&[u8]) -> Result<PngMutation, protocol::ProtocolError>,
}

pub const REGISTRY: &[Entry] = &[
    crate::schema::mutations::set_snapshot::binary::CODEC,
    crate::schema::mutations::patch_snapshot::binary::CODEC,
    crate::schema::mutations::change_gamma::binary::CODEC,
    crate::schema::mutations::patch_pixels::binary::CODEC,
    crate::schema::mutations::paint_native_samples::binary::CODEC,
];

impl protocol::OpBinary for PngMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let (tag, payload) = REGISTRY.iter().find_map(|entry| (entry.encode)(self).map(|payload| (entry.tag, payload))).expect("every PNG mutation has a direct binary owner");
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
