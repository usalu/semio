//! 🧭️ Declarative SpreadsheetML mutation plans: every plan reads the BASE snapshot and builds the sparse typed diff of its own kind straight from the
//! payload — never a clone of the snapshot, never a recomputed difference — together with the concrete inverse that restores the exact absolute base
//! value. Revision-bound addresses an inverse must carry are computed against the state the forward leaves behind.

use super::*;
use crate::schema::diff::{NamedModified, NamedTripleDiff, XlsxOpcContentTypesDiff, XlsxOpcDiff, XlsxOpcRelListDiff, XlsxXmlPartDiff};
use crate::schema::snapshot::XlsxXmlPart;
use crate::schema::vocabulary::{attribute_value, column_index, column_letter, column_letters_of, element_matches, expanded_element_name, namespace_scope, REL_TYPE_WORKSHEET, R_NS, R_NS_STRICT, SML_NS, SML_NS_STRICT, WORKSHEET_CONTENT_TYPE};
use crate::standards::v_ecma_376::subsets::base::schema::construction::worksheet_to_xml_with_namespace;
use semio_s_artifact_stdio_xml::schema::diff::{diff_at_path, XmlAttrModified, XmlAttributesDiff, XmlChildAdded, XmlChildModified, XmlChildrenDiff, XmlDiff, XmlElementDiff, XmlNodeDiff};
use semio_s_artifact_stdio_zip::opc::{fresh_relationship_id, resolve_relationship_target, OpcRelationship, OpcTargetMode};

const SPREADSHEETML_NAMESPACES: [&str; 2] = [SML_NS, SML_NS_STRICT];
const OFFICE_RELATIONSHIP_NAMESPACES: [&str; 2] = [R_NS, R_NS_STRICT];

/// 🧩️ The sparse diff one leaf produces, with the concrete mutations that undo it.
pub(crate) struct XlsxPlan {
    pub(crate) diff: XlsxDiff,
    pub(crate) inverse: Vec<XlsxMutation>,
}

type Planned = Result<XlsxPlan, String>;

//#region 🔖️DiffHelpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn qualified_like(parent: &str, local: &str) -> String {
    parent.split_once(':').map_or_else(|| local.into(), |(prefix, _)| format!("{prefix}:{local}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn text_element(name: String, text: impl Into<String>) -> XmlNode {
    XmlNode::Element { name, attrs: Vec::new(), children: vec![XmlNode::Text { text: text.into() }] }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn set_attr(attrs: &mut Vec<XmlAttr>, name: &str, value: Option<String>) {
    if let Some(index) = attrs.iter().position(|attr| attr.name == name) {
        if let Some(value) = value {
            attrs[index].value = value;
        } else {
            attrs.remove(index);
        }
    } else if let Some(value) = value {
        attrs.push(XmlAttr { name: name.into(), value });
    }
}

/// 🌳 The element diff that only edits the children of its target.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn children_leaf(removed: Vec<usize>, modified: Vec<XmlChildModified>, added: Vec<XmlChildAdded>) -> XmlNodeDiff {
    XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed, modified, added }) })
}

/// 🌳 The element diff that sets attribute `name` of an element holding `attrs` to `value`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn attribute_leaf(attrs: &[XmlAttr], name: &str, value: &str) -> XmlNodeDiff {
    XmlNodeDiff::Element(XmlElementDiff {
        name: None,
        attributes: Some(XmlAttributesDiff { order: attrs.iter().map(|attr| attr.name.clone()).collect(), modified: vec![XmlAttrModified { name: name.into(), value: value.into() }], ..Default::default() }),
        children: None,
    })
}

/// 🧩️ The diff that applies `leaf` to the node `node_path` names inside XML part `part_path`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part_diff(part_path: &str, node_path: &[usize], leaf: XmlNodeDiff) -> XlsxDiff {
    modified_parts(vec![(part_path.to_string(), diff_at_path(node_path, leaf))])
}

/// 🧩️ The diff that applies one XML diff to each named part.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn modified_parts(edits: Vec<(String, XmlDiff)>) -> XlsxDiff {
    if edits.is_empty() {
        return XlsxDiff::default();
    }
    let modified = edits.into_iter().map(|(key, document)| NamedModified { key, diff: XlsxXmlPartDiff { content_type: None, document: Some(document) } }).collect();
    XlsxDiff { opc: None, xml_parts: Some(NamedTripleDiff { modified, ..Default::default() }) }
}

