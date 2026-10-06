//! deflate rep for stdio.deflate 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_rfc1950::subsets::any::schema::diff::*;
use crate::schema::snapshot::DeflateLevelHint;
use crate::DeflateSnapshot;
use protocol::MutationDiff;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;

impl protocol::DiffBinary for DeflateDiff {
/// 🧪️ P2-FG2: REAL binary frame (`format u8 | flags u8 | [compression_method][window_bits]
/// [compression_level_hint][dict_id][payload]`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes`
/// shape -- upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100% of
/// stdio's `DiffCodec` impls were still on that shortcut per the P2-W0 census). `flags` bits
/// 0-4 mark `compression_method`/`window_bits`/`compression_level_hint`/`dict_id`/`payload`
/// presence in that fixed order; each present field's own (possibly tri-state) payload
/// follows in the same order, `payload` last so it can be bare "rest of buffer" bytes with
/// no length prefix (it is the only opaque, unbounded field in the frame).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut flags: u8 = 0;
    if self.compression_method.is_some() {
        flags |= 0b0_0001;
    }
    if self.window_bits.is_some() {
        flags |= 0b0_0010;
    }
    if self.compression_level_hint.is_some() {
        flags |= 0b0_0100;
    }
    if self.dict_id.is_some() {
        flags |= 0b0_1000;
    }
    if self.payload.is_some() {
        flags |= 0b1_0000;
    }
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
    if let Some(v) = self.compression_method {
        out.push(v);
    }
    if let Some(v) = self.window_bits {
        out.push(v);
    }
    if let Some(v) = self.compression_level_hint {
        out.push(v.to_bits());
    }
    if let Some(dict_id) = &self.dict_id {
        out.push(if dict_id.is_some() { 1 } else { 0 });
        if let Some(id) = dict_id {
            out.extend_from_slice(&id.to_le_bytes());
        }
    }
    if let Some(payload) = &self.payload {
        out.extend_from_slice(payload);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
    let compression_method = if flags & 0b0_0001 != 0 { Some(reader.read_u8().map_err(|e| malformed("diff compression_method", reader.position(), e.to_string()))?) } else { None };
    let window_bits = if flags & 0b0_0010 != 0 { Some(reader.read_u8().map_err(|e| malformed("diff window_bits", reader.position(), e.to_string()))?) } else { None };
    let compression_level_hint = if flags & 0b0_0100 != 0 {
        let bits = reader.read_u8().map_err(|e| malformed("diff compression_level_hint", reader.position(), e.to_string()))?;
        Some(DeflateLevelHint::from_bits(bits))
    } else {
        None
    };
    let dict_id = if flags & 0b0_1000 != 0 {
        let has = reader.read_u8().map_err(|e| malformed("diff dict_id presence", reader.position(), e.to_string()))?;
        Some(if has != 0 {
            let bytes4 = reader.read_bytes(4).map_err(|e| malformed("diff dict_id", reader.position(), e.to_string()))?;
            Some(u32::from_le_bytes([bytes4[0], bytes4[1], bytes4[2], bytes4[3]]))
        } else {
            None
        })
    } else {
        None
    };
    let payload = if flags & 0b1_0000 != 0 {
        let rest = reader.read_bytes(reader.remaining()).map_err(|e| malformed("diff payload", reader.position(), e.to_string()))?;
        Some(rest.to_vec())
    } else {
        None
    };
    Ok(DeflateDiff { compression_method, window_bits, compression_level_hint, dict_id, payload })
}
}
}
pub use diff_codec::*;
