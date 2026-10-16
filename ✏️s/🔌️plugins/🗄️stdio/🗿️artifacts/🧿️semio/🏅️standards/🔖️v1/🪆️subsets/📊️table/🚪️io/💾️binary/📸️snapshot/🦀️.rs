//! 💾️ Binary representation codec surface for `stdio.semio.table` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_value_model::io::text::diff::dec_semio_value;
use semio_s_artifact_stdio_semio_value_model::io::text::diff::enc_semio_value;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_str_lp;
use semio_s_artifact_stdio_semio_base::io::binary::codec::write_str_lp;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use framework_schema::ArtifactSchema;

/// 🧪️ `write_str_lp`/`read_str_lp` are IMPORTED from `🔢️value`'s own `🔺️diff` module (real
/// LEB128-varint-length-prefixed binary primitives, `store::pack_rt::write_varint_u64`/
/// `store::ByteReader`-backed) — reused, not re-derived.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_column(out: &mut Vec<u8>, c: &SemioTableColumn) {
    write_str_lp(out, &c.name);
    out.push(match c.kind {
        SemioTableCellKind::Null => 0,
        SemioTableCellKind::Bool => 1,
        SemioTableCellKind::Int => 2,
        SemioTableCellKind::Float => 3,
        SemioTableCellKind::Str => 4,
        SemioTableCellKind::Bytes => 5,
    });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_column(reader: &mut store::ByteReader<'_>) -> Result<SemioTableColumn, String> {
    let name = read_str_lp(reader)?;
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    let kind = match tag {
        0 => SemioTableCellKind::Null,
        1 => SemioTableCellKind::Bool,
        2 => SemioTableCellKind::Int,
        3 => SemioTableCellKind::Float,
        4 => SemioTableCellKind::Str,
        5 => SemioTableCellKind::Bytes,
        other => return Err(format!("unsupported cell kind tag {other}")),
    };
    Ok(SemioTableColumn { name, kind })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_row(out: &mut Vec<u8>, r: &SemioTableRow) {
    store::pack_rt::write_varint_u64(out, r.cells.len() as u64);
    for cell in &r.cells {
        semio_s_artifact_stdio_semio_value_model::io::binary::diff::enc_semio_value_bin(cell, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_row(reader: &mut store::ByteReader<'_>) -> Result<SemioTableRow, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut cells = Vec::with_capacity(count as usize);
    for _ in 0..count {
        cells.push(semio_s_artifact_stdio_semio_value_model::io::binary::diff::dec_semio_value_bin(reader)?);
    }
    Ok(SemioTableRow { cells })
}

/// 🎁 `format u8` + varint-length-prefixed `schema` UTF-8 — both genuinely, individually
/// protocol-walkable, matching `📡️.protocol.semio`'s header/segment fields exactly —
/// then `columns`/`rows` (varint counts + per-item real recursive encodings) as the honest opaque
/// `payload` tail (`protocol-array-of-records` gap — homogeneous, variable-length repeated
/// records), same boundary `🔤️text`'s own snapshot binary uses for `runs`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_table_snapshot_binary(s: &SemioTableSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.columns.len() as u64);
    for c in &s.columns {
        write_column(&mut out, c);
    }
    store::pack_rt::write_varint_u64(&mut out, s.rows.len() as u64);
    for r in &s.rows {
        write_row(&mut out, r);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_table_snapshot_binary(bytes: &[u8]) -> Result<SemioTableSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let column_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut columns = Vec::with_capacity(column_count as usize);
    for _ in 0..column_count {
        columns.push(read_column(&mut reader)?);
    }
    let row_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut rows = Vec::with_capacity(row_count as usize);
    for _ in 0..row_count {
        rows.push(read_row(&mut reader)?);
    }
    Ok(SemioTableSnapshot { schema, columns, rows })
}

impl pack::snapshot::ArtifactPack for SemioTableSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_table_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_table_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SemioTableSnapshot {
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> { Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec()) }
}
}

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_value_model::io::text::diff::dec_semio_value;
use semio_s_artifact_stdio_semio_value_model::io::text::diff::enc_semio_value;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_str_lp;
use semio_s_artifact_stdio_semio_base::io::binary::codec::write_str_lp;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use framework_schema::ArtifactSchema;
use crate::io::text::snapshot::*;
/// 📦️ Encodes a [`SemioTableSnapshot`] as a semio pack envelope — the binary twin of the DSL text, produced by a
/// SEPARATE codec, which is what makes the two committed encodings of one document able to
/// contradict each other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_table_pack(snapshot: &SemioTableSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(snapshot)
}
/// 📦️ Decodes a semio pack envelope into a [`SemioTableSnapshot`] — the inverse of
/// [`encode_semio_table_pack`], reading `../../🖼️assets/📃️sheet/🎒️.pack.semio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_table_pack(bytes: &[u8]) -> Result<SemioTableSnapshot, String> {
    <SemioTableSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
}
pub use native_snapshot_codec::*;
