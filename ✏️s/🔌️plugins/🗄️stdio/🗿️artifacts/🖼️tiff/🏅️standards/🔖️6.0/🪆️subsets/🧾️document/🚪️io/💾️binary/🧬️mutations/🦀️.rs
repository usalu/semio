//! 💾️ Framing and direct binary registry for TiffMutation.
use crate::schema::mutations::TiffMutation;

#[path = "🧾️entry/🦀️.rs"]
pub mod entry;
pub use entry::*;

//#region Registry
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
pub const REGISTRY: &[Entry] = &[
    insert_ifd::CODEC,
    remove_ifd::CODEC,
    replace_tag::CODEC,
    remove_tag::CODEC,
    paint_region::CODEC,
    replace_samples::CODEC,
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

#[path = "🧮️replace-samples/🦀️.rs"]
pub mod replace_samples;
