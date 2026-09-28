//! 🧭️ Stable, namespace-aware addresses for canonical DOCX XML mutations.

use super::*;
use crate::standards::v_ecma_376::subsets::base::io::namespaces::{XML_NAMESPACE, apply_bindings, expanded_name, is_word_name, qualified_word_prefix, set_word_attr, word_attr};

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

/// 🧭️ Identifies one XML node by part, child-index path, expanded name, and lineage-bound revision.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocxXmlAddress {
    pub part_path: String,
    pub node_path: Vec<usize>,
    pub expected_name: String,
    pub revision: String,
}

/// 🔎️ Validated canonical XML address result.
pub struct ResolvedDocxXmlAddress<'a> {
    pub part_index: usize,
    pub node: &'a XmlNode,
    bindings: Vec<(String, String)>,
    parent_bindings: Vec<(String, String)>,
}

/// ✏️ One canonical run projection for a windowed document editor.
#[derive(Clone, Debug, PartialEq)]
pub struct DocxEditableRun {
    pub text: String,
    pub address: DocxXmlAddress,
}

fn node_identity(node: &XmlNode, bindings: &[(String, String)]) -> Result<String, String> {
    match node {
        XmlNode::Element { name, .. } => expanded_name(name, bindings),
        XmlNode::Text { .. } => Ok("#text".into()),
        XmlNode::CData { .. } => Ok("#cdata".into()),
        XmlNode::Comment { .. } => Ok("#comment".into()),
        XmlNode::ProcessingInstruction { target, .. } => Ok(format!("?{target}")),
    }
}

fn validate_address_shape(address: &DocxXmlAddress) -> Result<(), String> {
    let path = &address.part_path;
    if path.is_empty() || path.starts_with('/') || path.ends_with('/') || path.contains("//") || path.contains('\\') || path.split('/').any(|component| component == "." || component == "..") {
        return Err("DOCX XML address partPath is not canonical".into());
    }
    if address.node_path.iter().any(|index| *index as u64 > MAX_SAFE_INTEGER) {
        return Err("DOCX XML address nodePath exceeds the cross-language safe integer range".into());
    }
    if address.expected_name.is_empty() {
        return Err("DOCX XML address expectedName is empty".into());
    }
    if address.revision.len() != 16 || !address.revision.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
        return Err("DOCX XML address revision must be 16 lowercase hexadecimal digits".into());
    }
    Ok(())
}

fn scoped_node_at_path<'a>(root: &'a XmlNode, path: &[usize]) -> Result<(&'a XmlNode, Vec<(String, String)>), String> {
    let mut node = root;
    let mut bindings = vec![("xml".into(), XML_NAMESPACE.into())];
    apply_bindings(node, &mut bindings);
    for &index in path {
        let XmlNode::Element { children, .. } = node else { return Err(format!("node path descends through non-element at child {index}")) };
        node = children.get(index).ok_or_else(|| format!("node path child {index} is outside {} children", children.len()))?;
        apply_bindings(node, &mut bindings);
    }
    Ok((node, bindings))
}

fn parent_bindings_at_path(root: &XmlNode, path: &[usize]) -> Result<Vec<(String, String)>, String> {
    let Some((_, parent_path)) = path.split_last() else { return Ok(vec![("xml".into(), XML_NAMESPACE.into())]) };
    scoped_node_at_path(root, parent_path).map(|(_, bindings)| bindings)
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

fn hash_node(hash: &mut u64, node: &XmlNode) {
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
            for child in children {
                hash_node(hash, child);
            }
        }
        XmlNode::Text { text } => {
            hash_bytes(hash, &[1]);
            hash_field(hash, text);
        }
        XmlNode::CData { text } => {
            hash_bytes(hash, &[2]);
            hash_field(hash, text);
        }
        XmlNode::Comment { text } => {
            hash_bytes(hash, &[3]);
            hash_field(hash, text);
        }
        XmlNode::ProcessingInstruction { target, data } => {
            hash_bytes(hash, &[4]);
            hash_field(hash, target);
            hash_field(hash, data);
        }
    }
}

