//! ⚙️ SpreadsheetML (xlsx) engine — real OPC container + workbook/worksheet/shared-strings
//! model. Zip/OPC/XML byte-level work is never reimplemented here: it is reused from the shared
//! `crate::artifacts::zip::opc` layer. Shared-string resolution (`t="s"` cells reference an
//! index into `xl/sharedStrings.xml`, not inline text) is resolved to literal text on decode and
//! rebuilt (deduplicated, first-use order) on encode — the #1 xlsx gotcha, handled here once.

use crate::artifacts::xlsx::{schema::snapshot::{XlsxCell, XlsxCellValue, XlsxRow, XlsxSheet, XlsxWorkbook}, XlsxArtifact, XlsxDiff, XlsxMutation, XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::{self, OpcPackage, OpcRelationship, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};

//#region 🔖️Error
/// ⚠️ Typed xlsx decode/encode failure — a workbook this engine cannot honestly interpret
/// (dangling relationship, out-of-range shared-string index, non-numeric numeric cell, …) is
/// never fabricated into a partial/empty workbook.
#[derive(Clone, Debug, PartialEq)]
pub enum XlsxError {
    Opc(opc::OpcError),
    MissingWorkbookRelationship,
    MissingPart(String),
    Xml { part: String, detail: String },
    Malformed(String),
}

impl std::fmt::Display for XlsxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opc(e) => write!(f, "xlsx: {e}"),
            Self::MissingWorkbookRelationship => write!(f, "xlsx: package root has no officeDocument relationship"),
            Self::MissingPart(p) => write!(f, "xlsx: missing required part {p}"),
            Self::Xml { part, detail } => write!(f, "xlsx: xml in {part}: {detail}"),
            Self::Malformed(detail) => write!(f, "xlsx: {detail}"),
        }
    }
}

impl std::error::Error for XlsxError {}

impl From<opc::OpcError> for XlsxError {
    fn from(e: opc::OpcError) -> Self { Self::Opc(e) }
}
//#endregion 🔖️Error

//#region 🔖️Constants
const SML_NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
const R_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const WORKBOOK_PART: &str = "xl/workbook.xml";
const SHARED_STRINGS_PART: &str = "xl/sharedStrings.xml";
const WORKBOOK_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml";
const WORKSHEET_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
const SHARED_STRINGS_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml";
const REL_TYPE_WORKSHEET: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
const REL_TYPE_SHARED_STRINGS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings";

fn attr(name: &str, value: &str) -> XmlAttr {
    XmlAttr { name: name.into(), value: value.into() }
}

fn attr_val<'a>(attrs: &'a [XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}
//#endregion 🔖️Constants

//#region 🔖️ColumnLetters
/// 🔤️ 0-indexed column number -> spreadsheet column letters (`0 -> "A"`, `25 -> "Z"`, `26 -> "AA"`).
pub fn column_letter(mut index: u32) -> String {
    let mut letters = Vec::new();
    loop {
        letters.push((b'A' + (index % 26) as u8) as char);
        if index < 26 { break; }
        index = index / 26 - 1;
    }
    letters.iter().rev().collect()
}
//#endregion 🔖️ColumnLetters

//#region 🔖️SharedStringsXml
fn collect_text(node: &XmlNode, out: &mut String) {
    if let XmlNode::Element { name, children, .. } = node {
        if name == "t" {
            for c in children {
                if let XmlNode::Text { text } = c {
                    out.push_str(text);
                }
            }
        } else {
            for c in children {
                collect_text(c, out);
            }
        }
    }
}

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
    }
}

fn shared_strings_from_xml(doc: &XmlDocument, part: &str) -> Result<Vec<String>, XlsxError> {
    let bad = |detail: String| XlsxError::Xml { part: part.into(), detail };
    let root = doc.root.as_ref().ok_or_else(|| bad("empty document".into()))?;
    let XmlNode::Element { name, children, .. } = root else { return Err(bad("root is not an element".into())) };
    if name != "sst" {
        return Err(bad(format!("expected <sst>, got <{name}>")));
    }
    let mut out = Vec::new();
    for si in children {
        if let XmlNode::Element { name, .. } = si {
            if name == "si" {
                let mut text = String::new();
                collect_text(si, &mut text);
                out.push(text);
            }
        }
    }
    Ok(out)
}

