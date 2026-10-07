//! 💾️ Framing and direct binary registry for BmpMutation.
/// 📦 Encodes a recognized mutation payload or declines another variant.
pub type BmpMutationPayloadEncoder = fn(&BmpMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>>;

use crate::standards::v_v3::subsets::any::schema::mutations::BmpMutation;

//#region Registry
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
pub struct Entry {
    pub tag: u8,
    pub encode: BmpMutationPayloadEncoder,
    pub decode: fn(&[u8]) -> Result<BmpMutation, protocol::ProtocolError>,
}
pub const REGISTRY: &[Entry] = &[crate::standards::v_v3::subsets::any::io::binary::mutations::set_snapshot::CODEC, crate::standards::v_v3::subsets::any::io::binary::mutations::patch_snapshot::CODEC, crate::standards::v_v3::subsets::any::io::binary::mutations::paint_indexed_region::CODEC, crate::standards::v_v3::subsets::any::io::binary::mutations::paint_direct_region::CODEC];
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

#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;

#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;

#[path = "🖌️paint-direct-region/🦀️.rs"]
pub mod paint_direct_region;

#[path = "🎨️paint-indexed-region/🦀️.rs"]
pub mod paint_indexed_region;
