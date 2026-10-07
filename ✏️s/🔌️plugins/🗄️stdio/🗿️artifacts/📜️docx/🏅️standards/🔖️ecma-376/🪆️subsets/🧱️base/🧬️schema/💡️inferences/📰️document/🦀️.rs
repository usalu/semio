//! 📰️ Projects semantic DOCX document views from owned XML nodes.
use crate::standards::v_ecma_376::subsets::base::schema::{snapshot::*,vocabulary::*,refusal::DocxError,namespaces::{is_word_name,scoped_bindings,word_attr,word_local_name}};
use crate::DocxSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument,XmlNode};
use semio_s_artifact_stdio_zip::opc::REL_TYPE_OFFICE_DOCUMENT;
fn child_elements(node: &XmlNode) -> &[XmlNode] {
    match node {
        XmlNode::Element { children, .. } => children.as_slice(),
        _ => &[],
    }
}

fn run_from_xml(node: &XmlNode, bindings: &[(String, String)]) -> DocxRun {
    let mut run = DocxRun::default();
    for child in child_elements(node) {
        let child_bindings = scoped_bindings(child, bindings);
        match word_local_name(child, &child_bindings) {
            Some("rPr") => {
                for prop in child_elements(child) {
                    let prop_bindings = scoped_bindings(prop, &child_bindings);
                    let enabled = !matches!(word_attr(prop, "val", &prop_bindings), Some("0" | "false" | "off" | "none"));
                    match word_local_name(prop, &prop_bindings) {
                        Some("b") => run.bold = enabled,
                        Some("i") => run.italic = enabled,
                        Some("u") => run.underline = enabled,
                        _ => run.extra_run_properties.push(prop.clone()),
                    }
                }
            }
            Some("t") => {
                for text_node in child_elements(child) {
                    if let XmlNode::Text { text } | XmlNode::CData { text } = text_node {
                        run.text.push_str(text);
                    }
                }
            }
            _ => {}
        }
    }
    run
}

fn collect_runs(node: &XmlNode, parent_bindings: &[(String, String)], runs: &mut Vec<DocxRun>) {
    let bindings = scoped_bindings(node, parent_bindings);
    if is_word_name(node, "r", &bindings) {
        runs.push(run_from_xml(node, &bindings));
        return;
    }
    for child in child_elements(node) {
        collect_runs(child, &bindings, runs);
    }
}

fn paragraph_from_xml(node: &XmlNode, bindings: &[(String, String)]) -> DocxParagraph {
    let mut paragraph = DocxParagraph::default();
    for child in child_elements(node) {
        let child_bindings = scoped_bindings(child, bindings);
        if is_word_name(child, "pPr", &child_bindings) {
            for prop in child_elements(child) {
                let prop_bindings = scoped_bindings(prop, &child_bindings);
                if is_word_name(prop, "pStyle", &prop_bindings) {
                    paragraph.style = word_attr(prop, "val", &prop_bindings).map(str::to_string);
                } else {
                    paragraph.extra_paragraph_properties.push(prop.clone());
                }
            }
        } else {
            collect_runs(child, bindings, &mut paragraph.runs);
        }
    }
    paragraph
}

fn cell_from_xml(node: &XmlNode, bindings: &[(String, String)]) -> DocxTableCell {
    let mut cell = DocxTableCell::default();
    for child in child_elements(node) {
        let child_bindings = scoped_bindings(child, bindings);
        match word_local_name(child, &child_bindings) {
            Some("tcPr") => cell.extra_cell_properties = child_elements(child).to_vec(),
            Some("p") => cell.blocks.push(DocxBlock::Paragraph(paragraph_from_xml(child, &child_bindings))),
            Some("tbl") => cell.blocks.push(DocxBlock::Table(table_from_xml(child, &child_bindings))),
            _ => {}
        }
    }
    cell
}

fn row_from_xml(node: &XmlNode, bindings: &[(String, String)]) -> DocxTableRow {
    let mut row = DocxTableRow::default();
    for child in child_elements(node) {
        let child_bindings = scoped_bindings(child, bindings);
        match word_local_name(child, &child_bindings) {
            Some("trPr") => row.extra_row_properties = child_elements(child).to_vec(),
            Some("tc") => row.cells.push(cell_from_xml(child, &child_bindings)),
            _ => {}
        }
    }
    row
}

fn table_from_xml(node: &XmlNode, bindings: &[(String, String)]) -> DocxTable {
    let mut table = DocxTable::default();
    for child in child_elements(node) {
        let child_bindings = scoped_bindings(child, bindings);
        match word_local_name(child, &child_bindings) {
            Some("tblPr") => table.extra_table_properties = child_elements(child).to_vec(),
            Some("tr") => table.rows.push(row_from_xml(child, &child_bindings)),
            _ => {}
        }
    }
    table
}

