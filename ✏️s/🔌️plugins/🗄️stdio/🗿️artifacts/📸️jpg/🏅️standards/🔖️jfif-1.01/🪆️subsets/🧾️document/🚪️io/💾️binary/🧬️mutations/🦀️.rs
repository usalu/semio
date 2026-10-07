//! 💾️ Framing and direct binary registry for JpgMutation.
/// 📦 Encodes a recognized mutation payload or declines another variant.
pub type JpgMutationPayloadEncoder = fn(&JpgMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>>;

use crate::schema::mutations::JpgMutation;

//#region Registry
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
pub struct Entry {
    pub tag: u8,
    pub encode: JpgMutationPayloadEncoder,
    pub decode: fn(&[u8]) -> Result<JpgMutation, protocol::ProtocolError>,
}
pub const REGISTRY: &[Entry] = &[
    crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::patch_snapshot::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::set_snapshot::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::change_jfif_header::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::replace_quant_table::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::remove_quant_table::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::replace_huffman_table::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::remove_huffman_table::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::change_restart_interval::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::insert_other_segment::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::remove_other_segment::CODEC,
    crate::standards::v_jfif_1_01::subsets::document::io::binary::mutations::replace_pixels::CODEC,
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

#[path = "📊️replace-quant/🦀️.rs"]
pub mod replace_quant_table;

#[path = "🪪️change-jfif/🦀️.rs"]
pub mod change_jfif_header;

#[path = "🧹️remove-quant/🦀️.rs"]
pub mod remove_quant_table;

#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;

#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;

#[path = "🌳️replace-huffman/🦀️.rs"]
pub mod replace_huffman_table;

#[path = "🗑️remove-other/🦀️.rs"]
pub mod remove_other_segment;

#[path = "📥️insert-other/🦀️.rs"]
pub mod insert_other_segment;


#[path = "🔲️replace-pixels/🦀️.rs"]
pub mod replace_pixels;

#[path = "🔁️change-restart/🦀️.rs"]
pub mod change_restart_interval;

#[path = "🪓️remove-huffman/🦀️.rs"]
pub mod remove_huffman_table;
