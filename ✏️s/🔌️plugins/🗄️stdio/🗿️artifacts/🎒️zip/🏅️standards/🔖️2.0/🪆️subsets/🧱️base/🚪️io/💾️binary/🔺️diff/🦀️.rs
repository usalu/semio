//! zip rep for stdio.zip 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v2_0::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::ZipSnapshot;
use crate::schema::snapshot::ZipEntryMetadata;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{HashMap, HashSet};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::schema::snapshot::ZipEntry;

impl protocol::DiffBinary for ZipDiff {
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let value = semio_framework_value::ToValue::to_value(self);
    Ok(store::pack_rt::encode_wire_value(&value))
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "zip diff", offset: 0, detail: error.to_string() })?;
    <Self as semio_framework_value::FromValue>::from_value(value).map_err(|error| protocol::ProtocolError::Malformed { what: "zip diff", offset: 0, detail: error.to_string() })
}
}
}
pub use diff_codec::*;
