//! 📝️ Text representation codec surface for `stdio.pptx` (diff).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type PptxDiffText = String;
//#endregion 🚚️Carrier

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

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct PptxDiffRecord {
    pub(crate) value: semio_framework_value::DslValue,
}

impl protocol::DiffText for PptxDiff {
fn print_diff(&self) -> String {
    let record = PptxDiffRecord { value: semio_framework_value::ToValue::to_value(self) };
    semio_framework_dsl_record::print(&record.__dsl_to_record(), &PptxDiffRecord::__dsl_spec(), semio_framework_dsl_record::JoinMode::Inline)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    let record = semio_framework_dsl_record::parse(line, &PptxDiffRecord::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits { max_bytes: 64 * 1024 * 1024, ..semio_framework_diagnostic::Limits::default() }, mode: semio_framework_dsl_record::SourceMode::Inline })?;
    let model = PptxDiffRecord::__dsl_from_record(&record)?;
    semio_framework_value::FromValue::from_value(model.value).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
}
pub use diff_codec::*;