fn hash_shallow_node(hash: &mut u64, node: &XmlNode) {
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
        }
        XmlNode::Text { text } => {
            hash_bytes(hash, &[1]);
            hash_field(hash, text);
        }
        XmlNode::CData { text } => {
            hash_bytes(hash, &[2]);
            hash_field(hash, text);
        }
        XmlNode::Comment { text } => {
            hash_bytes(hash, &[3]);
            hash_field(hash, text);
        }
        XmlNode::ProcessingInstruction { target, data } => {
            hash_bytes(hash, &[4]);
            hash_field(hash, target);
            hash_field(hash, data);
        }
    }
}

fn address_revision(root: &XmlNode, path: &[usize], replacement: Option<&XmlNode>) -> Result<String, String> {
    let mut hash = 0xcbf29ce484222325;
    let mut node = root;
    for (depth, &index) in path.iter().enumerate() {
        let XmlNode::Element { children, .. } = node else { return Err(format!("node path descends through non-element at child {index}")) };
        hash_shallow_node(&mut hash, node);
        hash_bytes(&mut hash, &(index as u64).to_le_bytes());
        for (child_index, child) in children.iter().enumerate() {
            let child = if depth + 1 == path.len() && child_index == index { replacement.unwrap_or(child) } else { child };
            hash_shallow_node(&mut hash, child);
        }
        node = children.get(index).ok_or_else(|| format!("node path child {index} is outside {} children", children.len()))?;
    }
    hash_node(&mut hash, replacement.unwrap_or(node));
    Ok(format!("{hash:016x}"))
}

/// 🧾️ Returns a deterministic revision for exactly one canonical XML subtree.
pub fn docx_xml_subtree_revision(node: &XmlNode) -> String {
    let mut hash = 0xcbf29ce484222325;
    hash_node(&mut hash, node);
    format!("{hash:016x}")
}

/// 🧭️ Constructs an address without projecting the semantic document or materializing other parts.
pub fn docx_xml_address(snapshot: &DocxSnapshot, part_path: impl AsRef<str>, node_path: Vec<usize>) -> Result<DocxXmlAddress, String> {
    let part_path = part_path.as_ref();
    if part_path.is_empty() || part_path.starts_with('/') || part_path.ends_with('/') || part_path.contains("//") || part_path.contains('\\') || part_path.split('/').any(|component| component == "." || component == "..") {
        return Err("DOCX XML address partPath is not canonical".into());
    }
    if node_path.iter().any(|index| *index as u64 > MAX_SAFE_INTEGER) {
        return Err("DOCX XML address nodePath exceeds the cross-language safe integer range".into());
    }
    let mut parts = snapshot.xml_parts.iter().enumerate().filter(|(_, part)| part.path == part_path);
    let (_, part) = parts.next().ok_or_else(|| format!("missing DOCX XML part {part_path}"))?;
    if parts.next().is_some() {
        return Err(format!("duplicate DOCX XML part {part_path}"));
    }
    let root = part.document.root.as_ref().ok_or_else(|| format!("DOCX XML part {part_path} has no root"))?;
    let (node, bindings) = scoped_node_at_path(root, &node_path)?;
    let expected_name = node_identity(node, &bindings)?;
    let revision = address_revision(root, &node_path, None)?;
    Ok(DocxXmlAddress { part_path: part_path.into(), node_path, expected_name, revision })
}