/// 🧭️ The complete final key order of a collection after `key` is inserted at `position` (`None`, or the end, appends, which needs no order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn insertion_order(existing: Vec<String>, key: String, position: Option<usize>) -> Vec<String> {
    let Some(position) = position.filter(|position| *position < existing.len()) else { return Vec::new() };
    let mut order = existing;
    order.insert(position, key);
    order
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn node_at<'a>(root: &'a XmlNode, path: &[usize]) -> Option<&'a XmlNode> {
    let Some((&index, rest)) = path.split_first() else { return Some(root) };
    let XmlNode::Element { children, .. } = root else { return None };
    node_at(children.get(index)?, rest)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn node_at_mut<'a>(root: &'a mut XmlNode, path: &[usize]) -> Option<&'a mut XmlNode> {
    let Some((&index, rest)) = path.split_first() else { return Some(root) };
    let XmlNode::Element { children, .. } = root else { return None };
    node_at_mut(children.get_mut(index)?, rest)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn part_root<'a>(snapshot: &'a XlsxSnapshot, part_path: &str) -> Result<&'a XmlNode, String> {
    snapshot.xml_part(part_path).and_then(|part| part.document.root.as_ref()).ok_or_else(|| format!("missing XML part {part_path}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn workbook_path(snapshot: &XlsxSnapshot) -> Result<String, String> {
    snapshot
        .opc
        .resolve_relationship("", semio_s_artifact_stdio_zip::opc::REL_TYPE_OFFICE_DOCUMENT)
        .or_else(|| snapshot.opc.resolve_relationship("", crate::standards::v_ecma_376::subsets::base::schema::vocabulary::REL_TYPE_OFFICE_DOCUMENT_STRICT))
        .ok_or_else(|| "missing workbook relationship".to_string())
}
//#endregion 🔖️DiffHelpers

//#region 🔖️CellNodes
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cell_value_nodes(cell_name: &str, current: &[XmlNode], scope: &[(String, String)], namespace: &str, value: &XlsxCellValue) -> Result<(Option<String>, Vec<XmlNode>), String> {
    let child_name = |local| qualified_like(cell_name, local);
    let mut formula_attrs = None;
    for node in current {
        let child_scope = namespace_scope(scope, node);
        if element_matches(node, &child_scope, &[namespace], "f")? {
            let XmlNode::Element { attrs, .. } = node else { unreachable!() };
            formula_attrs = Some(attrs.clone());
            break;
        }
    }
    let v = |text: String| text_element(child_name("v"), text);
    Ok(match value {
        XlsxCellValue::Number(number) => (None, vec![v(number.to_string())]),
        XlsxCellValue::SharedString(index) => (Some("s".into()), vec![v(index.to_string())]),
        XlsxCellValue::InlineString(text) => (
            Some("inlineStr".into()),
            vec![XmlNode::Element {
                name: child_name("is"),
                attrs: Vec::new(),
                children: vec![XmlNode::Element { name: child_name("t"), attrs: vec![XmlAttr { name: "xml:space".into(), value: "preserve".into() }], children: vec![XmlNode::Text { text: text.clone() }] }],
            }],
        ),
        XlsxCellValue::Boolean(value) => (Some("b".into()), vec![v(if *value { "1".into() } else { "0".into() })]),
        XlsxCellValue::Error(error) => (Some("e".into()), vec![v(error.clone())]),
        XlsxCellValue::Formula { expr, cached } => {
            let mut children = vec![XmlNode::Element { name: child_name("f"), attrs: formula_attrs.unwrap_or_default(), children: vec![XmlNode::Text { text: expr.clone() }] }];
            let cell_type = match cached.as_deref() {
                Some(XlsxCellValue::Number(number)) => {
                    children.push(v(number.to_string()));
                    None
                }
                Some(XlsxCellValue::SharedString(index)) => {
                    children.push(v(index.to_string()));
                    Some("s".into())
                }
                Some(XlsxCellValue::InlineString(text)) => {
                    children.push(v(text.clone()));
                    Some("str".into())
                }
                Some(XlsxCellValue::Boolean(value)) => {
                    children.push(v(if *value { "1".into() } else { "0".into() }));
                    Some("b".into())
                }
                Some(XlsxCellValue::Error(error)) => {
                    children.push(v(error.clone()));
                    Some("e".into())
                }
                _ => None,
            };
            (cell_type, children)
        }
        XlsxCellValue::Empty => (None, Vec::new()),
    })
}

/// 🔎️ The typed value the cell element `node` holds, read through the same projection the snapshot's workbook view uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cell_value_of(snapshot: &XlsxSnapshot, part_path: &str, node: &XmlNode, scope: &[(String, String)], namespace: &str) -> Result<XlsxCellValue, String> {
    let XmlNode::Element { children, .. } = node else { return Err("XLSX cell address resolved a non-element".into()) };
    let cell_type = attribute_value(node, scope, &[""], "t")?;
    let shared = shared_strings(snapshot).map_or(0, |table| table.positions.len());
    crate::standards::v_ecma_376::subsets::base::schema::inferences::workbook::extract_cell_value(children, scope, namespace, cell_type, shared, part_path).map_err(|error| error.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cell_column(node: &XmlNode, scope: &[(String, String)]) -> Result<Option<u32>, String> {
    if !element_matches(node, scope, &SPREADSHEETML_NAMESPACES, "c")? {
        return Ok(None);
    }
    let Some(reference) = attribute_value(node, scope, &[""], "r")? else { return Ok(None) };
    Ok(column_index(column_letters_of(reference)))
}
//#endregion 🔖️CellNodes

//#region 🔖️CellPlans
/// ✍️ Replaces the value of the addressed cell; the inverse restores the value it held, named by the address the cell carries afterwards.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn set_cell_plan(snapshot: &XlsxSnapshot, address: &cell_address::XlsxCellAddress, value: &XlsxCellValue) -> Planned {
    let resolved = cell_address::resolve_xlsx_cell_address(snapshot, address)?;
    let scope = cell_address::addressed_cell_scope(snapshot, address)?;
    let XmlNode::Element { name, attrs, children } = resolved.node else { return Err("XLSX cell address resolved a non-element".into()) };
    let old_value = cell_value_of(snapshot, &address.part_path, resolved.node, &scope, &address.namespace_uri)?;
    let (cell_type, replacement) = cell_value_nodes(name, children, &scope, &address.namespace_uri, value)?;
    let mut new_attrs = attrs.clone();
    set_attr(&mut new_attrs, "t", cell_type);
    let mut kept = Vec::new();
    let mut insertion = None;
    for child in children {
        let child_scope = namespace_scope(&scope, child);
        let mut is_value_child = false;
        for local in ["f", "v", "is"] {
            is_value_child |= element_matches(child, &child_scope, &[address.namespace_uri.as_str()], local)?;
        }
        if is_value_child {
            insertion.get_or_insert(kept.len());
        } else {
            kept.push(child.clone());
        }
    }
    let insertion = insertion.unwrap_or(kept.len()).min(kept.len());
    kept.splice(insertion..insertion, replacement);
    let next = XmlNode::Element { name: name.clone(), attrs: new_attrs, children: kept };
    if next == *resolved.node {
        return Ok(XlsxPlan { diff: XlsxDiff::default(), inverse: Vec::new() });
    }
    let mut after = part_root(snapshot, &address.part_path)?.clone();
    *node_at_mut(&mut after, &address.node_path).ok_or_else(|| "XLSX cell address is stale".to_string())? = next.clone();
    let restored = cell_address::cell_address_in(snapshot, &address.part_path, &after, address.node_path.clone())?;
    Ok(XlsxPlan {
        diff: part_diff(&address.part_path, &address.node_path, XmlNodeDiff::Replace { node: Some(next) }),
        inverse: vec![XlsxMutation::SetCell(set_cell::SetCell { address: restored, value: old_value })],
    })
}

/// ➕️ Inserts one cell into a vacancy, creating its row when the row is missing; the inverse removes the cell by the address it carries afterwards.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn insert_cell_plan(snapshot: &XlsxSnapshot, address: &cell_address::XlsxCellVacancyAddress, value: &XlsxCellValue) -> Planned {
    let resolved = cell_address::resolve_xlsx_cell_vacancy_address(snapshot, address)?;
    let worksheet = &address.worksheet;
    let namespace = worksheet.namespace_uri.clone();
    let row_number = address.row;
    let column = address.column;
    let reference = format!("{}{}", column_letter(column), row_number);
    let sheet_data_scope = cell_address::addressed_worksheet_scope(snapshot, worksheet)?;
    let XmlNode::Element { name: sheet_data_name, children: rows, .. } = resolved.node else { return Err("worksheet address resolved a non-element".into()) };
    let make_cell = |cell_name: String, scope: &[(String, String)]| -> Result<XmlNode, String> {
        let (cell_type, children) = cell_value_nodes(&cell_name, &[], scope, &namespace, value)?;
        let mut attrs = vec![XmlAttr { name: "r".into(), value: reference.clone() }];
        if let Some(cell_type) = cell_type {
            attrs.push(XmlAttr { name: "t".into(), value: cell_type });
        }
        Ok(XmlNode::Element { name: cell_name, attrs, children })
    };
    let mut matching_row = None;
    let mut row_insertion = rows.len();
    let mut last_row = None;
    for (index, node) in rows.iter().enumerate() {
        let row_scope = namespace_scope(&sheet_data_scope, node);
        if !element_matches(node, &row_scope, &[namespace.as_str()], "row")? {
            continue;
        }
        let Some(existing) = attribute_value(node, &row_scope, &[""], "r")?.and_then(|value| value.parse::<u32>().ok()) else { continue };
        if existing == row_number {
            matching_row = Some((index, row_scope));
            break;
        }
        if existing > row_number {
            row_insertion = index;
            break;
        }
        last_row = Some(index);
    }
    let mut after = part_root(snapshot, &worksheet.part_path)?.clone();
    let (diff, cell_path) = if let Some((row_index, row_scope)) = matching_row {
        let XmlNode::Element { name: row_name, children, .. } = &rows[row_index] else { unreachable!() };
        let mut insertion = children.len();
        let mut last_cell = None;
        for (index, node) in children.iter().enumerate() {
            let cell_scope = namespace_scope(&row_scope, node);
            let Some(existing_column) = cell_column(node, &cell_scope)? else { continue };
            if existing_column > column {
                insertion = index;
                break;
            }
            last_cell = Some(index);
        }
        if insertion == children.len() {
            insertion = last_cell.map_or(0, |index| index + 1);
        }
        let cell = make_cell(qualified_like(row_name, "c"), &row_scope)?;
        let mut row_path = worksheet.node_path.clone();
        row_path.push(row_index);
        let XmlNode::Element { children: after_children, .. } = node_at_mut(&mut after, &row_path).ok_or_else(|| "XLSX worksheet address is stale".to_string())? else { unreachable!() };
        after_children.insert(insertion, cell.clone());
        let mut cell_path = row_path.clone();
        cell_path.push(insertion);
        (part_diff(&worksheet.part_path, &row_path, children_leaf(Vec::new(), Vec::new(), vec![XmlChildAdded { index: insertion, item: cell }])), cell_path)
    } else {
        if row_insertion == rows.len() {
            row_insertion = last_row.map_or(0, |index| index + 1);
        }
        let row_name = qualified_like(sheet_data_name, "row");
        let cell = make_cell(qualified_like(&row_name, "c"), &sheet_data_scope)?;
        let row = XmlNode::Element { name: row_name, attrs: vec![XmlAttr { name: "r".into(), value: row_number.to_string() }], children: vec![cell] };
        let XmlNode::Element { children: after_rows, .. } = node_at_mut(&mut after, &worksheet.node_path).ok_or_else(|| "XLSX worksheet address is stale".to_string())? else { unreachable!() };
        after_rows.insert(row_insertion, row.clone());
        let mut cell_path = worksheet.node_path.clone();
        cell_path.extend([row_insertion, 0]);
        (part_diff(&worksheet.part_path, &worksheet.node_path, children_leaf(Vec::new(), Vec::new(), vec![XmlChildAdded { index: row_insertion, item: row }])), cell_path)
    };
    let created = cell_address::cell_address_in(snapshot, &worksheet.part_path, &after, cell_path)?;
    Ok(XlsxPlan { diff, inverse: vec![XlsxMutation::RemoveCell(remove_cell::RemoveCell { address: created })] })
}

