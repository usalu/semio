//! 💾️ Binary representation codec surface for `stdio.semio.text` (diff) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::text::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextMark;

impl protocol::DiffBinary for SemioTextDiff {
/// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=`runs`) are two REAL fixed fields; when present, the
/// index-keyed `runs` triple follows as the same `enc_runs` text the text codec prints (one UTF-8 payload to the end).
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    use crate::standards::v1::subsets::text::io::text::diff::enc_runs;
    let presence: u8 = if self.runs.is_some() { 0b0000_0001 } else { 0 };
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    if let Some(runs) = &self.runs {
        out.extend_from_slice(enc_runs(runs).as_bytes());
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    const DIFF_BINARY_FORMAT: u8 = 1;
    use crate::standards::v1::subsets::text::io::text::diff::dec_runs;
    if bytes.len() < 2 {
        return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
    }
    if bytes[0] != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
    }
    let runs = if bytes[1] & 0b0000_0001 != 0 {
        let text = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "diff runs", offset: 2, detail: e.to_string() })?;
        Some(dec_runs(text).map_err(|e| protocol::ProtocolError::Malformed { what: "diff runs", offset: 2, detail: e })?)
    } else {
        None
    };
    Ok(SemioTextDiff { runs })
}
}
}
pub use diff_codec::*;