/// 🔎️ Resolves and validates a canonical address without projecting the semantic document.
pub fn resolve_docx_xml_address<'a>(snapshot: &'a DocxSnapshot, address: &DocxXmlAddress) -> Result<ResolvedDocxXmlAddress<'a>, String> {
    validate_address_shape(address)?;
    let part_path = address.part_path.as_str();
    let mut parts = snapshot.xml_parts.iter().enumerate().filter(|(_, part)| part.path == part_path);
    let (part_index, part) = parts.next().ok_or_else(|| format!("missing DOCX XML part {part_path}"))?;
    if parts.next().is_some() {
        return Err(format!("duplicate DOCX XML part {part_path}"));
    }
    let root = part.document.root.as_ref().ok_or_else(|| format!("DOCX XML part {part_path} has no root"))?;
    let (node, bindings) = scoped_node_at_path(root, &address.node_path)?;
    let parent_bindings = parent_bindings_at_path(root, &address.node_path)?;
    let actual_name = node_identity(node, &bindings)?;
    if actual_name != address.expected_name {
        return Err(format!("DOCX XML address expected {} but resolved {actual_name}", address.expected_name));
    }
    let actual_revision = address_revision(root, &address.node_path, None)?;
    if actual_revision != address.revision {
        return Err(format!("DOCX XML address revision changed from {} to {actual_revision}", address.revision));
    }
    Ok(ResolvedDocxXmlAddress { part_index, node, bindings, parent_bindings })
}

pub(super) fn revision_after_replacement(snapshot: &DocxSnapshot, address: &DocxXmlAddress, replacement: &XmlNode) -> Result<String, String> {
    let part = snapshot.xml_part(&address.part_path).ok_or_else(|| format!("missing DOCX XML part {}", address.part_path))?;
    let root = part.document.root.as_ref().ok_or_else(|| format!("DOCX XML part {} has no root", address.part_path))?;
    address_revision(root, &address.node_path, Some(replacement))
}

fn word_child_path(snapshot: &DocxSnapshot, part_path: &str, parent_path: &[usize], locals: &[&str], ordinal: usize) -> Result<Vec<usize>, String> {
    let part = snapshot.xml_part(part_path).ok_or_else(|| format!("missing DOCX XML part {part_path}"))?;
    let root = part.document.root.as_ref().ok_or_else(|| format!("DOCX XML part {part_path} has no root"))?;
    let (parent, _) = scoped_node_at_path(root, parent_path)?;
    let XmlNode::Element { children, .. } = parent else { return Err("DOCX XML path parent is not an element".into()) };
    let mut seen = 0usize;
    for (index, _) in children.iter().enumerate() {
        let mut path = parent_path.to_vec();
        path.push(index);
        let (child, bindings) = scoped_node_at_path(root, &path)?;
        if locals.iter().any(|local| is_word_name(child, local, &bindings)) {
            if seen == ordinal {
                return Ok(path);
            }
            seen += 1;
        }
    }
    Err(format!("missing WordprocessingML child {} at ordinal {ordinal}", locals.join("/")))
}

fn collect_word_descendants(snapshot: &DocxSnapshot, part_path: &str, parent_path: &[usize], local: &str, paths: &mut Vec<Vec<usize>>) -> Result<(), String> {
    let part = snapshot.xml_part(part_path).ok_or_else(|| format!("missing DOCX XML part {part_path}"))?;
    let root = part.document.root.as_ref().ok_or_else(|| format!("DOCX XML part {part_path} has no root"))?;
    let (parent, _) = scoped_node_at_path(root, parent_path)?;
    let XmlNode::Element { children, .. } = parent else { return Ok(()) };
    for index in 0..children.len() {
        let mut path = parent_path.to_vec();
        path.push(index);
        let (child, bindings) = scoped_node_at_path(root, &path)?;
        if is_word_name(child, local, &bindings) {
            paths.push(path.clone());
        }
        collect_word_descendants(snapshot, part_path, &path, local, paths)?;
    }
    Ok(())
}

fn main_body_path(snapshot: &DocxSnapshot) -> Result<(String, Vec<usize>), String> {
    let part_path = crate::standards::v_ecma_376::subsets::base::io::import::deserializers::main_document_path(&snapshot.opc).map_err(|error| error.to_string())?;
    let body_path = word_child_path(snapshot, &part_path, &[], &["body"], 0)?;
    Ok((part_path, body_path))
}