/// ➖️ Removes the addressed cell, dropping its row when the cell was the row's only content; the inverse inserts the value the cell held into the vacancy.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn remove_cell_plan(snapshot: &XlsxSnapshot, address: &cell_address::XlsxCellAddress) -> Planned {
    let resolved = cell_address::resolve_xlsx_cell_address(snapshot, address)?;
    let scope = cell_address::addressed_cell_scope(snapshot, address)?;
    let old_value = cell_value_of(snapshot, &address.part_path, resolved.node, &scope, &address.namespace_uri)?;
    let (&cell_index, row_path) = address.node_path.split_last().ok_or_else(|| "cannot remove worksheet root".to_string())?;
    let (&row_index, sheet_data_path) = row_path.split_last().ok_or_else(|| "cell has no row".to_string())?;
    let reference = attribute_value(resolved.node, &scope, &[""], "r")?.ok_or_else(|| "cell has no reference".to_string())?;
    let column = column_index(column_letters_of(reference)).ok_or_else(|| format!("cell reference {reference:?} has no valid column"))?;
    let row: u32 = reference.trim_start_matches(|character: char| character.is_ascii_alphabetic()).parse().map_err(|_| format!("cell reference {reference:?} has no valid row"))?;
    let root = part_root(snapshot, &address.part_path)?;
    let XmlNode::Element { attrs: row_attrs, children: row_children, .. } = node_at(root, row_path).ok_or_else(|| "cell parent path is stale".to_string())? else { return Err("cell parent is not an element".into()) };
    let drops_row = row_children.len() == 1 && row_attrs.len() == 1 && row_attrs[0].name == "r";
    let mut after = root.clone();
    let diff = if drops_row {
        let XmlNode::Element { children, .. } = node_at_mut(&mut after, sheet_data_path).ok_or_else(|| "cell parent path is stale".to_string())? else { unreachable!() };
        children.remove(row_index);
        part_diff(&address.part_path, sheet_data_path, children_leaf(vec![row_index], Vec::new(), Vec::new()))
    } else {
        let XmlNode::Element { children, .. } = node_at_mut(&mut after, row_path).ok_or_else(|| "cell parent path is stale".to_string())? else { unreachable!() };
        children.remove(cell_index);
        part_diff(&address.part_path, row_path, children_leaf(vec![cell_index], Vec::new(), Vec::new()))
    };
    let worksheet = cell_address::worksheet_address_in(snapshot, &address.part_path, &after, sheet_data_path.to_vec())?;
    Ok(XlsxPlan { diff, inverse: vec![XlsxMutation::InsertCell(insert_cell::InsertCell { address: cell_address::XlsxCellVacancyAddress { worksheet, row, column }, value: old_value })] })
}
//#endregion 🔖️CellPlans

