//! 🧩️ SpreadsheetML (xlsx) import — `xl/workbook.xml`/`xl/worksheets/sheetN.xml`/
//! `xl/sharedStrings.xml` XML parse into an `XlsxWorkbook`, real OPC package decode, and
//! magic-shape sniff. Zip/OPC/XML byte-level work is never reimplemented here: it is reused from
//! the shared `crate::artifacts::zip::opc` layer.

use super::super::super::{attr_val, column_index, column_letters_of, XlsxError, REL_TYPE_OFFICE_DOCUMENT_STRICT, REL_TYPE_SHARED_STRINGS, REL_TYPE_SHARED_STRINGS_STRICT};
use crate::artifacts::xlsx::{
    schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook},
    XlsxSnapshot,
};
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, XmlDocument, XmlNode};
use crate::artifacts::zip::opc::{self, REL_TYPE_OFFICE_DOCUMENT};

//#region 🔖️SharedStringsXml
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn workbook_sheets_from_xml(doc: &XmlDocument, part: &str) -> Result<Vec<SheetRef>, XlsxError> {
    let bad = |detail: String| XlsxError::Xml { part: part.into(), detail };
    let root = doc.root.as_ref().ok_or_else(|| bad("empty document".into()))?;
    let XmlNode::Element { name, children, .. } = root else { return Err(bad("root is not an element".into())) };
    if name != "workbook" {
        return Err(bad(format!("expected <workbook>, got <{name}>")));
    }
    let sheets_el = children
        .iter()
        .find_map(|c| match c {
            XmlNode::Element { name, children, .. } if name == "sheets" => Some(children),
            _ => None,
        })
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
            let is_children = children.iter().find_map(|c| match c {
                XmlNode::Element { name, children, .. } if name == "is" => Some(children),
                _ => None,
            });
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn extract_cell_value(children: &[XmlNode], t: Option<&str>, sst_len: usize, part: &str) -> Result<XlsxCellValue, XlsxError> {
    if let Some(expr) = find_f_text(children) {
        let cached = if find_v_text(children).is_some() { Some(Box::new(extract_typed_value(children, t, sst_len, part)?)) } else { None };
        return Ok(XlsxCellValue::Formula { expr, cached });
    }
    extract_typed_value(children, t, sst_len, part)
}

/// 🌳 Flattens `<sheetData>`'s `<row>`-then-`<c>` nesting into `sheet.cells`'s sparse
/// `(row, col)`-addressed list — `row` from the enclosing `<row r>`, `col` from the cell's own
/// `<c r>` column-letter prefix (`col` in the cell's own `r` MUST agree with the row-digit suffix
/// per spec; only the column letters carry information this decoder doesn't already have).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn worksheet_cells_from_xml(doc: &XmlDocument, sst_len: usize, part: &str) -> Result<Vec<XlsxCell>, XlsxError> {
    let bad = |detail: String| XlsxError::Xml { part: part.into(), detail };
    let root = doc.root.as_ref().ok_or_else(|| bad("empty document".into()))?;
    let XmlNode::Element { name, children, .. } = root else { return Err(bad("root is not an element".into())) };
    if name != "worksheet" {
        return Err(bad(format!("expected <worksheet>, got <{name}>")));
    }
    let sheet_data = children
        .iter()
        .find_map(|c| match c {
            XmlNode::Element { name, children, .. } if name == "sheetData" => Some(children),
            _ => None,
        })
        .ok_or_else(|| bad("missing <sheetData>".into()))?;
    let mut cells = Vec::new();
    for row_node in sheet_data {
        let XmlNode::Element { name, attrs, children: row_children } = row_node else { continue };
        if name != "row" {
            continue;
        }
        let row = attr_val(attrs, "r").ok_or_else(|| bad("<row> missing r".into()))?.parse::<u32>().map_err(|_| bad("<row> r attribute is not a valid integer".into()))?;
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_xlsx(data: &[u8]) -> Result<XlsxSnapshot, XlsxError> {
    let opc = opc::decode_opc(data)?;
    // 🏅️ Recognizes either the Transitional or Strict officeDocument relationship TYPE (see the
    // `REL_TYPE_OFFICE_DOCUMENT_STRICT` doc comment above) -- additive, doesn't change decode for
    // any existing Transitional package.
    let workbook_path = opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).or_else(|| opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT_STRICT)).ok_or(XlsxError::MissingWorkbookRelationship)?;
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
        let rel = workbook_rels.iter().find(|r| r.id == sheet_ref.r_id).ok_or_else(|| XlsxError::Malformed(format!("sheet {:?} references unknown relationship id {}", sheet_ref.name, sheet_ref.r_id)))?;
        let path = opc::resolve_relationship_target(&workbook_path, &rel.target);
        let bytes = opc.part_bytes(&path).ok_or_else(|| XlsxError::MissingPart(path.clone()))?;
        let text = String::from_utf8(bytes.to_vec()).map_err(|_| XlsxError::Xml { part: path.clone(), detail: "not valid utf-8".into() })?;
        let doc = xml_document_from_text(&text).map_err(|e| XlsxError::Xml { part: path.clone(), detail: e })?;
        let cells = worksheet_cells_from_xml(&doc, sst_len, &path)?;
        sheets.push(XlsxSheet { name: sheet_ref.name.clone(), cells });
    }

    Ok(XlsxSnapshot::from_parts(opc, XlsxWorkbook { sheets, shared_strings }))
}
//#endregion 🔖️Codec

//#region 🔖️Sniff
/// 🕵️ Real xlsx sniff: OPC-shaped bytes whose root officeDocument relationship resolves under
/// `xl/` — disambiguates from docx/pptx sharing the same zip magic and OPC shape.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sniff_xlsx_bytes(data: &[u8]) -> bool {
    let Ok(opc) = opc::decode_opc(data) else { return false };
    let path = opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).or_else(|| opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT_STRICT));
    match path {
        Some(path) => path.starts_with("xl/"),
        None => false,
    }
}
//#endregion 🔖️Sniff
