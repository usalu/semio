//! ⚙️ SpreadsheetML (xlsx) engine — real OPC container + workbook/worksheet/shared-strings
//! model. Zip/OPC/XML byte-level work is never reimplemented here: it is reused from the shared
//! `crate::artifacts::zip::opc` layer. Shared strings (`t="s"` cells reference an index into
//! `xl/sharedStrings.xml`) are decoded/encoded as an EXPLICIT `workbook.shared_strings` table —
//! never eagerly resolved into cell text — so the `t="s"` (shared-string reference) vs
//! `t="inlineStr"` (literal text) distinction the format itself makes survives round-trip, and a
//! diff over `shared_strings` means something (see `🧬️schema/🔺️diff`).

use crate::artifacts::xlsx::{schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook}, XlsxArtifact, XlsxDiff, XlsxMutation, XlsxSnapshot, STDIO_XLSX_DOCUMENT_SCHEMA};
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
/// 🏅️ ISO/IEC 29500-1 Strict's officeDocument relationship TYPE for the package-root -> workbook
/// pointer (Strict's Annex replaces every `schemas.openxmlformats.org` relationship-type URI with
/// a `purl.oclc.org/ooxml` equivalent, not just the content markup namespaces -- ticket
/// 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3's `✳️strict` subset). Recognized here
/// (decode/sniff, additively, alongside the Transitional URI above) so a genuinely Strict-shaped
/// package can be decoded at all -- without this, `decode_xlsx` would reject every real Strict
/// document with `MissingWorkbookRelationship` before the `✳️strict` subset analyzer ever ran.
const REL_TYPE_OFFICE_DOCUMENT_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument";
/// 🏅️ Strict's `sharedStrings` relationship TYPE, same rationale as above -- without recognizing
/// it, any Strict document using shared strings would hard-fail decode with an out-of-range
/// shared-string index (the shared-strings part would never be found).
const REL_TYPE_SHARED_STRINGS_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships/sharedStrings";

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

/// 🔤️ Inverse of `column_letter`: spreadsheet column letters -> 0-indexed column number
/// (`"A" -> 0`, `"Z" -> 25`, `"AA" -> 26`). `None` on empty or non-alphabetic input.
pub fn column_index(letters: &str) -> Option<u32> {
    if letters.is_empty() || !letters.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let mut idx: u64 = 0;
    for c in letters.chars() {
        idx = idx * 26 + (c.to_ascii_uppercase() as u64 - 'A' as u64 + 1);
    }
    Some((idx - 1) as u32)
}

