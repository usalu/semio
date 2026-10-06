//! binary rep for stdio.csv 🧬️mutations

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_rfc4180::subsets::any::schema::mutations::*;
use crate::schema::diff::{diff_set_snapshot, CsvDiff, CsvFieldDiff, CsvRecordAdded, CsvRecordDiff, CsvRecordModified, CsvRecordsDiff};
use crate::standards::v_rfc4180::subsets::any::io::text::diff::{dec_str};
use crate::standards::v_rfc4180::subsets::any::io::text::diff::{enc_str};
use crate::standards::v_rfc4180::subsets::any::io::text::diff::{dec_record};
use crate::standards::v_rfc4180::subsets::any::io::text::diff::{enc_record};
use crate::standards::v_rfc4180::subsets::any::io::text::diff::{strip_brackets};
use crate::standards::v_rfc4180::subsets::any::io::text::diff::{split_top_level};
use crate::schema::snapshot::{CsvField, CsvRecord};
use crate::CsvSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};

/// 🧪️ P2-P1: **real binary op-frame** for `CsvMutation` — upgraded from the F6-era
/// `print_op().into_bytes()` text-as-binary shortcut. `tag u8` ordinal (hand-assigned, this
/// enum cannot use `#[derive(dsl::DslOps)]`, see the doc comment above) + per-variant fields,
/// via `dsl::ByteWriter`/`dsl::ByteReader` (the real framework LEB128-varint/length-prefixed
/// primitives, `🧰️framework/…/🎒️pack/🧾️codec/🦀️.rs`, reachable from stdio because
/// `extern crate self as pack;` is re-exported at the kernel crate root and `dsl`/`store`/
/// `protocol` all alias that SAME crate root). Matches
/// `../💾️binary/📡️.protocol.semio`'s real `repeat`/`arm` shape exactly — see that
/// file's own doc comment for why the deeply nested `CsvSnapshot`/`CsvRecord` payload inside
/// arms 1/3 is one honest opaque tail blob rather than individually walked.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_str(w: &mut dsl::ByteWriter, s: &str) {
    let bytes = s.as_bytes();
    w.write_varint_u64(bytes.len() as u64);
    w.write_bytes(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_str(r: &mut dsl::ByteReader<'_>) -> Result<String, dsl::PackRefusal> {
    let len = r.read_varint_u64()? as usize;
    let bytes = r.read_bytes(len)?;
    String::from_utf8(bytes.to_vec()).map_err(|e| dsl::PackRefusal::Malformed { kind:semio_framework_value::ValueRefusalKind::InvalidValue, what: "csv binary utf8 string", offset: 0, detail: e.to_string() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_field(w: &mut dsl::ByteWriter, f: &CsvField) {
    write_bin_str(w, &f.value);
    w.write_u8(if f.quoted { 1 } else { 0 });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_field(r: &mut dsl::ByteReader<'_>) -> Result<CsvField, dsl::PackRefusal> {
    let value = read_bin_str(r)?;
    let quoted = r.read_u8()? != 0;
    Ok(CsvField { value, quoted })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_record(w: &mut dsl::ByteWriter, rec: &CsvRecord) {
    w.write_varint_u64(rec.fields.len() as u64);
    for f in &rec.fields {
        write_bin_field(w, f);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_record(r: &mut dsl::ByteReader<'_>) -> Result<CsvRecord, dsl::PackRefusal> {
    let n = r.read_varint_u64()? as usize;
    let mut fields = Vec::with_capacity(n);
    for _ in 0..n {
        fields.push(read_bin_field(r)?);
    }
    Ok(CsvRecord { fields })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_snapshot(w: &mut dsl::ByteWriter, s: &CsvSnapshot) {
    write_bin_str(w, &s.schema);
    w.write_u8(if s.has_header { 1 } else { 0 });
    w.write_varint_u64(s.records.len() as u64);
    for r in &s.records {
        write_bin_record(w, r);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_snapshot(r: &mut dsl::ByteReader<'_>) -> Result<CsvSnapshot, dsl::PackRefusal> {
    let schema = read_bin_str(r)?;
    let has_header = r.read_u8()? != 0;
    let n = r.read_varint_u64()? as usize;
    let mut records = Vec::with_capacity(n);
    for _ in 0..n {
        records.push(read_bin_record(r)?);
    }
    Ok(CsvSnapshot { schema, has_header, records })
}

impl OpBinary for CsvMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut w = dsl::ByteWriter::new();
        match self {
            CsvMutation::PatchSnapshot(payload) => {
                w.write_u8(patch_snapshot::binary::BINARY_TAG);
                w.write_bytes(&payload.patch.encode_op()?);
            }
            CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => {
                w.write_u8(TAG_SET_SNAPSHOT);
                write_bin_snapshot(&mut w, snapshot);
            }
            CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header }) => {
                w.write_u8(TAG_SET_HAS_HEADER);
                w.write_u8(if *has_header { 1 } else { 0 });
            }
            CsvMutation::InsertRecord(insert_record::InsertRecord { index, record }) => {
                w.write_u8(TAG_INSERT_RECORD);
                w.write_varint_u64(*index as u64);
                write_bin_record(&mut w, record);
            }
            CsvMutation::RemoveRecord(remove_record::RemoveRecord { index }) => {
                w.write_u8(TAG_REMOVE_RECORD);
                w.write_varint_u64(*index as u64);
            }
            CsvMutation::SetField(set_field::SetField { record_index, field_index, value, quoted }) => {
                w.write_u8(TAG_SET_FIELD);
                w.write_varint_u64(*record_index as u64);
                w.write_varint_u64(*field_index as u64);
                w.write_u8(if *quoted { 1 } else { 0 });
                write_bin_str(&mut w, value);
            }
        }
        Ok(w.into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut r = dsl::ByteReader::new(bytes);
        let ordinal = r.read_u8().map_err(protocol::ProtocolError::from)?;
        let mutation = match ordinal {
            patch_snapshot::binary::BINARY_TAG => return patch_snapshot::binary::decode(&bytes[1..]),
            TAG_SET_SNAPSHOT => CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: read_bin_snapshot(&mut r).map_err(protocol::ProtocolError::from)? }),
            TAG_SET_HAS_HEADER => CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: r.read_u8().map_err(protocol::ProtocolError::from)? != 0 }),
            TAG_INSERT_RECORD => {
                let index = r.read_varint_u64().map_err(protocol::ProtocolError::from)? as usize;
                let record = read_bin_record(&mut r).map_err(protocol::ProtocolError::from)?;
                CsvMutation::InsertRecord(insert_record::InsertRecord { index, record })
            }
            TAG_REMOVE_RECORD => CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: r.read_varint_u64().map_err(protocol::ProtocolError::from)? as usize }),
            TAG_SET_FIELD => {
                let record_index = r.read_varint_u64().map_err(protocol::ProtocolError::from)? as usize;
                let field_index = r.read_varint_u64().map_err(protocol::ProtocolError::from)? as usize;
                let quoted = r.read_u8().map_err(protocol::ProtocolError::from)? != 0;
                let value = read_bin_str(&mut r).map_err(protocol::ProtocolError::from)?;
                CsvMutation::SetField(set_field::SetField { record_index, field_index, value, quoted })
            }
            other => {
                return Err(protocol::ProtocolError::Malformed { what: "csv op ordinal", offset: 0, detail: format!("unknown ordinal {other}") });
            }
        };
        Ok(mutation)
    }
}
}
pub use mutations_codec::*;

//#region 🏷️WireTags
/// 🏷️ Op tags of `CsvMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_SET_HAS_HEADER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-has-header");
const TAG_INSERT_RECORD: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-record");
const TAG_REMOVE_RECORD: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-record");
const TAG_SET_FIELD: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-field");
//#endregion 🏷️WireTags
