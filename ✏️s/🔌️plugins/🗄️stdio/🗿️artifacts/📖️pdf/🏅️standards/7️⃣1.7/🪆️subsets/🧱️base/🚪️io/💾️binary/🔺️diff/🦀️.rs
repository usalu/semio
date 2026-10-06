//! pdf rep for stdio.pdf 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_7::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1_7::subsets::base::io::carry_graph_edit;
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{HashMap, HashSet};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfIndirectObject;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::standards::v1_7::subsets::base::schema::snapshot::ObjRef;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage;
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfStreamFilter;

impl protocol::DiffBinary for PdfDiff {
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
    out.extend_from_slice(&store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(self)));
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    match bytes.first() {
        Some(format) if *format == store::pack_rt::OP_BINARY_FORMAT => {}
        Some(format) => return Err(malformed("diff format", 0, format!("expected {}, got {format}", store::pack_rt::OP_BINARY_FORMAT))),
        None => return Err(malformed("diff format", 0, "empty diff".into())),
    }
    let value = store::pack_rt::decode_wire_value(&bytes[1..]).map_err(|error| malformed("diff body", 1, error.to_string()))?;
    <Self as semio_framework_value::FromValue>::from_value(value).map_err(|error| malformed("diff value", 1, error.to_string()))
}
}
}
pub use diff_codec::*;
