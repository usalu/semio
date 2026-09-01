//! 🧵️ WordprocessingML (docx) export — `DocxDocument` → `word/document.xml`/`word/styles.xml`
//! XML render, and the OPC package assembly/sync around it. Zip/OPC/XML byte-level work is never
//! reimplemented here: it is reused from the shared `crate::artifacts::zip::opc` layer and,
//! transitively, `crate::artifacts::zip::engine` + `crate::artifacts::xml::schema::snapshot`.

use super::super::super::{DocxError, MAIN_DOCUMENT_CONTENT_TYPE, MAIN_DOCUMENT_PART, REL_TYPE_STYLES, STRICT_REL_TYPE_OFFICE_DOCUMENT, STRICT_REL_TYPE_STYLES, STYLES_CONTENT_TYPE, STYLES_PART, STYLES_REL_TARGET, W_NS};
use crate::artifacts::docx::{
    schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow},
    DocxSnapshot,
};
use crate::artifacts::xml::schema::snapshot::{xml_document_to_text, XmlAttr, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::{self, OpcPackage, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};

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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn block_to_xml(b: &DocxBlock) -> XmlNode {
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
    XmlDocument { root: Some(elem("w:document", vec![attr("xmlns:w", W_NS)], vec![elem("w:body", vec![], body_children)])), doctype: None, declaration: None, prolog: Vec::new() }
}
//#endregion 🔖️DocumentMapping

//#region 🔖️StylesMapping
const STYLES_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn style_to_xml(s: &DocxStyle) -> XmlNode {
    let mut sc = vec![elem("w:name", vec![attr("w:val", &s.name)], vec![])];
    if let Some(based_on) = &s.based_on {
        sc.push(elem("w:basedOn", vec![attr("w:val", based_on)], vec![]));
    }
    elem("w:style", vec![attr("w:styleId", &s.id)], sc)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn styles_to_xml(styles: &[DocxStyle]) -> XmlDocument {
    let children = styles.iter().map(style_to_xml).collect();
    XmlDocument { root: Some(elem("w:styles", vec![attr("xmlns:w", STYLES_NS)], children)), doctype: None, declaration: None, prolog: Vec::new() }
}
//#endregion 🔖️StylesMapping

//#region 🔖️Codec
/// 🏗️ Assembles a brand-new, minimal-but-valid OPC package around `document` — correct
/// `[Content_Types].xml`, a root `_rels/.rels` pointing at `word/document.xml`, and the
/// serialized parts themselves (`word/styles.xml` too, when `document.styles` is non-empty). Real
/// Office/LibreOffice-shaped readers accept this container.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

/// 🔬️ Whether the part currently at `path` already decodes to exactly `expected`. A part whose own
/// bytes still project to the typed view MUST NOT be rewritten: `DocxDocument` is a semantic VIEW of
/// `word/document.xml`, not its total content, so re-rendering an unchanged part from the view is a
/// pure loss — it discards the root element's real attributes (`w:document`'s ECMA-376
/// `conformance`, a Strict `xmlns:w`, `mc:Ignorable`), the XML declaration, and every `w:body` child
/// this vocabulary does not model. That loss is what made the ✳️transitional conformance-class
/// vocabulary report `set-conformance-attribute` as applied while the written package carried no
/// such attribute. Same guard `pptx`'s `encode_pptx` already applies to `ppt/presentation.xml`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part_already_projects<T: PartialEq>(snap: &DocxSnapshot, path: &str, expected: &T, project: impl Fn(&XmlDocument) -> Option<T>) -> bool {
    let Some(part) = snap.opc.part(path) else { return false };
    let Ok(text) = std::str::from_utf8(&part.bytes) else { return false };
    let Ok(document) = crate::artifacts::xml::schema::snapshot::xml_document_from_text(text) else { return false };
    project(&document).is_some_and(|actual| &actual == expected)
}

/// 🧬️ Renders the typed body INTO the main part's OWN xml shape: root element name and attributes,
/// declaration, doctype, prolog and every `w:body` child that is neither `w:p` nor `w:tbl`
/// (`w:sectPr` above all) come from the package that was READ, never from constants. Only the
/// `w:p`/`w:tbl` sequence — the exact span `DocxDocument::body` is the view of — is regenerated.
/// Falls back to a freshly built `w:document` when there is no readable main part yet (the
/// `build_minimal_docx` path).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn document_into_part(existing: Option<&XmlDocument>, doc: &DocxDocument) -> XmlDocument {
    let rendered: Vec<XmlNode> = doc.body.iter().map(block_to_xml).collect();
    let Some(existing) = existing else { return document_to_xml(doc) };
    let Some(XmlNode::Element { name: root_name, attrs: root_attrs, children }) = existing.root.as_ref() else { return document_to_xml(doc) };
    if root_name != "w:document" {
        return document_to_xml(doc);
    }
    let mut root_children = Vec::with_capacity(children.len());
    let mut wrote_body = false;
    for child in children {
        match child {
            XmlNode::Element { name, attrs, children: body_children } if name == "w:body" => {
                let unmodeled = body_children.iter().filter(|node| !matches!(node, XmlNode::Element { name, .. } if name == "w:p" || name == "w:tbl")).cloned();
                let mut merged = rendered.clone();
                merged.extend(unmodeled);
                root_children.push(XmlNode::Element { name: name.clone(), attrs: attrs.clone(), children: merged });
                wrote_body = true;
            }
            other => root_children.push(other.clone()),
        }
    }
    if !wrote_body {
        root_children.push(elem("w:body", vec![], rendered));
    }
    XmlDocument { root: Some(XmlNode::Element { name: root_name.clone(), attrs: root_attrs.clone(), children: root_children }), doctype: existing.doctype.clone(), declaration: existing.declaration.clone(), prolog: existing.prolog.clone() }
}

