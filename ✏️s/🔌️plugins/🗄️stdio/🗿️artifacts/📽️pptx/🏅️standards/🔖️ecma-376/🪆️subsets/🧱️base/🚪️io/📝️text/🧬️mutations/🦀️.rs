//! 📝️ Text representation codec surface for `stdio.pptx` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v_ecma_376::subsets::base::schema::mutations::*;
use crate::schema::diff::PptxDiff;
use crate::schema::snapshot::{PptxParagraph, PptxShape, PptxSlide, PptxTransform};
use crate::PptxSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use xml_address::{PptxShapeAddress, PptxSlideAddress, PptxXmlAddress, PptxXmlVacancyAddress};

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
struct PptxMutationRecord {
    kind: String,
    value: semio_framework_value::DslValue,
}

impl OpText for PptxMutation {
    fn print_op(&self) -> String {
        let record = PptxMutationRecord { kind: "mutation".into(), value: semio_framework_value::ToValue::to_value(self) };
        semio_framework_dsl_record::print(&record.__dsl_to_record(), &PptxMutationRecord::__dsl_spec(), semio_framework_dsl_record::JoinMode::Inline)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let record =
            semio_framework_dsl_record::parse(line, &PptxMutationRecord::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits { max_bytes: 64 * 1024 * 1024, ..semio_framework_diagnostic::Limits::default() }, mode: semio_framework_dsl_record::SourceMode::Inline })?;
        let model = PptxMutationRecord::__dsl_from_record(&record)?;
        match model.kind.as_str() {
            "mutation" => semio_framework_value::FromValue::from_value(model.value).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1))),
            _ => Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PPTX mutation record kind mismatch", semio_framework_diagnostic::TextSpan::at(1, 1))),
        }
    }
}
}
pub use mutations_codec::*;

