//! binary rep for stdio.csv 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_rfc4180::subsets::any::schema::diff::*;
use crate::schema::snapshot::{CsvField, CsvRecord, CsvSnapshot};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, HashMap};

/// 🧪️ P2-P1: **real binary diff-frame** for `CsvDiff` — upgraded from the F6-era
/// `print_diff().into_bytes()` text-as-binary shortcut. `CsvDiff` is a STRUCT
/// (`has_header: Option<bool>`, `records: Option<CsvRecordsDiff>`), so the frame is one
/// presence-flag byte PER field (not an ordinal dispatch — that's the mutations enum's own
/// shape) directly modeling `../💾️binary/📡️.protocol.semio`'s real
/// `field X u8` / `field Y u8 if X eq 1` conditional-presence layout (P2-M2 item 4). The
/// `records` triple's own recursive removed/modified/added contents are hand-rolled via
/// `dsl::ByteWriter`/`dsl::ByteReader` and placed LAST so they can honestly consume "rest of
/// buffer" with no length prefix — see the protocol file's own doc comment for why (no
/// `Ref`-to-struct / no heterogeneous `Array` in this dialect yet).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_field_diff(w: &mut dsl::ByteWriter, d: &CsvFieldDiff) {
    match &d.value {
        None => w.write_u8(0),
        Some(v) => {
            w.write_u8(1);
            let bytes = v.as_bytes();
            w.write_varint_u64(bytes.len() as u64);
            w.write_bytes(bytes);
        }
    }
    match d.quoted {
        None => w.write_u8(0),
        Some(v) => {
            w.write_u8(1);
            w.write_u8(if v { 1 } else { 0 });
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_field_diff(r: &mut dsl::ByteReader<'_>) -> Result<CsvFieldDiff, dsl::PackRefusal> {
    let mut d = CsvFieldDiff::default();
    if r.read_u8()? == 1 {
        let len = r.read_varint_u64()? as usize;
        let bytes = r.read_bytes(len)?;
        d.value = Some(String::from_utf8(bytes.to_vec()).map_err(|e| dsl::PackRefusal::Malformed { kind:semio_framework_value::ValueRefusalKind::InvalidValue, what: "csv diff field value utf8", offset: 0, detail: e.to_string() })?);
    }
    if r.read_u8()? == 1 {
        d.quoted = Some(r.read_u8()? != 0);
    }
    Ok(d)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_record_diff(w: &mut dsl::ByteWriter, d: &CsvRecordDiff) {
    match &d.fields {
        None => w.write_u8(0),
        Some(v) => {
            w.write_u8(1);
            w.write_varint_u64(v.len() as u64);
            for item in v {
                match item {
                    None => w.write_u8(0),
                    Some(fd) => {
                        w.write_u8(1);
                        write_bin_field_diff(w, fd);
                    }
                }
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_record_diff(r: &mut dsl::ByteReader<'_>) -> Result<CsvRecordDiff, dsl::PackRefusal> {
    let fields = if r.read_u8()? == 1 {
        let n = r.read_varint_u64()? as usize;
        let mut items = Vec::with_capacity(n);
        for _ in 0..n {
            items.push(if r.read_u8()? == 1 { Some(read_bin_field_diff(r)?) } else { None });
        }
        Some(items)
    } else {
        None
    };
    Ok(CsvRecordDiff { fields })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_records_diff(w: &mut dsl::ByteWriter, d: &CsvRecordsDiff) {
    w.write_varint_u64(d.removed.len() as u64);
    for idx in &d.removed {
        w.write_varint_u64(*idx as u64);
    }
    w.write_varint_u64(d.modified.len() as u64);
    for m in &d.modified {
        w.write_varint_u64(m.index as u64);
        write_bin_record_diff(w, &m.diff);
    }
    w.write_varint_u64(d.added.len() as u64);
    for a in &d.added {
        w.write_varint_u64(a.index as u64);
        crate::schema::mutations::write_bin_record(w, &a.record);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_records_diff(r: &mut dsl::ByteReader<'_>) -> Result<CsvRecordsDiff, dsl::PackRefusal> {
    let removed_n = r.read_varint_u64()? as usize;
    let mut removed = Vec::with_capacity(removed_n);
    for _ in 0..removed_n {
        removed.push(r.read_varint_u64()? as usize);
    }
    let modified_n = r.read_varint_u64()? as usize;
    let mut modified = Vec::with_capacity(modified_n);
    for _ in 0..modified_n {
        let index = r.read_varint_u64()? as usize;
        let diff = read_bin_record_diff(r)?;
        modified.push(CsvRecordModified { index, diff });
    }
    let added_n = r.read_varint_u64()? as usize;
    let mut added = Vec::with_capacity(added_n);
    for _ in 0..added_n {
        let index = r.read_varint_u64()? as usize;
        let record = crate::schema::mutations::read_bin_record(r)?;
        added.push(CsvRecordAdded { index, record });
    }
    Ok(CsvRecordsDiff { removed, modified, added })
}

impl protocol::DiffBinary for CsvDiff {
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut w = dsl::ByteWriter::new();
    match self.has_header {
        Some(v) => {
            w.write_u8(1);
            w.write_u8(if v { 1 } else { 0 });
        }
        None => w.write_u8(0),
    }
    match &self.records {
        Some(r) => {
            w.write_u8(1);
            write_bin_records_diff(&mut w, r);
        }
        None => w.write_u8(0),
    }
    Ok(w.into_bytes())
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut r = dsl::ByteReader::new(bytes);
    let hh_flag = r.read_u8().map_err(protocol::ProtocolError::from)?;
    let has_header = if hh_flag == 1 { Some(r.read_u8().map_err(protocol::ProtocolError::from)? != 0) } else { None };
    let rec_flag = r.read_u8().map_err(protocol::ProtocolError::from)?;
    let records = if rec_flag == 1 { Some(read_bin_records_diff(&mut r).map_err(protocol::ProtocolError::from)?) } else { None };
    Ok(CsvDiff { has_header, records })
}
}

}
pub use diff_codec::*;
