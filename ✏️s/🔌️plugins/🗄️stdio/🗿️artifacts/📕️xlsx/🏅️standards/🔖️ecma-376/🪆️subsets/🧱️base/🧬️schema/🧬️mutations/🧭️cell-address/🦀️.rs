//! 🧭️ Namespace-aware, lineage-bound addresses for authoritative SpreadsheetML cells.

use crate::standards::v_ecma_376::subsets::base::io::{attribute_value, column_letter, element_matches, expanded_element_name, namespace_scope, REL_TYPE_OFFICE_DOCUMENT_STRICT, R_NS, R_NS_STRICT, SML_NS, SML_NS_STRICT};
use crate::XlsxSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use semio_s_artifact_stdio_zip::opc::{resolve_relationship_target, REL_TYPE_OFFICE_DOCUMENT};

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
const SPREADSHEETML_NAMESPACES: [&str; 2] = [SML_NS, SML_NS_STRICT];
const OFFICE_RELATIONSHIP_NAMESPACES: [&str; 2] = [R_NS, R_NS_STRICT];

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct XlsxCellAddress {
    pub part_path: String,
    pub node_path: Vec<usize>,
    pub namespace_uri: String,
    pub local_name: String,
    pub revision: String,
}

pub struct ResolvedXlsxCellAddress<'a> {
    pub part_index: usize,
    pub node: &'a XmlNode,
}

fn scoped_node_at_path<'a>(root: &'a XmlNode, path: &[usize]) -> Result<(&'a XmlNode, Vec<(String, String)>), String> {
    let mut node = root;
    let mut bindings = namespace_scope(&[], node);
    for &index in path {
        let XmlNode::Element { children, .. } = node else { return Err(format!("node path descends through non-element at child {index}")) };
        node = children.get(index).ok_or_else(|| format!("node path child {index} is outside {} children", children.len()))?;
        bindings = namespace_scope(&bindings, node);
    }
    Ok((node, bindings))
}

fn node_mut_at_path<'a>(node: &'a mut XmlNode, path: &[usize]) -> Option<&'a mut XmlNode> {
    let Some((&index, rest)) = path.split_first() else { return Some(node) };
    let XmlNode::Element { children, .. } = node else { return None };
    node_mut_at_path(children.get_mut(index)?, rest)
}

fn hash_bytes(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x100000001b3);
    }
}

fn hash_field(hash: &mut u64, value: &str) {
    hash_bytes(hash, &(value.len() as u64).to_le_bytes());
    hash_bytes(hash, value.as_bytes());
}

fn hash_node(hash: &mut u64, node: &XmlNode, recursive: bool) {
    match node {
        XmlNode::Element { name, attrs, children } => {
            hash_bytes(hash, &[0]);
            hash_field(hash, name);
            hash_bytes(hash, &(attrs.len() as u64).to_le_bytes());
            for attr in attrs {
                hash_field(hash, &attr.name);
                hash_field(hash, &attr.value);
            }
            hash_bytes(hash, &(children.len() as u64).to_le_bytes());
            if recursive {
                for child in children {
                    hash_node(hash, child, true);
                }
            }
        }
        XmlNode::Text { text } | XmlNode::CData { text } | XmlNode::Comment { text } => hash_field(hash, text),
        XmlNode::ProcessingInstruction { target, data } => {
            hash_field(hash, target);
            hash_field(hash, data);
        }
    }
}

fn address_revision(root: &XmlNode, path: &[usize]) -> Result<String, String> {
    let mut hash = 0xcbf29ce484222325;
    let mut node = root;
    for &index in path {
        let XmlNode::Element { children, .. } = node else { return Err(format!("node path descends through non-element at child {index}")) };
        hash_node(&mut hash, node, false);
        hash_bytes(&mut hash, &(index as u64).to_le_bytes());
        for child in children {
            hash_node(&mut hash, child, false);
        }
        node = children.get(index).ok_or_else(|| format!("node path child {index} is outside {} children", children.len()))?;
    }
    hash_node(&mut hash, node, true);
    Ok(format!("{hash:016x}"))
}

fn workbook_path(snapshot: &XlsxSnapshot) -> Result<String, String> {
    snapshot.opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).or_else(|| snapshot.opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT_STRICT)).ok_or_else(|| "missing workbook relationship".into())
}