/// 🧬️ Same principle as [`document_into_part`] for `word/styles.xml`: the root element, the prolog
/// and every non-`w:style` child (`w:docDefaults`, `w:latentStyles`) are the read package's, and a
/// style the model still carries keeps its OWN real definition — only `w:name`/`w:basedOn`, the two
/// fields `DocxStyle` is the view of, are written back onto it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn styles_into_part(existing: Option<&XmlDocument>, styles: &[DocxStyle]) -> XmlDocument {
    let Some(existing) = existing else { return styles_to_xml(styles) };
    let Some(XmlNode::Element { name: root_name, attrs: root_attrs, children }) = existing.root.as_ref() else { return styles_to_xml(styles) };
    if root_name != "w:styles" {
        return styles_to_xml(styles);
    }
    let mut root_children: Vec<XmlNode> = children.iter().filter(|node| !matches!(node, XmlNode::Element { name, .. } if name == "w:style")).cloned().collect();
    for style in styles {
        let prior = children.iter().find(|node| matches!(node, XmlNode::Element { name, attrs, .. } if name == "w:style" && attrs.iter().any(|a| a.name == "w:styleId" && a.value == style.id)));
        root_children.push(match prior {
            Some(XmlNode::Element { name, attrs, children: inner }) => {
                let mut merged: Vec<XmlNode> = inner.iter().filter(|node| !matches!(node, XmlNode::Element { name, .. } if name == "w:name" || name == "w:basedOn")).cloned().collect();
                merged.insert(0, elem("w:name", vec![attr("w:val", &style.name)], vec![]));
                if let Some(based_on) = &style.based_on {
                    merged.insert(1, elem("w:basedOn", vec![attr("w:val", based_on)], vec![]));
                }
                XmlNode::Element { name: name.clone(), attrs: attrs.clone(), children: merged }
            }
            _ => style_to_xml(style),
        });
    }
    XmlDocument { root: Some(XmlNode::Element { name: root_name.clone(), attrs: root_attrs.clone(), children: root_children }), doctype: existing.doctype.clone(), declaration: existing.declaration.clone(), prolog: existing.prolog.clone() }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parsed_part(snap: &DocxSnapshot, path: &str) -> Option<XmlDocument> {
    let part = snap.opc.part(path)?;
    let text = std::str::from_utf8(&part.bytes).ok()?;
    crate::artifacts::xml::schema::snapshot::xml_document_from_text(text).ok()
}

/// 🔄️ Syncs `snap.opc`'s `word/document.xml` (and `word/styles.xml`, when styles are present) part
/// bytes -- and their relationships, if missing -- from `snap.document`. The same materialization
/// `encode_docx` always performs before writing real bytes. Exposed so a builder can call it
/// BEFORE running a subset's conformance check on the still-in-memory snapshot (a check like
/// `✳️transitional`'s needs a materialized main part to find at all — see its own builder's doc
/// comment for why).
///
/// 🩹 A part that ALREADY projects to the typed view is left byte-for-byte alone (see
/// [`part_already_projects`]), and a part that does have to be rewritten keeps its own root element
/// and prolog (see [`document_into_part`]/[`styles_into_part`]). Both halves exist for one reason:
/// this snapshot's `opc` is the authority on everything `DocxDocument` does not model, and a writer
/// may never spend that authority to re-render markup nothing asked it to change.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sync_main_part(snap: &mut DocxSnapshot) {
    if !part_already_projects(snap, MAIN_DOCUMENT_PART, &snap.document.body, |document| crate::artifacts::docx::standards::v_ecma_376::subsets::base::io::import::deserializers::document_from_xml(document).ok()) {
        let bytes = xml_document_to_text(&document_into_part(parsed_part(snap, MAIN_DOCUMENT_PART).as_ref(), &snap.document)).into_bytes();
        let content_type = snap.opc.content_types.resolve(MAIN_DOCUMENT_PART).map(str::to_string).unwrap_or_else(|| MAIN_DOCUMENT_CONTENT_TYPE.into());
        snap.opc.set_part(MAIN_DOCUMENT_PART, &content_type, bytes);
    }
    let has_office_document_rel = snap.opc.relationships_for("").iter().any(|r| r.rel_type == REL_TYPE_OFFICE_DOCUMENT || r.rel_type == STRICT_REL_TYPE_OFFICE_DOCUMENT);
    if !has_office_document_rel {
        snap.opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
    }
    if !snap.document.styles.is_empty() {
        if !part_already_projects(snap, STYLES_PART, &snap.document.styles, |document| crate::artifacts::docx::standards::v_ecma_376::subsets::base::io::import::deserializers::styles_from_xml(document).ok()) {
            let styles_bytes = xml_document_to_text(&styles_into_part(parsed_part(snap, STYLES_PART).as_ref(), &snap.document.styles)).into_bytes();
            let styles_content_type = snap.opc.content_types.resolve(STYLES_PART).map(str::to_string).unwrap_or_else(|| STYLES_CONTENT_TYPE.into());
            snap.opc.set_part(STYLES_PART, &styles_content_type, styles_bytes);
        }
        let has_styles_rel = snap.opc.relationships_for(MAIN_DOCUMENT_PART).iter().any(|r| r.rel_type == REL_TYPE_STYLES || r.rel_type == STRICT_REL_TYPE_STYLES);
        if !has_styles_rel {
            snap.opc.add_relationship(MAIN_DOCUMENT_PART, "rId2", REL_TYPE_STYLES, STYLES_REL_TARGET);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_docx(snap: &DocxSnapshot) -> Result<Vec<u8>, DocxError> {
    let mut synced = snap.clone();
    sync_main_part(&mut synced);
    Ok(opc::encode_opc_with_package_order(&synced.opc)?)
}
//#endregion 🔖️Codec
