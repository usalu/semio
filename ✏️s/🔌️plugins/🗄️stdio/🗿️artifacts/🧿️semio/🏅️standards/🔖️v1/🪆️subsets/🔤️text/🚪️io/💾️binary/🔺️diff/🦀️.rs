//! 💾️ Binary representation codec surface for `stdio.semio.text` (diff) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::text::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextMark;

impl protocol::DiffBinary for SemioTextDiff {
/// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=`runs`) are two REAL fixed
/// fields; when present, `runs` follows as a real varint count + per-run binary encoding
/// (reusing the snapshot facet's own `write_run`/`read_run`) rather than a text-blob-in-binary
/// shortcut — `text`'s diff has exactly one collection field, so no opaque multi-field payload
/// chain is needed.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    use crate::standards::v1::subsets::text::schema::snapshot::write_run;
    let presence: u8 = if self.runs.is_some() { 0b0000_0001 } else { 0 };
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(list) = &self.runs {
        store::pack_rt::write_varint_u64(&mut out, list.values.len() as u64);
        for r in &list.values {
            write_run(&mut out, r);
        }
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    use crate::standards::v1::subsets::text::schema::snapshot::read_run;
    if bytes.len() < 2 {
        return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
    }
    if bytes[0] != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
    }
    let presence = bytes[1];
    let mut reader = store::ByteReader::new(&bytes[2..]);
    let runs = if presence & 0b0000_0001 != 0 {
        let count = reader.read_varint_u64().map_err(|e| protocol::ProtocolError::Malformed { what: "diff runs count", offset: 2, detail: e.to_string() })?;
        let mut values = Vec::with_capacity(count as usize);
        for _ in 0..count {
            values.push(read_run(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what: "diff run", offset: 2, detail: e })?);
        }
        Some(SemioTextRunList { values })
    } else {
        None
    };
    Ok(SemioTextDiff { runs })
}
}
}
pub use diff_codec::*;