/// 🧮️ Counts top-level canonical body blocks without constructing the semantic document projection.
pub fn docx_top_level_block_count(snapshot: &DocxSnapshot) -> Result<usize, String> {
    let (part_path, body_path) = main_body_path(snapshot)?;
    let part = snapshot.xml_part(&part_path).ok_or_else(|| format!("missing DOCX XML part {part_path}"))?;
    let root = part.document.root.as_ref().ok_or_else(|| format!("DOCX XML part {part_path} has no root"))?;
    let (body, _) = scoped_node_at_path(root, &body_path)?;
    let XmlNode::Element { children, .. } = body else { return Err("WordprocessingML body is not an element".into()) };
    let mut count = 0usize;
    for index in 0..children.len() {
        let mut path = body_path.clone();
        path.push(index);
        let (node, bindings) = scoped_node_at_path(root, &path)?;
        if is_word_name(node, "p", &bindings) || is_word_name(node, "tbl", &bindings) {
            count += 1;
        }
    }
    Ok(count)
}

fn top_level_block_path(snapshot: &DocxSnapshot, block_index: usize) -> Result<(String, Vec<usize>), String> {
    let (part_path, body_path) = main_body_path(snapshot)?;
    let block_path = word_child_path(snapshot, &part_path, &body_path, &["p", "tbl"], block_index)?;
    Ok((part_path, block_path))
}

/// 🧭️ Addresses one top-level paragraph or table without projecting the semantic document.
pub fn docx_top_level_block_address(snapshot: &DocxSnapshot, block_index: usize) -> Result<DocxXmlAddress, String> {
    let (part_path, body_path) = main_body_path(snapshot)?;
    let block_path = word_child_path(snapshot, &part_path, &body_path, &["p", "tbl"], block_index)?;
    docx_xml_address(snapshot, part_path, block_path)
}

/// 🧮️ Counts editable paragraph and nested table runs in one canonical body block.
pub fn docx_top_level_run_count(snapshot: &DocxSnapshot, block_index: usize) -> Result<usize, String> {
    let (part_path, paragraph_path) = top_level_block_path(snapshot, block_index)?;
    let mut runs = Vec::new();
    collect_word_descendants(snapshot, &part_path, &paragraph_path, "r", &mut runs)?;
    Ok(runs.len())
}

fn word_text(node: &XmlNode, bindings: &[(String, String)]) -> String {
    fn append(node: &XmlNode, bindings: &[(String, String)], inside_text: bool, output: &mut String) {
        match node {
            XmlNode::Text { text } if inside_text => output.push_str(text),
            XmlNode::Element { children, .. } => {
                let mut bindings = bindings.to_vec();
                apply_bindings(node, &mut bindings);
                let inside_text = inside_text || is_word_name(node, "t", &bindings);
                for child in children {
                    append(child, &bindings, inside_text, output);
                }
            }
            _ => {}
        }
    }
    let mut output = String::new();
    append(node, bindings, false, &mut output);
    output
}

/// ✏️ Projects one editable run within a paragraph or nested table and its canonical address.
pub fn docx_top_level_run_at(snapshot: &DocxSnapshot, block_index: usize, run_index: usize) -> Result<DocxEditableRun, String> {
    let (part_path, block_path) = top_level_block_path(snapshot, block_index)?;
    let mut runs = Vec::new();
    collect_word_descendants(snapshot, &part_path, &block_path, "r", &mut runs)?;
    let run_path = runs.get(run_index).cloned().ok_or_else(|| format!("DOCX body block {block_index} has no run {run_index}"))?;
    let address = docx_xml_address(snapshot, part_path, run_path)?;
    let resolved = resolve_docx_xml_address(snapshot, &address)?;
    let text = word_text(resolved.node, &resolved.bindings);
    Ok(DocxEditableRun { text, address })
}

