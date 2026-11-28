//! 📘️ Logical SpreadsheetML workbook projection from authoritative XML parts.

use crate::standards::v_ecma_376::subsets::base::schema::{refusal::XlsxError, vocabulary::*};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
use crate::standards::v_ecma_376::subsets::base::schema::shared_formula::{expand_shared_formula, SharedMaster};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument, XmlNode};
use std::collections::HashMap;

/// 📋️ Shared-formula masters keyed by group index (`si`).
pub type SharedFormulas = HashMap<String, SharedMaster>;
use semio_s_artifact_stdio_zip::opc::{self, REL_TYPE_OFFICE_DOCUMENT};

//#region 🔖️SharedStringsXml
const SPREADSHEETML_NAMESPACES: [&str; 2] = [SML_NS, SML_NS_STRICT];
const OFFICE_RELATIONSHIP_NAMESPACES: [&str; 2] = [R_NS, R_NS_STRICT];

fn spreadsheet_root<'a>(doc: &'a XmlDocument, part: &str, local: &str) -> Result<(&'a XmlNode, Vec<(String, String)>, String), XlsxError> {
    let bad = |detail: String| XlsxError::Xml { part: part.into(), detail };
    let root = doc.root.as_ref().ok_or_else(|| bad("empty document".into()))?;
    let scope = namespace_scope(&[], root);
    if !element_matches(root, &scope, &SPREADSHEETML_NAMESPACES, local).map_err(&bad)? {
        let actual = match root {
            XmlNode::Element { name, .. } => expanded_element_name(name, &scope).map_err(&bad)?.1,
            _ => return Err(bad("root is not an element".into())),
        };
        return Err(bad(format!("expected SpreadsheetML <{local}>, got <{actual}>")));
    }
    let XmlNode::Element { name, .. } = root else { unreachable!() };
    let namespace = expanded_element_name(name, &scope).map_err(&bad)?.0;
    Ok((root, scope, namespace))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn collect_text(node: &XmlNode, parent_scope: &[(String, String)], namespace: &str, out: &mut String) -> Result<(), String> {
    let scope = namespace_scope(parent_scope, node);
    if let XmlNode::Element { name, children, .. } = node {
        if expanded_element_name(name, &scope)? == (namespace.into(), "t".into()) {
            for c in children {
                if let XmlNode::Text { text } = c {
                    out.push_str(text);
                }
            }
        } else {
            for c in children {
                collect_text(c, &scope, namespace, out)?;
            }
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shared_strings_from_xml(doc: &XmlDocument, part: &str) -> Result<Vec<String>, XlsxError> {
    let bad = |detail: String| XlsxError::Xml { part: part.into(), detail };
    let (root, root_scope, namespace) = spreadsheet_root(doc, part, "sst")?;
    let XmlNode::Element { children, .. } = root else { unreachable!() };
    let mut out = Vec::new();
    for si in children {
        let scope = namespace_scope(&root_scope, si);
        if element_matches(si, &scope, &[namespace.as_str()], "si").map_err(&bad)? {
            let mut text = String::new();
            collect_text(si, &root_scope, &namespace, &mut text).map_err(&bad)?;
            out.push(text);
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
    let (root, root_scope, namespace) = spreadsheet_root(doc, part, "workbook")?;
    let XmlNode::Element { children, .. } = root else { unreachable!() };
    let mut sheets_match = None;
    for child in children {
        let scope = namespace_scope(&root_scope, child);
        if element_matches(child, &scope, &[namespace.as_str()], "sheets").map_err(&bad)? {
            let XmlNode::Element { children, .. } = child else { unreachable!() };
            sheets_match = Some((children, scope));
            break;
        }
    }
    let (sheets_el, sheets_scope) = sheets_match.ok_or_else(|| bad("missing SpreadsheetML <sheets>".into()))?;
    let mut out = Vec::new();
    for s in sheets_el {
        let scope = namespace_scope(&sheets_scope, s);
        if !element_matches(s, &scope, &[namespace.as_str()], "sheet").map_err(&bad)? {
            continue;
        }
        let sheet_name = attribute_value(s, &scope, &[""], "name").map_err(&bad)?.ok_or_else(|| bad("<sheet> missing unprefixed name".into()))?.to_string();
        let r_id = attribute_value(s, &scope, &OFFICE_RELATIONSHIP_NAMESPACES, "id").map_err(&bad)?.ok_or_else(|| bad("<sheet> missing namespaced relationship id".into()))?.to_string();
        out.push(SheetRef { name: sheet_name, r_id });
    }
    Ok(out)
}
//#endregion 🔖️WorkbookXml

//#region 🔖️WorksheetXml
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn child_text(children: &[XmlNode], parent_scope: &[(String, String)], namespace: &str, local: &str) -> Result<Option<String>, String> {
    for child in children {
        let scope = namespace_scope(parent_scope, child);
        if element_matches(child, &scope, &[namespace], local)? {
            let XmlNode::Element { children, .. } = child else { unreachable!() };
            let mut text = String::new();
            for node in children {
                if let XmlNode::Text { text: value } = node {
                    text.push_str(value);
                }
            }
            return Ok(Some(text));
        }
    }
    Ok(None)
}

/// 🔎️ Resolves a non-formula `<c>`'s value/cached-value given its `t` attribute (`None` =
/// numeric default). `t="s"` is bounds-checked against `sst_len` (an out-of-range index is a hard
/// `Malformed` error, never a silently-empty cell) but NOT resolved to text here — the caller
/// keeps the index (see the module doc comment). `t="e"`/non-formula `t="str"` normalize to
/// `InlineString` (a documented normalization: this union has no dedicated error/formula-string
/// variant for a BARE cell — see `Formula.cached`, which IS typed, for the formula case).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn extract_typed_value(children: &[XmlNode], cell_scope: &[(String, String)], namespace: &str, t: Option<&str>, sst_len: usize, part: &str) -> Result<XlsxCellValue, XlsxError> {
    let child_text = |local| child_text(children, cell_scope, namespace, local).map_err(|detail| XlsxError::Xml { part: part.into(), detail });
    match t {
        Some("s") => {
            let v = child_text("v")?.ok_or_else(|| XlsxError::Xml { part: part.into(), detail: "t=\"s\" cell missing <v>".into() })?;
            let idx: usize = v.trim().parse().map_err(|_| XlsxError::Malformed(format!("cell in {part}: shared-string index {v:?} is not an integer")))?;
            if idx >= sst_len {
                return Err(XlsxError::Malformed(format!("cell in {part}: shared-string index {idx} out of range ({sst_len} entries)")));
            }
            Ok(XlsxCellValue::SharedString(idx))
        }
        Some("str") => Ok(XlsxCellValue::InlineString(child_text("v")?.unwrap_or_default())),
        Some("inlineStr") => {
            let mut text = String::new();
            for child in children {
                let scope = namespace_scope(cell_scope, child);
                if element_matches(child, &scope, &[namespace], "is").map_err(|detail| XlsxError::Xml { part: part.into(), detail })? {
                    collect_text(child, cell_scope, namespace, &mut text).map_err(|detail| XlsxError::Xml { part: part.into(), detail })?;
                    break;
                }
            }
            Ok(XlsxCellValue::InlineString(text))
        }
        Some("b") => {
            let v = child_text("v")?.unwrap_or_default();
            Ok(XlsxCellValue::Boolean(v.trim() == "1" || v.trim().eq_ignore_ascii_case("true")))
        }
        Some("e") => Ok(XlsxCellValue::Error(child_text("v")?.unwrap_or_default())),
        None | Some("n") => match child_text("v")? {
            Some(v) => v.trim().parse::<f64>().map(XlsxCellValue::Number).map_err(|_| XlsxError::Malformed(format!("cell in {part}: invalid numeric value {v:?}"))),
            None => Ok(XlsxCellValue::Empty),
        },
        Some(_) => Ok(XlsxCellValue::InlineString(child_text("v")?.unwrap_or_default())),
    }
}

/// 🧭️ One worksheet cell element with its document path and logical coordinates (1-based row, 0-based column).
pub struct WorksheetCellRef<'a> {
    pub path: Vec<usize>,
    pub row: u32,
    pub col: u32,
    pub node: &'a XmlNode,
    pub scope: Vec<(String, String)>,
    pub namespace: String,
}

/// 🧭️ Every cell of the worksheet whose root element is `root`, in document order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn worksheet_cell_refs(root: &XmlNode) -> Result<Vec<WorksheetCellRef<'_>>, String> {
    let root_scope = namespace_scope(&[], root);
    let XmlNode::Element { name, children, .. } = root else { return Err("worksheet root is not an element".into()) };
    let (namespace, _) = expanded_element_name(name, &root_scope)?;
    let mut cells = Vec::new();
    for (sheet_data_index, child) in children.iter().enumerate() {
        let scope = namespace_scope(&root_scope, child);
        if !element_matches(child, &scope, &[namespace.as_str()], "sheetData")? {
            continue;
        }
        let XmlNode::Element { children: rows, .. } = child else { unreachable!() };
        for row in positioned_worksheet_children(rows, &scope, &namespace, WorksheetAxis::Rows) {
            let row = row?;
            let XmlNode::Element { children: row_children, .. } = row.node else { unreachable!() };
            for cell in positioned_worksheet_children(row_children, &row.scope, &namespace, WorksheetAxis::Columns(row.position)) {
                let cell = cell?;
                cells.push(WorksheetCellRef { path: vec![sheet_data_index, row.index, cell.index], row: row.position, col: cell.position, node: cell.node, scope: cell.scope, namespace: namespace.clone() });
            }
        }
        break;
    }
    Ok(cells)
}

/// 🔎️ The `<f>` element of a cell and its `t`/`si` attributes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn formula_element<'a>(children: &'a [XmlNode], cell_scope: &[(String, String)], namespace: &str) -> Result<Option<(&'a XmlNode, Vec<(String, String)>)>, String> {
    for child in children {
        let scope = namespace_scope(cell_scope, child);
        if element_matches(child, &scope, &[namespace], "f")? {
            return Ok(Some((child, scope)));
        }
    }
    Ok(None)
}

/// 🔖️ The shared-formula group index a `<f t="shared" si>` element belongs to; `None` for every other formula.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn shared_formula_index(children: &[XmlNode], cell_scope: &[(String, String)], namespace: &str) -> Result<Option<String>, String> {
    let Some((node, scope)) = formula_element(children, cell_scope, namespace)? else { return Ok(None) };
    if attribute_value(node, &scope, &[""], "t")? != Some("shared") {
        return Ok(None);
    }
    Ok(attribute_value(node, &scope, &[""], "si")?.map(str::to_string))
}

/// 📋️ The master of every shared-formula group of `cells`, keyed by its `si`: the cell of that group that spells the formula text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn shared_formula_masters(cells: &[WorksheetCellRef<'_>]) -> Result<SharedFormulas, String> {
    let mut masters = SharedFormulas::new();
    for cell in cells {
        let XmlNode::Element { children, .. } = cell.node else { continue };
        let Some(index) = shared_formula_index(children, &cell.scope, &cell.namespace)? else { continue };
        let expr = child_text(children, &cell.scope, &cell.namespace, "f")?.unwrap_or_default();
        if !expr.is_empty() {
            masters.entry(index).or_insert(SharedMaster { expr, row: cell.row, col: cell.col });
        }
    }
    Ok(masters)
}