/// 🔤️ Splits an A1-style cell reference (`"B2"`) into its column-letter prefix (`"B"`) — only the
/// column part is needed by the decoder, since row is already known from the enclosing `<row r>`.
fn column_letters_of(reference: &str) -> &str {
    reference.trim_end_matches(|c: char| c.is_ascii_digit())
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
fn cached_value_xml(cached: &XlsxCellValue) -> (Option<XmlAttr>, Option<XmlNode>) {
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

fn find_f_text(children: &[XmlNode]) -> Option<String> {
    children.iter().find_map(|c| match c {
        XmlNode::Element { name, children, .. } if name == "f" => {
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

/// 🔎️ Resolves a non-formula `<c>`'s value/cached-value given its `t` attribute (`None` =
/// numeric default). `t="s"` is bounds-checked against `sst_len` (an out-of-range index is a hard
/// `Malformed` error, never a silently-empty cell) but NOT resolved to text here — the caller
/// keeps the index (see the module doc comment). `t="e"`/non-formula `t="str"` normalize to
/// `InlineString` (a documented normalization: this union has no dedicated error/formula-string
/// variant for a BARE cell — see `Formula.cached`, which IS typed, for the formula case).
fn extract_typed_value(children: &[XmlNode], t: Option<&str>, sst_len: usize, part: &str) -> Result<XlsxCellValue, XlsxError> {
    match t {
        Some("s") => {
            let v = find_v_text(children).ok_or_else(|| XlsxError::Xml { part: part.into(), detail: "t=\"s\" cell missing <v>".into() })?;
            let idx: usize = v.trim().parse().map_err(|_| XlsxError::Malformed(format!("cell in {part}: shared-string index {v:?} is not an integer")))?;
            if idx >= sst_len {
                return Err(XlsxError::Malformed(format!("cell in {part}: shared-string index {idx} out of range ({sst_len} entries)")));
            }
            Ok(XlsxCellValue::SharedString(idx))
        }
        Some("str") => Ok(XlsxCellValue::InlineString(find_v_text(children).unwrap_or_default())),
        Some("inlineStr") => {
            let is_children = children.iter().find_map(|c| match c { XmlNode::Element { name, children, .. } if name == "is" => Some(children), _ => None });
            let mut text = String::new();
            if let Some(is_children) = is_children {
                for c in is_children {
                    collect_text(c, &mut text);
                }
            }
            Ok(XlsxCellValue::InlineString(text))
        }
        Some("b") => {
            let v = find_v_text(children).unwrap_or_default();
            Ok(XlsxCellValue::Boolean(v.trim() == "1" || v.trim().eq_ignore_ascii_case("true")))
        }
        Some("e") => Ok(XlsxCellValue::InlineString(find_v_text(children).unwrap_or_default())),
        None | Some(_) => match find_v_text(children) {
            Some(v) => v.trim().parse::<f64>().map(XlsxCellValue::Number).map_err(|_| XlsxError::Malformed(format!("cell in {part}: invalid numeric value {v:?}"))),
            None => Ok(XlsxCellValue::Empty),
        },
    }
}

/// 🔎️ Resolves one `<c>` element's full value — a `<f>` child present makes this a `Formula`
/// cell (ECMA-376 §18.3.1.40); its `cached` is the SAME `<v>`/`t` pair, re-typed by
/// `extract_typed_value` (absent `<v>` = uncalculated, `cached: None`).
fn extract_cell_value(children: &[XmlNode], t: Option<&str>, sst_len: usize, part: &str) -> Result<XlsxCellValue, XlsxError> {
    if let Some(expr) = find_f_text(children) {
        let cached = if find_v_text(children).is_some() {
            Some(Box::new(extract_typed_value(children, t, sst_len, part)?))
        } else {
            None
        };
        return Ok(XlsxCellValue::Formula { expr, cached });
    }
    extract_typed_value(children, t, sst_len, part)
}

/// 🌳 Flattens `<sheetData>`'s `<row>`-then-`<c>` nesting into `sheet.cells`'s sparse
/// `(row, col)`-addressed list — `row` from the enclosing `<row r>`, `col` from the cell's own
/// `<c r>` column-letter prefix (`col` in the cell's own `r` MUST agree with the row-digit suffix
/// per spec; only the column letters carry information this decoder doesn't already have).
fn worksheet_cells_from_xml(doc: &XmlDocument, sst_len: usize, part: &str) -> Result<Vec<XlsxCell>, XlsxError> {
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
    let mut cells = Vec::new();
    for row_node in sheet_data {
        let XmlNode::Element { name, attrs, children: row_children } = row_node else { continue };
        if name != "row" {
            continue;
        }
        let row = attr_val(attrs, "r")
            .ok_or_else(|| bad("<row> missing r".into()))?
            .parse::<u32>()
            .map_err(|_| bad("<row> r attribute is not a valid integer".into()))?;
        for c_node in row_children {
            let XmlNode::Element { name, attrs, children: c_children } = c_node else { continue };
            if name != "c" {
                continue;
            }
            let reference = attr_val(attrs, "r").ok_or_else(|| bad("<c> missing r".into()))?;
            let col = column_index(column_letters_of(reference)).ok_or_else(|| bad(format!("<c> r={reference:?} has no valid column-letter prefix")))?;
            let t = attr_val(attrs, "t");
            let value = extract_cell_value(c_children, t, sst_len, part)?;
            cells.push(XlsxCell { row, col, value });
        }
    }
    Ok(cells)
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
    opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
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
    Ok(opc::encode_opc(&opc)?)
}

pub fn decode_xlsx(data: &[u8]) -> Result<XlsxSnapshot, XlsxError> {
    let opc = opc::decode_opc(data)?;
    // 🏅️ Recognizes either the Transitional or Strict officeDocument relationship TYPE (see the
    // `REL_TYPE_OFFICE_DOCUMENT_STRICT` doc comment above) -- additive, doesn't change decode for
    // any existing Transitional package.
    let workbook_path = opc
        .resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT)
        .or_else(|| opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT_STRICT))
        .ok_or(XlsxError::MissingWorkbookRelationship)?;
    let workbook_bytes = opc.part_bytes(&workbook_path).ok_or_else(|| XlsxError::MissingPart(workbook_path.clone()))?;
    let workbook_text = String::from_utf8(workbook_bytes.to_vec()).map_err(|_| XlsxError::Xml { part: workbook_path.clone(), detail: "not valid utf-8".into() })?;
    let workbook_xml = xml_document_from_text(&workbook_text).map_err(|e| XlsxError::Xml { part: workbook_path.clone(), detail: e })?;
    let sheet_refs = workbook_sheets_from_xml(&workbook_xml, &workbook_path)?;

    let workbook_rels = opc.relationships_for(&workbook_path);
    let shared_strings = match workbook_rels.iter().find(|r| r.rel_type == REL_TYPE_SHARED_STRINGS || r.rel_type == REL_TYPE_SHARED_STRINGS_STRICT) {
        Some(rel) => {
            let path = opc::resolve_relationship_target(&workbook_path, &rel.target);
            let bytes = opc.part_bytes(&path).ok_or_else(|| XlsxError::MissingPart(path.clone()))?;
            let text = String::from_utf8(bytes.to_vec()).map_err(|_| XlsxError::Xml { part: path.clone(), detail: "not valid utf-8".into() })?;
            let doc = xml_document_from_text(&text).map_err(|e| XlsxError::Xml { part: path.clone(), detail: e })?;
            shared_strings_from_xml(&doc, &path)?
        }
        None => Vec::new(),
    };

    let sst_len = shared_strings.len();
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
        let cells = worksheet_cells_from_xml(&doc, sst_len, &path)?;
        sheets.push(XlsxSheet { name: sheet_ref.name.clone(), cells });
    }

    Ok(XlsxSnapshot::from_parts(opc, XlsxWorkbook { sheets, shared_strings }))
}

pub fn empty_xlsx_snapshot() -> XlsxSnapshot { XlsxSnapshot::default() }

/// 📄️ FG-wave: the demo `stdio.xlsx` document — a genuinely non-trivial `XlsxSnapshot` exercising
/// every `XlsxCellValue` variant (`SharedString`, `Number`, `Boolean`, `Formula` with a cached
/// value, `InlineString`), two sheets, and one unmodeled raw OPC part (`xl/styles.xml`,
/// verbatim-retained). The single source of truth for
/// `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are literally
/// this snapshot's `print_dsl`/`encode_pack` output, asserted equal by `fixture_honesty_law`
/// below) — same shape docx's own `demo_docx_snapshot()` establishes (this wave's OPC
/// pattern-setter).
pub fn demo_xlsx_snapshot() -> XlsxSnapshot {
    let workbook = XlsxWorkbook {
        sheets: vec![
            XlsxSheet {
                name: "Sheet1".into(),
                cells: vec![
                    XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) },
                    XlsxCell { row: 1, col: 1, value: XlsxCellValue::SharedString(1) },
                    XlsxCell { row: 2, col: 0, value: XlsxCellValue::SharedString(2) },
                    XlsxCell { row: 2, col: 1, value: XlsxCellValue::Number(95.5) },
                    XlsxCell { row: 3, col: 0, value: XlsxCellValue::Boolean(true) },
                    XlsxCell { row: 3, col: 1, value: XlsxCellValue::Formula { expr: "SUM(B2:B2)".into(), cached: Some(Box::new(XlsxCellValue::Number(95.5))) } },
                ],
            },
            XlsxSheet { name: "Totals".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::InlineString("Total Score".into()) }] },
        ],
        shared_strings: vec!["Name".into(), "Score".into(), "Alice".into()],
    };
    let mut snap = build_minimal_xlsx(workbook);
    snap.opc.set_part("xl/styles.xml", "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml", b"<styleSheet/>".to_vec());
    // 🩹 Normalize `opc.parts`' ORDER to the canonical post-regeneration shape `encode_xlsx`
    // always produces (`regenerate_workbook_parts`'s `retain` keeps any unmodeled part -- here
    // `xl/styles.xml` -- in its CURRENT relative position, then re-appends `workbook.xml`/
    // `sharedStrings.xml`/every worksheet AFTER it; that shape is a fixed point of a further
    // `encode_xlsx`/`decode_xlsx` round trip, but the pre-round-trip in-memory order this
    // function would otherwise return is NOT). Without this, `fixture_honesty_law`'s direct
    // `parsed == demo()` comparison fails on part ORDER alone even though every part's CONTENT
    // round-trips correctly (`XlsxSnapshot`'s derived `PartialEq` is order-sensitive on
    // `opc.parts: Vec<OpcPart>`) -- a real, previously-undiscovered fixture-construction bug this
    // wave's own `fixture_honesty_law` caught live, not assumed.
    let bytes = encode_xlsx(&snap).expect("encode demo xlsx for part-order normalization");
    decode_xlsx(&bytes).expect("decode demo xlsx for part-order normalization")
}

