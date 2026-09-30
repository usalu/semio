//! 🧬️ DocxMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, key/index-aware.

use crate::schema::diff::{
    dec_block, dec_bool, dec_str, dec_style, dec_xml_node, dec_xml_node_bin, decode_option, enc_block, enc_bool, enc_list, enc_str, enc_style, enc_xml_node, enc_xml_node_bin, encode_option, hex_decode, hex_encode, parse_usize, split_top_level,
    strip_brackets,
};
use crate::schema::diff::{diff_set_snapshot, DocxBlockPath, DocxDiff, DocxPathSegment, DocxXmlPartDiff, NamedModified, NamedTripleDiff};
#[cfg(test)]
use crate::schema::snapshot::DocxDocument;
use crate::schema::snapshot::{docx_part_is_xml, DocxBlock, DocxStyle, DocxXmlPart};
#[cfg(test)]
use crate::schema::snapshot::{DocxParagraph, DocxRun, DocxTable, DocxTableCell, DocxTableRow};
use crate::DocxSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use semio_s_artifact_stdio_xml::schema::diff::{diff_at_path as xml_diff_at_path, XmlChildAdded, XmlChildrenDiff, XmlElementDiff, XmlNodeDiff};
use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_from_text, XmlAttr, XmlNode};
#[cfg(test)]
use semio_s_artifact_stdio_zip::opc::{OpcTargetMode, RELS_CONTENT_TYPE, REL_TYPE_OFFICE_DOCUMENT};

//#region 🔖️Mutations
#[path = "➕insert-block/🦀️.rs"]
pub mod insert_block;
#[path = "🖌️insert-style/🦀️.rs"]
pub mod insert_style;
#[path = "➕️insert-table-row/🦀️.rs"]
pub mod insert_table_row;
#[path = "🧩️insert-xml-node/🦀️.rs"]
pub mod insert_xml_node;
#[path = "➖remove-block/🦀️.rs"]
pub mod remove_block;
#[path = "🗑️remove-part/🦀️.rs"]
pub mod remove_part;
#[path = "🧹remove-style/🦀️.rs"]
pub mod remove_style;
#[path = "➖️remove-table-row/🦀️.rs"]
pub mod remove_table_row;
#[path = "🧹️remove-xml-node/🦀️.rs"]
pub mod remove_xml_node;
#[path = "✍️set-block-content/🦀️.rs"]
pub mod set_block_content;
#[path = "🖌️set-paragraph-style/🦀️.rs"]
pub mod set_paragraph_style;
#[path = "📦set-part/🦀️.rs"]
pub mod set_part;
#[path = "🎨set-run-formatting/🦀️.rs"]
pub mod set_run_formatting;
#[path = "🔤set-run-text/🦀️.rs"]
pub mod set_run_text;
#[path = "🧭️xml-address/🦀️.rs"]
pub mod xml_address;
pub use xml_address::{
    docx_block_run_address, docx_top_level_block_address, docx_top_level_block_count, docx_top_level_run_at, docx_top_level_run_count, docx_xml_address, docx_xml_subtree_revision, resolve_docx_xml_address, DocxEditableRun, DocxXmlAddress,
    ResolvedDocxXmlAddress,
};
#[path = "🧩️replace-xml-node/🦀️.rs"]
pub mod replace_xml_node;
/// 📐️ Typed content mutation for `stdio.docx`. Beyond the baseline `SetSnapshot`, this addresses
/// the `document.body` block tree via `DocxBlockPath` (segments navigate through nested `Table`s,
/// mirrors svg's `NodePath` precedent), named styles by `DocxStyle::id`, and the raw OPC layer by
/// part path (for content this typed layer doesn't cover).
/// 🧪️ F6 VERIFIED: `#[derive(dsl::DslOps)]` on this enum ALSO fails (independent confirmation
/// beyond `DocxDiff`'s `DiffCodec` blocker, real `cargo check -p semio-s-plugin-stdio --lib`
/// output, then reverted) — `SetSnapshot{snapshot: DocxSnapshot}` fails with `DocxSnapshot:
/// DslField` is not satisfied (its `document.body: Vec<DocxBlock>` reaches the same data-carrying
/// enum `DocxDiff` hits); `InsertBlock`/`SetBlockContent`'s `block: DocxBlock` fails directly for
/// the same reason (`DocxBlock: DslField` is not satisfied); `InsertStyle`'s `style: DocxStyle` and
/// every `path: DocxBlockPath`-carrying variant also fail (`DocxStyle`/`DocxBlockPath: DslField` is
/// not satisfied — neither is itself `#[derive(dsl::DslRecord)]`, a SEPARATE reason from the enum
/// blocker, but confirms hand-roll is required regardless). `OpText`/`OpBinary` hand-rolled below,
/// reusing `DocxDiff`'s `pub(crate)` grammar primitives (`hex_encode`/`enc_block`/`enc_style`/
/// `split_top_level`/...).
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🌳️set-style-based-on/🦀️.rs"]
pub mod set_style_based_on;
#[path = "🏷️set-style-name/🦀️.rs"]
pub mod set_style_name;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = DocxSnapshot, diff = DocxDiff, schema = "DocxMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum DocxMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// ➕️ Inserts `block` at `path` (`path.index` = insertion index, FINAL state).
    InsertBlock(insert_block::InsertBlock),
    /// ➖️ Removes the block at `path` (`path.index` = BASE-state index).
    RemoveBlock(remove_block::RemoveBlock),
    /// ✍️ Replaces the full content of the block at `path` with `block`.
    SetBlockContent(set_block_content::SetBlockContent),
    /// ✍️ Replaces the literal text of run `run_index` in the paragraph at `path`.
    SetRunText(set_run_text::SetRunText),
    /// ↩️ Replaces one canonical XML node exactly; used by compact inverses.
    ReplaceXmlNode(replace_xml_node::ReplaceXmlNode),
    /// 🎨️ Sets run `run_index`'s bold/italic/underline flags in the paragraph at `path`.
    SetRunFormatting(set_run_formatting::SetRunFormatting),
    /// 🖌️ Sets or clears one paragraph style reference.
    SetParagraphStyle(set_paragraph_style::SetParagraphStyle),
    /// ➕️ Inserts one canonical table row.
    InsertTableRow(insert_table_row::InsertTableRow),
    /// ➖️ Removes one canonical table row.
    RemoveTableRow(remove_table_row::RemoveTableRow),
    /// 🧩️ Inserts one exact canonical XML child; used by compact inverses.
    InsertXmlNode(insert_xml_node::InsertXmlNode),
    /// 🧹️ Removes one exact canonical XML child; used by compact inverses.
    RemoveXmlNode(remove_xml_node::RemoveXmlNode),
    /// ➕️ Inserts a named style.
    InsertStyle(insert_style::InsertStyle),
    /// ➖️ Removes the style with id `id`.
    RemoveStyle(remove_style::RemoveStyle),
    /// 🏷️ Renames the style with id `id`.
    SetStyleName(set_style_name::SetStyleName),
    /// 🔗 Sets (or, if `None`, clears) the style with id `id`'s `based_on`.
    SetStyleBasedOn(set_style_based_on::SetStyleBasedOn),
    /// ✍️ Sets a raw OPC part (content this typed layer doesn't model), inserting or replacing.
    SetPart(set_part::SetPart),
    /// ➖️ Removes a raw OPC part by path.
    RemovePart(remove_part::RemovePart),
}