/// 🔎️ Resolves one `<c>` element's full value — a `<f>` child present makes this a `Formula`
/// cell (ECMA-376 §18.3.1.40); its `cached` is the SAME `<v>`/`t` pair, re-typed by
/// `extract_typed_value` (absent `<v>` = uncalculated, `cached: None`). A dependent shared-formula
/// cell (`<f t="shared" si/>` without text) spells its master's formula translated to `position`
/// (an orphan or an unknown position keeps the empty text).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn extract_cell_value(children: &[XmlNode], cell_scope: &[(String, String)], namespace: &str, t: Option<&str>, sst_len: usize, part: &str, shared: &SharedFormulas, position: Option<(u32, u32)>) -> Result<XlsxCellValue, XlsxError> {
    let bad = |detail| XlsxError::Xml { part: part.into(), detail };
    if let Some(mut expr) = child_text(children, cell_scope, namespace, "f").map_err(bad)? {
        if expr.is_empty() {
            let master = shared_formula_index(children, cell_scope, namespace).map_err(bad)?.and_then(|index| shared.get(&index));
            if let (Some(master), Some((row, col))) = (master, position) {
                expr = expand_shared_formula(master, row, col);
            }
        }
        let cached = if child_text(children, cell_scope, namespace, "v").map_err(bad)?.is_some() { Some(Box::new(extract_typed_value(children, cell_scope, namespace, t, sst_len, part)?)) } else { None };
        return Ok(XlsxCellValue::Formula { expr, cached });
    }
    extract_typed_value(children, cell_scope, namespace, t, sst_len, part)
}