pub fn register() {
    crate::artifacts::xlsx::io_registry::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::xlsx::schema::xlsx_artifact_schema_descriptor());
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<XlsxSnapshot, XlsxMutation>(STDIO_XLSX_DOCUMENT_SCHEMA));
    // 🛡️ D5's generic validate-on-build hook: registers the ✳️strict/✳️transitional subsets'
    // SubsetValidators so `io_dispatch`/`wire_artifact_compose` re-check them for free. Their
    // ComposerEntry values are registered separately via this standard's own `composer::entries()`
    // aggregation (see `crate::artifacts::xlsx::io_registry::register()` above).
    crate::artifacts::xlsx::standards::v_ecma_376::subsets::strict::io::register();
    crate::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::io::register();
}

/// 📌️ FG-wave: 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr), per docx's own
/// just-landed `register_pilot_languages` exemplar (this wave's OPC pattern-setter) --
/// `stdio.xlsx`/`.op`/`.diff`/`.pack`/`.spr`, all `dsl::passthrough_hooks`. `diff`'s `protocol`
/// slot stays `None`, matching the exemplar's own shape exactly (the 5-role scheme has no
/// dedicated "diff binary" role, even though `🔺️diff/💾️binary/📡️component.protocol.semio` is a
/// real, conformance-tested file — its binary form is exercised directly by `protocol_walk_law`
/// below).
///
/// `register_schema_spec` (P2-M3's `FullResolver` insertion API) is deliberately NOT called here —
/// filed as this wave's own `mechanism_gaps` entry: it requires `fn() -> RecordSpec`, and
/// `XlsxSnapshot`/`XlsxDiff`/`XlsxMutation` have none (all three are hand-rolled — see
/// `🧬️schema/🔺️diff/🦀️component.rs`'s/`🧬️schema/🧬️mutations/🦀️component.rs`'s own F6-verification
/// doc comments confirming `#[derive(dsl::Dsl*)]` fails to compile on every one of these types,
/// root cause `XlsxCellValue: DslField` not satisfied plus the generic `NamedTripleDiff<K,D,T>`
/// collection type has no `DslField` impl either), same root cause the sibling json/csv/zip/png
/// pilots' own `register_pilot_languages` doc comments already document.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.xlsx", extension: Some("xlsx"), role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::xlsx::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::xlsx::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::xlsx::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::xlsx::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.xlsx"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.xlsx.op", extension: None, role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::xlsx::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::xlsx::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::xlsx::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::xlsx::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.xlsx.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.xlsx.diff", extension: None, role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::xlsx::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::xlsx::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.xlsx.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.xlsx.pack", extension: None, role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::xlsx::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::xlsx::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.xlsx.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.xlsx.spr", extension: None, role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::xlsx::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::xlsx::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.xlsx.spr"),
    });
}
//#endregion 🔖️Codec

