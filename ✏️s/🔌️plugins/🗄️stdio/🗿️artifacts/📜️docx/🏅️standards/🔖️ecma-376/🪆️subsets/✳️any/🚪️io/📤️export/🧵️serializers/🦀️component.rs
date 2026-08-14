//! 🧵️ WordprocessingML (docx) export — `DocxDocument` → `word/document.xml`/`word/styles.xml`
//! XML render, and the OPC package assembly/sync around it. Zip/OPC/XML byte-level work is never
//! reimplemented here: it is reused from the shared `crate::artifacts::zip::opc` layer and,
//! transitively, `crate::artifacts::zip::engine` + `crate::artifacts::xml::schema::snapshot`.

use crate::artifacts::docx::{schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow}, DocxSnapshot};
use crate::artifacts::xml::schema::snapshot::{xml_document_to_text, XmlAttr, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::{self, OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};
use super::super::super::{DocxError, MAIN_DOCUMENT_CONTENT_TYPE, MAIN_DOCUMENT_PART, REL_TYPE_STYLES, STRICT_REL_TYPE_OFFICE_DOCUMENT, STYLES_CONTENT_TYPE, STYLES_PART, STYLES_REL_TARGET, W_NS};

//#region 🔖️XmlHelpers
fn elem(name: &str, attrs: Vec<XmlAttr>, children: Vec<XmlNode>) -> XmlNode {
    XmlNode::Element { name: name.into(), attrs, children }
}

fn attr(name: &str, value: &str) -> XmlAttr {
    XmlAttr { name: name.into(), value: value.into() }
}
//#endregion 🔖️XmlHelpers

//#region 🔖️RunMapping
fn run_to_xml(r: &DocxRun) -> XmlNode {
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
fn paragraph_to_xml(p: &DocxParagraph) -> XmlNode {
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
fn cell_to_xml(c: &DocxTableCell) -> XmlNode {
    let mut children = Vec::new();
    if !c.extra_cell_properties.is_empty() {
        children.push(elem("w:tcPr", vec![], c.extra_cell_properties.clone()));
    }
    children.extend(c.blocks.iter().map(block_to_xml));
    elem("w:tc", vec![], children)
}

fn row_to_xml(r: &DocxTableRow) -> XmlNode {
    let mut children = Vec::new();
    if !r.extra_row_properties.is_empty() {
        children.push(elem("w:trPr", vec![], r.extra_row_properties.clone()));
    }
    children.extend(r.cells.iter().map(cell_to_xml));
    elem("w:tr", vec![], children)
}

fn table_to_xml(t: &DocxTable) -> XmlNode {
    let mut children = Vec::new();
    if !t.extra_table_properties.is_empty() {
        children.push(elem("w:tblPr", vec![], t.extra_table_properties.clone()));
    }
    children.extend(t.rows.iter().map(row_to_xml));
    elem("w:tbl", vec![], children)
}
//#endregion 🔖️TableMapping

//#region 🔖️BlockMapping
fn block_to_xml(b: &DocxBlock) -> XmlNode {
    match b {
        DocxBlock::Paragraph(p) => paragraph_to_xml(p),
        DocxBlock::Table(t) => table_to_xml(t),
    }
}
//#endregion 🔖️BlockMapping

//#region 🔖️DocumentMapping
pub fn document_to_xml(doc: &DocxDocument) -> XmlDocument {
    let body_children = doc.body.iter().map(block_to_xml).collect();
    XmlDocument {
        root: Some(elem("w:document", vec![attr("xmlns:w", W_NS)], vec![elem("w:body", vec![], body_children)])),
        doctype: None,
        declaration: None,
        prolog: Vec::new(),
    }
}
//#endregion 🔖️DocumentMapping

//#region 🔖️StylesMapping
const STYLES_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

fn styles_to_xml(styles: &[DocxStyle]) -> XmlDocument {
    let children = styles
        .iter()
        .map(|s| {
            let mut sc = vec![elem("w:name", vec![attr("w:val", &s.name)], vec![])];
            if let Some(based_on) = &s.based_on {
                sc.push(elem("w:basedOn", vec![attr("w:val", based_on)], vec![]));
            }
            elem("w:style", vec![attr("w:styleId", &s.id)], sc)
        })
        .collect();
    XmlDocument {
        root: Some(elem("w:styles", vec![attr("xmlns:w", STYLES_NS)], children)),
        doctype: None,
        declaration: None,
        prolog: Vec::new(),
    }
}
//#endregion 🔖️StylesMapping

//#region 🔖️Codec
/// 🏗️ Assembles a brand-new, minimal-but-valid OPC package around `document` — correct
/// `[Content_Types].xml`, a root `_rels/.rels` pointing at `word/document.xml`, and the
/// serialized parts themselves (`word/styles.xml` too, when `document.styles` is non-empty). Real
/// Office/LibreOffice-shaped readers accept this container.
pub fn build_minimal_docx(document: DocxDocument) -> DocxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    let bytes = xml_document_to_text(&document_to_xml(&document)).into_bytes();
    opc.set_part(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE, bytes);
    opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
    if !document.styles.is_empty() {
        let styles_bytes = xml_document_to_text(&styles_to_xml(&document.styles)).into_bytes();
        opc.set_part(STYLES_PART, STYLES_CONTENT_TYPE, styles_bytes);
        opc.add_relationship(MAIN_DOCUMENT_PART, "rId2", REL_TYPE_STYLES, STYLES_REL_TARGET);
    }
    DocxSnapshot::from_parts(opc, document)
}

/// 🔄️ Syncs `snap.opc`'s `word/document.xml` (and `word/styles.xml`, when styles are present) part
/// bytes -- and their relationships, if missing -- from `snap.document`. The same materialization
/// `encode_docx` always performs before writing real bytes. Exposed so a builder can call it
/// BEFORE running a subset's conformance check on the still-in-memory snapshot (a check like
/// `✳️transitional`'s needs a materialized main part to find at all — see its own builder's doc
/// comment for why).
pub fn sync_main_part(snap: &mut DocxSnapshot) {
    let bytes = xml_document_to_text(&document_to_xml(&snap.document)).into_bytes();
    let content_type = snap.opc.content_types.resolve(MAIN_DOCUMENT_PART).map(str::to_string).unwrap_or_else(|| MAIN_DOCUMENT_CONTENT_TYPE.into());
    snap.opc.set_part(MAIN_DOCUMENT_PART, &content_type, bytes);
    let has_office_document_rel = snap.opc.relationships_for("").iter().any(|r| r.rel_type == REL_TYPE_OFFICE_DOCUMENT || r.rel_type == STRICT_REL_TYPE_OFFICE_DOCUMENT);
    if !has_office_document_rel {
        snap.opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
    }
    if !snap.document.styles.is_empty() {
        let styles_bytes = xml_document_to_text(&styles_to_xml(&snap.document.styles)).into_bytes();
        let styles_content_type = snap.opc.content_types.resolve(STYLES_PART).map(str::to_string).unwrap_or_else(|| STYLES_CONTENT_TYPE.into());
        snap.opc.set_part(STYLES_PART, &styles_content_type, styles_bytes);
        let has_styles_rel = snap.opc.relationships_for(MAIN_DOCUMENT_PART).iter().any(|r| r.rel_type == REL_TYPE_STYLES);
        if !has_styles_rel {
            snap.opc.add_relationship(MAIN_DOCUMENT_PART, "rId2", REL_TYPE_STYLES, STYLES_REL_TARGET);
        }
    }
}

pub fn encode_docx(snap: &DocxSnapshot) -> Result<Vec<u8>, DocxError> {
    let mut synced = snap.clone();
    sync_main_part(&mut synced);
    Ok(opc::encode_opc(&synced.opc)?)
}
//#endregion 🔖️Codec