fn shared_index_of(shared: &mut Vec<String>, s: &str) -> usize {
    if let Some(i) = shared.iter().position(|t| t == s) { i } else { shared.push(s.to_string()); shared.len() - 1 }
}
//#endregion 🔖️SharedStringsXml

//#region 🔖️WorkbookXml
struct SheetRef {
    name: String,
    r_id: String,
}

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
            attrs: vec![attr("xmlns", SML_NS), attr("xmlns:r", R_NS)],
            children: vec![XmlNode::Element { name: "sheets".into(), attrs: vec![], children: sheets }],
        }),
        doctype: None,
        declaration: None,
    }
}

fn workbook_sheets_from_xml(doc: &XmlDocument, part: &str) -> Result<Vec<SheetRef>, XlsxError> {
    let bad = |detail: String| XlsxError::Xml { part: part.into(), detail };
    let root = doc.root.as_ref().ok_or_else(|| bad("empty document".into()))?;
    let XmlNode::Element { name, children, .. } = root else { return Err(bad("root is not an element".into())) };
    if name != "workbook" {
        return Err(bad(format!("expected <workbook>, got <{name}>")));
    }
    let sheets_el = children
        .iter()
        .find_map(|c| match c { XmlNode::Element { name, children, .. } if name == "sheets" => Some(children), _ => None })
        .ok_or_else(|| bad("missing <sheets>".into()))?;
    let mut out = Vec::new();
    for s in sheets_el {
        let XmlNode::Element { name, attrs, .. } = s else { continue };
        if name != "sheet" {
            continue;
        }
        let sheet_name = attr_val(attrs, "name").ok_or_else(|| bad("<sheet> missing name".into()))?.to_string();
        let r_id = attr_val(attrs, "r:id").ok_or_else(|| bad("<sheet> missing r:id".into()))?.to_string();
        out.push(SheetRef { name: sheet_name, r_id });
    }
    Ok(out)
}
//#endregion 🔖️WorkbookXml

//#region 🔖️WorksheetXml
fn v_element(text: &str) -> XmlNode {
    XmlNode::Element { name: "v".into(), attrs: vec![], children: vec![XmlNode::Text { text: text.into() }] }
}

fn format_number(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 { format!("{}", n as i64) } else { n.to_string() }
}

fn cell_to_xml(cell: &XlsxCell, shared: &mut Vec<String>) -> XmlNode {
    let mut attrs = vec![attr("r", &cell.reference)];
    match &cell.value {
        XlsxCellValue::Number(n) => XmlNode::Element { name: "c".into(), attrs, children: vec![v_element(&format_number(*n))] },
        XlsxCellValue::Text(s) => {
            let idx = shared_index_of(shared, s);
            attrs.push(attr("t", "s"));
            XmlNode::Element { name: "c".into(), attrs, children: vec![v_element(&idx.to_string())] }
        }
        XlsxCellValue::Bool(b) => {
            attrs.push(attr("t", "b"));
            XmlNode::Element { name: "c".into(), attrs, children: vec![v_element(if *b { "1" } else { "0" })] }
        }
        XlsxCellValue::Empty => XmlNode::Element { name: "c".into(), attrs, children: vec![] },
    }
}

