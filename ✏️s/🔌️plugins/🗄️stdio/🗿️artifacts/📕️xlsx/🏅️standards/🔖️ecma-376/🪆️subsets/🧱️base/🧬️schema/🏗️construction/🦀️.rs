//! 🏗️ Typed SpreadsheetML document and package construction.

use crate::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook, XlsxXmlPart};
use crate::schema::vocabulary::*;
use crate::XlsxSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument, XmlNode};
use semio_s_artifact_stdio_zip::opc::{OpcPackage, OpcRelationship, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT};

fn text_children(text: &str) -> Vec<XmlNode> {
    if text.is_empty() { Vec::new() } else { vec![XmlNode::Text { text: text.into() }] }
}

//#region 🔖️SharedStringsXml
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sst_to_xml(shared: &[String], reference_count: usize) -> XmlDocument {
    let children =
        shared.iter().map(|s| XmlNode::Element { name: "si".into(), attrs: vec![], children: vec![XmlNode::Element { name: "t".into(), attrs: vec![attr("xml:space", "preserve")], children: text_children(s) }] }).collect();
    XmlDocument {
        root: Some(XmlNode::Element { name: "sst".into(), attrs: vec![attr("xmlns", SML_NS), attr("count", &reference_count.to_string()), attr("uniqueCount", &shared.len().to_string())], children }),
        doctype: None,
        declaration: None,
        prolog: Vec::new(),
        epilog: Vec::new(),
    }
}
//#endregion 🔖️SharedStringsXml

//#region 🔖️WorkbookXml
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn workbook_to_xml(workbook: &XlsxWorkbook, rids: &[String]) -> XmlDocument {
    let sheets = workbook
        .sheets
        .iter()
        .zip(rids.iter())
        .enumerate()
        .map(|(i, (sheet, rid))| XmlNode::Element { name: "sheet".into(), attrs: vec![attr("name", &sheet.name), attr("sheetId", &(i + 1).to_string()), attr("r:id", rid)], children: vec![] })
        .collect();
    XmlDocument {
        root: Some(XmlNode::Element { name: "workbook".into(), attrs: vec![attr("xmlns", SML_NS), attr("xmlns:r", R_NS)], children: vec![XmlNode::Element { name: "sheets".into(), attrs: vec![], children: sheets }] }),
        doctype: None,
        declaration: None,
        prolog: Vec::new(),
        epilog: Vec::new(),
    }
}
//#endregion 🔖️WorkbookXml

