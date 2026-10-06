//! 💾️ Framing and direct binary registry for TiffMutation.
/// 📦 Encodes a recognized mutation payload or declines another variant.
pub type TiffMutationPayloadEncoder = fn(&TiffMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>>;

use crate::schema::mutations::TiffMutation;

//#region Registry
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
pub struct Entry {
    pub tag: u8,
    pub encode: TiffMutationPayloadEncoder,
    pub decode: fn(&[u8]) -> Result<TiffMutation, protocol::ProtocolError>,
}
pub const REGISTRY: &[Entry] = &[
    crate::standards::v6_0::subsets::document::schema::mutations::patch_snapshot::CODEC,
    crate::standards::v6_0::subsets::document::schema::mutations::set_snapshot::CODEC,
    crate::standards::v6_0::subsets::document::schema::mutations::change_byte_order::CODEC,
    crate::standards::v6_0::subsets::document::schema::mutations::insert_ifd::CODEC,
    crate::standards::v6_0::subsets::document::schema::mutations::remove_ifd::CODEC,
    crate::standards::v6_0::subsets::document::schema::mutations::replace_tag::CODEC,
    crate::standards::v6_0::subsets::document::schema::mutations::remove_tag::CODEC,
    crate::standards::v6_0::subsets::document::schema::mutations::paint_region::CODEC,
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

#[path = "🧭️change-byte-order/🦀️.rs"]
pub mod change_byte_order;

#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;

#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;

#[path = "🗑️remove-tag/🦀️.rs"]
pub mod remove_tag;

#[path = "📥️insert-ifd/🦀️.rs"]
pub mod insert_ifd;

#[path = "🏷️replace-tag/🦀️.rs"]
pub mod replace_tag;

#[path = "📤️remove-ifd/🦀️.rs"]
pub mod remove_ifd;

#[path = "🎨️paint-region/🦀️.rs"]
pub mod paint_region;