fn worksheet_to_xml(sheet: &XlsxSheet, shared: &mut Vec<String>) -> XmlDocument {
    let rows = sheet
        .rows
        .iter()
        .map(|row| {
            let cells = row.cells.iter().map(|cell| cell_to_xml(cell, shared)).collect();
            XmlNode::Element { name: "row".into(), attrs: vec![attr("r", &row.index.to_string())], children: cells }
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
    }
}

fn find_v_text(children: &[XmlNode]) -> Option<String> {
    children.iter().find_map(|c| match c {
        XmlNode::Element { name, children, .. } if name == "v" => {
            let mut text = String::new();
            for t in children {
                if let XmlNode::Text { text: t } = t {
                    text.push_str(t);
                }
            }
            Some(text)
        }
        _ => None,
    })
}

/// 🔎️ Resolves one `<c>` element's value, given its `t` attribute (`None` = numeric default).
/// `t="s"` resolves through `shared` — an out-of-range index is a hard `Malformed` error, never
/// a silently-empty cell.
fn extract_cell_value(children: &[XmlNode], t: Option<&str>, shared: &[String], part: &str) -> Result<XlsxCellValue, XlsxError> {
    match t {
        Some("s") => {
            let v = find_v_text(children).ok_or_else(|| XlsxError::Xml { part: part.into(), detail: "t=\"s\" cell missing <v>".into() })?;
            let idx: usize = v.trim().parse().map_err(|_| XlsxError::Malformed(format!("cell in {part}: shared-string index {v:?} is not an integer")))?;
            let text = shared.get(idx).ok_or_else(|| XlsxError::Malformed(format!("cell in {part}: shared-string index {idx} out of range ({} entries)", shared.len())))?;
            Ok(XlsxCellValue::Text(text.clone()))
        }
        Some("str") => Ok(XlsxCellValue::Text(find_v_text(children).unwrap_or_default())),
        Some("inlineStr") => {
            let is_children = children.iter().find_map(|c| match c { XmlNode::Element { name, children, .. } if name == "is" => Some(children), _ => None });
            let mut text = String::new();
            if let Some(is_children) = is_children {
                for c in is_children {
                    collect_text(c, &mut text);
                }
            }
            Ok(XlsxCellValue::Text(text))
        }
        Some("b") => {
            let v = find_v_text(children).unwrap_or_default();
            Ok(XlsxCellValue::Bool(v.trim() == "1" || v.trim().eq_ignore_ascii_case("true")))
        }
        Some("e") => Ok(XlsxCellValue::Text(find_v_text(children).unwrap_or_default())),
        None | Some(_) => match find_v_text(children) {
            Some(v) => v.trim().parse::<f64>().map(XlsxCellValue::Number).map_err(|_| XlsxError::Malformed(format!("cell in {part}: invalid numeric value {v:?}"))),
            None => Ok(XlsxCellValue::Empty),
        },
    }
}

fn worksheet_rows_from_xml(doc: &XmlDocument, shared: &[String], part: &str) -> Result<Vec<XlsxRow>, XlsxError> {
    let bad = |detail: String| XlsxError::Xml { part: part.into(), detail };
    let root = doc.root.as_ref().ok_or_else(|| bad("empty document".into()))?;
    let XmlNode::Element { name, children, .. } = root else { return Err(bad("root is not an element".into())) };
    if name != "worksheet" {
        return Err(bad(format!("expected <worksheet>, got <{name}>")));
    }
    let sheet_data = children
        .iter()
        .find_map(|c| match c { XmlNode::Element { name, children, .. } if name == "sheetData" => Some(children), _ => None })
        .ok_or_else(|| bad("missing <sheetData>".into()))?;
    let mut rows = Vec::new();
    for row_node in sheet_data {
        let XmlNode::Element { name, attrs, children: row_children } = row_node else { continue };
        if name != "row" {
            continue;
        }
        let index = attr_val(attrs, "r")
            .ok_or_else(|| bad("<row> missing r".into()))?
            .parse::<u32>()
            .map_err(|_| bad("<row> r attribute is not a valid integer".into()))?;
        let mut cells = Vec::new();
        for c_node in row_children {
            let XmlNode::Element { name, attrs, children: c_children } = c_node else { continue };
            if name != "c" {
                continue;
            }
            let reference = attr_val(attrs, "r").ok_or_else(|| bad("<c> missing r".into()))?.to_string();
            let t = attr_val(attrs, "t");
            let value = extract_cell_value(c_children, t, shared, part)?;
            cells.push(XlsxCell { reference, value });
        }
        rows.push(XlsxRow { index, cells });
    }
    Ok(rows)
}
//#endregion 🔖️WorksheetXml

//#region 🔖️Codec
/// 🔄 Regenerates every xlsx-owned part (`xl/workbook.xml`, every `xl/worksheets/sheetN.xml`,
/// `xl/sharedStrings.xml`, and `xl/workbook.xml`'s relationships) from `workbook`, discarding
/// stale worksheet parts a shrinking sheet list would otherwise leave orphaned. Unrelated parts
/// (styles, themes, media, …) are untouched.
fn regenerate_workbook_parts(opc: &mut OpcPackage, workbook: &XlsxWorkbook) {
    opc.parts.retain(|p| !p.path.starts_with("xl/worksheets/") && p.path != WORKBOOK_PART && p.path != SHARED_STRINGS_PART);
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
    opc.content_types.set_default("xml", "application/xml");

    let mut shared: Vec<String> = Vec::new();
    let mut sheet_bytes = Vec::with_capacity(workbook.sheets.len());
    for sheet in &workbook.sheets {
        let xml = worksheet_to_xml(sheet, &mut shared);
        sheet_bytes.push(xml_document_to_text(&xml).into_bytes());
    }

    let sst_bytes = xml_document_to_text(&sst_to_xml(&shared)).into_bytes();
    opc.set_part(SHARED_STRINGS_PART, SHARED_STRINGS_CONTENT_TYPE, sst_bytes);

    let mut rids = Vec::with_capacity(workbook.sheets.len());
    let mut workbook_rels = Vec::new();
    for (i, bytes) in sheet_bytes.into_iter().enumerate() {
        let path = format!("xl/worksheets/sheet{}.xml", i + 1);
        opc.set_part(&path, WORKSHEET_CONTENT_TYPE, bytes);
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
    Ok(opc::encode_opc(&opc)?)
}

pub fn decode_xlsx(data: &[u8]) -> Result<XlsxSnapshot, XlsxError> {
    let opc = opc::decode_opc(data)?;
    let workbook_path = opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).ok_or(XlsxError::MissingWorkbookRelationship)?;
    let workbook_bytes = opc.part_bytes(&workbook_path).ok_or_else(|| XlsxError::MissingPart(workbook_path.clone()))?;
    let workbook_text = String::from_utf8(workbook_bytes.to_vec()).map_err(|_| XlsxError::Xml { part: workbook_path.clone(), detail: "not valid utf-8".into() })?;
    let workbook_xml = xml_document_from_text(&workbook_text).map_err(|e| XlsxError::Xml { part: workbook_path.clone(), detail: e })?;
    let sheet_refs = workbook_sheets_from_xml(&workbook_xml, &workbook_path)?;

    let workbook_rels = opc.relationships_for(&workbook_path);
    let shared_strings = match workbook_rels.iter().find(|r| r.rel_type == REL_TYPE_SHARED_STRINGS) {
        Some(rel) => {
            let path = opc::resolve_relationship_target(&workbook_path, &rel.target);
            let bytes = opc.part_bytes(&path).ok_or_else(|| XlsxError::MissingPart(path.clone()))?;
            let text = String::from_utf8(bytes.to_vec()).map_err(|_| XlsxError::Xml { part: path.clone(), detail: "not valid utf-8".into() })?;
            let doc = xml_document_from_text(&text).map_err(|e| XlsxError::Xml { part: path.clone(), detail: e })?;
            shared_strings_from_xml(&doc, &path)?
        }
        None => Vec::new(),
    };

    let mut sheets = Vec::with_capacity(sheet_refs.len());
    for sheet_ref in &sheet_refs {
        let rel = workbook_rels
            .iter()
            .find(|r| r.id == sheet_ref.r_id)
            .ok_or_else(|| XlsxError::Malformed(format!("sheet {:?} references unknown relationship id {}", sheet_ref.name, sheet_ref.r_id)))?;
        let path = opc::resolve_relationship_target(&workbook_path, &rel.target);
        let bytes = opc.part_bytes(&path).ok_or_else(|| XlsxError::MissingPart(path.clone()))?;
        let text = String::from_utf8(bytes.to_vec()).map_err(|_| XlsxError::Xml { part: path.clone(), detail: "not valid utf-8".into() })?;
        let doc = xml_document_from_text(&text).map_err(|e| XlsxError::Xml { part: path.clone(), detail: e })?;
        let rows = worksheet_rows_from_xml(&doc, &shared_strings, &path)?;
        sheets.push(XlsxSheet { name: sheet_ref.name.clone(), rows });
    }

    Ok(XlsxSnapshot::from_parts(opc, XlsxWorkbook { sheets }))
}

pub fn empty_xlsx_snapshot() -> XlsxSnapshot { XlsxSnapshot::default() }

pub fn register() {
    crate::artifacts::xlsx::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::xlsx::schema::xlsx_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<XlsxSnapshot, XlsxMutation>(STDIO_XLSX_DOCUMENT_SCHEMA));
}
//#endregion 🔖️Codec

