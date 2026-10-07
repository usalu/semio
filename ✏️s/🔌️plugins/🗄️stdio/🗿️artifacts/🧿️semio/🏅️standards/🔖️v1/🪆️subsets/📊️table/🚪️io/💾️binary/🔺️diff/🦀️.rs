//! 💾️ Binary representation codec surface for `stdio.semio.table` (diff) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::table::schema::diff::*;
use crate::standards::v1::subsets::base::io::text::snapshot::split_top_level;
use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableRow, SemioTableSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
/// 🧪️ Hand-rolled `protocol::DiffCodec`. Unlike `🔤️text` (one mutable field), `table` has TWO —
/// `print_diff` MUST stay ONE PHYSICAL LINE: present fields are joined with `;` (empty string when
/// neither present, `columns=[...]` alone, `rows=[...]` alone, or `columns=[...];rows=[...]` when
/// both present). `split_top_level(line, ';')` parses back (bracket-nesting aware, so a `;` can
/// never appear inside an encoded column/row's own hex/bracket payload — there is none — this is
/// purely a top-level field separator).
use crate::standards::v1::subsets::table::io::text::snapshot::{dec_row};
use crate::standards::v1::subsets::table::io::text::snapshot::{enc_row};
use crate::standards::v1::subsets::table::io::text::snapshot::{dec_column};
use crate::standards::v1::subsets::table::io::text::snapshot::{enc_column};

impl protocol::DiffBinary for SemioTableDiff {
/// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=`columns`, bit1=`rows`) are
/// two REAL fixed fields; each present section follows as a real varint count + per-item
/// binary encoding (reusing the snapshot facet's own `write_column`/`read_column` and the
/// value subset's own `enc_semio_value_bin`/`dec_semio_value_bin` for row cells).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    use crate::standards::v1::subsets::table::io::binary::snapshot::write_column;
    use crate::standards::v1::subsets::value::io::binary::diff::enc_semio_value_bin;
    let presence: u8 = (if self.columns.is_some() { 0b0000_0001 } else { 0 }) | (if self.rows.is_some() { 0b0000_0010 } else { 0 });
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(list) = &self.columns {
        store::pack_rt::write_varint_u64(&mut out, list.values.len() as u64);
        for c in &list.values {
            write_column(&mut out, c);
        }
    }
    if let Some(list) = &self.rows {
        store::pack_rt::write_varint_u64(&mut out, list.values.len() as u64);
        for r in &list.values {
            store::pack_rt::write_varint_u64(&mut out, r.cells.len() as u64);
            for cell in &r.cells {
                enc_semio_value_bin(cell, &mut out);
            }
        }
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    use crate::standards::v1::subsets::table::io::binary::snapshot::read_column;
    use crate::standards::v1::subsets::table::schema::snapshot::SemioTableRow;
    use crate::standards::v1::subsets::value::io::binary::diff::dec_semio_value_bin;
    if bytes.len() < 2 {
        return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
    }
    if bytes[0] != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
    }
    let presence = bytes[1];
    let mut reader = store::ByteReader::new(&bytes[2..]);
    let columns = if presence & 0b0000_0001 != 0 {
        let count = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "diff columns count", offset: 2, detail: e.to_string() })?;
        let mut values = Vec::with_capacity(count as usize);
        for _ in 0..count {
            values.push(read_column(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff column", offset: 2, detail: e })?);
        }
        Some(SemioTableColumnList { values })
    } else {
        None
    };
    let rows = if presence & 0b0000_0010 != 0 {
        let count = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "diff rows count", offset: 2, detail: e.to_string() })?;
        let mut values = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let cell_count = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "diff row cell count", offset: 2, detail: e.to_string() })?;
            let mut cells = Vec::with_capacity(cell_count as usize);
            for _ in 0..cell_count {
                cells.push(dec_semio_value_bin(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff cell", offset: 2, detail: e })?);
            }
            values.push(SemioTableRow { cells });
        }
        Some(SemioTableRowList { values })
    } else {
        None
    };
    Ok(SemioTableDiff { columns, rows })
}
}
}
pub use diff_codec::*;
