//! 💾️ Direct change-physical-dims binary codec.
use super::*;
use crate::schema::mutations::binary::Entry;
pub const BINARY_TAG: u8 = 8;
pub const CODEC: Entry = Entry { tag: BINARY_TAG, encode, decode };

pub fn encode(value: &PngMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>> {
    let PngMutation::ChangePhysicalDims(payload) = value else { return None };
    Some(encode_payload(payload))
}
pub fn encode_payload(payload: &ChangePhysicalDimsMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    let ChangePhysicalDimsMutation { phys } = payload;
    let mut w = dsl::ByteWriter::new();
    write_bin_option(&mut w, phys, write_bin_physical_dims);
    Ok(w.into_bytes())
}
fn op_pack_err(error: &dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "change-physical-dims", offset: 0, detail: error.to_string() }
}
pub fn decode(bytes: &[u8]) -> Result<PngMutation, protocol::ProtocolError> {
    let mut r = dsl::ByteReader::new(bytes);
    let result: Result<PngMutation, protocol::ProtocolError> = Ok(PngMutation::ChangePhysicalDims(ChangePhysicalDimsMutation { phys: read_bin_option(&mut r, read_bin_physical_dims).map_err(|error| op_pack_err(&error))? }));
    let position = r.position();
    if position != bytes.len() {
        return Err(protocol::ProtocolError::Malformed { what: "change-physical-dims", offset: position as u64, detail: "trailing payload bytes".into() });
    }
    result
}
