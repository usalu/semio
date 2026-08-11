//! ⚙️ WordprocessingML (docx) engine — real OPC container + `word/document.xml` block-tree
//! (paragraph/table) model + `word/styles.xml` named-style model. Zip/OPC/XML byte-level work is
//! never reimplemented here: it is reused from the shared `crate::artifacts::zip::opc` layer and,
//! transitively, `crate::artifacts::zip::engine` + `crate::artifacts::xml::schema::snapshot`.

use crate::artifacts::docx::{schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow}, DocxArtifact, DocxMutation, DocxSnapshot, STDIO_DOCX_DOCUMENT_SCHEMA};
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::{self, OpcPackage, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

/// 🏅️ ISO/IEC 29500-1 Strict's officeDocument relationship type (`✳️strict`'s
/// `STRICT_REL_BASE`/`officeDocument`) — decode must recognize this alongside the transitional
/// `REL_TYPE_OFFICE_DOCUMENT` above, since this `✳️any`-level decoder is shared by every subset
/// including `✳️strict`, which legitimately never uses the transitional relationship type.
const STRICT_REL_TYPE_OFFICE_DOCUMENT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument";

//#region 🔖️Error
/// ⚠️ Typed docx decode/encode failure — a package this engine cannot honestly interpret is
/// never fabricated into a partial/empty document.
#[derive(Clone, Debug, PartialEq)]
pub enum DocxError {
    Opc(opc::OpcError),
    MissingMainDocumentRelationship,
    MissingPart(String),
    Xml { part: String, detail: String },
    Malformed(String),
}

impl std::fmt::Display for DocxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opc(e) => write!(f, "docx: {e}"),
            Self::MissingMainDocumentRelationship => write!(f, "docx: package root has no officeDocument relationship"),
            Self::MissingPart(p) => write!(f, "docx: missing required part {p}"),
            Self::Xml { part, detail } => write!(f, "docx: xml in {part}: {detail}"),
            Self::Malformed(detail) => write!(f, "docx: {detail}"),
        }
    }
}

impl std::error::Error for DocxError {}

impl From<opc::OpcError> for DocxError {
    fn from(e: opc::OpcError) -> Self { Self::Opc(e) }
}
//#endregion 🔖️Error

//#region 🔖️Constants
const W_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const R_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const MAIN_DOCUMENT_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const MAIN_DOCUMENT_PART: &str = "word/document.xml";
const STYLES_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml";
const STYLES_PART: &str = "word/styles.xml";
/// 🧭️ The styles relationship's `Target`, RELATIVE TO ITS OWNER'S DIRECTORY (`word/`) per OPC
/// §9.3 -- NOT `STYLES_PART` verbatim, which is package-root-relative and would resolve (via
/// `resolve_relationship_target("word/document.xml", "word/styles.xml")`) to the wrong path
/// `word/word/styles.xml`. This is the OPC module's own documented "#1 relative-target gotcha".
const STYLES_REL_TARGET: &str = "styles.xml";
const REL_TYPE_STYLES: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
//#endregion 🔖️Constants

//#region 🔖️XmlHelpers
fn elem(name: &str, attrs: Vec<XmlAttr>, children: Vec<XmlNode>) -> XmlNode {
    XmlNode::Element { name: name.into(), attrs, children }
}

fn attr(name: &str, value: &str) -> XmlAttr {
    XmlAttr { name: name.into(), value: value.into() }
}

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
fn cell_to_xml(c: &DocxTableCell) -> XmlNode {
    let mut children = Vec::new();
    if !c.extra_cell_properties.is_empty() {
        children.push(elem("w:tcPr", vec![], c.extra_cell_properties.clone()));
    }
    children.extend(c.blocks.iter().map(block_to_xml));
    elem("w:tc", vec![], children)
}

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

