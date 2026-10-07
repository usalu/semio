//! 🏗️ Builds owned WordprocessingML nodes and authoritative DOCX parts.
use crate::standards::v_ecma_376::subsets::base::schema::{snapshot::*,vocabulary::*,refusal::DocxError};
use crate::DocxSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr,XmlDocument,XmlNode};
use semio_s_artifact_stdio_zip::opc::{OpcPackage,RELS_CONTENT_TYPE,REL_TYPE_OFFICE_DOCUMENT};
//#region 🔖️XmlHelpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn elem(name: &str, attrs: Vec<XmlAttr>, children: Vec<XmlNode>) -> XmlNode {
    XmlNode::Element { name: name.into(), attrs, children }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn attr(name: &str, value: &str) -> XmlAttr {
    XmlAttr { name: name.into(), value: value.into() }
}
//#endregion 🔖️XmlHelpers

//#region 🔖️RunMapping
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn run_to_xml(r: &DocxRun) -> XmlNode {
    let mut rc = Vec::new();
    if r.bold || r.italic || r.underline || !r.extra_run_properties.is_empty() {
        let mut rpr = Vec::new();
        if r.bold {
            rpr.push(elem("w:b", vec![], vec![]));
        }
        if r.italic {
            rpr.push(elem("w:i", vec![], vec![]));
        }
        if r.underline {
            rpr.push(elem("w:u", vec![attr("w:val", "single")], vec![]));
        }
        rpr.extend(r.extra_run_properties.iter().cloned());
        rc.push(elem("w:rPr", vec![], rpr));
    }
    rc.push(elem("w:t", vec![attr("xml:space", "preserve")], vec![XmlNode::Text { text: r.text.clone() }]));
    elem("w:r", vec![], rc)
}
//#endregion 🔖️RunMapping

//#region 🔖️ParagraphMapping
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn paragraph_to_xml(p: &DocxParagraph) -> XmlNode {
    let mut children = Vec::new();
    if p.style.is_some() || !p.extra_paragraph_properties.is_empty() {
        let mut ppr = Vec::new();
        if let Some(style) = &p.style {
            ppr.push(elem("w:pStyle", vec![attr("w:val", style)], vec![]));
        }
        ppr.extend(p.extra_paragraph_properties.iter().cloned());
        children.push(elem("w:pPr", vec![], ppr));
    }
    children.extend(p.runs.iter().map(run_to_xml));
    elem("w:p", vec![], children)
}
//#endregion 🔖️ParagraphMapping

//#region 🔖️TableMapping
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cell_to_xml(c: &DocxTableCell) -> XmlNode {
    let mut children = Vec::new();
    if !c.extra_cell_properties.is_empty() {
        children.push(elem("w:tcPr", vec![], c.extra_cell_properties.clone()));
    }
    children.extend(c.blocks.iter().map(block_to_xml));
    elem("w:tc", vec![], children)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn row_to_xml(r: &DocxTableRow) -> XmlNode {
    let mut children = Vec::new();
    if !r.extra_row_properties.is_empty() {
        children.push(elem("w:trPr", vec![], r.extra_row_properties.clone()));
    }
    children.extend(r.cells.iter().map(cell_to_xml));
    elem("w:tr", vec![], children)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn table_to_xml(t: &DocxTable) -> XmlNode {
    let mut children = Vec::new();
    if !t.extra_table_properties.is_empty() {
        children.push(elem("w:tblPr", vec![], t.extra_table_properties.clone()));
    }
    children.extend(t.rows.iter().map(row_to_xml));
    elem("w:tbl", vec![], children)
}
//#endregion 🔖️TableMapping

//#region 🔖️BlockMapping
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn block_to_xml(b: &DocxBlock) -> XmlNode {
    match b {
        DocxBlock::Paragraph(p) => paragraph_to_xml(p),
        DocxBlock::Table(t) => table_to_xml(t),
    }
}
//#endregion 🔖️BlockMapping

//#region 🔖️DocumentMapping
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn document_to_xml(doc: &DocxDocument) -> XmlDocument {
    let body_children = doc.body.iter().map(block_to_xml).collect();
    XmlDocument { root: Some(elem("w:document", vec![attr("xmlns:w", W_NS)], vec![elem("w:body", vec![], body_children)])), doctype: None, declaration: None, prolog: Vec::new(), epilog: Vec::new() }
}
//#endregion 🔖️DocumentMapping

//#region 🔖️StylesMapping
const STYLES_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn style_to_xml(s: &DocxStyle) -> XmlNode {
    let mut sc = vec![elem("w:name", vec![attr("w:val", &s.name)], vec![])];
    if let Some(based_on) = &s.based_on {
        sc.push(elem("w:basedOn", vec![attr("w:val", based_on)], vec![]));
    }
    elem("w:style", vec![attr("w:styleId", &s.id)], sc)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn styles_to_xml(styles: &[DocxStyle]) -> XmlDocument {
    let children = styles.iter().map(style_to_xml).collect();
    XmlDocument { root: Some(elem("w:styles", vec![attr("xmlns:w", STYLES_NS)], children)), doctype: None, declaration: None, prolog: Vec::new(), epilog: Vec::new() }
}
//#endregion 🔖️StylesMapping

//#region 🔖️Codec
/// 🏗️ Assembles a brand-new, minimal valid OPC package around one semantic projection.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn build_minimal_docx(document: DocxDocument) -> DocxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    opc.content_types.set_override(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE);
    opc.add_generated_relationship("", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
    let mut xml_parts = vec![DocxXmlPart::try_from_document(MAIN_DOCUMENT_PART.into(), MAIN_DOCUMENT_CONTENT_TYPE.into(), document_to_xml(&document))
        .expect("the generated main document has bounded retained XML ownership")];
    if !document.styles.is_empty() {
        opc.content_types.set_override(STYLES_PART, STYLES_CONTENT_TYPE);
        xml_parts.push(
            DocxXmlPart::try_from_document(STYLES_PART.into(), STYLES_CONTENT_TYPE.into(), styles_to_xml(&document.styles))
                .expect("the generated styles document has bounded retained XML ownership"),
        );
        opc.add_generated_relationship(MAIN_DOCUMENT_PART, REL_TYPE_STYLES, STYLES_REL_TARGET);
    }
    DocxSnapshot::from_parts(opc, xml_parts).expect("the bounded minimal DOCX package has valid retained ownership")
}

