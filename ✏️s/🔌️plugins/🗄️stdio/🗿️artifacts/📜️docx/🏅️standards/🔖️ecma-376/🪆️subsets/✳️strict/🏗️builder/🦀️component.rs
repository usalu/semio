//! 🏗️ DocxStrictBuilder (ecma-376/✳️strict) — a typed builder whose ergonomic construction path
//! writes ISO/IEC 29500-1:2016 Strict-conformant OPC output FROM THE START: the strict
//! WordprocessingML namespace, a `conformance="strict"` root attribute, and the strict
//! officeDocument relationship base (`build_minimal_strict_docx`, below) -- unlike the ✳️any
//! subset's shared `engine::build_minimal_docx`, which always emits the transitional namespace and
//! relationship base. `build()` additionally re-runs `check_strict_conformance` (the SAME single
//! source of truth `DocxStrictComposer` hard-gates on) unconditionally, so a hard violation
//! reaching this builder via the generic `SetSnapshot`/`mutate` escape hatch can never leave
//! `build()` as `Ok`.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::docx::schema::snapshot::{DocxDocument, DocxParagraph, DocxRun};
use crate::artifacts::docx::standards::v_ecma_376::subsets::strict::analyzer::{check_strict_conformance, STRICT_REL_BASE};
use crate::artifacts::docx::{DocxDiff, DocxMutation, DocxSnapshot};
use crate::artifacts::xml::schema::snapshot::{xml_document_to_text, XmlAttr, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::{OpcPackage, RELS_CONTENT_TYPE};

//#region 🔖️Namespaces
const STRICT_MAIN_NS: &str = "http://purl.oclc.org/ooxml/wordprocessingml/main";
const MAIN_DOCUMENT_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const MAIN_DOCUMENT_PART: &str = "word/document.xml";
//#endregion 🔖️Namespaces

//#region 🔖️Seed
/// 🌱️ Assembles a fresh, minimal-but-strict-conformant OPC package around `document` -- real
/// construction (mirrors the ✳️any subset's `build_minimal_docx` shape), just with the strict
/// namespace, root `conformance="strict"` attribute, and strict officeDocument relationship base
/// written from the start instead of the transitional ones.
fn build_minimal_strict_docx(document: DocxDocument) -> DocxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    let bytes = xml_document_to_text(&document_to_strict_xml(&document)).into_bytes();
    opc.set_part(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE, bytes);
    opc.add_relationship("", "rId1", &format!("{STRICT_REL_BASE}/officeDocument"), MAIN_DOCUMENT_PART);
    DocxSnapshot::from_parts(opc, document)
}

/// ✍️ Same paragraph/run -> XML shape as the ✳️any subset's `engine::document_to_xml`, just with
/// the strict `xmlns:w` value and an added `conformance="strict"` root attribute.
/// ✍️ Renders only the `Paragraph` blocks of `doc.body` (strict conformance's ergonomic
/// construction path is paragraph/run-only, same scope as before this ticket's table/style
/// enrichment; a `Table` block reaching this builder via `SetSnapshot`/raw `mutate` still survives
/// losslessly through the shared `✳️any` engine's `document_to_xml`, this fn is only the TYPED
/// convenience path for `add_paragraph`/`add_text_paragraph`/`add_runs`).
fn document_to_strict_xml(doc: &DocxDocument) -> XmlDocument {
    let body_children = doc
        .body
        .iter()
        .filter_map(|block| match block {
            crate::artifacts::docx::schema::snapshot::DocxBlock::Paragraph(p) => Some(p),
            _ => None,
        })
        .map(|p| {
            let run_children = p
                .runs
                .iter()
                .map(|r| {
                    let mut rc = Vec::new();
                    if r.bold || r.italic || r.underline {
                        let mut rpr = Vec::new();
                        if r.bold {
                            rpr.push(XmlNode::Element { name: "w:b".into(), attrs: vec![], children: vec![] });
                        }
                        if r.italic {
                            rpr.push(XmlNode::Element { name: "w:i".into(), attrs: vec![], children: vec![] });
                        }
                        if r.underline {
                            rpr.push(XmlNode::Element { name: "w:u".into(), attrs: vec![XmlAttr { name: "w:val".into(), value: "single".into() }], children: vec![] });
                        }
                        rc.push(XmlNode::Element { name: "w:rPr".into(), attrs: vec![], children: rpr });
                    }
                    rc.push(XmlNode::Element {
                        name: "w:t".into(),
                        attrs: vec![XmlAttr { name: "xml:space".into(), value: "preserve".into() }],
                        children: vec![XmlNode::Text { text: r.text.clone() }],
                    });
                    XmlNode::Element { name: "w:r".into(), attrs: vec![], children: rc }
                })
                .collect();
            XmlNode::Element { name: "w:p".into(), attrs: vec![], children: run_children }
        })
        .collect();
    XmlDocument {
        root: Some(XmlNode::Element {
            name: "w:document".into(),
            attrs: vec![
                XmlAttr { name: "xmlns:w".into(), value: STRICT_MAIN_NS.into() },
                XmlAttr { name: "conformance".into(), value: "strict".into() },
            ],
            children: vec![XmlNode::Element { name: "w:body".into(), attrs: vec![], children: body_children }],
        }),
        doctype: None,
        declaration: None,
    }
}
//#endregion 🔖️Seed

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct DocxStrictBuilder {
    snapshot: DocxSnapshot,
}