fn worksheet_path(snapshot: &XlsxSnapshot, sheet_name: &str) -> Result<String, String> {
    let workbook_path = workbook_path(snapshot)?;
    let workbook = snapshot.xml_part(&workbook_path).ok_or_else(|| format!("missing workbook part {workbook_path}"))?;
    let root = workbook.document.root.as_ref().ok_or_else(|| format!("workbook part {workbook_path} has no root"))?;
    let root_bindings = namespace_scope(&[], root);
    if !element_matches(root, &root_bindings, &SPREADSHEETML_NAMESPACES, "workbook")? {
        return Err("workbook root is not a SpreadsheetML workbook element".into());
    }
    let XmlNode::Element { children, .. } = root else { unreachable!() };
    let mut sheets_match = None;
    for node in children {
        let bindings = namespace_scope(&root_bindings, node);
        if element_matches(node, &bindings, &SPREADSHEETML_NAMESPACES, "sheets")? {
            let XmlNode::Element { children, .. } = node else { unreachable!() };
            sheets_match = Some((children, bindings));
            break;
        }
    }
    let (sheets, sheets_bindings) = sheets_match.ok_or_else(|| "workbook has no SpreadsheetML sheets element".to_string())?;
    let mut relationship_id = None;
    for node in sheets {
        let bindings = namespace_scope(&sheets_bindings, node);
        if element_matches(node, &bindings, &SPREADSHEETML_NAMESPACES, "sheet")? && attribute_value(node, &bindings, &[""], "name")? == Some(sheet_name) {
            relationship_id = attribute_value(node, &bindings, &OFFICE_RELATIONSHIP_NAMESPACES, "id")?;
            break;
        }
    }
    let relationship_id = relationship_id.ok_or_else(|| format!("missing worksheet {sheet_name:?}"))?;
    let relationship = snapshot.opc.relationships_for(&workbook_path).iter().find(|relationship| relationship.id == relationship_id).ok_or_else(|| format!("worksheet {sheet_name:?} references unknown relationship {relationship_id}"))?;
    Ok(resolve_relationship_target(&workbook_path, &relationship.target))
}

fn cell_path(root: &XmlNode, row: u32, column: u32) -> Result<Vec<usize>, String> {
    let reference = format!("{}{}", column_letter(column), row);
    let row_text = row.to_string();
    let root_bindings = namespace_scope(&[], root);
    if !element_matches(root, &root_bindings, &SPREADSHEETML_NAMESPACES, "worksheet")? {
        return Err("worksheet root is not a SpreadsheetML worksheet element".into());
    }
    let XmlNode::Element { children, .. } = root else { unreachable!() };
    let mut sheet_data_match = None;
    for (index, node) in children.iter().enumerate() {
        let bindings = namespace_scope(&root_bindings, node);
        if element_matches(node, &bindings, &SPREADSHEETML_NAMESPACES, "sheetData")? {
            let XmlNode::Element { children, .. } = node else { unreachable!() };
            sheet_data_match = Some((index, children, bindings));
            break;
        }
    }
    let (sheet_data_index, sheet_data, sheet_data_bindings) = sheet_data_match.ok_or_else(|| "worksheet has no SpreadsheetML sheetData element".to_string())?;
    for (row_index, row_node) in sheet_data.iter().enumerate() {
        let row_bindings = namespace_scope(&sheet_data_bindings, row_node);
        let XmlNode::Element { children, .. } = row_node else { continue };
        if !element_matches(row_node, &row_bindings, &SPREADSHEETML_NAMESPACES, "row")? || attribute_value(row_node, &row_bindings, &[""], "r")? != Some(row_text.as_str()) {
            continue;
        }
        for (cell_index, node) in children.iter().enumerate() {
            let bindings = namespace_scope(&row_bindings, node);
            if element_matches(node, &bindings, &SPREADSHEETML_NAMESPACES, "c")? && attribute_value(node, &bindings, &[""], "r")? == Some(reference.as_str()) {
                return Ok(vec![sheet_data_index, row_index, cell_index]);
            }
        }
    }
    Err(format!("missing cell {reference}"))
}

