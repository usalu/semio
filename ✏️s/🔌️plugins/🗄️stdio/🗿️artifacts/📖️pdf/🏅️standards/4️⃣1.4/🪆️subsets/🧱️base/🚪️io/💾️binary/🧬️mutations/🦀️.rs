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
    (crate::standards::v1_4::subsets::base::schema::mutations::insert_page::TAG, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::encode, crate::standards::v1_4::subsets::base::schema::mutations::insert_page::decode),
    (crate::standards::v1_4::subsets::base::schema::mutations::remove_page::TAG, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::encode, crate::standards::v1_4::subsets::base::schema::mutations::remove_page::decode),
    (crate::standards::v1_4::subsets::base::schema::mutations::move_page::TAG, crate::standards::v1_4::subsets::base::schema::mutations::move_page::encode, crate::standards::v1_4::subsets::base::schema::mutations::move_page::decode),
    (crate::standards::v1_4::subsets::base::schema::mutations::resize_page::TAG, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::encode, crate::standards::v1_4::subsets::base::schema::mutations::resize_page::decode),
    (crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::TAG, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::encode, crate::standards::v1_4::subsets::base::schema::mutations::replace_page_text::decode),
    (crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::TAG, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::encode, crate::standards::v1_4::subsets::base::schema::mutations::set_snapshot::decode),
    (crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::TAG, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::encode, crate::standards::v1_4::subsets::base::schema::mutations::patch_snapshot::decode),
];
//#endregion 🔖️Registry

//#region 🔖️Primitives
pub(super) fn put_index(value: usize, out: &mut Vec<u8>) -> Result<(), String> {
    out.extend_from_slice(&u64::try_from(value).map_err(|error| error.to_string())?.to_le_bytes());
    Ok(())
}

pub(super) fn put_text(value: &str, out: &mut Vec<u8>) -> Result<(), String> {
    put_index(value.len(), out)?;
    out.extend_from_slice(value.as_bytes());
    Ok(())
}

pub(super) struct Reader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Reader<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }
    fn take(&mut self, count: usize) -> Result<&'a [u8], String> {
        let end = self.position.checked_add(count).ok_or("Payload length overflow")?;
        let result = self.bytes.get(self.position..end).ok_or("Truncated mutation payload")?;
        self.position = end;
        Ok(result)
    }
    pub(super) fn index(&mut self) -> Result<usize, String> {
        usize::try_from(u64::from_le_bytes(self.take(8)?.try_into().unwrap())).map_err(|error| error.to_string())
    }
    pub(super) fn number(&mut self) -> Result<f64, String> {
        let value = f64::from_le_bytes(self.take(8)?.try_into().unwrap());
        if !value.is_finite() {
            return Err("Non-finite geometry".into());
        }
        Ok(value)
    }
    pub(super) fn text(&mut self) -> Result<String, String> {
        let length = self.index()?;
        String::from_utf8(self.take(length)?.to_vec()).map_err(|error| error.to_string())
    }
    pub(super) fn finish(self) -> Result<(), String> {
        if self.position == self.bytes.len() {
            Ok(())
        } else {
            Err("Trailing mutation payload bytes".into())
        }
    }
}
//#endregion 🔖️Primitives

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

#[path="../../../🧬️schema/🧬️mutations/📦️codec/🫳️borrowed/🦀️.rs"]
mod borrowed_operation_source;

#[path = "🔀️move-page/🦀️.rs"]
pub mod move_page;

#[path = "🗑️remove-page/🦀️.rs"]
pub mod remove_page;

#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;

#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;

#[path = "♻️replace-page-text/🦀️.rs"]
pub mod replace_page_text;

#[path = "📥️insert-page/🦀️.rs"]
pub mod insert_page;

#[path = "📐️resize-page/🦀️.rs"]
pub mod resize_page;
