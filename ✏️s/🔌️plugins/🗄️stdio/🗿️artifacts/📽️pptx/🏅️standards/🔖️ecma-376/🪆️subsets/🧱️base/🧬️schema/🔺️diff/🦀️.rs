//! 🔺️ PptxDiff -- sparse diff over `PptxSnapshot` (`schema`, `opc: OpcPackage`, `xml_parts`). The OPC layer is diffed part by part, content-type
//! entry by content-type entry and relationship by relationship; each XML part is diffed as a part (its content type, and its document through the
//! XML artifact's own recursive `XmlDiff`). A diff names only what changed -- never a whole `opc` or `xml_parts` owner.

use crate::schema::snapshot::{PptxParagraph, PptxPresentation, PptxRun, PptxShape, PptxSlide, PptxTransform, PptxXmlPart};
use crate::PptxSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_xml::schema::diff::XmlDiff;
#[cfg(test)]
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument;
use semio_s_artifact_stdio_xml::{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA};
#[cfg(test)]
use semio_s_artifact_stdio_zip::opc::{OpcContentTypes, OpcPackage, OpcPart, OpcRelationship, OpcTargetMode};
use semio_s_artifact_stdio_contract::kernel::list_delta::RowPatch;
use semio_s_artifact_stdio_contract::list_delta::compose_optional;
pub use semio_s_artifact_stdio_zip::opc::diff::OpcDiff;

//#region 🔖️XmlPartDiffTypes
/// 🩹 The sparse patch of one XML part: its content type and its document diff.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct PptxXmlPartDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<XmlDiff>,
}

semio_s_artifact_stdio_contract::stdio_list_delta! {
    /// 🪡️ The positional delta of the XML parts, keyed by part name.
    pub PptxXmlPartsDelta { removal: PptxXmlPartRemoval, insertion: PptxXmlPartInsertion, relocation: PptxXmlPartRelocation, modification: PptxXmlPartModification, row: PptxXmlPart, patch: PptxXmlPartDiff, key: path }
}
//#endregion 🔖️XmlPartDiffTypes

//#region 🔖️Diff
/// 🔺️ Sparse diff over the canonical PPTX snapshot fields.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pptx.diff")]
pub struct PptxDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub opc: Option<OpcDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub xml_parts: Option<PptxXmlPartsDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️XmlPartDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_snapshot(document: &XmlDocument) -> XmlSnapshot {
    XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc: document.clone() }
}

fn apply_xml_part(part: &mut PptxXmlPart, diff: &PptxXmlPartDiff, capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
    if let Some(content_type) = &diff.content_type {
        part.content_type.clone_from(content_type);
    }
    if let Some(document) = &diff.document {
        part.document = document.apply(&xml_snapshot(&part.document), capability).map_err(|error| error.under(["document"]))?.doc;
    }
    Ok(())
}

impl RowPatch<PptxXmlPart> for PptxXmlPartDiff {
    fn commit_into(&self, row: &mut PptxXmlPart, capability: protocol::ApplyCapability) -> Result<(), MutationApplyError> {
        apply_xml_part(row, self, capability)
    }

    fn absorb(&mut self, later: Self) {
        if later.content_type.is_some() {
            self.content_type = later.content_type;
        }
        self.document = match (self.document.take(), later.document) {
            (None, value) | (value, None) => value,
            (Some(mut left), Some(right)) => {
                left.absorb(right);
                Some(left)
            }
        };
    }

    fn inverse(&self, row: &PptxXmlPart) -> Self {
        Self { content_type: self.content_type.as_ref().map(|_| row.content_type.clone()), document: self.document.as_ref().map(|document| document.inverse(&xml_snapshot(&row.document))) }
    }

    fn is_empty(&self) -> bool {
        self.content_type.is_none() && self.document.is_none()
    }
}

fn apply_xml_parts(items: &mut Vec<PptxXmlPart>, diff: &PptxXmlPartsDelta, capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
    *items = diff.commit_onto(items, capability)?;
    Ok(())
}

fn rewind_xml_parts(base: &Vec<PptxXmlPart>, diff: &PptxXmlPartsDelta) -> PptxXmlPartsDelta {
    diff.inverse(base)
}
//#endregion 🔖️XmlPartDiffLogic

//#region 🔖️Apply
impl MutationDiff<PptxSnapshot> for PptxDiff {
    fn apply(&self, base: &PptxSnapshot, capability: protocol::ApplyCapability) -> MutationApplyResult<PptxSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema.clone_from(schema);
        }
        if let Some(diff) = &self.opc {
            diff.commit_into(&mut next.opc, capability).map_err(|error| error.under(["opc"]))?;
        }
        if let Some(diff) = &self.xml_parts {
            apply_xml_parts(&mut next.xml_parts, diff, capability).map_err(|error| error.under(["xmlParts"]))?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        self.opc = match (self.opc.take(), other.opc) {
            (None, value) | (value, None) => value,
            (Some(mut left), Some(right)) => {
                left.absorb(right);
                Some(left)
            }
        };
        self.xml_parts = compose_optional(self.xml_parts.take(), other.xml_parts);
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<PptxSnapshot> for PptxDiff {
    fn inverse(&self, base: &PptxSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            opc: self.opc.as_ref().map(|diff| diff.rewind(&base.opc)),
            xml_parts: self.xml_parts.as_ref().map(|diff| rewind_xml_parts(&base.xml_parts, diff)),
        }
    }

    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.opc.is_none() && self.xml_parts.is_none()
    }
}
//#endregion 🔖️DiffAlgebra


#[cfg(test)]
pub(crate) fn demo_snapshot_a() -> PptxSnapshot {
    crate::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx(PptxPresentation {
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
    crate::standards::v_ecma_376::subsets::base::schema::construction::minimal::build_minimal_pptx(PptxPresentation {
        slides: vec![PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("new")], position: PptxTransform { x: 9, y: 9, cx: 9, cy: 9 } }] }],
    })
}

/// 🧪️ The demo cases proper, built declaratively: `default()` (empty diff), a schema change and an archive comment edit.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<PptxDiff> {
    vec![
        PptxDiff::default(),
        PptxDiff { schema: Some("stdio.pptx.v2".into()), opc: Some(OpcDiff { comment: Some("archive comment".into()), ..Default::default() }), xml_parts: None },
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️result-apply/🦀️.rs"]
mod result_apply_tests;