//#region 🔖️SharedStringPlans
struct SharedStrings<'a> {
    path: String,
    scope: Vec<(String, String)>,
    namespace: String,
    attrs: &'a [XmlAttr],
    children: &'a [XmlNode],
    positions: Vec<usize>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shared_strings(snapshot: &XlsxSnapshot) -> Result<SharedStrings<'_>, String> {
    let workbook = workbook_path(snapshot)?;
    let relationship = snapshot.opc.relationships_for(&workbook).iter().find(|relationship| relationship.rel_type.ends_with("/sharedStrings")).ok_or_else(|| "workbook has no shared strings relationship".to_string())?;
    let path = resolve_relationship_target(&workbook, &relationship.target);
    let root = part_root(snapshot, &path).map_err(|_| format!("missing shared strings part {path}"))?;
    let scope = namespace_scope(&[], root);
    let XmlNode::Element { name, attrs, children } = root else { return Err("shared strings part has no element root".into()) };
    let (namespace, local) = expanded_element_name(name, &scope)?;
    if local != "sst" || !SPREADSHEETML_NAMESPACES.contains(&namespace.as_str()) {
        return Err("shared strings root is not a SpreadsheetML sst element".into());
    }
    let mut positions = Vec::new();
    for (position, node) in children.iter().enumerate() {
        if element_matches(node, &namespace_scope(&scope, node), &[namespace.as_str()], "si")? {
            positions.push(position);
        }
    }
    Ok(SharedStrings { path, scope, namespace, attrs, children, positions })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn si_text(node: &XmlNode, parent_scope: &[(String, String)], namespace: &str, out: &mut String) -> Result<(), String> {
    let scope = namespace_scope(parent_scope, node);
    if let XmlNode::Element { name, children, .. } = node {
        if expanded_element_name(name, &scope)? == (namespace.to_string(), "t".to_string()) {
            for child in children {
                if let XmlNode::Text { text } = child {
                    out.push_str(text);
                }
            }
        } else {
            for child in children {
                si_text(child, &scope, namespace, out)?;
            }
        }
    }
    Ok(())
}

/// 🌳 The root diff of the shared strings part: `children` plus the `uniqueCount` the table advertises, when it advertises one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shared_strings_root_diff(table: &SharedStrings<'_>, removed: Vec<usize>, modified: Vec<XmlChildModified>, added: Vec<XmlChildAdded>, count: usize) -> XmlNodeDiff {
    let attributes = table
        .attrs
        .iter()
        .find(|attr| attr.name == "uniqueCount" && attr.value != count.to_string())
        .map(|_| XmlAttributesDiff { order: table.attrs.iter().map(|attr| attr.name.clone()).collect(), modified: vec![XmlAttrModified { name: "uniqueCount".into(), value: count.to_string() }], ..Default::default() });
    let children = (!removed.is_empty() || !modified.is_empty() || !added.is_empty()).then_some(XmlChildrenDiff { removed, modified, added });
    XmlNodeDiff::Element(XmlElementDiff { name: None, attributes, children })
}