/// 🌳 Projects sparse cells from explicit and ordinal SpreadsheetML references.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn worksheet_cells_from_xml(doc: &XmlDocument, sst_len: usize, part: &str) -> Result<Vec<XlsxCell>, XlsxError> {
    let bad = |detail: String| XlsxError::Xml { part: part.into(), detail };
    let (root, _, _) = spreadsheet_root(doc, part, "worksheet")?;
    let refs = worksheet_cell_refs(root).map_err(&bad)?;
    if refs.is_empty() && !has_sheet_data(root).map_err(&bad)? {
        return Err(bad("missing SpreadsheetML <sheetData>".into()));
    }
    let masters = shared_formula_masters(&refs).map_err(&bad)?;
    let mut cells = Vec::with_capacity(refs.len());
    for cell in &refs {
        let XmlNode::Element { children, .. } = cell.node else { unreachable!() };
        let t = attribute_value(cell.node, &cell.scope, &[""], "t").map_err(&bad)?;
        let value = extract_cell_value(children, &cell.scope, &cell.namespace, t, sst_len, part, &masters, Some((cell.row, cell.col)))?;
        cells.push(XlsxCell { row: cell.row, col: cell.col, value });
    }
    Ok(cells)
}

/// 🔎️ Whether the worksheet `root` has a SpreadsheetML `sheetData` child.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn has_sheet_data(root: &XmlNode) -> Result<bool, String> {
    let root_scope = namespace_scope(&[], root);
    let XmlNode::Element { name, children, .. } = root else { return Ok(false) };
    let (namespace, _) = expanded_element_name(name, &root_scope)?;
    for child in children {
        if element_matches(child, &namespace_scope(&root_scope, child), &[namespace.as_str()], "sheetData")? {
            return Ok(true);
        }
    }
    Ok(false)
}
//#endregion 🔖️WorksheetXml

