//! 💾️ PDF 1.4 mutation binary framing and executable direct-leaf registry.

use crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation;
use protocol::OpBinary;

//#region 🔖️Protocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 🔖️Protocol

//#region 🔖️Registry
type Encoder = fn(&PdfMutation) -> Option<Result<Vec<u8>, String>>;
type Decoder = fn(&[u8]) -> Result<PdfMutation, String>;
pub const REGISTRY: &[(u8, Encoder, Decoder)] = &[
    (insert_page::TAG, insert_page::encode, insert_page::decode),
    (remove_page::TAG, remove_page::encode, remove_page::decode),
    (move_page::TAG, move_page::encode, move_page::decode),
    (resize_page::TAG, resize_page::encode, resize_page::decode),
    (replace_page_text::TAG, replace_page_text::encode, replace_page_text::decode),
];
//#endregion 🔖️Registry

pub use semio_s_artifact_stdio_pdf_snapshot::standards::v1_4::subsets::base::io::binary::mutations::wire;

//#region 🔖️Framing
fn malformed(detail: String) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "PDF 1.4 mutation", offset: 0, detail }
}

impl OpBinary for PdfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let (tag, payload) = REGISTRY.iter().find_map(|(tag, encode, _)| encode(self).map(|result| (*tag, result))).ok_or_else(|| malformed("Missing mutation encoder".into()))?;
        let mut bytes = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        bytes.extend(payload.map_err(malformed)?);
        Ok(bytes)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        if bytes.first() != Some(&store::pack_rt::OP_BINARY_FORMAT) {
            return Err(malformed("Unknown or missing binary format".into()));
        }
        let tag = *bytes.get(1).ok_or_else(|| malformed("Missing mutation tag".into()))?;
        let (_, _, decode) = REGISTRY.iter().find(|(identity, _, _)| *identity == tag).ok_or_else(|| malformed("Unknown mutation tag".into()))?;
        decode(&bytes[2..]).map_err(malformed)
    }
}
//#endregion 🔖️Framing

#[path="🫳️borrowed/🦀️.rs"]
mod borrowed_operation_source;

#[path = "🔀️move-page/🦀️.rs"]
pub mod move_page;

#[path = "🗑️remove-page/🦀️.rs"]
pub mod remove_page;

#[path = "♻️replace-page-text/🦀️.rs"]
pub mod replace_page_text;

#[path = "📥️insert-page/🦀️.rs"]
pub mod insert_page;

#[path = "📐️resize-page/🦀️.rs"]
pub mod resize_page;