//#region 🔖️WorksheetXml
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn v_element(text: &str) -> XmlNode {
    XmlNode::Element { name: "v".into(), attrs: vec![], children: text_children(text) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_element(text: &str) -> XmlNode {
    XmlNode::Element { name: "is".into(), attrs: vec![], children: vec![XmlNode::Element { name: "t".into(), attrs: vec![attr("xml:space", "preserve")], children: text_children(text) }] }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn f_element(expr: &str) -> XmlNode {
    XmlNode::Element { name: "f".into(), attrs: vec![], children: text_children(expr) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn format_number(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        n.to_string()
    }
}

/// 🔎️ Renders a CACHED formula value (the `<v>`/`t` pair that follows `<f>expr</f>`, if any) —
/// mirrors `cell_to_xml`'s own top-level match, but never itself recurses into `Formula` (a
/// formula's cached value is never itself a formula in a spec-conformant document).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cached_value_xml(cached: &XlsxCellValue) -> (Option<semio_s_artifact_stdio_xml::schema::snapshot::XmlAttr>, Option<XmlNode>) {
    match cached {
        XlsxCellValue::Number(n) => (None, Some(v_element(&format_number(*n)))),
        XlsxCellValue::SharedString(idx) => (Some(attr("t", "s")), Some(v_element(&idx.to_string()))),
        XlsxCellValue::InlineString(s) => (Some(attr("t", "str")), Some(v_element(s))),
        XlsxCellValue::Boolean(b) => (Some(attr("t", "b")), Some(v_element(if *b { "1" } else { "0" }))),
        XlsxCellValue::Error(error) => (Some(attr("t", "e")), Some(v_element(error))),
        XlsxCellValue::Formula { .. } => (None, None),
        XlsxCellValue::Empty => (None, None),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cell_to_xml(cell: &XlsxCell) -> XmlNode {
    let r = format!("{}{}", column_letter(cell.col), cell.row);
    let mut attrs = vec![attr("r", &r)];
    match &cell.value {
        XlsxCellValue::Number(n) => XmlNode::Element { name: "c".into(), attrs, children: vec![v_element(&format_number(*n))] },
        XlsxCellValue::SharedString(idx) => {
            attrs.push(attr("t", "s"));
            XmlNode::Element { name: "c".into(), attrs, children: vec![v_element(&idx.to_string())] }
        }
        XlsxCellValue::InlineString(s) => {
            attrs.push(attr("t", "inlineStr"));
            XmlNode::Element { name: "c".into(), attrs, children: vec![is_element(s)] }
        }
        XlsxCellValue::Boolean(b) => {
            attrs.push(attr("t", "b"));
            XmlNode::Element { name: "c".into(), attrs, children: vec![v_element(if *b { "1" } else { "0" })] }
        }
        XlsxCellValue::Error(error) => {
            attrs.push(attr("t", "e"));
            XmlNode::Element { name: "c".into(), attrs, children: vec![v_element(error)] }
        }
        XlsxCellValue::Formula { expr, cached } => {
            let mut children = vec![f_element(expr)];
            if let Some(cached) = cached {
                let (t_attr, v_node) = cached_value_xml(cached);
                if let Some(t_attr) = t_attr {
                    attrs.push(t_attr);
                }
                if let Some(v_node) = v_node {
                    children.push(v_node);
                }
            }
            XmlNode::Element { name: "c".into(), attrs, children }
        }
        XlsxCellValue::Empty => XmlNode::Element { name: "c".into(), attrs, children: vec![] },
    }
}

/// 🌳 Groups `sheet.cells` (sparse, unordered `(row, col)` pairs) into SpreadsheetML's required
/// `<row>`-then-`<c>` nesting, sorted ascending on both axes (spec order, and needed for
/// deterministic bytes).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn worksheet_to_xml_with_namespace(sheet: &XlsxSheet, namespace: &str) -> XmlDocument {
    let mut by_row: std::collections::BTreeMap<u32, Vec<&XlsxCell>> = std::collections::BTreeMap::new();
    for cell in &sheet.cells {
        by_row.entry(cell.row).or_default().push(cell);
    }
    let rows = by_row
        .into_iter()
        .map(|(row_index, mut cells)| {
            cells.sort_by_key(|c| c.col);
            let cell_nodes = cells.iter().map(|c| cell_to_xml(c)).collect();
            XmlNode::Element { name: "row".into(), attrs: vec![attr("r", &row_index.to_string())], children: cell_nodes }
        })
        .collect();
    XmlDocument {
        root: Some(XmlNode::Element { name: "worksheet".into(), attrs: vec![attr("xmlns", namespace)], children: vec![XmlNode::Element { name: "sheetData".into(), attrs: vec![], children: rows }] }),
        doctype: None,
        declaration: None,
        prolog: Vec::new(),
        epilog: Vec::new(),
    }
}

fn worksheet_to_xml(sheet: &XlsxSheet) -> XmlDocument {
    worksheet_to_xml_with_namespace(sheet, SML_NS)
}
//#endregion 🔖️WorksheetXml

/// 🔗️ Builds workbook relationships for the new typed worksheet collection and string table.
fn workbook_relationships(sheet_count: usize) -> (Vec<String>, Vec<OpcRelationship>) {
    let rids: Vec<String> = (1..=sheet_count).map(|index| format!("rId{index}")).collect();
    let mut relationships: Vec<OpcRelationship> = rids.iter().enumerate().map(|(index, id)| OpcRelationship {
        id: id.clone(), rel_type: REL_TYPE_WORKSHEET.into(), target: format!("worksheets/sheet{}.xml", index + 1), target_mode: OpcTargetMode::Internal,
    }).collect();
    relationships.push(OpcRelationship { id: format!("rId{}", sheet_count + 1), rel_type: REL_TYPE_SHARED_STRINGS.into(), target: "sharedStrings.xml".into(), target_mode: OpcTargetMode::Internal });
    (rids, relationships)
}

//#region 🔖️Codec
/// 🧱️ Builds logical XML parts and package metadata directly from typed workbook data.
pub fn build_minimal_xlsx(workbook: XlsxWorkbook) -> XlsxSnapshot {
    let mut opc = OpcPackage::empty();
    opc.content_types.set_default("rels", semio_s_artifact_stdio_zip::opc::RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");
    let (rids, relationships) = workbook_relationships(workbook.sheets.len());
    opc.relationships.replace_owner(WORKBOOK_PART.into(), relationships);
    opc.add_generated_relationship("", REL_TYPE_OFFICE_DOCUMENT, WORKBOOK_PART);
    let reference_count = workbook.sheets.iter().flat_map(|sheet| &sheet.cells).filter(|cell| match &cell.value {
        XlsxCellValue::SharedString(_) => true,
        XlsxCellValue::Formula { cached: Some(value), .. } => matches!(value.as_ref(), XlsxCellValue::SharedString(_)),
        _ => false,
    }).count();
    let mut xml_parts = vec![
        XlsxXmlPart { path: WORKBOOK_PART.into(), content_type: WORKBOOK_CONTENT_TYPE.into(), document: workbook_to_xml(&workbook, &rids) },
        XlsxXmlPart { path: SHARED_STRINGS_PART.into(), content_type: SHARED_STRINGS_CONTENT_TYPE.into(), document: sst_to_xml(&workbook.shared_strings, reference_count) },
    ];
    for (index, sheet) in workbook.sheets.iter().enumerate() {
        xml_parts.push(XlsxXmlPart { path: format!("xl/worksheets/sheet{}.xml", index + 1), content_type: WORKSHEET_CONTENT_TYPE.into(), document: worksheet_to_xml(sheet) });
    }
    for part in &xml_parts {
        opc.content_types.set_override(&part.path, &part.content_type);
    }
    xml_parts.sort_by(|left, right| left.path.cmp(&right.path));
    XlsxSnapshot::from_parts(opc, xml_parts)
}