impl DocxStrictBuilder {
    /// ➕️ Appends a paragraph, re-serializing the strict-namespaced `word/document.xml` part.
    pub fn add_paragraph(mut self, paragraph: DocxParagraph) -> Self {
        self.snapshot.document.body.push(crate::artifacts::docx::schema::snapshot::DocxBlock::Paragraph(paragraph));
        self.snapshot = build_minimal_strict_docx(self.snapshot.document);
        self
    }

    /// ➕️ Appends a single-run plain-text paragraph.
    pub fn add_text_paragraph(self, text: impl Into<String>) -> Self {
        self.add_paragraph(DocxParagraph::text(text.into()))
    }

    /// ➕️ Appends a paragraph made of the given runs (basic bold/italic/underline formatting).
    pub fn add_runs(self, runs: Vec<DocxRun>) -> Self {
        self.add_paragraph(DocxParagraph { runs, style: None, extra_paragraph_properties: Vec::new() })
    }
}

impl ArtifactBuilder for DocxStrictBuilder {
    type Snapshot = DocxSnapshot;
    type Mutation = DocxMutation;
    type Diff = DocxDiff;

    fn empty() -> Self {
        Self { snapshot: build_minimal_strict_docx(DocxDocument::default()) }
    }

    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot }
    }

    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }

    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }

    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = crate::artifacts::docx::schema::mutations::apply_docx_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }

    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <DocxDiff as protocol::MutationDiff<DocxSnapshot>>::apply(&diff, &self.snapshot);
        self
    }

    /// 🛡️ The real construction gate: re-runs `check_strict_conformance` unconditionally,
    /// regardless of which path produced the in-flight snapshot (typed `add_paragraph`,
    /// `from_binary`, a raw `SetSnapshot` mutation) -- a hard violation can never leave `build()`
    /// as `Ok`.
    fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
        let hard: Vec<Diagnostic> = check_strict_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
        if hard.is_empty() {
            Ok(self.snapshot)
        } else {
            Err(hard)
        }
    }
}
//#endregion 🔖️Builder

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_builder_is_strict_conformant() {
        let snapshot = DocxStrictBuilder::empty().build().expect("empty strict builder must be conformant");
        assert!(snapshot.opc.part_bytes(MAIN_DOCUMENT_PART).is_some());
    }

    #[test]
    fn add_paragraph_stays_strict_conformant() {
        let snapshot = DocxStrictBuilder::empty().add_text_paragraph("Hello, strict world!").build().expect("must build");
        assert_eq!(snapshot.document.body.len(), 1);
        let bytes = snapshot.opc.part_bytes(MAIN_DOCUMENT_PART).unwrap();
        assert!(String::from_utf8_lossy(bytes).contains(STRICT_MAIN_NS));
    }

    #[test]
    fn hard_violation_injected_via_raw_mutate_still_fails_build() {
        let mut snapshot = DocxStrictBuilder::empty().add_text_paragraph("clean").build().unwrap();
        snapshot.opc.set_part("word/legacyDrawing.xml", "application/xml", b"<v:shape xmlns:v=\"urn:schemas-microsoft-com:vml\"/>".to_vec());
        let (mutated, _diff) = DocxStrictBuilder::from_snapshot(DocxSnapshot::default()).mutate(DocxMutation::SetSnapshot { snapshot });
        let err = mutated.build().expect_err("VML content must fail build()");
        assert!(err.iter().any(|d| d.code.0 == crate::artifacts::docx::standards::v_ecma_376::subsets::strict::analyzer::CODE_VML_PRESENT));
    }
}