//#region 🔖️Sniff
/// 🕵️ Real xlsx sniff: OPC-shaped bytes whose root officeDocument relationship resolves under
/// `xl/` — disambiguates from docx/pptx sharing the same zip magic and OPC shape.
pub fn sniff_xlsx_bytes(data: &[u8]) -> bool {
    let Ok(opc) = opc::decode_opc(data) else { return false };
    match opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT) {
        Some(path) => path.starts_with("xl/"),
        None => false,
    }
}
//#endregion 🔖️Sniff

//#region 🔖️ArtifactEngine
pub struct XlsxEngine { artifact_state: XlsxArtifact, snapshot_state: XlsxSnapshot }
impl XlsxEngine {
    pub fn new(snapshot: XlsxSnapshot) -> Self {
        Self { artifact_state: XlsxArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn row(index: u32, values: &[(&str, XlsxCellValue)]) -> XlsxRow {
        XlsxRow { index, cells: values.iter().map(|(r, v)| XlsxCell { reference: r.to_string(), value: v.clone() }).collect() }
    }

    fn sample_workbook() -> XlsxWorkbook {
        XlsxWorkbook {
            sheets: vec![
                XlsxSheet {
                    name: "Numbers".into(),
                    rows: vec![
                        row(1, &[("A1", XlsxCellValue::Text("Name".into())), ("B1", XlsxCellValue::Text("Score".into()))]),
                        row(2, &[("A2", XlsxCellValue::Text("Alice".into())), ("B2", XlsxCellValue::Number(9.5))]),
                        row(3, &[("A3", XlsxCellValue::Text("Alice".into())), ("B3", XlsxCellValue::Number(-3.0))]),
                        row(4, &[("A4", XlsxCellValue::Bool(true)), ("B4", XlsxCellValue::Empty)]),
                    ],
                },
                XlsxSheet { name: "Second".into(), rows: vec![row(1, &[("A1", XlsxCellValue::Text("Alice".into()))])] },
            ],
        }
    }

    #[test]
    fn column_letters_follow_spreadsheet_convention() {
        assert_eq!(column_letter(0), "A");
        assert_eq!(column_letter(25), "Z");
        assert_eq!(column_letter(26), "AA");
        assert_eq!(column_letter(27), "AB");
        assert_eq!(column_letter(51), "AZ");
        assert_eq!(column_letter(52), "BA");
    }

    #[test]
    fn builder_produces_minimal_valid_package_that_decodes_back() {
        let snap = build_minimal_xlsx(sample_workbook());
        let bytes = encode_xlsx(&snap).expect("encode minimal package");
        assert!(opc::sniff_opc_bytes(&bytes));
        assert!(sniff_xlsx_bytes(&bytes));
        let decoded = decode_xlsx(&bytes).expect("decode minimal package");
        assert_eq!(decoded.workbook, sample_workbook());
    }

    #[test]
    fn shared_strings_dedupe_and_resolve_correctly() {
        // "Alice" appears 3 times across 2 sheets -> exactly one shared-string entry, referenced
        // by index from every cell. This is the exact bug class the #1 xlsx gotcha describes.
        let snap = build_minimal_xlsx(sample_workbook());
        let sst_bytes = snap.opc.part_bytes("xl/sharedStrings.xml").expect("sharedStrings.xml part present");
        let sst_xml = xml_document_from_text(std::str::from_utf8(sst_bytes).unwrap()).expect("parse sst");
        let strings = shared_strings_from_xml(&sst_xml, "xl/sharedStrings.xml").expect("parse strings");
        assert_eq!(strings.iter().filter(|s| *s == "Alice").count(), 1, "Alice must be deduplicated to a single shared-string entry");

        // Round trip through real bytes and confirm every "Alice" cell across both sheets still
        // resolves to the same literal text.
        let bytes = encode_xlsx(&snap).expect("encode");
        let re_decoded = decode_xlsx(&bytes).expect("decode");
        assert_eq!(re_decoded.workbook.sheets[0].rows[1].cells[0].value, XlsxCellValue::Text("Alice".into()));
        assert_eq!(re_decoded.workbook.sheets[0].rows[2].cells[0].value, XlsxCellValue::Text("Alice".into()));
        assert_eq!(re_decoded.workbook.sheets[1].rows[0].cells[0].value, XlsxCellValue::Text("Alice".into()));
    }

    #[test]
    fn decode_resolves_real_hand_built_package_with_shared_strings() {
        // Hand-built OOXML: real workbook.xml + worksheet + sharedStrings.xml + all rels wired
        // by hand, not a generator shortcut.
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");

        let sst_xml = concat!(
            r#"<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="2" uniqueCount="2">"#,
            "<si><t>Quarter</t></si>",
            "<si><t>Revenue &amp; Profit</t></si>",
            "</sst>",
        );
        opc.set_part(SHARED_STRINGS_PART, SHARED_STRINGS_CONTENT_TYPE, sst_xml.as_bytes().to_vec());

        let sheet_xml = concat!(
            r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">"#,
            "<sheetData>",
            r#"<row r="1"><c r="A1" t="s"><v>0</v></c><c r="B1" t="s"><v>1</v></c></row>"#,
            r#"<row r="2"><c r="A2"><v>4</v></c><c r="B2"><v>123.5</v></c></row>"#,
            "</sheetData>",
            "</worksheet>",
        );
        opc.set_part("xl/worksheets/sheet1.xml", WORKSHEET_CONTENT_TYPE, sheet_xml.as_bytes().to_vec());

        let workbook_xml = concat!(
            r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
            r#"<sheets><sheet name="Q1" sheetId="1" r:id="rId1"/></sheets>"#,
            "</workbook>",
        );
        opc.set_part(WORKBOOK_PART, WORKBOOK_CONTENT_TYPE, workbook_xml.as_bytes().to_vec());

        opc.add_relationship(WORKBOOK_PART, "rId1", REL_TYPE_WORKSHEET, "worksheets/sheet1.xml");
        opc.add_relationship(WORKBOOK_PART, "rId2", REL_TYPE_SHARED_STRINGS, "sharedStrings.xml");
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, WORKBOOK_PART);

        let bytes = opc::encode_opc(&opc).expect("encode hand-built package");
        let decoded = decode_xlsx(&bytes).expect("decode hand-built xlsx");

        assert_eq!(decoded.workbook.sheets.len(), 1);
        assert_eq!(decoded.workbook.sheets[0].name, "Q1");
        let rows = &decoded.workbook.sheets[0].rows;
        assert_eq!(rows[0].cells[0].value, XlsxCellValue::Text("Quarter".into()));
        assert_eq!(rows[0].cells[1].value, XlsxCellValue::Text("Revenue & Profit".into()));
        assert_eq!(rows[1].cells[0].value, XlsxCellValue::Number(4.0));
        assert_eq!(rows[1].cells[1].value, XlsxCellValue::Number(123.5));
    }

    #[test]
    fn decode_rejects_out_of_range_shared_string_index() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.set_part(SHARED_STRINGS_PART, SHARED_STRINGS_CONTENT_TYPE, br#"<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="0" uniqueCount="0"></sst>"#.to_vec());
        opc.set_part(
            "xl/worksheets/sheet1.xml",
            WORKSHEET_CONTENT_TYPE,
            br#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1"><c r="A1" t="s"><v>7</v></c></row></sheetData></worksheet>"#.to_vec(),
        );
        opc.set_part(
            WORKBOOK_PART,
            WORKBOOK_CONTENT_TYPE,
            br#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="S" sheetId="1" r:id="rId1"/></sheets></workbook>"#.to_vec(),
        );
        opc.add_relationship(WORKBOOK_PART, "rId1", REL_TYPE_WORKSHEET, "worksheets/sheet1.xml");
        opc.add_relationship(WORKBOOK_PART, "rId2", REL_TYPE_SHARED_STRINGS, "sharedStrings.xml");
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, WORKBOOK_PART);
        let bytes = opc::encode_opc(&opc).expect("encode");

        let err = decode_xlsx(&bytes).expect_err("out-of-range shared-string index must be rejected, not silently empty");
        assert!(matches!(err, XlsxError::Malformed(_)));
    }