/// 📇️ Kebab-case spelling of every `DocxMutation` variant, in declaration order -- the exhaustive
/// mutation catalog `../🔣️oracle.json`'s `kinds` array is required to match verbatim
/// (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps that honest; the
/// framework never parses Rust to check it itself). Mirrors `print_docx_mutation`'s own keyword
/// match entry-for-entry, so `KINDS[i]` is exactly what `print_op()` emits for the enum's `i`-th
/// variant (via `demo_mutation_cases()`, which already carries one instance per variant in this
/// same order).
pub const KINDS: &[&str] = &[
    "set-snapshot",
    "insert-block",
    "remove-block",
    "set-block-content",
    "set-run-text",
    "replace-xml-node",
    "set-run-formatting",
    "set-paragraph-style",
    "insert-table-row",
    "remove-table-row",
    "insert-xml-node",
    "remove-xml-node",
    "insert-style",
    "remove-style",
    "set-style-name",
    "set-style-based-on",
    "set-part",
    "remove-part",
];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` -- the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_docx_mutation(snapshot: &mut DocxSnapshot, mutation: &DocxMutation) -> protocol::MutationOutcome<DocxDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

/// 📨️ Builds the operation of semantic kind `kind` from its editable payload JSON — the leaf wire (`payload_value()`) a
/// `🥒️.feature` row carries — through the derive's generic `from_payload_value`.
pub fn decode_docx_mutation_payload(kind: &str, payload: &str) -> Result<DocxMutation, String> {
    protocol::os_pack::from_json_str(payload).and_then(|value| <DocxMutation as Mutation<DocxSnapshot>>::from_payload_value(kind, value)).map_err(|error| error.to_string())
}

/// 🔙️ The operations that undo `mutation` on `base` — the aggregate's own leaf-owned `Mutation::inverse`, the law a
/// case's inverse scenario holds this implementation to.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_docx_mutation(mutation: &DocxMutation, base: &DocxSnapshot) -> Vec<DocxMutation> {
    <DocxMutation as Mutation<DocxSnapshot>>::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
fn element_children_mut<'a>(node: &'a mut XmlNode, expected: &str) -> Option<&'a mut Vec<XmlNode>> {
    match node {
        XmlNode::Element { name, children, .. } if name == expected => Some(children),
        _ => None,
    }
}

fn nth_element_index(nodes: &[XmlNode], name: &str, ordinal: usize) -> Option<usize> {
    nodes.iter().enumerate().filter(|(_, node)| matches!(node, XmlNode::Element { name: actual, .. } if actual == name)).nth(ordinal).map(|(index, _)| index)
}

fn block_slot(nodes: &[XmlNode], ordinal: usize) -> Option<usize> {
    nodes.iter().enumerate().filter(|(_, node)| matches!(node, XmlNode::Element { name, .. } if name == "w:p" || name == "w:tbl")).nth(ordinal).map(|(index, _)| index)
}

fn block_insert_slot(nodes: &[XmlNode], ordinal: usize) -> Option<usize> {
    if let Some(index) = block_slot(nodes, ordinal) {
        return Some(index);
    }
    let count = nodes.iter().filter(|node| matches!(node, XmlNode::Element { name, .. } if name == "w:p" || name == "w:tbl")).count();
    if ordinal != count {
        return None;
    }
    Some(nodes.iter().rposition(|node| matches!(node, XmlNode::Element { name, .. } if name == "w:p" || name == "w:tbl")).map_or(0, |index| index + 1))
}

fn main_body_mut(snapshot: &mut DocxSnapshot) -> Option<&mut Vec<XmlNode>> {
    let main_path = crate::standards::v_ecma_376::subsets::base::io::import::deserializers::main_document_path(&snapshot.opc).ok()?;
    let document = &mut snapshot.xml_part_mut(&main_path)?.document;
    let root = document.root.as_mut()?;
    let children = element_children_mut(root, "w:document")?;
    let body_index = nth_element_index(children, "w:body", 0)?;
    element_children_mut(&mut children[body_index], "w:body")
}

fn nested_blocks_mut<'a>(blocks: &'a mut Vec<XmlNode>, segments: &[DocxPathSegment]) -> Option<&'a mut Vec<XmlNode>> {
    let Some((segment, rest)) = segments.split_first() else { return Some(blocks) };
    let table_index = block_slot(blocks, segment.block_index)?;
    let rows = element_children_mut(&mut blocks[table_index], "w:tbl")?;
    let row_index = nth_element_index(rows, "w:tr", segment.row)?;
    let cells = element_children_mut(&mut rows[row_index], "w:tr")?;
    let cell_index = nth_element_index(cells, "w:tc", segment.cell)?;
    let cell_children = element_children_mut(&mut cells[cell_index], "w:tc")?;
    nested_blocks_mut(cell_children, rest)
}

fn styles_root_mut(snapshot: &mut DocxSnapshot) -> Option<&mut Vec<XmlNode>> {
    let main_path = crate::standards::v_ecma_376::subsets::base::io::import::deserializers::main_document_path(&snapshot.opc).ok()?;
    let styles_path = snapshot
        .opc
        .resolve_relationship(&main_path, crate::standards::v_ecma_376::subsets::base::io::REL_TYPE_STYLES)
        .or_else(|| snapshot.opc.resolve_relationship(&main_path, crate::standards::v_ecma_376::subsets::base::io::STRICT_REL_TYPE_STYLES))?;
    let root = snapshot.xml_part_mut(&styles_path)?.document.root.as_mut()?;
    element_children_mut(root, "w:styles")
}

fn style_index(nodes: &[XmlNode], id: &str) -> Option<usize> {
    nodes.iter().position(|node| matches!(node, XmlNode::Element { name, attrs, .. } if name == "w:style" && attrs.iter().any(|attribute| attribute.name == "w:styleId" && attribute.value == id)))
}

fn is_addressed_xml_mutation(mutation: &DocxMutation) -> bool {
    matches!(
        mutation,
        DocxMutation::SetRunText(_)
            | DocxMutation::ReplaceXmlNode(_)
            | DocxMutation::SetRunFormatting(_)
            | DocxMutation::SetParagraphStyle(_)
            | DocxMutation::InsertTableRow(_)
            | DocxMutation::RemoveTableRow(_)
            | DocxMutation::InsertXmlNode(_)
            | DocxMutation::RemoveXmlNode(_)
    )
}

fn apply_to_snapshot(base: &DocxSnapshot, mutation: &DocxMutation) -> Option<DocxSnapshot> {
    let mut next = base.clone();
    if is_addressed_xml_mutation(mutation) {
        apply_addressed_xml_mutation_in_place(&mut next, mutation).ok()?;
        return Some(next);
    }
    match mutation {
        DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => return Some(snapshot.clone()),
        DocxMutation::InsertBlock(insert_block::InsertBlock { path, block }) => {
            let blocks = nested_blocks_mut(main_body_mut(&mut next)?, &path.segments)?;
            let index = block_insert_slot(blocks, path.index)?;
            blocks.insert(index, crate::standards::v_ecma_376::subsets::base::io::export::serializers::block_to_xml(block));
        }
        DocxMutation::RemoveBlock(remove_block::RemoveBlock { path }) => {
            let blocks = nested_blocks_mut(main_body_mut(&mut next)?, &path.segments)?;
            blocks.remove(block_slot(blocks, path.index)?);
        }
        DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path, block }) => {
            let blocks = nested_blocks_mut(main_body_mut(&mut next)?, &path.segments)?;
            let index = block_slot(blocks, path.index)?;
            blocks[index] = crate::standards::v_ecma_376::subsets::base::io::export::serializers::block_to_xml(block);
        }
        DocxMutation::SetRunText(_)
        | DocxMutation::ReplaceXmlNode(_)
        | DocxMutation::SetRunFormatting(_)
        | DocxMutation::SetParagraphStyle(_)
        | DocxMutation::InsertTableRow(_)
        | DocxMutation::RemoveTableRow(_)
        | DocxMutation::InsertXmlNode(_)
        | DocxMutation::RemoveXmlNode(_) => unreachable!(),
        DocxMutation::InsertStyle(insert_style::InsertStyle { style }) => {
            let styles = styles_root_mut(&mut next)?;
            if style_index(styles, &style.id).is_some() {
                return None;
            }
            styles.push(crate::standards::v_ecma_376::subsets::base::io::export::serializers::style_to_xml(style));
        }
        DocxMutation::RemoveStyle(remove_style::RemoveStyle { id }) => {
            let styles = styles_root_mut(&mut next)?;
            styles.remove(style_index(styles, id)?);
        }
        DocxMutation::SetStyleName(set_style_name::SetStyleName { id, name }) => {
            let styles = styles_root_mut(&mut next)?;
            let index = style_index(styles, id)?;
            let children = element_children_mut(&mut styles[index], "w:style")?;
            children.retain(|node| !matches!(node, XmlNode::Element { name, .. } if name == "w:name"));
            children.insert(0, XmlNode::Element { name: "w:name".into(), attrs: vec![XmlAttr { name: "w:val".into(), value: name.clone() }], children: Vec::new() });
        }
        DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id, based_on }) => {
            let styles = styles_root_mut(&mut next)?;
            let index = style_index(styles, id)?;
            let children = element_children_mut(&mut styles[index], "w:style")?;
            children.retain(|node| !matches!(node, XmlNode::Element { name, .. } if name == "w:basedOn"));
            if let Some(based_on) = based_on {
                children.push(XmlNode::Element { name: "w:basedOn".into(), attrs: vec![XmlAttr { name: "w:val".into(), value: based_on.clone() }], children: Vec::new() });
            }
        }
        DocxMutation::SetPart(set_part::SetPart { path, content_type, bytes }) => {
            let path = path.trim_start_matches('/').to_string();
            if docx_part_is_xml(&path, content_type) {
                let text = std::str::from_utf8(bytes).ok()?;
                let document = xml_document_from_text(text).ok()?;
                next.opc.content_types.set_override(&path, content_type);
                if let Some(part) = next.xml_part_mut(&path) {
                    part.content_type.clone_from(content_type);
                    part.document = document;
                } else {
                    next.xml_parts.push(DocxXmlPart { path, content_type: content_type.clone(), document });
                    next.xml_parts.sort_by(|left, right| left.path.cmp(&right.path));
                }
            } else {
                next.opc.set_part(&path, content_type, bytes.clone());
            }
        }
        DocxMutation::RemovePart(remove_part::RemovePart { path }) => {
            let path = path.trim_start_matches('/');
            let before = next.xml_parts.len() + next.opc.parts.len();
            next.xml_parts.retain(|part| part.path != path);
            next.opc.parts.retain(|part| part.path != path);
            (before != next.xml_parts.len() + next.opc.parts.len()).then_some(())?;
        }
    }
    Some(next)
}

/// 🧩️ Compact canonical XML mutation plan consumed by direct and retained execution.
#[derive(Clone, Debug, PartialEq)]
pub struct PreparedDocxXmlMutation {
    pub diff: DocxDiff,
    pub inverse: DocxMutation,
    pub changed: bool,
    address: DocxXmlAddress,
    replacement: XmlNode,
}

fn part_diff(address: &DocxXmlAddress, leaf: XmlNodeDiff) -> DocxDiff {
    let document = xml_diff_at_path(&address.node_path, leaf);
    DocxDiff { opc: None, xml_parts: Some(NamedTripleDiff { modified: vec![NamedModified { key: address.part_path.clone(), diff: DocxXmlPartDiff { content_type: None, document: Some(document) } }], ..Default::default() }) }
}

fn replacement_plan(snapshot: &DocxSnapshot, address: &DocxXmlAddress, replacement: XmlNode) -> Result<PreparedDocxXmlMutation, String> {
    let resolved = resolve_docx_xml_address(snapshot, address)?;
    xml_address::validate_replacement_identity(&resolved, &replacement)?;
    let previous = resolved.node.clone();
    let changed = previous != replacement;
    let inverse_address =
        DocxXmlAddress { part_path: address.part_path.clone(), node_path: address.node_path.clone(), expected_name: address.expected_name.clone(), revision: xml_address::revision_after_replacement(snapshot, address, &replacement)? };
    let diff = changed.then(|| part_diff(address, XmlNodeDiff::Replace { node: Some(replacement.clone()) })).unwrap_or_default();
    Ok(PreparedDocxXmlMutation { diff, inverse: DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address: inverse_address, node: previous }), changed, address: address.clone(), replacement })
}

fn child_insert_plan(snapshot: &DocxSnapshot, parent: &DocxXmlAddress, index: usize, node: XmlNode) -> Result<PreparedDocxXmlMutation, String> {
    let resolved = resolve_docx_xml_address(snapshot, parent)?;
    let XmlNode::Element { children, .. } = resolved.node else { return Err("insert-xml-node parent is not an element".into()) };
    if index > children.len() {
        return Err(format!("XML child insertion index {index} exceeds {} children", children.len()));
    }
    let expected_name = xml_address::child_identity(&resolved, &node)?;
    let revision = docx_xml_subtree_revision(&node);
    let mut replacement = resolved.node.clone();
    let XmlNode::Element { children, .. } = &mut replacement else { unreachable!() };
    children.insert(index, node.clone());
    let inverse_parent = DocxXmlAddress { part_path: parent.part_path.clone(), node_path: parent.node_path.clone(), expected_name: parent.expected_name.clone(), revision: xml_address::revision_after_replacement(snapshot, parent, &replacement)? };
    let diff = part_diff(parent, XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![XmlChildAdded { index, item: node }] }) }));
    Ok(PreparedDocxXmlMutation { diff, inverse: DocxMutation::RemoveXmlNode(remove_xml_node::RemoveXmlNode { parent: inverse_parent, index, expected_name, revision }), changed: true, address: parent.clone(), replacement })
}

fn child_remove_plan(snapshot: &DocxSnapshot, parent: &DocxXmlAddress, index: usize, expected_name: &str, revision: &str) -> Result<PreparedDocxXmlMutation, String> {
    let resolved = resolve_docx_xml_address(snapshot, parent)?;
    let XmlNode::Element { children, .. } = resolved.node else { return Err("remove-xml-node parent is not an element".into()) };
    let previous = children.get(index).ok_or_else(|| format!("XML child removal index {index} exceeds {} children", children.len()))?.clone();
    let actual_name = xml_address::child_identity(&resolved, &previous)?;
    if actual_name != expected_name {
        return Err(format!("XML child removal expected {expected_name} but resolved {actual_name}"));
    }
    let actual_revision = docx_xml_subtree_revision(&previous);
    if actual_revision != revision {
        return Err(format!("XML child removal revision changed from {revision} to {actual_revision}"));
    }
    let mut replacement = resolved.node.clone();
    let XmlNode::Element { children, .. } = &mut replacement else { unreachable!() };
    children.remove(index);
    let inverse_parent = DocxXmlAddress { part_path: parent.part_path.clone(), node_path: parent.node_path.clone(), expected_name: parent.expected_name.clone(), revision: xml_address::revision_after_replacement(snapshot, parent, &replacement)? };
    let diff = part_diff(parent, XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: vec![index], modified: Vec::new(), added: Vec::new() }) }));
    Ok(PreparedDocxXmlMutation { diff, inverse: DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent: inverse_parent, index, node: previous }), changed: true, address: parent.clone(), replacement })
}

/// 🧩️ Validates one addressed edit and prepares its compact target-part diff and exact inverse.
pub fn prepare_addressed_xml_mutation(snapshot: &DocxSnapshot, mutation: &DocxMutation) -> Result<PreparedDocxXmlMutation, String> {
    match mutation {
        DocxMutation::SetRunText(set_run_text::SetRunText { address, text }) => {
            let resolved = resolve_docx_xml_address(snapshot, address)?;
            replacement_plan(snapshot, address, xml_address::run_with_text(&resolved, text)?)
        }
        DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address, node }) => replacement_plan(snapshot, address, node.clone()),
        DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address, bold, italic, underline }) => {
            let resolved = resolve_docx_xml_address(snapshot, address)?;
            replacement_plan(snapshot, address, xml_address::run_with_formatting(&resolved, *bold, *italic, *underline)?)
        }
        DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address, style_id }) => {
            let resolved = resolve_docx_xml_address(snapshot, address)?;
            replacement_plan(snapshot, address, xml_address::paragraph_with_style(snapshot, &resolved, style_id.as_deref())?)
        }
        DocxMutation::InsertTableRow(insert_table_row::InsertTableRow { address, index, cells }) => {
            let resolved = resolve_docx_xml_address(snapshot, address)?;
            let (physical, row) = xml_address::table_row_insertion(&resolved, *index, cells)?;
            child_insert_plan(snapshot, address, physical, row)
        }
        DocxMutation::RemoveTableRow(remove_table_row::RemoveTableRow { address, index }) => {
            let resolved = resolve_docx_xml_address(snapshot, address)?;
            let (physical, row) = xml_address::table_row_removal(&resolved, *index)?;
            let expected_name = xml_address::child_identity(&resolved, &row)?;
            child_remove_plan(snapshot, address, physical, &expected_name, &docx_xml_subtree_revision(&row))
        }
        DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent, index, node }) => child_insert_plan(snapshot, parent, *index, node.clone()),
        DocxMutation::RemoveXmlNode(remove_xml_node::RemoveXmlNode { parent, index, expected_name, revision }) => child_remove_plan(snapshot, parent, *index, expected_name, revision),
        _ => Err("mutation is not an addressed canonical XML edit".into()),
    }
}

/// ▶️ Applies one revision-bound canonical XML edit without cloning or projecting unrelated parts.
pub fn apply_addressed_xml_mutation_in_place(snapshot: &mut DocxSnapshot, mutation: &DocxMutation) -> Result<PreparedDocxXmlMutation, String> {
    let prepared = prepare_addressed_xml_mutation(snapshot, mutation)?;
    if prepared.changed {
        xml_address::replace_addressed_node(snapshot, &prepared.address, prepared.replacement.clone())?;
    }
    Ok(prepared)
}

pub(crate) fn agg_diff(this: &DocxMutation, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
    if is_addressed_xml_mutation(this) {
        return match prepare_addressed_xml_mutation(base, this) {
            Ok(prepared) => protocol::MutationOutcome::new(prepared.diff),
            Err(message) => protocol::MutationOutcome::error("mutation.target-mismatch", message, mutation_target(this)),
        };
    }
    protocol::MutationOutcome::new(apply_to_snapshot(base, this).map_or_else(DocxDiff::default, |next| diff_set_snapshot(base, &next)))
}

fn mutation_target(mutation: &DocxMutation) -> Vec<String> {
    let address = match mutation {
        DocxMutation::SetRunText(value) => Some(&value.address),
        DocxMutation::ReplaceXmlNode(value) => Some(&value.address),
        DocxMutation::SetRunFormatting(value) => Some(&value.address),
        DocxMutation::SetParagraphStyle(value) => Some(&value.address),
        DocxMutation::InsertTableRow(value) => Some(&value.address),
        DocxMutation::RemoveTableRow(value) => Some(&value.address),
        DocxMutation::InsertXmlNode(value) => Some(&value.parent),
        DocxMutation::RemoveXmlNode(value) => Some(&value.parent),
        _ => None,
    };
    address.map_or_else(Vec::new, |address| std::iter::once(address.part_path.clone()).chain(address.node_path.iter().map(usize::to_string)).collect())
}

pub(crate) fn agg_inverse(this: &DocxMutation, base: &DocxSnapshot) -> Vec<DocxMutation> {
    if is_addressed_xml_mutation(this) {
        return prepare_addressed_xml_mutation(base, this).map(|prepared| vec![prepared.inverse]).unwrap_or_default();
    }
    vec![DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })]
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `DocxMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above) — reuses `DocxDiff`'s `pub(crate)` grammar primitives
/// (`hex_encode`/`enc_block`/`enc_style`/`enc_opc_part`/`split_top_level`/...) rather than
/// duplicating them a second time in this file. Grammar: `keyword arg=value ...`
/// (space-separated), same shape the derive's own handcrafted-wrapper convention uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_path_segment(seg: &DocxPathSegment) -> String {
    format!("[{},{},{}]", seg.block_index, seg.row, seg.cell)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_path_segment(s: &str) -> Result<DocxPathSegment, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [block_index, row, cell] = parts.as_slice() else { return Err(format!("path segment: expected 3 fields, got {}", parts.len())) };
    Ok(DocxPathSegment { block_index: parse_usize(block_index)?, row: parse_usize(row)?, cell: parse_usize(cell)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_block_path(p: &DocxBlockPath) -> String {
    format!("[{},{}]", enc_list(&p.segments, enc_path_segment), p.index)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_block_path(s: &str) -> Result<DocxBlockPath, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [segments, index] = parts.as_slice() else { return Err(format!("block path: expected 2 fields, got {}", parts.len())) };
    Ok(DocxBlockPath { segments: dec_list_segments(segments)?, index: parse_usize(index)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_list_segments(s: &str) -> Result<Vec<DocxPathSegment>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_path_segment).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_docx_snapshot(snapshot: &DocxSnapshot) -> String {
    hex_encode(dsl::json::to_json_string(snapshot).as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_docx_snapshot(value: &str) -> Result<DocxSnapshot, String> {
    let bytes = hex_decode(value)?;
    let text = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    dsl::json::from_json_str(text).map_err(|error| error.to_string())
}

fn enc_xml_address(address: &DocxXmlAddress) -> String {
    hex_encode(dsl::json::to_json_string(address).as_bytes())
}

fn dec_xml_address(value: &str) -> Result<DocxXmlAddress, String> {
    let bytes = hex_decode(value)?;
    let text = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    dsl::json::from_json_str(text).map_err(|error| error.to_string())
}

fn dec_string_list(value: &str) -> Result<Vec<String>, String> {
    split_top_level(strip_brackets(value)?, ',').into_iter().filter(|value| !value.is_empty()).map(dec_str).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_docx_mutation(m: &DocxMutation) -> String {
    match m {
        DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_docx_snapshot(snapshot)),
        DocxMutation::InsertBlock(insert_block::InsertBlock { path, block }) => format!("insert-block path={} block={}", enc_block_path(path), enc_block(block)),
        DocxMutation::RemoveBlock(remove_block::RemoveBlock { path }) => format!("remove-block path={}", enc_block_path(path)),
        DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path, block }) => format!("set-block-content path={} block={}", enc_block_path(path), enc_block(block)),
        DocxMutation::SetRunText(set_run_text::SetRunText { address, text }) => format!("set-run-text address={} text={}", enc_xml_address(address), enc_str(text)),
        DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address, node }) => format!("replace-xml-node address={} node={}", enc_xml_address(address), enc_xml_node(node)),
        DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address, bold, italic, underline }) => {
            format!("set-run-formatting address={} bold={} italic={} underline={}", enc_xml_address(address), enc_bool(bold), enc_bool(italic), enc_bool(underline))
        }
        DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address, style_id }) => {
            format!("set-paragraph-style address={} style-id={}", enc_xml_address(address), encode_option(style_id, |value| enc_str(value)))
        }
        DocxMutation::InsertTableRow(insert_table_row::InsertTableRow { address, index, cells }) => {
            format!("insert-table-row address={} index={} cells={}", enc_xml_address(address), index, enc_list(cells, |value| enc_str(value)))
        }
        DocxMutation::RemoveTableRow(remove_table_row::RemoveTableRow { address, index }) => format!("remove-table-row address={} index={}", enc_xml_address(address), index),
        DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent, index, node }) => {
            format!("insert-xml-node parent={} index={} node={}", enc_xml_address(parent), index, enc_xml_node(node))
        }
        DocxMutation::RemoveXmlNode(remove_xml_node::RemoveXmlNode { parent, index, expected_name, revision }) => {
            format!("remove-xml-node parent={} index={} expected-name={} revision={}", enc_xml_address(parent), index, enc_str(expected_name), enc_str(revision))
        }
        DocxMutation::InsertStyle(insert_style::InsertStyle { style }) => format!("insert-style style={}", enc_style(style)),
        DocxMutation::RemoveStyle(remove_style::RemoveStyle { id }) => format!("remove-style id={}", enc_str(id)),
        DocxMutation::SetStyleName(set_style_name::SetStyleName { id, name }) => format!("set-style-name id={} name={}", enc_str(id), enc_str(name)),
        DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id, based_on }) => format!("set-style-based-on id={} based-on={}", enc_str(id), encode_option(based_on, |v| enc_str(v))),
        DocxMutation::SetPart(set_part::SetPart { path, content_type, bytes }) => format!("set-part path={} content-type={} bytes={}", enc_str(path), enc_str(content_type), hex_encode(bytes)),
        DocxMutation::RemovePart(remove_part::RemovePart { path }) => format!("remove-part path={}", enc_str(path)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_docx_mutation(line: &str) -> Result<DocxMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("docx mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("docx mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_docx_snapshot(arg("snapshot")?)? })),
        "insert-block" => Ok(DocxMutation::InsertBlock(insert_block::InsertBlock { path: dec_block_path(arg("path")?)?, block: dec_block(arg("block")?)? })),
        "remove-block" => Ok(DocxMutation::RemoveBlock(remove_block::RemoveBlock { path: dec_block_path(arg("path")?)? })),
        "set-block-content" => Ok(DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path: dec_block_path(arg("path")?)?, block: dec_block(arg("block")?)? })),
        "set-run-text" => Ok(DocxMutation::SetRunText(set_run_text::SetRunText { address: dec_xml_address(arg("address")?)?, text: dec_str(arg("text")?)? })),
        "replace-xml-node" => Ok(DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address: dec_xml_address(arg("address")?)?, node: dec_xml_node(arg("node")?)? })),
        "set-run-formatting" => {
            Ok(DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address: dec_xml_address(arg("address")?)?, bold: dec_bool(arg("bold")?)?, italic: dec_bool(arg("italic")?)?, underline: dec_bool(arg("underline")?)? }))
        }
        "set-paragraph-style" => Ok(DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address: dec_xml_address(arg("address")?)?, style_id: decode_option(arg("style-id")?, dec_str)? })),
        "insert-table-row" => Ok(DocxMutation::InsertTableRow(insert_table_row::InsertTableRow { address: dec_xml_address(arg("address")?)?, index: usize_arg("index")?, cells: dec_string_list(arg("cells")?)? })),
        "remove-table-row" => Ok(DocxMutation::RemoveTableRow(remove_table_row::RemoveTableRow { address: dec_xml_address(arg("address")?)?, index: usize_arg("index")? })),
        "insert-xml-node" => Ok(DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent: dec_xml_address(arg("parent")?)?, index: usize_arg("index")?, node: dec_xml_node(arg("node")?)? })),
        "remove-xml-node" => {
            Ok(DocxMutation::RemoveXmlNode(remove_xml_node::RemoveXmlNode { parent: dec_xml_address(arg("parent")?)?, index: usize_arg("index")?, expected_name: dec_str(arg("expected-name")?)?, revision: dec_str(arg("revision")?)? }))
        }
        "insert-style" => Ok(DocxMutation::InsertStyle(insert_style::InsertStyle { style: dec_style(arg("style")?)? })),
        "remove-style" => Ok(DocxMutation::RemoveStyle(remove_style::RemoveStyle { id: dec_str(arg("id")?)? })),
        "set-style-name" => Ok(DocxMutation::SetStyleName(set_style_name::SetStyleName { id: dec_str(arg("id")?)?, name: dec_str(arg("name")?)? })),
        "set-style-based-on" => Ok(DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: dec_str(arg("id")?)?, based_on: decode_option(arg("based-on")?, dec_str)? })),
        "set-part" => Ok(DocxMutation::SetPart(set_part::SetPart { path: dec_str(arg("path")?)?, content_type: dec_str(arg("content-type")?)?, bytes: hex_decode(arg("bytes")?)? })),
        "remove-part" => Ok(DocxMutation::RemovePart(remove_part::RemovePart { path: dec_str(arg("path")?)? })),
        other => Err(format!("docx mutation: unknown keyword {other:?}")),
    }
}