fn row_to_xml(r: &DocxTableRow) -> XmlNode {
    let mut children = Vec::new();
    if !r.extra_row_properties.is_empty() {
        children.push(elem("w:trPr", vec![], r.extra_row_properties.clone()));
    }
    children.extend(r.cells.iter().map(cell_to_xml));
    elem("w:tr", vec![], children)
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

fn table_to_xml(t: &DocxTable) -> XmlNode {
    let mut children = Vec::new();
    if !t.extra_table_properties.is_empty() {
        children.push(elem("w:tblPr", vec![], t.extra_table_properties.clone()));
    }
    children.extend(t.rows.iter().map(row_to_xml));
    elem("w:tbl", vec![], children)
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

//#region 🔖️BlockMapping
fn block_to_xml(b: &DocxBlock) -> XmlNode {
    match b {
        DocxBlock::Paragraph(p) => paragraph_to_xml(p),
        DocxBlock::Table(t) => table_to_xml(t),
    }
}
//#endregion 🔖️BlockMapping

//#region 🔖️DocumentMapping
fn document_to_xml(doc: &DocxDocument) -> XmlDocument {
    let body_children = doc.body.iter().map(block_to_xml).collect();
    XmlDocument {
        root: Some(elem("w:document", vec![attr("xmlns:w", W_NS)], vec![elem("w:body", vec![], body_children)])),
        doctype: None,
        declaration: None,
    }
}

fn document_from_xml(doc: &XmlDocument) -> Result<Vec<DocxBlock>, DocxError> {
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
    }
}

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

pub fn decode_docx(data: &[u8]) -> Result<DocxSnapshot, DocxError> {
    let opc = opc::decode_opc(data)?;
    let main_path = opc
        .resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT)
        .or_else(|| opc.resolve_relationship("", STRICT_REL_TYPE_OFFICE_DOCUMENT))
        .ok_or(DocxError::MissingMainDocumentRelationship)?;
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

    let _ = R_NS; // documented namespace constant, not independently emitted (w: prefix carries relationship refs when needed)
    Ok(DocxSnapshot::from_parts(opc, DocxDocument { body, styles }))
}

pub fn empty_docx_snapshot() -> DocxSnapshot { DocxSnapshot::default() }

pub fn register() {
    crate::artifacts::docx::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::docx::schema::docx_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<DocxSnapshot, DocxMutation>(STDIO_DOCX_DOCUMENT_SCHEMA));
    // 🛡️ D5's generic validate-on-build hook: registers the ✳️strict/✳️transitional subsets'
    // SubsetValidators so `io_dispatch`/`wire_artifact_compose` re-check them for free. Each
    // ComposerEntry itself is registered separately via this standard's own `composer::entries()`
    // aggregation (called above via `crate::artifacts::docx::composer::register()`).
    crate::artifacts::docx::standards::v_ecma_376::subsets::strict::composer::register();
    crate::artifacts::docx::standards::v_ecma_376::subsets::transitional::composer::register();
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