/// 🧭️ Resolves a projected block/run selection to its canonical XML address without projecting the document.
pub fn docx_block_run_address(snapshot: &DocxSnapshot, path: &DocxBlockPath, run_index: usize) -> Result<DocxXmlAddress, String> {
    let (part_path, body_path) = main_body_path(snapshot)?;
    let mut blocks_path = body_path;
    for segment in &path.segments {
        let table_path = word_child_path(snapshot, &part_path, &blocks_path, &["p", "tbl"], segment.block_index)?;
        let part = snapshot.xml_part(&part_path).ok_or_else(|| format!("missing DOCX XML part {part_path}"))?;
        let root = part.document.root.as_ref().ok_or_else(|| format!("DOCX XML part {part_path} has no root"))?;
        let (table, bindings) = scoped_node_at_path(root, &table_path)?;
        if !is_word_name(table, "tbl", &bindings) {
            return Err(format!("DOCX block {} is not a table", segment.block_index));
        }
        let row_path = word_child_path(snapshot, &part_path, &table_path, &["tr"], segment.row)?;
        blocks_path = word_child_path(snapshot, &part_path, &row_path, &["tc"], segment.cell)?;
    }
    let paragraph_path = word_child_path(snapshot, &part_path, &blocks_path, &["p", "tbl"], path.index)?;
    let part = snapshot.xml_part(&part_path).ok_or_else(|| format!("missing DOCX XML part {part_path}"))?;
    let root = part.document.root.as_ref().ok_or_else(|| format!("DOCX XML part {part_path} has no root"))?;
    let (paragraph, bindings) = scoped_node_at_path(root, &paragraph_path)?;
    if !is_word_name(paragraph, "p", &bindings) {
        return Err(format!("DOCX block {} is not a paragraph", path.index));
    }
    let mut runs = Vec::new();
    collect_word_descendants(snapshot, &part_path, &paragraph_path, "r", &mut runs)?;
    let run_path = runs.get(run_index).cloned().ok_or_else(|| format!("DOCX paragraph has no run {run_index}"))?;
    docx_xml_address(snapshot, part_path, run_path)
}

fn needs_preserved_space(text: &str) -> bool {
    let xml_space = |character: char| matches!(character, ' ' | '\t' | '\r' | '\n');
    text.chars().next().is_some_and(xml_space) || text.chars().next_back().is_some_and(xml_space)
}

fn set_xml_space(attrs: &mut Vec<XmlAttr>, preserve: bool) {
    attrs.retain(|attr| attr.name != "xml:space");
    if preserve {
        attrs.push(XmlAttr { name: "xml:space".into(), value: "preserve".into() });
    }
}

fn word_name_like(node: &XmlNode, local: &str) -> Result<String, String> {
    let XmlNode::Element { name, .. } = node else { return Err("WordprocessingML parent is not an element".into()) };
    Ok(name.split_once(':').map_or_else(|| local.to_string(), |(prefix, _)| format!("{prefix}:{local}")))
}

fn direct_word_child_index(node: &XmlNode, parent_bindings: &[(String, String)], local: &str) -> Option<usize> {
    let XmlNode::Element { children, .. } = node else { return None };
    children.iter().enumerate().find_map(|(index, child)| {
        let mut bindings = parent_bindings.to_vec();
        apply_bindings(child, &mut bindings);
        is_word_name(child, local, &bindings).then_some(index)
    })
}

fn direct_word_child_indices(node: &XmlNode, parent_bindings: &[(String, String)], local: &str) -> Vec<usize> {
    let XmlNode::Element { children, .. } = node else { return Vec::new() };
    children
        .iter()
        .enumerate()
        .filter_map(|(index, child)| {
            let mut bindings = parent_bindings.to_vec();
            apply_bindings(child, &mut bindings);
            is_word_name(child, local, &bindings).then_some(index)
        })
        .collect()
}

