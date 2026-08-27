//! 💾️ Direct remove-palette-entry binary codec.
use super::*;
use crate::artifacts::bmp::schema::diff::{self, *};
use crate::artifacts::bmp::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 4;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &BmpMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let BmpMutation::RemovePaletteEntry(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &RemovePaletteEntryMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    store::pack_rt::encode_record_body(&super::text::spec(), &dsl::__rt::newtype_variant_to_record(payload), &store::PackEncodeOptions::default()).map_err(Into::into)
}
pub fn decode(bytes: &[u8]) -> Result<BmpMutation, protocol::ProtocolError> {
    let (record, _) = store::pack_rt::decode_record_body(bytes, &super::text::spec(), &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
    dsl::__rt::newtype_variant_from_record(&record).map(BmpMutation::RemovePaletteEntry).map_err(|error| protocol::ProtocolError::Malformed { what: "remove-palette-entry", offset: 0, detail: error.to_string() })
}