impl OpText for DocxMutation {
    fn print_op(&self) -> String {
        print_docx_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_docx_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryCodec
/// 🧪️ FG-wave: real recursive binary primitives backing the upgraded `OpBinary` impl below --
/// mirrors `📰️xml/…/🧬️mutations/🦀️.rs`'s own `enc_node_path_bin`/`enc_xml_snapshot_bin`
/// shape, reusing `store::pack_rt::write_varint_u64`/`store::ByteReader` plus `DocxDiff`'s own
/// `write_str_lp`/`read_str_lp`/`write_bytes_lp`/`read_bytes_lp`/`enc_block_bin`/`dec_block_bin`/
/// `enc_style_bin`/`dec_style_bin`/`enc_opc_part_bin`/`dec_opc_part_bin`/`enc_rel_bin`/
/// `dec_rel_bin` (`../🔺️diff/🦀️.rs`, `pub(crate)` to this artifact).
use crate::schema::diff::{dec_block_bin, dec_style_bin, enc_block_bin, enc_style_bin, read_bytes_lp, read_str_lp, write_bytes_lp, write_str_lp};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_path_segment_bin(seg: &DocxPathSegment, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, seg.block_index as u64);
    store::pack_rt::write_varint_u64(out, seg.row as u64);
    store::pack_rt::write_varint_u64(out, seg.cell as u64);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_path_segment_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxPathSegment, String> {
    let block_index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let row = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let cell = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(DocxPathSegment { block_index, row, cell })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_block_path_bin(p: &DocxBlockPath, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, p.segments.len() as u64);
    for seg in &p.segments {
        enc_path_segment_bin(seg, out);
    }
    store::pack_rt::write_varint_u64(out, p.index as u64);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_block_path_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxBlockPath, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut segments = Vec::with_capacity(count as usize);
    for _ in 0..count {
        segments.push(dec_path_segment_bin(reader)?);
    }
    let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(DocxBlockPath { segments, index })
}

fn enc_xml_address_bin(address: &DocxXmlAddress, output: &mut Vec<u8>) {
    write_str_lp(output, &address.part_path);
    store::pack_rt::write_varint_u64(output, address.node_path.len() as u64);
    for index in &address.node_path {
        store::pack_rt::write_varint_u64(output, *index as u64);
    }
    write_str_lp(output, &address.expected_name);
    write_str_lp(output, &address.revision);
}

fn dec_xml_address_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxXmlAddress, String> {
    let part_path = read_str_lp(reader)?;
    let count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut node_path = Vec::with_capacity(count as usize);
    for _ in 0..count {
        node_path.push(reader.read_varint_u64().map_err(|error| error.to_string())? as usize);
    }
    let expected_name = read_str_lp(reader)?;
    let revision = read_str_lp(reader)?;
    Ok(DocxXmlAddress { part_path, node_path, expected_name, revision })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_docx_snapshot_bin(snapshot: &DocxSnapshot, out: &mut Vec<u8>) {
    write_bytes_lp(out, dsl::json::to_json_string(snapshot).as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_docx_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<DocxSnapshot, String> {
    let bytes = read_bytes_lp(reader)?;
    let text = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    dsl::json::from_json_str(text).map_err(|error| error.to_string())
}
//#endregion 🔖️OpBinaryCodec

//#region 🏷️WireTags
/// 🏷️ Op tags of `DocxMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("💾️binary/📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_INSERT_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-block");
const TAG_REMOVE_BLOCK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-block");
const TAG_SET_BLOCK_CONTENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-block-content");
const TAG_SET_RUN_TEXT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-run-text");
const TAG_SET_RUN_FORMATTING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-run-formatting");
const TAG_INSERT_STYLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-style");
const TAG_REMOVE_STYLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-style");
const TAG_SET_STYLE_NAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-style-name");
const TAG_SET_STYLE_BASED_ON: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-style-based-on");
const TAG_SET_PART: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-part");
const TAG_REMOVE_PART: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-part");
const TAG_REPLACE_XML_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "replace-xml-node");
const TAG_SET_PARAGRAPH_STYLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-paragraph-style");
const TAG_INSERT_TABLE_ROW: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-table-row");
const TAG_REMOVE_TABLE_ROW: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-table-row");
const TAG_INSERT_XML_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-xml-node");
const TAG_REMOVE_XML_NODE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-xml-node");
//#endregion 🏷️WireTags

/// 🧪️ FG-wave: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape --
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the
/// `DocxMutation` variant ordinal, in the same 0-11 order `print_docx_mutation`'s own keyword
/// match uses.
impl OpBinary for DocxMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { .. }) => TAG_SET_SNAPSHOT,
            DocxMutation::InsertBlock(insert_block::InsertBlock { .. }) => TAG_INSERT_BLOCK,
            DocxMutation::RemoveBlock(remove_block::RemoveBlock { .. }) => TAG_REMOVE_BLOCK,
            DocxMutation::SetBlockContent(set_block_content::SetBlockContent { .. }) => TAG_SET_BLOCK_CONTENT,
            DocxMutation::SetRunText(set_run_text::SetRunText { .. }) => TAG_SET_RUN_TEXT,
            DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { .. }) => TAG_REPLACE_XML_NODE,
            DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { .. }) => TAG_SET_RUN_FORMATTING,
            DocxMutation::SetParagraphStyle(_) => TAG_SET_PARAGRAPH_STYLE,
            DocxMutation::InsertTableRow(_) => TAG_INSERT_TABLE_ROW,
            DocxMutation::RemoveTableRow(_) => TAG_REMOVE_TABLE_ROW,
            DocxMutation::InsertXmlNode(_) => TAG_INSERT_XML_NODE,
            DocxMutation::RemoveXmlNode(_) => TAG_REMOVE_XML_NODE,
            DocxMutation::InsertStyle(insert_style::InsertStyle { .. }) => TAG_INSERT_STYLE,
            DocxMutation::RemoveStyle(remove_style::RemoveStyle { .. }) => TAG_REMOVE_STYLE,
            DocxMutation::SetStyleName(set_style_name::SetStyleName { .. }) => TAG_SET_STYLE_NAME,
            DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { .. }) => TAG_SET_STYLE_BASED_ON,
            DocxMutation::SetPart(set_part::SetPart { .. }) => TAG_SET_PART,
            DocxMutation::RemovePart(remove_part::RemovePart { .. }) => TAG_REMOVE_PART,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_docx_snapshot_bin(snapshot, &mut out),
            DocxMutation::InsertBlock(insert_block::InsertBlock { path, block }) => {
                enc_block_path_bin(path, &mut out);
                enc_block_bin(block, &mut out);
            }
            DocxMutation::RemoveBlock(remove_block::RemoveBlock { path }) => enc_block_path_bin(path, &mut out),
            DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path, block }) => {
                enc_block_path_bin(path, &mut out);
                enc_block_bin(block, &mut out);
            }
            DocxMutation::SetRunText(set_run_text::SetRunText { address, text }) => {
                enc_xml_address_bin(address, &mut out);
                write_str_lp(&mut out, text);
            }
            DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address, node }) => {
                enc_xml_address_bin(address, &mut out);
                enc_xml_node_bin(node, &mut out);
            }
            DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address, bold, italic, underline }) => {
                enc_xml_address_bin(address, &mut out);
                out.push(*bold as u8);
                out.push(*italic as u8);
                out.push(*underline as u8);
            }
            DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address, style_id }) => {
                enc_xml_address_bin(address, &mut out);
                out.push(style_id.is_some() as u8);
                if let Some(style_id) = style_id {
                    write_str_lp(&mut out, style_id);
                }
            }
            DocxMutation::InsertTableRow(insert_table_row::InsertTableRow { address, index, cells }) => {
                enc_xml_address_bin(address, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                store::pack_rt::write_varint_u64(&mut out, cells.len() as u64);
                for cell in cells {
                    write_str_lp(&mut out, cell);
                }
            }
            DocxMutation::RemoveTableRow(remove_table_row::RemoveTableRow { address, index }) => {
                enc_xml_address_bin(address, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent, index, node }) => {
                enc_xml_address_bin(parent, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_xml_node_bin(node, &mut out);
            }
            DocxMutation::RemoveXmlNode(remove_xml_node::RemoveXmlNode { parent, index, expected_name, revision }) => {
                enc_xml_address_bin(parent, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                write_str_lp(&mut out, expected_name);
                write_str_lp(&mut out, revision);
            }
            DocxMutation::InsertStyle(insert_style::InsertStyle { style }) => enc_style_bin(style, &mut out),
            DocxMutation::RemoveStyle(remove_style::RemoveStyle { id }) => write_str_lp(&mut out, id),
            DocxMutation::SetStyleName(set_style_name::SetStyleName { id, name }) => {
                write_str_lp(&mut out, id);
                write_str_lp(&mut out, name);
            }
            DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id, based_on }) => {
                write_str_lp(&mut out, id);
                out.push(if based_on.is_some() { 1 } else { 0 });
                if let Some(based_on) = based_on {
                    write_str_lp(&mut out, based_on);
                }
            }
            DocxMutation::SetPart(set_part::SetPart { path, content_type, bytes }) => {
                write_str_lp(&mut out, path);
                write_str_lp(&mut out, content_type);
                write_bytes_lp(&mut out, bytes);
            }
            DocxMutation::RemovePart(remove_part::RemovePart { path }) => write_str_lp(&mut out, path),
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            TAG_SET_SNAPSHOT => {
                let snapshot = dec_docx_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            TAG_INSERT_BLOCK => {
                let path = dec_block_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let block = dec_block_bin(&mut reader).map_err(|e| malformed("op block", reader.position(), e))?;
                Ok(DocxMutation::InsertBlock(insert_block::InsertBlock { path, block }))
            }
            TAG_REMOVE_BLOCK => {
                let path = dec_block_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                Ok(DocxMutation::RemoveBlock(remove_block::RemoveBlock { path }))
            }
            TAG_SET_BLOCK_CONTENT => {
                let path = dec_block_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let block = dec_block_bin(&mut reader).map_err(|e| malformed("op block", reader.position(), e))?;
                Ok(DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path, block }))
            }
            TAG_SET_RUN_TEXT => {
                let address = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?;
                let text = read_str_lp(&mut reader).map_err(|e| malformed("op text", reader.position(), e))?;
                Ok(DocxMutation::SetRunText(set_run_text::SetRunText { address, text }))
            }
            TAG_REPLACE_XML_NODE => {
                let address = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?;
                let node = dec_xml_node_bin(&mut reader).map_err(|e| malformed("op node", reader.position(), e))?;
                Ok(DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address, node }))
            }
            TAG_SET_RUN_FORMATTING => {
                let address = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?;
                let bold = reader.read_u8().map_err(|e| malformed("op bold", reader.position(), e.to_string()))? != 0;
                let italic = reader.read_u8().map_err(|e| malformed("op italic", reader.position(), e.to_string()))? != 0;
                let underline = reader.read_u8().map_err(|e| malformed("op underline", reader.position(), e.to_string()))? != 0;
                Ok(DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address, bold, italic, underline }))
            }
            TAG_SET_PARAGRAPH_STYLE => {
                let address = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?;
                let has_style = reader.read_u8().map_err(|e| malformed("op style_id presence", reader.position(), e.to_string()))?;
                let style_id = (has_style != 0).then(|| read_str_lp(&mut reader)).transpose().map_err(|e| malformed("op style_id", reader.position(), e))?;
                Ok(DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address, style_id }))
            }
            TAG_INSERT_TABLE_ROW => {
                let address = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let count = reader.read_varint_u64().map_err(|e| malformed("op cells count", reader.position(), e.to_string()))?;
                let mut cells = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    cells.push(read_str_lp(&mut reader).map_err(|e| malformed("op cell", reader.position(), e))?);
                }
                Ok(DocxMutation::InsertTableRow(insert_table_row::InsertTableRow { address, index, cells }))
            }
            TAG_REMOVE_TABLE_ROW => {
                let address = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(DocxMutation::RemoveTableRow(remove_table_row::RemoveTableRow { address, index }))
            }
            TAG_INSERT_XML_NODE => {
                let parent = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op parent", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let node = dec_xml_node_bin(&mut reader).map_err(|e| malformed("op node", reader.position(), e))?;
                Ok(DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent, index, node }))
            }
            TAG_REMOVE_XML_NODE => {
                let parent = dec_xml_address_bin(&mut reader).map_err(|e| malformed("op parent", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let expected_name = read_str_lp(&mut reader).map_err(|e| malformed("op expected_name", reader.position(), e))?;
                let revision = read_str_lp(&mut reader).map_err(|e| malformed("op revision", reader.position(), e))?;
                Ok(DocxMutation::RemoveXmlNode(remove_xml_node::RemoveXmlNode { parent, index, expected_name, revision }))
            }
            TAG_INSERT_STYLE => {
                let style = dec_style_bin(&mut reader).map_err(|e| malformed("op style", reader.position(), e))?;
                Ok(DocxMutation::InsertStyle(insert_style::InsertStyle { style }))
            }
            TAG_REMOVE_STYLE => {
                let id = read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?;
                Ok(DocxMutation::RemoveStyle(remove_style::RemoveStyle { id }))
            }
            TAG_SET_STYLE_NAME => {
                let id = read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?;
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                Ok(DocxMutation::SetStyleName(set_style_name::SetStyleName { id, name }))
            }
            TAG_SET_STYLE_BASED_ON => {
                let id = read_str_lp(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?;
                let has = reader.read_u8().map_err(|e| malformed("op based_on presence", reader.position(), e.to_string()))?;
                let based_on = if has != 0 { Some(read_str_lp(&mut reader).map_err(|e| malformed("op based_on", reader.position(), e))?) } else { None };
                Ok(DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id, based_on }))
            }
            TAG_SET_PART => {
                let path = read_str_lp(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let content_type = read_str_lp(&mut reader).map_err(|e| malformed("op content_type", reader.position(), e))?;
                let bytes = read_bytes_lp(&mut reader).map_err(|e| malformed("op bytes", reader.position(), e))?;
                Ok(DocxMutation::SetPart(set_part::SetPart { path, content_type, bytes }))
            }
            TAG_REMOVE_PART => {
                let path = read_str_lp(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                Ok(DocxMutation::RemovePart(remove_part::RemovePart { path }))
            }
            other => Err(malformed("op tag", 1, format!("unknown DocxMutation tag {other}"))),
        }
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `DocxMutation` values -- one per variant -- the single source of
/// truth reused by this file's own `mutation_diff_law`/`inverse_law`/`op_text_binary_roundtrip_law`
/// tests below AND by `⚙️engine/🦀️.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same shape `📷️png/…/🧬️mutations/🦀️.rs`'s own
/// `demo_mutation_cases()` establishes.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fixture() -> DocxSnapshot {
    let mut snapshot = crate::engine::build_minimal_docx(DocxDocument {
        body: vec![
            DocxBlock::paragraph("first"),
            DocxBlock::Table(DocxTable {
                rows: vec![
                    DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("cell")], ..Default::default() }], ..Default::default() },
                    DocxTableRow { cells: vec![DocxTableCell { blocks: vec![DocxBlock::paragraph("cell two")], ..Default::default() }], ..Default::default() },
                ],
                ..Default::default()
            }),
        ],
        styles: vec![DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None }],
    });
    snapshot.opc.set_part("word/media/original.bin", "application/octet-stream", vec![1, 2, 3]);
    snapshot
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn table_path(block_index: usize, row: usize, cell: usize, index: usize) -> DocxBlockPath {
    DocxBlockPath { segments: vec![DocxPathSegment { block_index, row, cell }], index }
}