pub(super) fn run_with_text(resolved: &ResolvedDocxXmlAddress<'_>, text: &str) -> Result<XmlNode, String> {
    if !is_word_name(resolved.node, "r", &resolved.bindings) {
        return Err(format!("set-run-text requires a WordprocessingML run, got {}", resolved.node_name()));
    }
    if word_text(resolved.node, &resolved.bindings) == text {
        return Ok(resolved.node.clone());
    }
    let mut run = resolved.node.clone();
    let XmlNode::Element { name: run_name, children, .. } = &mut run else { unreachable!() };
    let mut found = false;
    for child in children.iter_mut() {
        let mut bindings = resolved.bindings.clone();
        apply_bindings(child, &mut bindings);
        if !is_word_name(child, "t", &bindings) {
            continue;
        }
        let XmlNode::Element { attrs, children, .. } = child else { unreachable!() };
        let insertion = children.iter().position(|child| matches!(child, XmlNode::Text { .. })).unwrap_or(0);
        children.retain(|child| !matches!(child, XmlNode::Text { .. }));
        if !found {
            children.insert(insertion.min(children.len()), XmlNode::Text { text: text.into() });
            set_xml_space(attrs, needs_preserved_space(text));
            found = true;
        } else {
            set_xml_space(attrs, false);
        }
    }
    if !found {
        let text_name = run_name.split_once(':').map_or_else(|| "t".to_string(), |(prefix, _)| format!("{prefix}:t"));
        let mut attrs = Vec::new();
        set_xml_space(&mut attrs, needs_preserved_space(text));
        children.push(XmlNode::Element { name: text_name, attrs, children: vec![XmlNode::Text { text: text.into() }] });
    }
    Ok(run)
}

pub(super) fn run_with_formatting(resolved: &ResolvedDocxXmlAddress<'_>, bold: bool, italic: bool, underline: bool) -> Result<XmlNode, String> {
    if !is_word_name(resolved.node, "r", &resolved.bindings) {
        return Err(format!("set-run-formatting requires a WordprocessingML run, got {}", resolved.node_name()));
    }
    let mut run = resolved.node.clone();
    let rpr_name = word_name_like(&run, "rPr")?;
    let XmlNode::Element { children, .. } = &mut run else { unreachable!() };
    let existing = direct_word_child_index(resolved.node, &resolved.bindings, "rPr");
    let rpr_index = existing.unwrap_or_else(|| {
        children.insert(0, XmlNode::Element { name: rpr_name.clone(), attrs: Vec::new(), children: Vec::new() });
        0
    });
    let mut bindings = resolved.bindings.clone();
    apply_bindings(&children[rpr_index], &mut bindings);
    for (local, value) in [("b", if bold { "1" } else { "0" }), ("i", if italic { "1" } else { "0" }), ("u", if underline { "single" } else { "none" })] {
        let indices = direct_word_child_indices(&children[rpr_index], &bindings, local);
        if indices.is_empty() {
            let prefix = qualified_word_prefix(&mut children[rpr_index], &mut bindings)?;
            let XmlNode::Element { children: properties, .. } = &mut children[rpr_index] else { unreachable!() };
            properties.push(XmlNode::Element { name: format!("{prefix}:{local}"), attrs: vec![XmlAttr { name: format!("{prefix}:val"), value: value.into() }], children: Vec::new() });
        } else {
            let XmlNode::Element { children: properties, .. } = &mut children[rpr_index] else { unreachable!() };
            for index in indices {
                let mut property_bindings = bindings.clone();
                apply_bindings(&properties[index], &mut property_bindings);
                set_word_attr(&mut properties[index], "val", value, &mut property_bindings)?;
            }
        }
    }
    Ok(run)
}

fn styles_part_path(snapshot: &DocxSnapshot) -> Result<String, String> {
    let main = crate::standards::v_ecma_376::subsets::base::io::import::deserializers::main_document_path(&snapshot.opc).map_err(|error| error.to_string())?;
    snapshot
        .opc
        .resolve_relationship(&main, crate::standards::v_ecma_376::subsets::base::io::REL_TYPE_STYLES)
        .or_else(|| snapshot.opc.resolve_relationship(&main, crate::standards::v_ecma_376::subsets::base::io::STRICT_REL_TYPE_STYLES))
        .ok_or_else(|| "DOCX main document has no internal styles relationship".into())
}