    #[test]
    fn unmodeled_parts_survive_decode_encode_verbatim() {
        let snap = build_minimal_xlsx(sample_workbook());
        let mut opc = snap.opc.clone();
        opc.set_part("xl/styles.xml", "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml", b"<styleSheet/>".to_vec());
        let bytes = opc::encode_opc(&opc).expect("encode");

        let decoded = decode_xlsx(&bytes).expect("decode");
        assert_eq!(decoded.opc.part_bytes("xl/styles.xml"), Some(b"<styleSheet/>".as_slice()));
        let re_encoded = encode_xlsx(&decoded).expect("re-encode");
        let re_decoded = decode_xlsx(&re_encoded).expect("re-decode");
        assert_eq!(re_decoded.opc.part_bytes("xl/styles.xml"), Some(b"<styleSheet/>".as_slice()));
        assert_eq!(re_decoded.workbook, sample_workbook());
    }

    #[test]
    fn analyzer_builder_round_trip() {
        let original = build_minimal_xlsx(sample_workbook());
        let bytes = encode_xlsx(&original).expect("encode");
        let analyzed = decode_xlsx(&bytes).expect("decode");
        let rebuilt = build_minimal_xlsx(analyzed.workbook.clone());
        let rebuilt_bytes = encode_xlsx(&rebuilt).expect("encode rebuilt");
        let reanalyzed = decode_xlsx(&rebuilt_bytes).expect("decode rebuilt");
        assert_eq!(reanalyzed.workbook, analyzed.workbook);
    }

    #[test]
    fn shrinking_sheet_count_drops_stale_worksheet_parts() {
        let mut wide = sample_workbook();
        let snap_wide = build_minimal_xlsx(wide.clone());
        assert!(snap_wide.opc.part("xl/worksheets/sheet2.xml").is_some());

        wide.sheets.truncate(1);
        let bytes = encode_xlsx(&XlsxSnapshot::from_parts(snap_wide.opc, wide)).expect("encode narrower workbook");
        let decoded = decode_xlsx(&bytes).expect("decode");
        assert!(decoded.opc.part("xl/worksheets/sheet2.xml").is_none(), "stale second sheet must be dropped, not left orphaned");
        assert_eq!(decoded.workbook.sheets.len(), 1);
    }
}
//#endregion 🧪️Tests