//#region 🔖️Fixtures
/// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field, both `document` and `opc`. Body uses
/// different-length lists so the recipe's naive positional `between_indexed` shows
/// removed+modified+added simultaneously (per this ticket's "known structural trap" note): a
/// removed tail on `sweep_a`, a modified-in-every-field first paragraph, and an added tail on
/// `sweep_b` (a table, exercising the recursive nested triple down to `blocks`). Styles (a
/// name-keyed collection, order-independent) get one removed, one modified-in-every-field, one
/// added. OPC content_types/parts/relationships each get one removed, one modified, one added.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sweep_a() -> DocxSnapshot {
    let mut snapshot = crate::engine::build_minimal_docx(DocxDocument { body: vec![DocxBlock::paragraph("old"), DocxBlock::paragraph("stay")], styles: vec![DocxStyle { id: "keep".into(), name: "Keep".into(), based_on: None }] });
    snapshot.opc.set_part("word/media/remove.bin", "application/octet-stream", vec![1, 2]);
    snapshot
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> DocxSnapshot {
    let mut snapshot = crate::engine::build_minimal_docx(DocxDocument {
        body: vec![DocxBlock::paragraph("new"), DocxBlock::paragraph("stay"), DocxBlock::paragraph("added")],
        styles: vec![DocxStyle { id: "keep".into(), name: "Keep renamed".into(), based_on: None }],
    });
    snapshot.opc.set_part("word/media/added.bin", "application/octet-stream", vec![3, 4]);
    snapshot
}
//#endregion 🔖️Fixtures

