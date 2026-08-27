//! 💾️ PDF 1.4 mutation binary framing and executable direct-leaf registry.

use super::PdfMutation;
use protocol::OpBinary;

//#region 🔖️Protocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 🔖️Protocol

//#region 🔖️Registry
type Encoder = fn(&PdfMutation) -> Option<Result<Vec<u8>, String>>;
type Decoder = fn(&[u8]) -> Result<PdfMutation, String>;
pub const REGISTRY: &[(u8, Encoder, Decoder)] = &[
    (super::insert_page::binary::TAG, super::insert_page::binary::encode, super::insert_page::binary::decode),
    (super::remove_page::binary::TAG, super::remove_page::binary::encode, super::remove_page::binary::decode),
    (super::move_page::binary::TAG, super::move_page::binary::encode, super::move_page::binary::decode),
    (super::resize_page::binary::TAG, super::resize_page::binary::encode, super::resize_page::binary::decode),
    (super::replace_page_text::binary::TAG, super::replace_page_text::binary::encode, super::replace_page_text::binary::decode),
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