pub fn xlsx_cell_address(snapshot: &XlsxSnapshot, sheet_name: &str, row: u32, column: u32) -> Result<XlsxCellAddress, String> {
    let part_path = worksheet_path(snapshot, sheet_name)?;
    let part = snapshot.xml_part(&part_path).ok_or_else(|| format!("missing worksheet part {part_path}"))?;
    let root = part.document.root.as_ref().ok_or_else(|| format!("worksheet part {part_path} has no root"))?;
    let node_path = cell_path(root, row, column)?;
    xlsx_cell_address_at_path(snapshot, &part_path, node_path)
}

pub fn xlsx_cell_address_at_path(snapshot: &XlsxSnapshot, part_path: &str, node_path: Vec<usize>) -> Result<XlsxCellAddress, String> {
    let part = snapshot.xml_part(part_path).ok_or_else(|| format!("missing worksheet part {part_path}"))?;
    let root = part.document.root.as_ref().ok_or_else(|| format!("worksheet part {part_path} has no root"))?;
    let (node, bindings) = scoped_node_at_path(root, &node_path)?;
    let (namespace_uri, local_name) = match node {
        XmlNode::Element { name, .. } => expanded_element_name(name, &bindings)?,
        _ => return Err("cell address resolved a non-element".into()),
    };
    let revision = address_revision(root, &node_path)?;
    Ok(XlsxCellAddress { part_path: part_path.into(), node_path, namespace_uri, local_name, revision })
}

pub fn resolve_xlsx_cell_address<'a>(snapshot: &'a XlsxSnapshot, address: &XlsxCellAddress) -> Result<ResolvedXlsxCellAddress<'a>, String> {
    if address.part_path.is_empty()
        || address.part_path.starts_with('/')
        || address.part_path.ends_with('/')
        || address.part_path.contains("//")
        || address.part_path.contains('\\')
        || address.part_path.split('/').any(|component| component == "." || component == "..")
    {
        return Err("XLSX cell address partPath is not canonical".into());
    }
    if address.node_path.iter().any(|index| *index as u64 > MAX_SAFE_INTEGER) {
        return Err("XLSX cell address nodePath exceeds the cross-language safe integer range".into());
    }
    if address.local_name != "c" || address.namespace_uri.is_empty() || address.revision.len() != 16 || !address.revision.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
        return Err("XLSX cell address identity is invalid".into());
    }
    let mut parts = snapshot.xml_parts.iter().enumerate().filter(|(_, part)| part.path == address.part_path);
    let (part_index, part) = parts.next().ok_or_else(|| format!("missing worksheet part {}", address.part_path))?;
    if parts.next().is_some() {
        return Err(format!("duplicate worksheet part {}", address.part_path));
    }
    let root = part.document.root.as_ref().ok_or_else(|| format!("worksheet part {} has no root", address.part_path))?;
    let (node, bindings) = scoped_node_at_path(root, &address.node_path)?;
    let (namespace_uri, local_name) = match node {
        XmlNode::Element { name, .. } => expanded_element_name(name, &bindings)?,
        _ => return Err("cell address resolved a non-element".into()),
    };
    if namespace_uri != address.namespace_uri || local_name != address.local_name || address_revision(root, &address.node_path)? != address.revision {
        return Err("XLSX cell address is stale".into());
    }
    Ok(ResolvedXlsxCellAddress { part_index, node })
}

pub(super) fn addressed_cell_scope(snapshot: &XlsxSnapshot, address: &XlsxCellAddress) -> Result<Vec<(String, String)>, String> {
    resolve_xlsx_cell_address(snapshot, address)?;
    let part = snapshot.xml_part(&address.part_path).ok_or_else(|| format!("missing worksheet part {}", address.part_path))?;
    let root = part.document.root.as_ref().ok_or_else(|| format!("worksheet part {} has no root", address.part_path))?;
    scoped_node_at_path(root, &address.node_path).map(|(_, bindings)| bindings)
}

pub fn addressed_cell_mut<'a>(snapshot: &'a mut XlsxSnapshot, address: &XlsxCellAddress) -> Result<&'a mut XmlNode, String> {
    let part_index = resolve_xlsx_cell_address(snapshot, address)?.part_index;
    let root = snapshot.xml_parts[part_index].document.root.as_mut().ok_or_else(|| format!("worksheet part {} has no root", address.part_path))?;
    node_mut_at_path(root, &address.node_path).ok_or_else(|| "XLSX cell address could not be reopened mutably".into())
}
