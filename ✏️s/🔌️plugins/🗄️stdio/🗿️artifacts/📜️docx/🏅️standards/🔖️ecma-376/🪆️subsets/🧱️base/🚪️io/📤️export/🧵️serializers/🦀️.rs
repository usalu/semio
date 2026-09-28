//! 🧵️ WordprocessingML (docx) export — `DocxDocument` → `word/document.xml`/`word/styles.xml`
//! XML render, and the OPC package assembly/sync around it. Zip/OPC/XML byte-level work is never
//! reimplemented here: it is reused from the shared `semio_s_artifact_stdio_zip::opc` layer and,
//! transitively, `semio_s_artifact_stdio_zip::engine` + `semio_s_artifact_stdio_xml::schema::snapshot`.

use super::super::super::{DocxError, MAIN_DOCUMENT_CONTENT_TYPE, MAIN_DOCUMENT_PART, REL_TYPE_STYLES, STRICT_REL_TYPE_OFFICE_DOCUMENT, STRICT_REL_TYPE_STYLES, STYLES_CONTENT_TYPE, STYLES_PART, STYLES_REL_TARGET, W_NS};
use crate::{
    schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow, DocxXmlPart},
    DocxSnapshot,
};
use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_to_text_checked, XmlAttr, XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::{self, OpcPackage, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};

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
    let mut xml_parts = vec![DocxXmlPart { path: MAIN_DOCUMENT_PART.into(), content_type: MAIN_DOCUMENT_CONTENT_TYPE.into(), document: document_to_xml(&document) }];
    if !document.styles.is_empty() {
        opc.content_types.set_override(STYLES_PART, STYLES_CONTENT_TYPE);
        xml_parts.push(DocxXmlPart { path: STYLES_PART.into(), content_type: STYLES_CONTENT_TYPE.into(), document: styles_to_xml(&document.styles) });
        opc.add_generated_relationship(MAIN_DOCUMENT_PART, REL_TYPE_STYLES, STYLES_REL_TARGET);
    }
    DocxSnapshot::from_parts(opc, xml_parts)
}

/// 📦️ Serializes each authoritative XML part exactly once alongside non-XML OPC payloads.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_docx(snap: &DocxSnapshot) -> Result<Vec<u8>, DocxError> {
    snap.validate_authority()?;
    snap.project_document()?;
    let mut package = snap.opc.clone();
    let mut paths: std::collections::HashSet<String> = package.parts.iter().map(|part| part.path.clone()).collect();
    for part in &snap.xml_parts {
        if !paths.insert(part.path.clone()) {
            return Err(DocxError::Malformed(format!("duplicate OPC part authority: {}", part.path)));
        }
        let text = xml_document_to_text_checked(&part.document).map_err(|detail| DocxError::Xml { part: part.path.clone(), detail })?;
        package.set_part(&part.path, &part.content_type, text.into_bytes());
    }
    let main_path = crate::standards::v_ecma_376::subsets::base::io::import::deserializers::main_document_path(&package)?;
    if !snap.xml_parts.iter().any(|part| part.path == main_path) {
        return Err(DocxError::MissingPart(main_path));
    }
    Ok(opc::encode_opc_with_package_order(&package)?)
}
//#endregion 🔖️Codec