//#region 🔖️Codec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn project_snapshot_workbook(snapshot: &XlsxSnapshot) -> Result<XlsxWorkbook, XlsxError> {
    let workbook_path = snapshot.opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).or_else(|| snapshot.opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT_STRICT)).ok_or(XlsxError::MissingWorkbookRelationship)?;
    let workbook_xml = &snapshot.xml_part(&workbook_path).ok_or_else(|| XlsxError::MissingPart(workbook_path.clone()))?.document;
    let sheet_refs = workbook_sheets_from_xml(workbook_xml, &workbook_path)?;

    let workbook_rels = snapshot.opc.relationships_for(&workbook_path);
    let shared_strings = match workbook_rels.iter().find(|r| r.rel_type == REL_TYPE_SHARED_STRINGS || r.rel_type == REL_TYPE_SHARED_STRINGS_STRICT) {
        Some(rel) => {
            let path = opc::resolve_relationship_target(&workbook_path, &rel.target);
            let doc = &snapshot.xml_part(&path).ok_or_else(|| XlsxError::MissingPart(path.clone()))?.document;
            shared_strings_from_xml(doc, &path)?
        }
        None => Vec::new(),
    };

    let sst_len = shared_strings.len();
    let mut sheets = Vec::with_capacity(sheet_refs.len());
    for sheet_ref in &sheet_refs {
        let rel = workbook_rels.iter().find(|r| r.id == sheet_ref.r_id).ok_or_else(|| XlsxError::Malformed(format!("sheet {:?} references unknown relationship id {}", sheet_ref.name, sheet_ref.r_id)))?;
        let path = opc::resolve_relationship_target(&workbook_path, &rel.target);
        let doc = &snapshot.xml_part(&path).ok_or_else(|| XlsxError::MissingPart(path.clone()))?.document;
        let cells = worksheet_cells_from_xml(doc, sst_len, &path)?;
        sheets.push(XlsxSheet { name: sheet_ref.name.clone(), cells });
    }

    Ok(XlsxWorkbook { sheets, shared_strings })
}

