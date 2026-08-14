//! 🧵️ SpreadsheetML (xlsx) export — `XlsxWorkbook` → `xl/workbook.xml`/`xl/worksheets/sheetN.xml`/
//! `xl/sharedStrings.xml` XML render, and the OPC package assembly/sync around it. Zip/OPC/XML
//! byte-level work is never reimplemented here: it is reused from the shared
//! `crate::artifacts::zip::opc` layer. Shared strings (`t="s"` cells reference an index into
//! `xl/sharedStrings.xml`) are decoded/encoded as an EXPLICIT `workbook.shared_strings` table —
//! never eagerly resolved into cell text — so the `t="s"` (shared-string reference) vs
//! `t="inlineStr"` (literal text) distinction the format itself makes survives round-trip, and a
//! diff over `shared_strings` means something (see `🧬️schema/🔺️diff`).

use crate::artifacts::xlsx::{schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook}, XlsxSnapshot};
use crate::artifacts::xml::schema::snapshot::{xml_document_to_text, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::{OpcPackage, OpcRelationship, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT};
use super::super::super::{attr, SHARED_STRINGS_CONTENT_TYPE, SHARED_STRINGS_PART, SML_NS, WORKBOOK_CONTENT_TYPE, WORKBOOK_PART, WORKSHEET_CONTENT_TYPE, REL_TYPE_SHARED_STRINGS, REL_TYPE_WORKSHEET, XlsxError};

//#region 🔖️SharedStringsXml
fn sst_to_xml(shared: &[String]) -> XmlDocument {
    let children = shared
        .iter()
        .map(|s| {
            XmlNode::Element {
                name: "si".into(),
                attrs: vec![],
                children: vec![XmlNode::Element {
                    name: "t".into(),
                    attrs: vec![attr("xml:space", "preserve")],
                    children: vec![XmlNode::Text { text: s.clone() }],
                }],
            }
        })
        .collect();
    XmlDocument {
        root: Some(XmlNode::Element {
            name: "sst".into(),
            attrs: vec![attr("xmlns", SML_NS), attr("count", &shared.len().to_string()), attr("uniqueCount", &shared.len().to_string())],
            children,
        }),
        doctype: None,
        declaration: None,
        prolog: Vec::new(),
    }
}
//#endregion 🔖️SharedStringsXml

//#region 🔖️WorkbookXml
fn workbook_to_xml(workbook: &XlsxWorkbook, rids: &[String]) -> XmlDocument {
    let sheets = workbook
        .sheets
        .iter()
        .zip(rids.iter())
        .enumerate()
        .map(|(i, (sheet, rid))| {
            XmlNode::Element {
                name: "sheet".into(),
                attrs: vec![attr("name", &sheet.name), attr("sheetId", &(i + 1).to_string()), attr("r:id", rid)],
                children: vec![],
            }
        })
        .collect();
    XmlDocument {
        root: Some(XmlNode::Element {
            name: "workbook".into(),
            attrs: vec![attr("xmlns", SML_NS), attr("xmlns:r", super::super::super::R_NS)],
            children: vec![XmlNode::Element { name: "sheets".into(), attrs: vec![], children: sheets }],
        }),
        doctype: None,
        declaration: None,
        prolog: Vec::new(),
    }
}
//#endregion 🔖️WorkbookXml

//#region 🔖️WorksheetXml
fn v_element(text: &str) -> XmlNode {
    XmlNode::Element { name: "v".into(), attrs: vec![], children: vec![XmlNode::Text { text: text.into() }] }
}

fn is_element(text: &str) -> XmlNode {
    XmlNode::Element { name: "is".into(), attrs: vec![], children: vec![XmlNode::Element { name: "t".into(), attrs: vec![attr("xml:space", "preserve")], children: vec![XmlNode::Text { text: text.into() }] }] }
}

fn f_element(expr: &str) -> XmlNode {
    XmlNode::Element { name: "f".into(), attrs: vec![], children: vec![XmlNode::Text { text: expr.into() }] }
}

fn format_number(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 { format!("{}", n as i64) } else { n.to_string() }
}

/// 🔎️ Renders a CACHED formula value (the `<v>`/`t` pair that follows `<f>expr</f>`, if any) —
/// mirrors `cell_to_xml`'s own top-level match, but never itself recurses into `Formula` (a
/// formula's cached value is never itself a formula in a spec-conformant document).
fn cached_value_xml(cached: &XlsxCellValue) -> (Option<crate::artifacts::xml::schema::snapshot::XmlAttr>, Option<XmlNode>) {
    match cached {
        XlsxCellValue::Number(n) => (None, Some(v_element(&format_number(*n)))),
        XlsxCellValue::SharedString(idx) => (Some(attr("t", "s")), Some(v_element(&idx.to_string()))),
        XlsxCellValue::InlineString(s) => (Some(attr("t", "str")), Some(v_element(s))),
        XlsxCellValue::Boolean(b) => (Some(attr("t", "b")), Some(v_element(if *b { "1" } else { "0" }))),
        XlsxCellValue::Formula { .. } => (None, None),
        XlsxCellValue::Empty => (None, None),
    }
}

fn cell_to_xml(cell: &XlsxCell) -> XmlNode {
    let r = format!("{}{}", super::super::super::column_letter(cell.col), cell.row);
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
fn worksheet_to_xml(sheet: &XlsxSheet) -> XmlDocument {
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
        root: Some(XmlNode::Element {
            name: "worksheet".into(),
            attrs: vec![attr("xmlns", SML_NS)],
            children: vec![XmlNode::Element { name: "sheetData".into(), attrs: vec![], children: rows }],
        }),
        doctype: None,
        declaration: None,
        prolog: Vec::new(),
    }
}
//#endregion 🔖️WorksheetXml

//#region 🔖️Codec
/// 🔄 Regenerates every xlsx-owned part (`xl/workbook.xml`, every `xl/worksheets/sheetN.xml`,
/// `xl/sharedStrings.xml`, and `xl/workbook.xml`'s relationships) from `workbook`, discarding
/// stale worksheet parts a shrinking sheet list would otherwise leave orphaned. Unrelated parts
/// (styles, themes, media, …) are untouched.
// 🩹 `WORKBOOK_PART` (the package's root/main part) is `set_part`'d FIRST, before
// `SHARED_STRINGS_PART`/worksheet parts — deliberately, NOT cosmetic. `opc.parts` is a
// name-keyed `Vec`, diffed/applied via `NamedTripleDiff`'s position-preserving-survivor
// convention (see the diff module's doc comment): a `between(a,b).apply(a) == b` round trip only
// reproduces `b`'s ACTUAL Vec order when survivor items keep a consistent relative position
// across every snapshot this engine builds. Putting the root part first (rather than last, as an
// earlier revision did) makes every `build_minimal_xlsx`/`encode_xlsx` output agree on "the part
// two arbitrary snapshots are most likely to share" sitting at a STABLE position — this is what
// makes `between_roundtrip_law`/`inverse_law`'s composed (non-`sweep_a`/`sweep_b`) fixture pairs
// hold, not just the hand-tuned `sweep_a`/`sweep_b` pair itself.
fn regenerate_workbook_parts(opc: &mut OpcPackage, workbook: &XlsxWorkbook) {
    opc.parts.retain(|p| !p.path.starts_with("xl/worksheets/") && p.path != WORKBOOK_PART && p.path != SHARED_STRINGS_PART);
    opc.content_types.set_default("rels", crate::artifacts::zip::opc::RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");

    let mut sheet_bytes = Vec::with_capacity(workbook.sheets.len());
    for sheet in &workbook.sheets {
        let xml = worksheet_to_xml(sheet);
        sheet_bytes.push(xml_document_to_text(&xml).into_bytes());
    }

    let mut rids = Vec::with_capacity(workbook.sheets.len());
    let mut workbook_rels = Vec::new();
    for i in 0..workbook.sheets.len() {
        let rid = format!("rId{}", i + 1);
        workbook_rels.push(OpcRelationship { id: rid.clone(), rel_type: REL_TYPE_WORKSHEET.into(), target: format!("worksheets/sheet{}.xml", i + 1), target_mode: OpcTargetMode::Internal });
        rids.push(rid);
    }
    workbook_rels.push(OpcRelationship {
        id: format!("rId{}", workbook.sheets.len() + 1),
        rel_type: REL_TYPE_SHARED_STRINGS.into(),
        target: "sharedStrings.xml".into(),
        target_mode: OpcTargetMode::Internal,
    });
    opc.relationships.insert(WORKBOOK_PART.to_string(), workbook_rels);

    let workbook_bytes = xml_document_to_text(&workbook_to_xml(workbook, &rids)).into_bytes();
    opc.set_part(WORKBOOK_PART, WORKBOOK_CONTENT_TYPE, workbook_bytes);

    // 🩹 `workbook.shared_strings` IS the SST — cells already carry `SharedString(idx)` indices
    // into it, so this is a direct serialize, never a text-dedup rebuild (the #1 xlsx gotcha this
    // engine used to paper over by eagerly resolving text; see the module doc comment).
    let sst_bytes = xml_document_to_text(&sst_to_xml(&workbook.shared_strings)).into_bytes();
    opc.set_part(SHARED_STRINGS_PART, SHARED_STRINGS_CONTENT_TYPE, sst_bytes);

    for (i, bytes) in sheet_bytes.into_iter().enumerate() {
        let path = format!("xl/worksheets/sheet{}.xml", i + 1);
        opc.set_part(&path, WORKSHEET_CONTENT_TYPE, bytes);
    }

    if opc.relationships_for("").iter().all(|r| r.rel_type != REL_TYPE_OFFICE_DOCUMENT) {
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, WORKBOOK_PART);
    }
}

/// 🏗️ Assembles a brand-new, minimal-but-valid OPC package around `workbook` — correct
/// `[Content_Types].xml`, root `_rels/.rels`, `xl/workbook.xml`, `xl/_rels/workbook.xml.rels`,
/// every worksheet, and a rebuilt `xl/sharedStrings.xml`.
pub fn build_minimal_xlsx(workbook: XlsxWorkbook) -> XlsxSnapshot {
    let mut opc = OpcPackage::empty();
    regenerate_workbook_parts(&mut opc, &workbook);
    XlsxSnapshot::from_parts(opc, workbook)
}

pub fn encode_xlsx(snap: &XlsxSnapshot) -> Result<Vec<u8>, XlsxError> {
    let mut opc = snap.opc.clone();
    regenerate_workbook_parts(&mut opc, &snap.workbook);
    Ok(crate::artifacts::zip::opc::encode_opc(&opc)?)
}
//#endregion 🔖️Codec