/// 🔢️ Every shared-string index a worksheet cell under `node` refers to.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn collect_references(node: &XmlNode, scope: &[(String, String)], namespace: &str, out: &mut Vec<usize>) -> Result<(), String> {
    let node_scope = namespace_scope(scope, node);
    let is_cell = element_matches(node, &node_scope, &[namespace], "c")?;
    let XmlNode::Element { attrs, children, .. } = node else { return Ok(()) };
    if is_cell {
        if attrs.iter().any(|attr| attr.name == "t" && attr.value == "s") {
            for child in children {
                if !element_matches(child, &namespace_scope(&node_scope, child), &[namespace], "v")? {
                    continue;
                }
                let XmlNode::Element { children: values, .. } = child else { continue };
                for value in values {
                    if let XmlNode::Text { text } = value {
                        out.push(text.trim().parse::<usize>().map_err(|_| format!("shared-string cell value {text:?} is not an integer"))?);
                    }
                }
            }
        }
        return Ok(());
    }
    for child in children {
        collect_references(child, &node_scope, namespace, out)?;
    }
    Ok(())
}

/// 🔀️ The element diff that renumbers every shared-string reference under `node` through `shift`; `None` when no reference moves.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shift_references(node: &XmlNode, scope: &[(String, String)], namespace: &str, shift: &dyn Fn(usize) -> usize) -> Result<Option<XmlNodeDiff>, String> {
    let node_scope = namespace_scope(scope, node);
    let is_cell = element_matches(node, &node_scope, &[namespace], "c")?;
    let XmlNode::Element { attrs, children, .. } = node else { return Ok(None) };
    if is_cell {
        if !attrs.iter().any(|attr| attr.name == "t" && attr.value == "s") {
            return Ok(None);
        }
        let mut modified = Vec::new();
        for (value_index, child) in children.iter().enumerate() {
            if !element_matches(child, &namespace_scope(&node_scope, child), &[namespace], "v")? {
                continue;
            }
            let XmlNode::Element { children: values, .. } = child else { continue };
            let mut texts = Vec::new();
            for (text_index, value) in values.iter().enumerate() {
                let XmlNode::Text { text } = value else { continue };
                let index = text.trim().parse::<usize>().map_err(|_| format!("shared-string cell value {text:?} is not an integer"))?;
                let shifted = shift(index);
                if shifted != index {
                    texts.push(XmlChildModified { index: text_index, diff: XmlNodeDiff::Text { text: Some(shifted.to_string()) } });
                }
            }
            if !texts.is_empty() {
                modified.push(XmlChildModified { index: value_index, diff: children_leaf(Vec::new(), texts, Vec::new()) });
            }
        }
        return Ok((!modified.is_empty()).then(|| children_leaf(Vec::new(), modified, Vec::new())));
    }
    let mut nested = Vec::new();
    for (index, child) in children.iter().enumerate() {
        if let Some(diff) = shift_references(child, &node_scope, namespace, shift)? {
            nested.push(XmlChildModified { index, diff });
        }
    }
    Ok((!nested.is_empty()).then(|| children_leaf(Vec::new(), nested, Vec::new())))
}

/// 🔀️ The per-worksheet diffs that renumber every shared-string reference through `shift`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn reference_shifts(snapshot: &XlsxSnapshot, shift: &dyn Fn(usize) -> usize) -> Result<Vec<(String, XmlDiff)>, String> {
    let mut edits = Vec::new();
    for part in &snapshot.xml_parts {
        let Some(root) = part.document.root.as_ref() else { continue };
        let XmlNode::Element { name, .. } = root else { continue };
        let (namespace, local) = expanded_element_name(name, &namespace_scope(&[], root))?;
        if local == "worksheet" && SPREADSHEETML_NAMESPACES.contains(&namespace.as_str()) {
            if let Some(leaf) = shift_references(root, &[], &namespace, shift)? {
                edits.push((part.path.clone(), diff_at_path(&[], leaf)));
            }
        }
    }
    Ok(edits)
}

/// ✍️ Replaces the text of the shared string at `index`; the inverse restores the text it held.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn set_shared_string_plan(snapshot: &XlsxSnapshot, index: usize, value: &str) -> Planned {
    let table = shared_strings(snapshot)?;
    let physical = *table.positions.get(index).ok_or_else(|| format!("shared string index {index} is outside the table"))?;
    let current = &table.children[physical];
    let scope = namespace_scope(&table.scope, current);
    let mut old_text = String::new();
    si_text(current, &table.scope, &table.namespace, &mut old_text)?;
    let mut next = current.clone();
    replace_text_contributions(&mut next, &scope, &table.namespace, value)?;
    if next == *current {
        return Ok(XlsxPlan { diff: XlsxDiff::default(), inverse: Vec::new() });
    }
    Ok(XlsxPlan { diff: part_diff(&table.path, &[physical], XmlNodeDiff::Replace { node: Some(next) }), inverse: vec![XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value: old_text })] })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn replace_text_contributions(node: &mut XmlNode, scope: &[(String, String)], namespace: &str, value: &str) -> Result<(), String> {
    fn visit(node: &mut XmlNode, scope: &[(String, String)], namespace: &str, value: &str, replaced: &mut bool) -> Result<(), String> {
        let node_scope = namespace_scope(scope, node);
        let is_text = element_matches(node, &node_scope, &[namespace], "t")?;
        let XmlNode::Element { children, .. } = node else { return Ok(()) };
        if is_text {
            for child in children {
                if let XmlNode::Text { text } = child {
                    text.clear();
                    if !*replaced {
                        text.push_str(value);
                        *replaced = true;
                    }
                }
            }
        } else {
            for child in children {
                visit(child, &node_scope, namespace, value, replaced)?;
            }
        }
        Ok(())
    }
    let mut replaced = false;
    visit(node, scope, namespace, value, &mut replaced)?;
    if !replaced {
        if let XmlNode::Element { name, children, .. } = node {
            children.push(text_element(qualified_like(name, "t"), value));
        }
    }
    Ok(())
}

