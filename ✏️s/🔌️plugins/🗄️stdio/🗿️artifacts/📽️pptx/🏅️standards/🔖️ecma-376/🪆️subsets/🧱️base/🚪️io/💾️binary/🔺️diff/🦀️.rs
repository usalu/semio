//! pptx rep for stdio.pptx 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::schema::snapshot::{PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxTransform, PptxXmlPart};
use crate::PptxSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyResult, MutationDiff};
#[cfg(test)]
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use semio_s_artifact_stdio_zip::opc::OpcPackage;

impl protocol::DiffBinary for PptxDiff {
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    Ok(store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(self)))
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "pptx diff", offset: 0, detail: error.to_string() })?;
    semio_framework_value::FromValue::from_value(value).map_err(|error| protocol::ProtocolError::Malformed { what: "pptx diff", offset: 0, detail: error.to_string() })
}
}
}
pub use diff_codec::*;
