//! binary rep for stdio.epw 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::energyplus::subsets::any::schema::diff::*;
use crate::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot, EPW_RECORD_FIELD_COUNT};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{DiffBinary,DiffCodec,DiffText};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{BTreeMap, HashMap};

impl protocol::DiffBinary for EpwDiff {
/// ⚡️ Binary = the text bytes verbatim, same simplification csv's/gif89a's hand-rolled
/// `DiffCodec`s use.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    Ok(self.print_diff().into_bytes())
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: e.to_string() })?;
    Self::parse_diff(line).map_err(|e| protocol::ProtocolError::Malformed { what: "diff text", offset: 0, detail: e.to_string() })
}
}
}
pub use diff_codec::*;
