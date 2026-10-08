//! 💾️ Binary representation codec surface for `s.stdio.semio.kit.diff` — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::kit::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitDesign, SemioKitSnapshot, SemioKitType};
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_design_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_design_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_type_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_type_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_link_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_link_list};
use crate::standards::v1::subsets::object::io::text::snapshot::{dec_child_opt};
use crate::standards::v1::subsets::object::io::text::snapshot::{enc_child_opt};
use crate::standards::v1::subsets::kit::io::text::snapshot::{dec_child_list};
use crate::standards::v1::subsets::kit::io::text::snapshot::{enc_child_list};

impl protocol::DiffBinary for SemioKitDiff {
/// ⚡️ Real binary diff frame: `format u8` + `presence u8` (bit0=types, bit1=designs, bit2=objects, bit3=models, bit4=properties,
/// bit5=representations), then each present field as a varint byte length plus the same text the text codec prints for it.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    use crate::standards::v1::subsets::kit::io::text::diff::kit_diff_sections;
    const DIFF_BINARY_FORMAT: u8 = 1;
    let sections = kit_diff_sections(self);
    let presence = sections.iter().enumerate().filter(|(_, section)| section.is_some()).fold(0u8, |bits, (index, _)| bits | (1 << index));
    let mut out = vec![DIFF_BINARY_FORMAT, presence];
    for section in sections.into_iter().flatten() {
        store::pack_rt::write_varint_u64(&mut out, section.len() as u64);
        out.extend_from_slice(section.as_bytes());
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    use crate::standards::v1::subsets::kit::io::text::diff::dec_kit_section;
    const DIFF_BINARY_FORMAT: u8 = 1;
    let malformed = |what: &'static str, detail: String| protocol::ProtocolError::Malformed { what, offset: 2, detail };
    if bytes.len() < 2 {
        return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
    }
    if bytes[0] != DIFF_BINARY_FORMAT {
        return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
    }
    let presence = bytes[1];
    let mut reader = store::ByteReader::new(&bytes[2..]);
    let mut diff = SemioKitDiff::default();
    for index in (0..6).filter(|index| presence & (1 << index) != 0) {
        let length = reader.read_varint_u64().map_err(|e| malformed("diff section length", e.to_string()))? as usize;
        let raw = reader.read_bytes(length).map_err(|e| malformed("diff section", e.to_string()))?;
        let text = std::str::from_utf8(raw).map_err(|e| malformed("diff section", e.to_string()))?;
        dec_kit_section(&mut diff, index, text).map_err(|e| malformed("diff section", e))?;
    }
    Ok(diff)
}
}
}
pub use diff_codec::*;