/// ➕️ Inserts a shared string at `index` (appended when `None`), renumbering the references behind it; the inverse removes it again.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn insert_shared_string_plan(snapshot: &XlsxSnapshot, value: &str, index: Option<usize>) -> Planned {
    let table = shared_strings(snapshot)?;
    let count = table.positions.len();
    let at = index.unwrap_or(count);
    if at > count {
        return Err(format!("shared string index {at} is outside the table"));
    }
    let physical = table.positions.get(at).copied().unwrap_or(table.children.len());
    let name = table.positions.first().map_or_else(
        || table.scope.iter().find(|(_, uri)| *uri == table.namespace).map_or_else(|| "si".into(), |(prefix, _)| if prefix.is_empty() { "si".into() } else { format!("{prefix}:si") }),
        |first| match &table.children[*first] {
            XmlNode::Element { name, .. } => name.clone(),
            _ => unreachable!(),
        },
    );
    let entry = XmlNode::Element {
        name: name.clone(),
        attrs: Vec::new(),
        children: vec![XmlNode::Element { name: qualified_like(&name, "t"), attrs: vec![XmlAttr { name: "xml:space".into(), value: "preserve".into() }], children: vec![XmlNode::Text { text: value.into() }] }],
    };
    let root = shared_strings_root_diff(&table, Vec::new(), Vec::new(), vec![XmlChildAdded { index: physical, item: entry }], count + 1);
    let mut edits = vec![(table.path.clone(), diff_at_path(&[], root))];
    if at < count {
        edits.extend(reference_shifts(snapshot, &|reference| if reference >= at { reference + 1 } else { reference })?);
    }
    Ok(XlsxPlan { diff: modified_parts(edits), inverse: vec![XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: at })] })
}

/// ➖️ Removes the unreferenced shared string at `index`, renumbering the references behind it; the inverse inserts its text back at `index`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn remove_shared_string_plan(snapshot: &XlsxSnapshot, index: usize) -> Planned {
    let table = shared_strings(snapshot)?;
    let physical = *table.positions.get(index).ok_or_else(|| format!("shared string index {index} is outside the table"))?;
    let mut references = Vec::new();
    for part in &snapshot.xml_parts {
        let Some(root) = part.document.root.as_ref() else { continue };
        let XmlNode::Element { name, .. } = root else { continue };
        let (namespace, local) = expanded_element_name(name, &namespace_scope(&[], root))?;
        if local == "worksheet" && SPREADSHEETML_NAMESPACES.contains(&namespace.as_str()) {
            collect_references(root, &[], &namespace, &mut references)?;
        }
    }
    if references.contains(&index) {
        return Err(format!("shared string index {index} is still referenced"));
    }
    let mut old_text = String::new();
    si_text(&table.children[physical], &table.scope, &table.namespace, &mut old_text)?;
    let root = shared_strings_root_diff(&table, vec![physical], Vec::new(), Vec::new(), table.positions.len() - 1);
    let mut edits = vec![(table.path.clone(), diff_at_path(&[], root))];
    edits.extend(reference_shifts(snapshot, &|reference| if reference > index { reference - 1 } else { reference })?);
    Ok(XlsxPlan { diff: modified_parts(edits), inverse: vec![XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: old_text, index: Some(index) })] })
}
//#endregion 🔖️SharedStringPlans

//#region 🔖️SheetPlans
struct SheetEntry {
    physical: usize,
    name: String,
    sheet_id: Option<u64>,
    relationship_id: Option<String>,
}

struct Sheets<'a> {
    workbook_path: String,
    sheets_index: usize,
    namespace: String,
    scope: Vec<(String, String)>,
    sheets_name: String,
    children: &'a [XmlNode],
    entries: Vec<SheetEntry>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn workbook_sheets(snapshot: &XlsxSnapshot) -> Result<Sheets<'_>, String> {
    let workbook_path = workbook_path(snapshot)?;
    let root = part_root(snapshot, &workbook_path).map_err(|_| format!("missing workbook part {workbook_path}"))?;
    let root_scope = namespace_scope(&[], root);
    let XmlNode::Element { name: root_name, children, .. } = root else { return Err("workbook root is not an element".into()) };
    let (namespace, local) = expanded_element_name(root_name, &root_scope)?;
    if local != "workbook" || !SPREADSHEETML_NAMESPACES.contains(&namespace.as_str()) {
        return Err("workbook root is not a SpreadsheetML workbook element".into());
    }
    for (sheets_index, child) in children.iter().enumerate() {
        let scope = namespace_scope(&root_scope, child);
        if !element_matches(child, &scope, &[namespace.as_str()], "sheets")? {
            continue;
        }
        let XmlNode::Element { name, children, .. } = child else { unreachable!() };
        let mut entries = Vec::new();
        for (physical, sheet) in children.iter().enumerate() {
            let sheet_scope = namespace_scope(&scope, sheet);
            if !element_matches(sheet, &sheet_scope, &[namespace.as_str()], "sheet")? {
                continue;
            }
            entries.push(SheetEntry {
                physical,
                name: attribute_value(sheet, &sheet_scope, &[""], "name")?.unwrap_or_default().to_string(),
                sheet_id: attribute_value(sheet, &sheet_scope, &[""], "sheetId")?.map(|value| value.parse::<u64>().map_err(|_| "worksheet sheetId is not an unsigned integer".to_string())).transpose()?,
                relationship_id: attribute_value(sheet, &sheet_scope, &OFFICE_RELATIONSHIP_NAMESPACES, "id")?.map(str::to_string),
            });
        }
        return Ok(Sheets { workbook_path, sheets_index, namespace, scope, sheets_name: qualified_like(name, "sheet"), children, entries });
    }
    Err("workbook has no SpreadsheetML sheets element".into())
}