/// 📖️ Reads WordprocessingML blocks using in-scope namespaces and direct run properties.
pub fn document_from_xml(doc: &XmlDocument) -> Result<Vec<DocxBlock>, DocxError> {
    let bad = |detail: &str| DocxError::Xml { part: MAIN_DOCUMENT_PART.into(), detail: detail.into() };
    let root = doc.root.as_ref().ok_or_else(|| bad("empty document"))?;
    let bindings = scoped_bindings(root, &[]);
    if !is_word_name(root, "document", &bindings) {
        return Err(bad("expected WordprocessingML document root"));
    }
    let (body, body_bindings) = child_elements(root)
        .iter()
        .find_map(|child| {
            let child_bindings = scoped_bindings(child, &bindings);
            is_word_name(child, "body", &child_bindings).then_some((child, child_bindings))
        })
        .ok_or_else(|| bad("missing WordprocessingML body"))?;
    let mut blocks = Vec::new();
    for node in child_elements(body) {
        let bindings = scoped_bindings(node, &body_bindings);
        match word_local_name(node, &bindings) {
            Some("p") => blocks.push(DocxBlock::Paragraph(paragraph_from_xml(node, &bindings))),
            Some("tbl") => blocks.push(DocxBlock::Table(table_from_xml(node, &bindings))),
            _ => {}
        }
    }
    Ok(blocks)
}

/// 🎨️ Reads style identities without treating default or foreign attributes as WordprocessingML.
pub fn styles_from_xml(doc: &XmlDocument) -> Result<Vec<DocxStyle>, DocxError> {
    let bad = |detail: &str| DocxError::Xml { part: STYLES_PART.into(), detail: detail.into() };
    let Some(root) = doc.root.as_ref() else { return Ok(Vec::new()) };
    let bindings = scoped_bindings(root, &[]);
    if !is_word_name(root, "styles", &bindings) {
        return Err(bad("expected WordprocessingML styles root"));
    }
    let mut styles = Vec::new();
    for child in child_elements(root) {
        let child_bindings = scoped_bindings(child, &bindings);
        if !is_word_name(child, "style", &child_bindings) {
            continue;
        }
        let Some(id) = word_attr(child, "styleId", &child_bindings).filter(|id| !id.is_empty()) else { continue };
        let mut style_name = id.to_string();
        let mut based_on = None;
        for prop in child_elements(child) {
            let prop_bindings = scoped_bindings(prop, &child_bindings);
            match word_local_name(prop, &prop_bindings) {
                Some("name") => style_name = word_attr(prop, "val", &prop_bindings).unwrap_or(&style_name).to_string(),
                Some("basedOn") => based_on = word_attr(prop, "val", &prop_bindings).map(str::to_string),
                _ => {}
            }
        }
        styles.push(DocxStyle { id: id.to_string(), name: style_name, based_on });
    }
    Ok(styles)
}

//#region 🔖️Codec
/// 🧭️ Resolves the authoritative WordprocessingML main document part.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn main_document_path(opc: &semio_s_artifact_stdio_zip::opc::retained::RetainedOpcPackage) -> Result<String, DocxError> {
    opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).or_else(|| opc.resolve_relationship("", STRICT_REL_TYPE_OFFICE_DOCUMENT)).ok_or(DocxError::MissingMainDocumentRelationship)
}

/// 📰️ Projects the semantic document view from authoritative XML parts without mutating them.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn project_document(opc: &semio_s_artifact_stdio_zip::opc::retained::RetainedOpcPackage, xml_parts: &DocxXmlParts) -> Result<DocxDocument, DocxError> {
    let main_path = main_document_path(opc)?;
    let main = xml_parts.iter().find(|part| part.path == main_path).ok_or_else(|| DocxError::MissingPart(main_path.clone()))?;
    let styles_part = opc
        .resolve_relationship(&main_path, REL_TYPE_STYLES)
        .or_else(|| opc.resolve_relationship(&main_path, STRICT_REL_TYPE_STYLES))
        .and_then(|styles_path| xml_parts.iter().find(|part| part.path == styles_path));
    let owned_bytes = styles_part.map_or(Ok(main.document.materialization_owned_bytes()?), |styles| {
        main.document
            .materialization_owned_bytes()?
            .checked_add(styles.document.materialization_owned_bytes()?)
            .ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "DOCX semantic projection ownership overflow"))
    })?;
    let mut progress = |_| true;
    let mut control = semio_framework_value::NativeEncodeControl::new(owned_bytes, &mut progress);
    let main_document = main.materialize_document(&mut control)?;
    let body = document_from_xml(&main_document)?;
    let styles = match styles_part {
        Some(styles) => styles_from_xml(&styles.materialize_document(&mut control)?)?,
        None => Vec::new(),
    };
    Ok(DocxDocument { body, styles })
}

/// 📰️ Projects the semantic document view from one canonical snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn project_snapshot_document(snapshot: &DocxSnapshot) -> Result<DocxDocument, DocxError> {
    project_document(&snapshot.opc, &snapshot.xml_parts)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
