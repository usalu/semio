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
/// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=`columns`, bit1=`rows`) are two REAL fixed fields; each present
/// section follows as a varint byte length plus the same `enc_indexed_triple` text this facet's `print_diff` emits.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    use crate::standards::v1::subsets::table::io::text::diff::{enc_columns, enc_rows};
    let presence: u8 = (if self.columns.is_some() { 0b0000_0001 } else { 0 }) | (if self.rows.is_some() { 0b0000_0010 } else { 0 });
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    for section in [self.columns.as_ref().map(enc_columns), self.rows.as_ref().map(enc_rows)].into_iter().flatten() {
        store::pack_rt::write_varint_u64(&mut out, section.len() as u64);
        out.extend_from_slice(section.as_bytes());
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    use crate::standards::v1::subsets::table::io::text::diff::{dec_columns, dec_rows};
    let malformed = |what: &'static str, detail: String| protocol::ProtocolError::Malformed { what, offset: 2, detail };
    if bytes.len() < 2 {
        return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
    }
    if bytes[0] != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
    }
    let presence = bytes[1];
    let mut reader = store::ByteReader::new(&bytes[2..]);
    let mut section = |what: &'static str| -> Result<String, protocol::ProtocolError> {
        let length = reader.read_varint_u64().map_err(|e| malformed(what, e.to_string()))? as usize;
        let raw = reader.read_bytes(length).map_err(|e| malformed(what, e.to_string()))?;
        String::from_utf8(raw.to_vec()).map_err(|e| malformed(what, e.to_string()))
    };
    let columns = if presence & 0b0000_0001 != 0 { Some(dec_columns(&section("diff columns")?).map_err(|e| malformed("diff columns", e))?) } else { None };
    let rows = if presence & 0b0000_0010 != 0 { Some(dec_rows(&section("diff rows")?).map_err(|e| malformed("diff rows", e))?) } else { None };
    Ok(SemioTableDiff { columns, rows })
}
}
pub use diff_codec::*;