/// 🧪️ The demo cases proper -- one representative `DocxMutation` per variant.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<DocxMutation> {
    let base = fixture();
    let address = docx_block_run_address(&base, &DocxBlockPath { segments: vec![], index: 0 }, 0).expect("demo run address");
    let mut node = resolve_docx_xml_address(&base, &address).expect("demo run").node.clone();
    if let XmlNode::Element { children, .. } = &mut node {
        children.push(XmlNode::Comment { text: "replacement".into() });
    }
    let paragraph_address = docx_top_level_block_address(&base, 0).expect("demo paragraph address");
    let table_address = docx_top_level_block_address(&base, 1).expect("demo table address");
    let resolved_run = resolve_docx_xml_address(&base, &address).expect("demo run");
    let XmlNode::Element { children: run_children, .. } = resolved_run.node else { panic!("demo run element") };
    let inserted = XmlNode::Comment { text: "inserted".into() };
    let removed = run_children.first().expect("demo run child").clone();
    let removed_name = xml_address::child_identity(&resolved_run, &removed).expect("demo child identity");
    let removed_revision = docx_xml_subtree_revision(&removed);
    vec![
        DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        DocxMutation::InsertBlock(insert_block::InsertBlock { path: DocxBlockPath { segments: vec![], index: 1 }, block: DocxBlock::paragraph("x") }),
        DocxMutation::RemoveBlock(remove_block::RemoveBlock { path: DocxBlockPath { segments: vec![], index: 0 } }),
        DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path: DocxBlockPath { segments: vec![], index: 0 }, block: DocxBlock::paragraph("y") }),
        DocxMutation::SetRunText(set_run_text::SetRunText { address: address.clone(), text: "z".into() }),
        DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address: address.clone(), node }),
        DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { address: address.clone(), bold: true, italic: false, underline: true }),
        DocxMutation::SetParagraphStyle(set_paragraph_style::SetParagraphStyle { address: paragraph_address, style_id: Some("Normal".into()) }),
        DocxMutation::InsertTableRow(insert_table_row::InsertTableRow { address: table_address.clone(), index: 1, cells: vec!["new cell".into()] }),
        DocxMutation::RemoveTableRow(remove_table_row::RemoveTableRow { address: table_address, index: 0 }),
        DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent: address.clone(), index: run_children.len(), node: inserted }),
        DocxMutation::RemoveXmlNode(remove_xml_node::RemoveXmlNode { parent: address, index: 0, expected_name: removed_name, revision: removed_revision }),
        DocxMutation::InsertStyle(insert_style::InsertStyle { style: DocxStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: None } }),
        DocxMutation::RemoveStyle(remove_style::RemoveStyle { id: "Normal".into() }),
        DocxMutation::SetStyleName(set_style_name::SetStyleName { id: "Normal".into(), name: "Body".into() }),
        DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: "Normal".into(), based_on: Some("Heading1".into()) }),
        DocxMutation::SetPart(set_part::SetPart { path: "word/numbering.xml".into(), content_type: "application/xml".into(), bytes: b"<w:numbering/>".to_vec() }),
        DocxMutation::RemovePart(remove_part::RemovePart { path: "word/media/original.bin".into() }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests
