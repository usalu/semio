//! 🧩️ WordprocessingML (docx) import — `word/document.xml`/`word/styles.xml` XML parse into a
//! `DocxDocument`, real OPC package decode, and magic-shape sniff. Zip/OPC/XML byte-level work is
//! never reimplemented here: it is reused from the shared `crate::artifacts::zip::opc` layer and,
//! transitively, `crate::artifacts::zip::engine` + `crate::artifacts::xml::schema::snapshot`.

use super::super::super::{DocxError, MAIN_DOCUMENT_PART, REL_TYPE_STYLES, STRICT_REL_TYPE_OFFICE_DOCUMENT, STYLES_PART};
use crate::artifacts::docx::schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow};
use crate::artifacts::docx::DocxSnapshot;
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, XmlAttr, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::{self, REL_TYPE_OFFICE_DOCUMENT};

//#region 🔖️XmlHelpers
fn find_attr<'a>(attrs: &'a [XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}

fn child_elements<'a>(node: &'a XmlNode) -> &'a [XmlNode] {
    match node {
        XmlNode::Element { children, .. } => children.as_slice(),
        _ => &[],
    }
}
//#endregion 🔖️XmlHelpers

//#region 🔖️RunMapping
fn run_from_xml(node: &XmlNode) -> DocxRun {
    let mut run = DocxRun::default();
    for child in child_elements(node) {
        let XmlNode::Element { name, children: inner, .. } = child else { continue };
        match name.as_str() {
            "w:rPr" => {
                for prop in inner {
                    let XmlNode::Element { name, .. } = prop else { continue };
                    match name.as_str() {
                        "w:b" => run.bold = true,
                        "w:i" => run.italic = true,
                        "w:u" => run.underline = true,
                        _ => run.extra_run_properties.push(prop.clone()),
                    }
                }
            }
            "w:t" => {
                for t in inner {
                    if let XmlNode::Text { text } = t {
                        run.text.push_str(text);
                    }
                }
            }
            _ => {}
        }
    }
    run
}
//#endregion 🔖️RunMapping

//#region 🔖️ParagraphMapping
fn paragraph_from_xml(node: &XmlNode) -> DocxParagraph {
    let mut paragraph = DocxParagraph::default();
    for child in child_elements(node) {
        let XmlNode::Element { name, children: inner, attrs, .. } = child else { continue };
        match name.as_str() {
            "w:pPr" => {
                for prop in inner {
                    let XmlNode::Element { name, attrs: pattrs, .. } = prop else { continue };
                    if name == "w:pStyle" {
                        paragraph.style = find_attr(pattrs, "w:val").map(str::to_string);
                    } else {
                        paragraph.extra_paragraph_properties.push(prop.clone());
                    }
                }
            }
            "w:r" => paragraph.runs.push(run_from_xml(child)),
            _ => {
                let _ = attrs;
            }
        }
    }
    paragraph
}
//#endregion 🔖️ParagraphMapping

//#region 🔖️TableMapping
fn cell_from_xml(node: &XmlNode) -> DocxTableCell {
    let mut cell = DocxTableCell::default();
    for child in child_elements(node) {
        let XmlNode::Element { name, children: inner, .. } = child else { continue };
        match name.as_str() {
            "w:tcPr" => cell.extra_cell_properties = inner.clone(),
            "w:p" => cell.blocks.push(DocxBlock::Paragraph(paragraph_from_xml(child))),
            "w:tbl" => cell.blocks.push(DocxBlock::Table(table_from_xml(child))),
            _ => {}
        }
    }
    cell
}

fn row_from_xml(node: &XmlNode) -> DocxTableRow {
    let mut row = DocxTableRow::default();
    for child in child_elements(node) {
        let XmlNode::Element { name, children: inner, .. } = child else { continue };
        match name.as_str() {
            "w:trPr" => row.extra_row_properties = inner.clone(),
            "w:tc" => row.cells.push(cell_from_xml(child)),
            _ => {}
        }
    }
    row
}

fn table_from_xml(node: &XmlNode) -> DocxTable {
    let mut table = DocxTable::default();
    for child in child_elements(node) {
        let XmlNode::Element { name, children: inner, .. } = child else { continue };
        match name.as_str() {
            "w:tblPr" => table.extra_table_properties = inner.clone(),
            "w:tr" => table.rows.push(row_from_xml(child)),
            _ => {}
        }
    }
    table
}
//#endregion 🔖️TableMapping

//#region 🔖️DocumentMapping
pub fn document_from_xml(doc: &XmlDocument) -> Result<Vec<DocxBlock>, DocxError> {
    let bad = |detail: &str| DocxError::Xml { part: MAIN_DOCUMENT_PART.into(), detail: detail.into() };
    let root = doc.root.as_ref().ok_or_else(|| bad("empty document"))?;
    let XmlNode::Element { name, children, .. } = root else { return Err(bad("root is not an element")) };
    if name != "w:document" {
        return Err(DocxError::Xml { part: MAIN_DOCUMENT_PART.into(), detail: format!("expected <w:document>, got <{name}>") });
    }
    let body = children
        .iter()
        .find_map(|c| match c {
            XmlNode::Element { name, children, .. } if name == "w:body" => Some(children),
            _ => None,
        })
        .ok_or_else(|| bad("missing <w:body>"))?;

    let mut blocks = Vec::new();
    for node in body {
        let XmlNode::Element { name, .. } = node else { continue };
        match name.as_str() {
            "w:p" => blocks.push(DocxBlock::Paragraph(paragraph_from_xml(node))),
            "w:tbl" => blocks.push(DocxBlock::Table(table_from_xml(node))),
            _ => {}
        }
    }
    Ok(blocks)
}
//#endregion 🔖️DocumentMapping

//#region 🔖️StylesMapping
fn styles_from_xml(doc: &XmlDocument) -> Result<Vec<DocxStyle>, DocxError> {
    let bad = |detail: &str| DocxError::Xml { part: STYLES_PART.into(), detail: detail.into() };
    let Some(root) = doc.root.as_ref() else { return Ok(Vec::new()) };
    let XmlNode::Element { name, children, .. } = root else { return Err(bad("root is not an element")) };
    if name != "w:styles" {
        return Err(DocxError::Xml { part: STYLES_PART.into(), detail: format!("expected <w:styles>, got <{name}>") });
    }
    let mut styles = Vec::new();
    for child in children {
        let XmlNode::Element { name, attrs, children: inner } = child else { continue };
        if name != "w:style" {
            continue;
        }
        let id = find_attr(attrs, "w:styleId").unwrap_or_default().to_string();
        let mut style_name = id.clone();
        let mut based_on = None;
        for prop in inner {
            let XmlNode::Element { name, attrs: pattrs, .. } = prop else { continue };
            match name.as_str() {
                "w:name" => style_name = find_attr(pattrs, "w:val").unwrap_or(&style_name).to_string(),
                "w:basedOn" => based_on = find_attr(pattrs, "w:val").map(str::to_string),
                _ => {}
            }
        }
        styles.push(DocxStyle { id, name: style_name, based_on });
    }
    Ok(styles)
}
//#endregion 🔖️StylesMapping

//#region 🔖️Codec
pub fn decode_docx(data: &[u8]) -> Result<DocxSnapshot, DocxError> {
    let opc = opc::decode_opc(data)?;
    let main_path = opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).or_else(|| opc.resolve_relationship("", STRICT_REL_TYPE_OFFICE_DOCUMENT)).ok_or(DocxError::MissingMainDocumentRelationship)?;
    let bytes = opc.part_bytes(&main_path).ok_or_else(|| DocxError::MissingPart(main_path.clone()))?;
    let text = String::from_utf8(bytes.to_vec()).map_err(|_| DocxError::Xml { part: main_path.clone(), detail: "not valid utf-8".into() })?;
    let xml = xml_document_from_text(&text).map_err(|e| DocxError::Xml { part: main_path.clone(), detail: e })?;
    let body = document_from_xml(&xml)?;

    let styles = match opc.resolve_relationship(&main_path, REL_TYPE_STYLES).and_then(|p| opc.part_bytes(&p).map(|b| (p, b.to_vec()))) {
        Some((styles_path, styles_bytes)) => {
            let text = String::from_utf8(styles_bytes).map_err(|_| DocxError::Xml { part: styles_path.clone(), detail: "not valid utf-8".into() })?;
            let xml = xml_document_from_text(&text).map_err(|e| DocxError::Xml { part: styles_path.clone(), detail: e })?;
            styles_from_xml(&xml)?
        }
        None => Vec::new(),
    };

    Ok(DocxSnapshot::from_parts(opc, DocxDocument { body, styles }))
}
//#endregion 🔖️Codec

//#region 🔖️Sniff
/// 🕵️ Real docx sniff: OPC-shaped (real `[Content_Types].xml`) *and* the root officeDocument
/// relationship resolves to a part under `word/` — disambiguates from xlsx/pptx, which share the
/// same zip magic and OPC shape but point at `xl/`/`ppt/` instead.
pub fn sniff_docx_bytes(data: &[u8]) -> bool {
    let Ok(opc) = opc::decode_opc(data) else { return false };
    match opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT) {
        Some(path) => path.starts_with("word/"),
        None => false,
    }
}
//#endregion 🔖️Sniff
