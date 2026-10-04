//! 🔺️ Sparse replacement diff over the three canonical PPTX snapshot fields.

use crate::schema::snapshot::{PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxTransform, PptxXmlPart};
use crate::PptxSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyResult, MutationDiff};
#[cfg(test)]
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use semio_s_artifact_stdio_zip::opc::OpcPackage;

/// 🔺️ Replaces only canonical fields that changed, retaining every untouched byte and XML node.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pptx.diff")]
pub struct PptxDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub opc: Option<OpcPackage>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub xml_parts: Option<Vec<PptxXmlPart>>,
}

impl MutationDiff<PptxSnapshot> for PptxDiff {
    fn apply(&self, base: &PptxSnapshot) -> MutationApplyResult<PptxSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(opc) = &self.opc {
            next.opc = opc.clone();
        }
        if let Some(xml_parts) = &self.xml_parts {
            next.xml_parts = xml_parts.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.opc.is_some() {
            self.opc = other.opc;
        }
        if other.xml_parts.is_some() {
            self.xml_parts = other.xml_parts;
        }
    }
}

impl DiffAlgebra<PptxSnapshot> for PptxDiff {
    fn inverse(&self, base: &PptxSnapshot) -> Self {
        Self { schema: self.schema.as_ref().map(|_| base.schema.clone()), opc: self.opc.as_ref().map(|_| base.opc.clone()), xml_parts: self.xml_parts.as_ref().map(|_| base.xml_parts.clone()) }
    }

    fn between(base: &PptxSnapshot, other: &PptxSnapshot) -> Self {
        Self { schema: (base.schema != other.schema).then(|| other.schema.clone()), opc: (base.opc != other.opc).then(|| other.opc.clone()), xml_parts: (base.xml_parts != other.xml_parts).then(|| other.xml_parts.clone()) }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.opc.is_none() && self.xml_parts.is_none()
    }
}

/// 🧩️ Builds an exact sparse field diff for a complete snapshot replacement.
pub fn diff_set_snapshot(base: &PptxSnapshot, next: &PptxSnapshot) -> PptxDiff {
    PptxDiff::between(base, next)
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
struct PptxDiffRecord {
    value: semio_framework_value::DslValue,
}

impl protocol::DiffCodec for PptxDiff {
    fn print_diff(&self) -> String {
        let record = PptxDiffRecord { value: semio_framework_value::ToValue::to_value(self) };
        semio_framework_dsl_record::print(&record.__dsl_to_record(), &PptxDiffRecord::__dsl_spec(), semio_framework_dsl_record::JoinMode::Inline)
    }

    fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let record = semio_framework_dsl_record::parse(line, &PptxDiffRecord::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits { max_bytes: 64 * 1024 * 1024, ..semio_framework_diagnostic::Limits::default() }, mode: semio_framework_dsl_record::SourceMode::Inline })?;
        let model = PptxDiffRecord::__dsl_from_record(&record)?;
        semio_framework_value::FromValue::from_value(model.value).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(self)))
    }

    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "pptx diff", offset: 0, detail: error.to_string() })?;
        semio_framework_value::FromValue::from_value(value).map_err(|error| protocol::ProtocolError::Malformed { what: "pptx diff", offset: 0, detail: error.to_string() })
    }
}

#[cfg(test)]
pub(crate) fn demo_snapshot_a() -> PptxSnapshot {
    crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_pptx(PptxPresentation {
        slides: vec![PptxSlide {
            shapes: vec![
                PptxShape::TextBox { text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "old".into(), bold: false, italic: false, font_size: Some(10) }] }], position: PptxTransform { x: 1, y: 1, cx: 1, cy: 1 } },
                PptxShape::Other { node: XmlNode::Element { name: "p:graphicFrame".into(), attrs: Vec::new(), children: Vec::new() } },
            ],
        }],
    })
}

#[cfg(test)]
pub(crate) fn demo_snapshot_b() -> PptxSnapshot {
    crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_pptx(PptxPresentation {
        slides: vec![PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("new")], position: PptxTransform { x: 9, y: 9, cx: 9, cy: 9 } }] }],
    })
}

#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<PptxDiff> {
    let a = demo_snapshot_a();
    let b = demo_snapshot_b();
    vec![PptxDiff::default(), PptxDiff::between(&a, &b), PptxDiff::between(&b, &a), PptxDiff::between(&a, &a)]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️result-apply/🦀️.rs"]
mod result_apply_tests;