//#region 🔖️ArtifactEngine
pub struct DocxEngine { artifact_state: DocxArtifact, snapshot_state: DocxSnapshot }
impl DocxEngine {
    pub fn new(snapshot: DocxSnapshot) -> Self {
        Self { artifact_state: DocxArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_document() -> DocxDocument {
        DocxDocument {
            body: vec![
                DocxBlock::Paragraph(DocxParagraph {
                    runs: vec![
                        DocxRun { text: "Hello, ".into(), bold: true, ..Default::default() },
                        DocxRun { text: "world!".into(), italic: true, ..Default::default() },
                    ],
                    style: None,
                    extra_paragraph_properties: Vec::new(),
                }),
                DocxBlock::paragraph("Second paragraph, plain."),
            ],
            styles: Vec::new(),
        }
    }

    fn sample_document_with_table_and_styles() -> DocxDocument {
        DocxDocument {
            body: vec![
                DocxBlock::Paragraph(DocxParagraph { style: Some("Heading1".into()), ..DocxParagraph::text("Title") }),
                DocxBlock::Table(DocxTable {
                    rows: vec![DocxTableRow {
                        cells: vec![
                            DocxTableCell { blocks: vec![DocxBlock::paragraph("R1C1")], ..Default::default() },
                            DocxTableCell { blocks: vec![DocxBlock::paragraph("R1C2")], ..Default::default() },
                        ],
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
            ],
            styles: vec![
                DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None },
                DocxStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) },
            ],
        }
    }

    #[test]
    fn builder_produces_minimal_valid_package_that_decodes_back() {
        let snap = build_minimal_docx(sample_document());
        let bytes = encode_docx(&snap).expect("encode minimal package");
        assert!(opc::sniff_opc_bytes(&bytes));
        assert!(sniff_docx_bytes(&bytes));
        let decoded = decode_docx(&bytes).expect("decode minimal package");
        assert_eq!(decoded.document, sample_document());
    }

    #[test]
    fn tables_and_styles_round_trip() {
        let snap = build_minimal_docx(sample_document_with_table_and_styles());
        let bytes = encode_docx(&snap).expect("encode");
        let decoded = decode_docx(&bytes).expect("decode");
        assert_eq!(decoded.document, sample_document_with_table_and_styles());
        let DocxBlock::Table(table) = &decoded.document.body[1] else { panic!("expected table") };
        assert_eq!(table.rows[0].cells.len(), 2);
    }

    #[test]
    fn decode_resolves_real_hand_built_package_with_formatting() {
        // Hand-built OOXML: correct Content_Types/.rels/part structure, not just "a zip with xml".
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        let xml = concat!(
            r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">"#,
            "<w:body>",
            r#"<w:p><w:r><w:rPr><w:b/></w:rPr><w:t xml:space="preserve">Bold run</w:t></w:r></w:p>"#,
            r#"<w:p><w:r><w:rPr><w:i/></w:rPr><w:t xml:space="preserve">Italic run</w:t></w:r></w:p>"#,
            r#"<w:p><w:r><w:t xml:space="preserve">Plain &amp; escaped</w:t></w:r></w:p>"#,
            "</w:body>",
            "</w:document>",
        );
        opc.set_part(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE, xml.as_bytes().to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
        let bytes = crate::artifacts::zip::opc::encode_opc(&opc).expect("encode opc");

        let decoded = decode_docx(&bytes).expect("decode hand-built docx");
        assert_eq!(decoded.document.body.len(), 3);
        let DocxBlock::Paragraph(p0) = &decoded.document.body[0] else { panic!("paragraph") };
        assert!(p0.runs[0].bold);
        let DocxBlock::Paragraph(p1) = &decoded.document.body[1] else { panic!("paragraph") };
        assert!(p1.runs[0].italic);
        let DocxBlock::Paragraph(p2) = &decoded.document.body[2] else { panic!("paragraph") };
        assert_eq!(p2.runs[0].text, "Plain & escaped");
    }

    #[test]
    fn unmodeled_parts_survive_decode_encode_verbatim() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE, xml_document_to_text(&document_to_xml(&sample_document())).into_bytes());
        opc.set_part("word/numbering.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml", b"<w:numbering/>".to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
        let bytes = crate::artifacts::zip::opc::encode_opc(&opc).expect("encode");

        let decoded = decode_docx(&bytes).expect("decode");
        assert_eq!(decoded.opc.part_bytes("word/numbering.xml"), Some(b"<w:numbering/>".as_slice()));
        let re_encoded = encode_docx(&decoded).expect("re-encode");
        let re_decoded = decode_docx(&re_encoded).expect("re-decode");
        assert_eq!(re_decoded.opc.part_bytes("word/numbering.xml"), Some(b"<w:numbering/>".as_slice()));
        assert_eq!(re_decoded.document, sample_document());
    }

    #[test]
    fn unmodeled_run_properties_survive_round_trip() {
        let mut run = DocxRun { text: "colored".into(), ..Default::default() };
        run.extra_run_properties.push(XmlNode::Element { name: "w:color".into(), attrs: vec![XmlAttr { name: "w:val".into(), value: "FF0000".into() }], children: vec![] });
        let doc = DocxDocument { body: vec![DocxBlock::Paragraph(DocxParagraph { runs: vec![run], style: None, extra_paragraph_properties: Vec::new() })], styles: Vec::new() };
        let snap = build_minimal_docx(doc.clone());
        let bytes = encode_docx(&snap).expect("encode");
        let decoded = decode_docx(&bytes).expect("decode");
        assert_eq!(decoded.document, doc);
    }

    #[test]
    fn decode_rejects_missing_main_document_relationship() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        let bytes = crate::artifacts::zip::opc::encode_opc(&opc).expect("encode");
        let err = decode_docx(&bytes).expect_err("must reject a package with no officeDocument relationship");
        assert_eq!(err, DocxError::MissingMainDocumentRelationship);
    }

    #[test]
    fn analyzer_builder_round_trip() {
        let original = build_minimal_docx(sample_document_with_table_and_styles());
        // Analyzer: real decode of the encoded bytes.
        let bytes = encode_docx(&original).expect("encode");
        let analyzed = decode_docx(&bytes).expect("decode");
        // Builder: reconstruct an equivalent document from the analyzed parts.
        let rebuilt = build_minimal_docx(analyzed.document.clone());
        let rebuilt_bytes = encode_docx(&rebuilt).expect("encode rebuilt");
        let reanalyzed = decode_docx(&rebuilt_bytes).expect("decode rebuilt");
        assert_eq!(reanalyzed.document, analyzed.document);
    }
}
//#endregion 🧪️Tests