/// 🏷️ Renames the sheet `name` to `new_name`; the inverse renames it back.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn rename_sheet_plan(snapshot: &XlsxSnapshot, name: &str, new_name: &str) -> Planned {
    let sheets = workbook_sheets(snapshot)?;
    if sheets.entries.iter().any(|entry| entry.name == new_name) {
        return Err(format!("worksheet {new_name:?} already exists"));
    }
    let entry = sheets.entries.iter().find(|entry| entry.name == name).ok_or_else(|| format!("missing worksheet {name:?}"))?;
    let XmlNode::Element { attrs, .. } = &sheets.children[entry.physical] else { unreachable!() };
    Ok(XlsxPlan {
        diff: part_diff(&sheets.workbook_path, &[sheets.sheets_index, entry.physical], attribute_leaf(attrs, "name", new_name)),
        inverse: vec![XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: new_name.into(), new_name: name.into() })],
    })
}

/// ➕️ Inserts a sheet at `index` (appended when `None`); the part, relationship and content-type entry follow the position of the sheet it lands before. The inverse removes it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn insert_sheet_plan(snapshot: &XlsxSnapshot, sheet: &XlsxSheet, index: Option<usize>) -> Planned {
    let sheets = workbook_sheets(snapshot)?;
    if sheets.entries.iter().any(|entry| entry.name == sheet.name) {
        return Err(format!("worksheet {:?} already exists", sheet.name));
    }
    if sheet.name.is_empty() || sheet.name.chars().count() > 31 || sheet.name.chars().any(|character| matches!(character, ':' | '\\' | '/' | '?' | '*' | '[' | ']')) {
        return Err("worksheet name is invalid".into());
    }
    let position = index.unwrap_or(sheets.entries.len());
    if position > sheets.entries.len() {
        return Err(format!("sheet index {position} is outside the workbook"));
    }
    let workbook_path = sheets.workbook_path.clone();
    let workbook_relationships = snapshot.opc.relationships_for(&workbook_path);
    let relationship_namespace = workbook_relationships
        .iter()
        .find(|relationship| relationship.rel_type.ends_with("/worksheet"))
        .map(|relationship| if relationship.rel_type.starts_with(R_NS_STRICT) { R_NS_STRICT } else { R_NS })
        .unwrap_or(if sheets.namespace == SML_NS_STRICT { R_NS_STRICT } else { R_NS });
    let relationship_prefix = sheets
        .scope
        .iter()
        .find(|(prefix, uri)| !prefix.is_empty() && uri == relationship_namespace)
        .map(|(prefix, _)| prefix.clone())
        .ok_or_else(|| "workbook has no namespace prefix for worksheet relationship attributes".to_string())?;
    let sheet_id = (1u64..).find(|candidate| sheets.entries.iter().all(|entry| entry.sheet_id != Some(*candidate))).expect("an unbounded range always yields a free sheet id");
    let directory = workbook_path.rsplit_once('/').map_or("", |(directory, _)| directory);
    let worksheet_path = (1usize..)
        .map(|ordinal| if directory.is_empty() { format!("worksheets/sheet{ordinal}.xml") } else { format!("{directory}/worksheets/sheet{ordinal}.xml") })
        .find(|candidate| snapshot.xml_part(candidate).is_none() && snapshot.opc.part(candidate).is_none())
        .expect("an unbounded range always yields a free part path");
    let target = worksheet_path.strip_prefix(&format!("{directory}/")).unwrap_or(&worksheet_path).to_string();
    let mut taken: Vec<String> = workbook_relationships.iter().map(|relationship| relationship.id.clone()).collect();
    let relationship_id = fresh_relationship_id(&mut taken);
    let relationship_type = if relationship_namespace == R_NS_STRICT { format!("{R_NS_STRICT}/worksheet") } else { REL_TYPE_WORKSHEET.into() };
    let relationship = OpcRelationship { id: relationship_id.clone(), rel_type: relationship_type, target, target_mode: OpcTargetMode::Internal };
    let entry = XmlNode::Element {
        name: sheets.sheets_name.clone(),
        attrs: vec![XmlAttr { name: "name".into(), value: sheet.name.clone() }, XmlAttr { name: "sheetId".into(), value: sheet_id.to_string() }, XmlAttr { name: format!("{relationship_prefix}:id"), value: relationship_id.clone() }],
        children: Vec::new(),
    };
    let before = sheets.entries.get(position);
    let neighbour = before.or_else(|| sheets.entries.last());
    let place = |found: Option<usize>| found.map(|index| if before.is_some() { index } else { index + 1 });
    let physical = before.map_or(sheets.children.len(), |entry| entry.physical);
    let neighbour_relationship = neighbour.and_then(|entry| entry.relationship_id.as_deref()).and_then(|id| workbook_relationships.iter().find(|relationship| relationship.id == id));
    let neighbour_path = neighbour_relationship.map(|relationship| resolve_relationship_target(&workbook_path, &relationship.target));
    let override_name = format!("/{worksheet_path}");
    let neighbour_override = neighbour_path.as_ref().map(|path| format!("/{path}"));
    let relationship_order = insertion_order(
        workbook_relationships.iter().map(|relationship| relationship.id.clone()).collect(),
        relationship_id,
        place(neighbour_relationship.and_then(|neighbour| workbook_relationships.iter().position(|relationship| relationship.id == neighbour.id))),
    );
    let override_order = insertion_order(
        snapshot.opc.content_types.overrides.iter().map(|(name, _)| name.clone()).collect(),
        override_name.clone(),
        place(neighbour_override.as_ref().and_then(|name| snapshot.opc.content_types.overrides.iter().position(|(existing, _)| existing == name))),
    );
    let part_order = insertion_order(snapshot.xml_parts.iter().map(|part| part.path.clone()).collect(), worksheet_path.clone(), place(neighbour_path.as_ref().and_then(|path| snapshot.xml_parts.iter().position(|part| part.path == *path))));
    let part = XlsxXmlPart { path: worksheet_path, content_type: WORKSHEET_CONTENT_TYPE.into(), document: worksheet_to_xml_with_namespace(sheet, &sheets.namespace) };
    let diff = XlsxDiff {
        opc: Some(XlsxOpcDiff {
            content_types: Some(XlsxOpcContentTypesDiff { defaults: None, overrides: Some(NamedTripleDiff { added: vec![(override_name, WORKSHEET_CONTENT_TYPE.into())], order: override_order, ..Default::default() }) }),
            relationships: Some(NamedTripleDiff { modified: vec![NamedModified { key: workbook_path.clone(), diff: XlsxOpcRelListDiff { added: vec![relationship], order: relationship_order, ..Default::default() } }], ..Default::default() }),
            ..Default::default()
        }),
        xml_parts: Some(NamedTripleDiff {
            modified: vec![NamedModified {
                key: workbook_path,
                diff: XlsxXmlPartDiff { content_type: None, document: Some(diff_at_path(&[sheets.sheets_index], children_leaf(Vec::new(), Vec::new(), vec![XmlChildAdded { index: physical, item: entry }]))) },
            }],
            added: vec![part],
            order: part_order,
            ..Default::default()
        }),
    };
    Ok(XlsxPlan { diff, inverse: vec![XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: sheet.name.clone() })] })
}

