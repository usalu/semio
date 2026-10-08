//! 🏗️ Typed strict WordprocessingML document construction.
use crate::{DocxSnapshot, schema::snapshot::{DocxDocument, DocxXmlPart}};
use crate::standards::v_ecma_376::subsets::strict::schema::conformance::STRICT_REL_BASE;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::{OpcPackage, RELS_CONTENT_TYPE};

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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn build_minimal_strict_docx(document: DocxDocument) -> DocxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.content_types.set_override(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE);
    opc.add_generated_relationship("", &format!("{STRICT_REL_BASE}/officeDocument"), MAIN_DOCUMENT_PART);
    DocxSnapshot::from_parts(
        opc,
        vec![DocxXmlPart::try_from_document(MAIN_DOCUMENT_PART.into(), MAIN_DOCUMENT_CONTENT_TYPE.into(), document_to_strict_xml(&document))
            .expect("the minimal strict XML document fits retained ownership")],
    )
        .expect("the minimal strict DOCX package fits retained OPC ownership")
}

/// ✍️ Same paragraph/run -> XML shape as the ✳️any subset's `engine::document_to_xml`, just with
/// the strict `xmlns:w` value and an added `conformance="strict"` root attribute.
/// ✍️ Renders only the `Paragraph` blocks of `doc.body` (strict conformance's ergonomic
/// construction path is paragraph/run-only, same scope as before this ticket's table/style
/// enrichment; a `Table` block reaching this builder via raw `mutate` still survives
/// losslessly through the shared `✳️any` engine's `document_to_xml`, this fn is only the TYPED
/// convenience path for `add_paragraph`/`add_text_paragraph`/`add_runs`).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn document_to_strict_xml(doc: &DocxDocument) -> XmlDocument {
    let body_children = doc
        .body
        .iter()
        .filter_map(|block| match block {
            crate::schema::snapshot::DocxBlock::Paragraph(p) => Some(p),
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
                    rc.push(XmlNode::Element { name: "w:t".into(), attrs: vec![XmlAttr { name: "xml:space".into(), value: "preserve".into() }], children: vec![XmlNode::Text { text: r.text.clone() }] });
                    XmlNode::Element { name: "w:r".into(), attrs: vec![], children: rc }
                })
                .collect();
            XmlNode::Element { name: "w:p".into(), attrs: vec![], children: run_children }
        })
        .collect();
    XmlDocument {
        prolog: Vec::new(),
        epilog: Vec::new(),
        root: Some(XmlNode::Element {
            name: "w:document".into(),
            attrs: vec![XmlAttr { name: "xmlns:w".into(), value: STRICT_MAIN_NS.into() }, XmlAttr { name: "conformance".into(), value: "strict".into() }],
            children: vec![XmlNode::Element { name: "w:body".into(), attrs: vec![], children: body_children }],
        }),
        doctype: None,
        declaration: None,
    }
}
//#endregion 🔖️Seed