//#region 🔖️Sniff
/// 🕵️ Real xlsx sniff: OPC-shaped bytes whose root officeDocument relationship resolves under
/// `xl/` — disambiguates from docx/pptx sharing the same zip magic and OPC shape.
pub fn sniff_xlsx_bytes(data: &[u8]) -> bool {
    let Ok(opc) = opc::decode_opc(data) else { return false };
    let path = opc
        .resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT)
        .or_else(|| opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT_STRICT));
    match path {
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

    fn cell(row: u32, col: u32, value: XlsxCellValue) -> XlsxCell {
        XlsxCell { row, col, value }
    }

    fn sample_workbook() -> XlsxWorkbook {
        XlsxWorkbook {
            sheets: vec![
                XlsxSheet {
                    name: "Numbers".into(),
                    cells: vec![
                        cell(1, 0, XlsxCellValue::SharedString(0)),
                        cell(1, 1, XlsxCellValue::SharedString(1)),
                        cell(2, 0, XlsxCellValue::SharedString(2)),
                        cell(2, 1, XlsxCellValue::Number(9.5)),
                        cell(3, 0, XlsxCellValue::SharedString(2)),
                        cell(3, 1, XlsxCellValue::Number(-3.0)),
                        cell(4, 0, XlsxCellValue::Boolean(true)),
                        cell(4, 1, XlsxCellValue::Empty),
                    ],
                },
                XlsxSheet { name: "Second".into(), cells: vec![cell(1, 0, XlsxCellValue::SharedString(2))] },
            ],
            shared_strings: vec!["Name".into(), "Score".into(), "Alice".into()],
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
    fn column_index_is_the_real_inverse_of_column_letter() {
        for i in [0u32, 1, 25, 26, 27, 51, 52, 700] {
            assert_eq!(column_index(&column_letter(i)), Some(i), "round trip failed for {i}");
        }
        assert_eq!(column_index(""), None);
        assert_eq!(column_index("1A"), None);
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
    fn shared_strings_are_carried_verbatim_never_resolved_or_deduped() {
        // 🎯️ The engine no longer resolves `SharedString(idx)` into literal text, nor dedupes on
        // encode -- `workbook.shared_strings` IS the SST, passed through directly. Confirms the
        // real bytes carry the table unchanged AND every cell keeps its own index (not a
        // resolved-text copy the old `Text` variant used to collapse into).
        let snap = build_minimal_xlsx(sample_workbook());
        let sst_bytes = snap.opc.part_bytes("xl/sharedStrings.xml").expect("sharedStrings.xml part present");
        let sst_xml = xml_document_from_text(std::str::from_utf8(sst_bytes).unwrap()).expect("parse sst");
        let strings = shared_strings_from_xml(&sst_xml, "xl/sharedStrings.xml").expect("parse strings");
        assert_eq!(strings, vec!["Name".to_string(), "Score".to_string(), "Alice".to_string()]);

        let bytes = encode_xlsx(&snap).expect("encode");
        let re_decoded = decode_xlsx(&bytes).expect("decode");
        assert_eq!(re_decoded.workbook.shared_strings, vec!["Name".to_string(), "Score".to_string(), "Alice".to_string()]);
        assert_eq!(re_decoded.workbook.sheets[0].cells.iter().find(|c| c.row == 2 && c.col == 0).unwrap().value, XlsxCellValue::SharedString(2));
        assert_eq!(re_decoded.workbook.sheets[0].cells.iter().find(|c| c.row == 3 && c.col == 0).unwrap().value, XlsxCellValue::SharedString(2));
        assert_eq!(re_decoded.workbook.sheets[1].cells[0].value, XlsxCellValue::SharedString(2));
    }

    #[test]
    fn decode_resolves_real_hand_built_package_with_every_cell_type() {
        // Hand-built OOXML: real workbook.xml + worksheet + sharedStrings.xml + all rels wired
        // by hand, not a generator shortcut. Exercises every `XlsxCellValue` variant: shared
        // string, number, boolean, inline string, and a formula with a cached numeric result.
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
            r#"<row r="3"><c r="A3" t="b"><v>1</v></c><c r="B3" t="inlineStr"><is><t>literal</t></is></c></row>"#,
            r#"<row r="4"><c r="A4"><f>SUM(A2:B2)</f><v>127.5</v></c></row>"#,
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
        assert_eq!(decoded.workbook.shared_strings, vec!["Quarter".to_string(), "Revenue & Profit".to_string()]);
        let cells = &decoded.workbook.sheets[0].cells;
        let at = |row: u32, col: u32| cells.iter().find(|c| c.row == row && c.col == col).map(|c| &c.value);
        assert_eq!(at(1, 0), Some(&XlsxCellValue::SharedString(0)));
        assert_eq!(at(1, 1), Some(&XlsxCellValue::SharedString(1)));
        assert_eq!(at(2, 0), Some(&XlsxCellValue::Number(4.0)));
        assert_eq!(at(2, 1), Some(&XlsxCellValue::Number(123.5)));
        assert_eq!(at(3, 0), Some(&XlsxCellValue::Boolean(true)));
        assert_eq!(at(3, 1), Some(&XlsxCellValue::InlineString("literal".into())));
        assert_eq!(at(4, 0), Some(&XlsxCellValue::Formula { expr: "SUM(A2:B2)".into(), cached: Some(Box::new(XlsxCellValue::Number(127.5))) }));
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

    #[test]
    fn decode_recognizes_strict_office_document_and_shared_strings_relationship_types() {
        // 🏅️ ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3: a genuinely
        // ISO/IEC 29500-1 Strict-shaped package uses the purl.oclc.org relationship TYPE URIs for
        // the package-root officeDocument pointer and the workbook's sharedStrings relationship --
        // without recognizing those (alongside the Transitional ones), decode would reject every
        // real Strict document outright.
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");

        let sst_xml = concat!(
            r#"<sst xmlns="http://purl.oclc.org/ooxml/spreadsheetml/main" count="1" uniqueCount="1">"#,
            "<si><t>Strict</t></si>",
            "</sst>",
        );
        opc.set_part(SHARED_STRINGS_PART, SHARED_STRINGS_CONTENT_TYPE, sst_xml.as_bytes().to_vec());

        let sheet_xml = concat!(
            r#"<worksheet xmlns="http://purl.oclc.org/ooxml/spreadsheetml/main">"#,
            "<sheetData>",
            r#"<row r="1"><c r="A1" t="s"><v>0</v></c></row>"#,
            "</sheetData>",
            "</worksheet>",
        );
        opc.set_part("xl/worksheets/sheet1.xml", WORKSHEET_CONTENT_TYPE, sheet_xml.as_bytes().to_vec());

        let workbook_xml = concat!(
            r#"<workbook xmlns="http://purl.oclc.org/ooxml/spreadsheetml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships" conformance="strict">"#,
            r#"<sheets><sheet name="S" sheetId="1" r:id="rId1"/></sheets>"#,
            "</workbook>",
        );
        opc.set_part(WORKBOOK_PART, WORKBOOK_CONTENT_TYPE, workbook_xml.as_bytes().to_vec());

        opc.add_relationship(WORKBOOK_PART, "rId1", "http://purl.oclc.org/ooxml/officeDocument/relationships/worksheet", "worksheets/sheet1.xml");
        opc.add_relationship(WORKBOOK_PART, "rId2", REL_TYPE_SHARED_STRINGS_STRICT, "sharedStrings.xml");
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT_STRICT, WORKBOOK_PART);

        let bytes = opc::encode_opc(&opc).expect("encode strict-shaped package");
        assert!(sniff_xlsx_bytes(&bytes), "a Strict-shaped package must still sniff as xlsx");
        let decoded = decode_xlsx(&bytes).expect("decode Strict-shaped package");
        assert_eq!(decoded.workbook.sheets.len(), 1);
        assert_eq!(decoded.workbook.sheets[0].cells[0].value, XlsxCellValue::SharedString(0));
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ FG-wave: per-artifact conformance laws (`📖️grammar-recipe.md` §4's checklist item) --
    /// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
    /// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
    /// bytes, and the fixture-honesty round-trip. Lives here (the engine's own test region), not
    /// any framework file -- same placement docx's own `conformance_laws` module uses (this
    /// wave's OPC pattern-setter); these tests are this artifact's OWN early-warning, plus direct
    /// coverage of the mutations/diff facets the framework's `m5` auto-discovery does not reach at
    /// all.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::xlsx::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect -- independent of, and cheaper than, the two
        /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
        /// message).
        #[test]
        fn committed_facet_files_parse() {
            for (label, text) in [
                ("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO),
                ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO),
            ] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [
                ("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO),
            ] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar models the real TEXT syntax of the
        /// XML parts an xlsx OPC package carries (`📸️snapshot/📝️text/📖️component.grammar.semio`'s
        /// own doc comment explains why -- this artifact's `ArtifactDsl::print_dsl` hex-dumps the
        /// WHOLE binary OPC package, matching this facet's SIBLING binary protocol, not this text
        /// grammar; the two facets describe different LAYERS of the same real artifact, same as
        /// every OPC-family member's own container/contained-parts split). So, UNLIKE a
        /// binary-native pilot's `grammar_conformance_law` (which feeds `print_dsl` output
        /// straight to the recognizer), this law decodes the REAL zip entries `encode_xlsx`
        /// genuinely produces (via `zip::engine::decode_zip`, the same real codec `opc::decode_opc`
        /// itself delegates to) and recognizes EACH real part's own text against the grammar --
        /// direct proof the grammar matches this artifact's own real per-part XML bytes, not an
        /// invented approximation. `worksheet-part`'s own production is generic over the sheet
        /// index, so both `xl/worksheets/sheet1.xml` and `sheet2.xml` are checked against it.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);

            let demo = demo_xlsx_snapshot();
            let bytes = encode_xlsx(&demo).expect("encode demo xlsx");
            let zip = crate::artifacts::zip::engine::decode_zip(&bytes).expect("decode zip");

            let modeled_fixed = ["[Content_Types].xml", "_rels/.rels", "xl/workbook.xml", "xl/_rels/workbook.xml.rels", "xl/sharedStrings.xml"];
            let mut checked = 0;
            for entry in &zip.entries {
                let is_modeled = modeled_fixed.contains(&entry.name.as_str())
                    || (entry.name.starts_with("xl/worksheets/") && entry.name.ends_with(".xml"));
                if !is_modeled {
                    continue;
                }
                let text = String::from_utf8(entry.data.clone()).unwrap_or_else(|e| panic!("part {:?}: not valid utf-8: {e}", entry.name));
                assert!(recognizer.recognize(&text).unwrap_or(false), "grammar did not recognize real part {:?}:\n{text}", entry.name);
                checked += 1;
            }
            // 5 fixed parts + 2 worksheet parts (`demo_xlsx_snapshot()`'s own 2 sheets).
            assert_eq!(checked, modeled_fixed.len() + 2, "not every modeled part was present in the real zip entries");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `XlsxMutation` variant (`mutations::demo_mutation_cases()`).
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `XlsxDiff` (`diff::demo_diff_cases()`).
        #[test]
        fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets --
        /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff`. The snapshot protocol
        /// declares `backward`/`jump` (restated from zip's own real ZIP layout), so `walk_protocol`
        /// correctly does NOT require landing on exactly `bytes.len()` (M2's own documented
        /// exception, `📖️grammar-recipe.md` §2.3) -- assert a sane in-range `consumed` there
        /// instead, same as zip's/docx's own `protocol_walk_law` does; the op/diff protocols have
        /// no such exception and must consume every byte.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let demo = demo_xlsx_snapshot();
            let packed = store::ArtifactPack::encode_pack(&demo);
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert!(trace.consumed > 0 && trace.consumed <= inner.len(), "pack walk consumed an out-of-range span");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_xlsx_snapshot()` -- `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin -- so the
        /// fixtures can never silently drift back to a fake `"68656c6c6f"`-style placeholder again
        /// (see this ticket's own recon note on the pre-FG-wave state of these two files).
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_xlsx_snapshot();

            let parsed = <XlsxSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_xlsx_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_xlsx_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <XlsxSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_xlsx_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_xlsx_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::XlsxComposer as XlsxRawAnyComposer;
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::strict::schema::XlsxStrictComposer;
    use crate::artifacts::xlsx::standards::v_ecma_376::subsets::transitional::schema::XlsxTransitionalComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| vec![composer_entry_of::<XlsxRawAnyComposer>(), composer_entry_of::<XlsxStrictComposer>(), composer_entry_of::<XlsxTransitionalComposer>()])
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