/// ➖️ Removes the sheet `name` together with its part, relationship and content-type entry; the inverse inserts its typed cells back at the position it held.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn remove_sheet_plan(snapshot: &XlsxSnapshot, name: &str) -> Planned {
    let sheets = workbook_sheets(snapshot)?;
    if sheets.entries.len() <= 1 {
        return Err("a workbook must retain at least one worksheet".into());
    }
    let logical = sheets.entries.iter().position(|entry| entry.name == name).ok_or_else(|| format!("missing worksheet {name:?}"))?;
    let entry = &sheets.entries[logical];
    let relationship_id = entry.relationship_id.clone().ok_or_else(|| format!("worksheet {name:?} has no relationship id"))?;
    let relationship = snapshot.opc.relationships_for(&sheets.workbook_path).iter().find(|relationship| relationship.id == relationship_id).ok_or_else(|| format!("worksheet {name:?} references an unknown relationship"))?;
    if relationship.target_mode != OpcTargetMode::Internal {
        return Err("worksheet relationship is external".into());
    }
    let worksheet_path = resolve_relationship_target(&sheets.workbook_path, &relationship.target);
    let restored = snapshot.project_workbook().map_err(|error| error.to_string())?.sheets.into_iter().find(|candidate| candidate.name == name).ok_or_else(|| format!("missing worksheet {name:?}"))?;
    let override_name = format!("/{worksheet_path}");
    let owns_relationships = snapshot.opc.relationships.groups().any(|(owner, _)| *owner == worksheet_path);
    let diff = XlsxDiff {
        opc: Some(XlsxOpcDiff {
            content_types: snapshot.opc.content_types.overrides.iter().any(|(existing, _)| *existing == override_name).then(|| XlsxOpcContentTypesDiff { defaults: None, overrides: Some(NamedTripleDiff { removed: vec![override_name], ..Default::default() }) }),
            relationships: Some(NamedTripleDiff {
                removed: if owns_relationships { vec![worksheet_path.clone()] } else { Vec::new() },
                modified: vec![NamedModified { key: sheets.workbook_path.clone(), diff: XlsxOpcRelListDiff { removed: vec![relationship_id], ..Default::default() } }],
                ..Default::default()
            }),
            ..Default::default()
        }),
        xml_parts: Some(NamedTripleDiff {
            removed: vec![worksheet_path],
            modified: vec![NamedModified {
                key: sheets.workbook_path.clone(),
                diff: XlsxXmlPartDiff { content_type: None, document: Some(diff_at_path(&[sheets.sheets_index], children_leaf(vec![entry.physical], Vec::new(), Vec::new()))) },
            }],
            ..Default::default()
        }),
    };
    Ok(XlsxPlan { diff, inverse: vec![XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: restored, index: Some(logical) })] })
}
//#endregion 🔖️SheetPlans