fn paragraph_style_exists(snapshot: &DocxSnapshot, style_id: &str) -> Result<bool, String> {
    let path = styles_part_path(snapshot)?;
    let part = snapshot.xml_part(&path).ok_or_else(|| format!("missing DOCX styles XML part {path}"))?;
    let root = part.document.root.as_ref().ok_or_else(|| format!("DOCX styles XML part {path} has no root"))?;
    let (_, root_bindings) = scoped_node_at_path(root, &[])?;
    if !is_word_name(root, "styles", &root_bindings) {
        return Err("DOCX styles root is not WordprocessingML styles".into());
    }
    let XmlNode::Element { children, .. } = root else { return Err("DOCX styles root is not an element".into()) };
    for child in children {
        let mut bindings = root_bindings.clone();
        apply_bindings(child, &mut bindings);
        if !is_word_name(child, "style", &bindings) {
            continue;
        }
        let id = word_attr(child, "styleId", &bindings);
        let kind = word_attr(child, "type", &bindings);
        if id == Some(style_id) {
            return Ok(kind.map_or(true, |kind| kind == "paragraph"));
        }
    }
    Ok(false)
}

pub(super) fn paragraph_with_style(snapshot: &DocxSnapshot, resolved: &ResolvedDocxXmlAddress<'_>, style_id: Option<&str>) -> Result<XmlNode, String> {
    if !is_word_name(resolved.node, "p", &resolved.bindings) {
        return Err(format!("set-paragraph-style requires a WordprocessingML paragraph, got {}", resolved.node_name()));
    }
    if let Some(style_id) = style_id {
        if style_id.is_empty() || !paragraph_style_exists(snapshot, style_id)? {
            return Err(format!("DOCX paragraph style {style_id:?} does not exist"));
        }
    }
    let mut paragraph = resolved.node.clone();
    let ppr_name = word_name_like(&paragraph, "pPr")?;
    let XmlNode::Element { children, .. } = &mut paragraph else { unreachable!() };
    let existing = direct_word_child_index(resolved.node, &resolved.bindings, "pPr");
    if existing.is_none() && style_id.is_none() {
        return Ok(paragraph);
    }
    let ppr_index = existing.unwrap_or_else(|| {
        children.insert(0, XmlNode::Element { name: ppr_name.clone(), attrs: Vec::new(), children: Vec::new() });
        0
    });
    let mut bindings = resolved.bindings.clone();
    apply_bindings(&children[ppr_index], &mut bindings);
    let indices = direct_word_child_indices(&children[ppr_index], &bindings, "pStyle");
    if let Some(style_id) = style_id {
        if indices.is_empty() {
            let prefix = qualified_word_prefix(&mut children[ppr_index], &mut bindings)?;
            let XmlNode::Element { children: properties, .. } = &mut children[ppr_index] else { unreachable!() };
            properties.insert(0, XmlNode::Element { name: format!("{prefix}:pStyle"), attrs: vec![XmlAttr { name: format!("{prefix}:val"), value: style_id.into() }], children: Vec::new() });
        } else {
            let XmlNode::Element { children: properties, .. } = &mut children[ppr_index] else { unreachable!() };
            for index in indices {
                let mut property_bindings = bindings.clone();
                apply_bindings(&properties[index], &mut property_bindings);
                set_word_attr(&mut properties[index], "val", style_id, &mut property_bindings)?;
            }
        }
    } else {
        let XmlNode::Element { children: properties, .. } = &mut children[ppr_index] else { unreachable!() };
        for index in indices.into_iter().rev() {
            properties.remove(index);
        }
    }
    Ok(paragraph)
}

fn table_row_node(table: &XmlNode, cells: &[String]) -> Result<XmlNode, String> {
    if cells.is_empty() {
        return Err("insert-table-row requires at least one cell".into());
    }
    let q = |local: &str| word_name_like(table, local);
    let mut row_cells = Vec::with_capacity(cells.len());
    for text in cells {
        let mut attrs = Vec::new();
        set_xml_space(&mut attrs, needs_preserved_space(text));
        let text_node = XmlNode::Element { name: q("t")?, attrs, children: vec![XmlNode::Text { text: text.clone() }] };
        let run = XmlNode::Element { name: q("r")?, attrs: Vec::new(), children: vec![text_node] };
        let paragraph = XmlNode::Element { name: q("p")?, attrs: Vec::new(), children: vec![run] };
        row_cells.push(XmlNode::Element { name: q("tc")?, attrs: Vec::new(), children: vec![paragraph] });
    }
    Ok(XmlNode::Element { name: q("tr")?, attrs: Vec::new(), children: row_cells })
}

pub(super) fn table_row_insertion(resolved: &ResolvedDocxXmlAddress<'_>, index: usize, cells: &[String]) -> Result<(usize, XmlNode), String> {
    if !is_word_name(resolved.node, "tbl", &resolved.bindings) {
        return Err(format!("insert-table-row requires a WordprocessingML table, got {}", resolved.node_name()));
    }
    let rows = direct_word_child_indices(resolved.node, &resolved.bindings, "tr");
    if index > rows.len() {
        return Err(format!("table row insertion index {index} exceeds {} rows", rows.len()));
    }
    let physical = rows.get(index).copied().or_else(|| rows.last().map(|value| value + 1)).unwrap_or_else(|| match resolved.node {
        XmlNode::Element { children, .. } => children.len(),
        _ => 0,
    });
    Ok((physical, table_row_node(resolved.node, cells)?))
}

pub(super) fn table_row_removal(resolved: &ResolvedDocxXmlAddress<'_>, index: usize) -> Result<(usize, XmlNode), String> {
    if !is_word_name(resolved.node, "tbl", &resolved.bindings) {
        return Err(format!("remove-table-row requires a WordprocessingML table, got {}", resolved.node_name()));
    }
    let rows = direct_word_child_indices(resolved.node, &resolved.bindings, "tr");
    if rows.len() <= 1 {
        return Err("remove-table-row cannot remove the table's last row".into());
    }
    let physical = *rows.get(index).ok_or_else(|| format!("table row removal index {index} exceeds {} rows", rows.len()))?;
    let XmlNode::Element { children, .. } = resolved.node else { unreachable!() };
    Ok((physical, children[physical].clone()))
}

pub(super) fn child_identity(resolved: &ResolvedDocxXmlAddress<'_>, node: &XmlNode) -> Result<String, String> {
    let mut bindings = resolved.bindings.clone();
    apply_bindings(node, &mut bindings);
    node_identity(node, &bindings)
}

impl ResolvedDocxXmlAddress<'_> {
    fn node_name(&self) -> String {
        node_identity(self.node, &self.bindings).unwrap_or_else(|_| "unknown".into())
    }
}

pub(super) fn validate_replacement_identity(resolved: &ResolvedDocxXmlAddress<'_>, replacement: &XmlNode) -> Result<(), String> {
    let mut bindings = resolved.parent_bindings.clone();
    apply_bindings(replacement, &mut bindings);
    let actual = node_identity(replacement, &bindings)?;
    let expected = node_identity(resolved.node, &resolved.bindings)?;
    if actual != expected {
        return Err(format!("replacement XML node identity {actual} does not match {expected}"));
    }
    Ok(())
}

pub(super) fn replace_addressed_node(snapshot: &mut DocxSnapshot, address: &DocxXmlAddress, replacement: XmlNode) -> Result<(), String> {
    let resolved = resolve_docx_xml_address(snapshot, address)?;
    validate_replacement_identity(&resolved, &replacement)?;
    let part_index = resolved.part_index;
    let root = snapshot.xml_parts[part_index].document.root.as_mut().ok_or_else(|| format!("DOCX XML part {} has no root", address.part_path))?;
    let node = node_mut_at_path(root, &address.node_path).ok_or_else(|| "DOCX XML address became stale during apply".to_string())?;
    *node = replacement;
    semio_s_artifact_stdio_xml::schema::snapshot::validate_xml_document_boundaries(&snapshot.xml_parts[part_index].document)?;
    snapshot.validate_authority().map_err(|error| error.to_string())
}
