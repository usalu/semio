//! 🧬️ DocxMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, key/index-aware.

use semio_framework_value::{ValueError,ValueRefusalKind};

















use crate::schema::diff::{DocxBlockPath, DocxDiff, DocxOpcContentTypesDiff, DocxOpcDiff, DocxOpcPartDiff, DocxPathSegment, DocxXmlPartDiff, NamedModified, NamedTripleDiff};
#[cfg(test)]
use crate::schema::snapshot::DocxDocument;
use crate::schema::snapshot::{docx_part_is_xml, DocxBlock, DocxStyle, DocxXmlPart};
#[cfg(test)]
use crate::schema::snapshot::{DocxParagraph, DocxRun, DocxTable, DocxTableCell, DocxTableRow};
use crate::DocxSnapshot;

use protocol::Mutation;
use semio_s_artifact_stdio_xml::schema::diff::{diff_at_path as xml_diff_at_path, XmlChildAdded, XmlChildrenDiff, XmlElementDiff, XmlNodeDiff};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};

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
    docx_block_run_address, docx_block_slot, docx_styles_root, docx_top_level_block_address, docx_top_level_block_count, docx_top_level_run_at, docx_top_level_run_count, docx_xml_address, docx_xml_subtree_revision, resolve_docx_xml_address, DocxEditableRun, DocxXmlAddress,
    ResolvedDocxXmlAddress, docx_top_level_text_targets, DocxEditableText, DocxTextTargetKind, docx_run_formatting, DocxRunFormatting,
};
#[path = "🧩️replace-xml-node/🦀️.rs"]
pub mod replace_xml_node;
/// 📐️ Typed content mutation for `stdio.docx`. It addresses
/// the `document.body` block tree via `DocxBlockPath` (segments navigate through nested `Table`s,
/// mirrors svg's `NodePath` precedent), named styles by `DocxStyle::id`, and the raw OPC layer by
/// part path (for content this typed layer doesn't cover).
/// 🧪️ F6 VERIFIED: `#[derive(dsl::DslOps)]` on this enum ALSO fails (independent confirmation
/// beyond `DocxDiff`'s `DiffCodec` blocker, real `cargo check -p semio-s-plugin-stdio --lib`
/// output, then reverted) — `DocxSnapshot` fails with `DocxSnapshot:
/// DslField` is not satisfied (its `document.body: Vec<DocxBlock>` reaches the same data-carrying
/// enum `DocxDiff` hits); `InsertBlock`/`SetBlockContent`'s `block: DocxBlock` fails directly for
/// the same reason (`DocxBlock: DslField` is not satisfied); `InsertStyle`'s `style: DocxStyle` and
/// every `path: DocxBlockPath`-carrying variant also fail (`DocxStyle`/`DocxBlockPath: DslField` is
/// not satisfied — neither is itself `#[derive(dsl::DslRecord)]`, a SEPARATE reason from the enum
/// blocker, but confirms hand-roll is required regardless). `OpText`/`OpBinary` hand-rolled below,
/// reusing `DocxDiff`'s `pub(crate)` grammar primitives (`hex_encode`/`enc_block`/`enc_style`/
/// `split_top_level`/...).
//#region 🔖️Leaves
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
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}


/// 🧮️ The part-level mutations that carry `base` to `next`: removed parts become `remove-part`, new or changed XML parts and non-XML package parts
/// become `set-part` (a new non-XML part at its position in `next`). `None` when `next` changes anything those kinds do not address (the
/// schema, relationships, the package comment, content-type defaults), which is detected by replaying the result against `base`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn net_mutations(base: &DocxSnapshot, next: &DocxSnapshot) -> Option<Vec<DocxMutation>> {
    if base.schema != next.schema {
        return None;
    }
    let base_opc = base.opc.materialize_package_exact().ok()?;
    let next_opc = next.opc.materialize_package_exact().ok()?;
    let mut leaves: Vec<DocxMutation> = Vec::new();
    leaves.extend(base.xml_parts.iter().filter(|part| next.xml_part(&part.path).is_none()).map(|part| DocxMutation::RemovePart(remove_part::RemovePart { path: part.path.clone() })));
    leaves.extend(base_opc.parts.iter().filter(|part| !next_opc.parts.iter().any(|other| other.path == part.path)).map(|part| DocxMutation::RemovePart(remove_part::RemovePart { path: part.path.clone() })));
    for part in next.xml_parts.iter() {
        if base.xml_part(&part.path) != Some(part) {
            leaves.push(DocxMutation::SetPart(set_part::SetPart { path: part.path.clone(), content_type: part.content_type.clone(), payload: set_part::DocxPartContent::Xml { document: part.materialize_document_exact().ok()? }, index: None }));
        }
    }
    for (at, part) in next_opc.parts.iter().enumerate() {
        if base_opc.parts.iter().find(|other| other.path == part.path) != Some(part) {
            leaves.push(DocxMutation::SetPart(set_part::SetPart { path: part.path.clone(), content_type: part.content_type.clone(), payload: set_part::DocxPartContent::Binary { bytes: part.bytes.clone() }, index: Some(at) }));
        }
    }
    let mut state = base.clone();
    for leaf in &leaves {
        if !apply_docx_mutation(&mut state, leaf).messages().is_empty() {
            return None;
        }
    }
    (state == *next).then_some(leaves)
}

//#endregion 🔖️Apply

//#region 🔖️MutationTrait
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

fn replacement_plan(snapshot: &DocxSnapshot, address: &DocxXmlAddress, replacement: XmlNode) -> Result<PreparedDocxXmlMutation, ValueError> {
    let resolved = resolve_docx_xml_address(snapshot, address)?;
    xml_address::validate_replacement_identity(&resolved, &replacement)?;
    let previous = resolved.node.clone();
    let changed = previous != replacement;
    let inverse_address =
        DocxXmlAddress { part_path: address.part_path.clone(), node_path: address.node_path.clone(), expected_name: address.expected_name.clone(), revision: xml_address::revision_after_replacement(snapshot, address, &replacement)? };
    let diff = changed.then(|| part_diff(address, XmlNodeDiff::Replace { node: Some(replacement.clone()) })).unwrap_or_default();
    Ok(PreparedDocxXmlMutation { diff, inverse: DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address: inverse_address, node: previous }), changed, address: address.clone(), replacement })
}

fn child_insert_plan(snapshot: &DocxSnapshot, parent: &DocxXmlAddress, index: usize, node: XmlNode) -> Result<PreparedDocxXmlMutation, ValueError> {
    let resolved = resolve_docx_xml_address(snapshot, parent)?;
    let XmlNode::Element { children, .. } = &resolved.node else { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"insert-xml-node parent is not an element")) };
    if index > children.len() {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("XML child insertion index {index} exceeds {} children", children.len())));
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

fn child_remove_plan(snapshot: &DocxSnapshot, parent: &DocxXmlAddress, index: usize, expected_name: &str, revision: &str) -> Result<PreparedDocxXmlMutation, ValueError> {
    let resolved = resolve_docx_xml_address(snapshot, parent)?;
    let XmlNode::Element { children, .. } = &resolved.node else { return Err(ValueError::new(ValueRefusalKind::InvalidValue,"remove-xml-node parent is not an element")) };
    let previous = children.get(index).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("XML child removal index {index} exceeds {} children", children.len())))?.clone();
    let actual_name = xml_address::child_identity(&resolved, &previous)?;
    if actual_name != expected_name {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("XML child removal expected {expected_name} but resolved {actual_name}")));
    }
    let actual_revision = docx_xml_subtree_revision(&previous);
    if actual_revision != revision {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("XML child removal revision changed from {revision} to {actual_revision}")));
    }
    let mut replacement = resolved.node.clone();
    let XmlNode::Element { children, .. } = &mut replacement else { unreachable!() };
    children.remove(index);
    let inverse_parent = DocxXmlAddress { part_path: parent.part_path.clone(), node_path: parent.node_path.clone(), expected_name: parent.expected_name.clone(), revision: xml_address::revision_after_replacement(snapshot, parent, &replacement)? };
    let diff = part_diff(parent, XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: vec![index], modified: Vec::new(), added: Vec::new() }) }));
    Ok(PreparedDocxXmlMutation { diff, inverse: DocxMutation::InsertXmlNode(insert_xml_node::InsertXmlNode { parent: inverse_parent, index, node: previous }), changed: true, address: parent.clone(), replacement })
}

/// 🧩️ Validates one addressed edit and prepares its compact target-part diff and exact inverse.
pub fn prepare_addressed_xml_mutation(snapshot: &DocxSnapshot, mutation: &DocxMutation) -> Result<PreparedDocxXmlMutation, ValueError> {
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
        _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"mutation is not an addressed canonical XML edit")),
    }
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

/// 🧾️ The outcome of a prepared plan: its compact diff, or the refusal naming why it cannot be built.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn plan_outcome(plan: Result<PreparedDocxXmlMutation, ValueError>) -> protocol::MutationOutcome<DocxDiff> {
    match plan {
        Ok(prepared) => protocol::MutationOutcome::new(prepared.diff),
        Err(error) => protocol::MutationOutcome::error("mutation.target-mismatch", error.into_message(), Vec::<String>::new()),
    }
}

/// ↩️ The exact inverse of a prepared plan; a refused plan has nothing to undo.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn plan_inverse(plan: Result<PreparedDocxXmlMutation, ValueError>) -> Vec<DocxMutation> {
    plan.map(|prepared| vec![prepared.inverse]).unwrap_or_default()
}

/// 🧾️ The outcome of an addressed canonical XML edit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn addressed_outcome(mutation: &DocxMutation, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
    match prepare_addressed_xml_mutation(base, mutation) {
        Ok(prepared) => protocol::MutationOutcome::new(prepared.diff),
        Err(error) => protocol::MutationOutcome::error("mutation.target-mismatch", error.into_message(), mutation_target(mutation)),
    }
}

/// ↩️ The exact inverse of an addressed canonical XML edit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn addressed_inverse(mutation: &DocxMutation, base: &DocxSnapshot) -> Result<Vec<DocxMutation>, ValueError> {
    prepare_addressed_xml_mutation(base, mutation).map(|prepared| vec![prepared.inverse])
}

/// ➕️ Inserts the XML of `block` at the slot block `path` names; the inverse removes that exact XML child.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn insert_block_plan(base: &DocxSnapshot, path: &DocxBlockPath, block: &DocxBlock) -> Result<PreparedDocxXmlMutation, ValueError> {
    let (container, slot) = docx_block_slot(base, path, true)?;
    child_insert_plan(base, &container, slot, crate::standards::v_ecma_376::subsets::base::schema::construction::block_to_xml(block))
}

/// ➖️ Removes the block `path` names; the inverse re-inserts that exact XML child at its original slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn remove_block_plan(base: &DocxSnapshot, path: &DocxBlockPath) -> Result<PreparedDocxXmlMutation, ValueError> {
    let (container, slot) = docx_block_slot(base, path, false)?;
    let resolved = resolve_docx_xml_address(base, &container)?;
    let XmlNode::Element { children, .. } = &resolved.node else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "remove-block container is not an element")) };
    let node = &children[slot];
    let expected_name = xml_address::child_identity(&resolved, node)?;
    child_remove_plan(base, &container, slot, &expected_name, &docx_xml_subtree_revision(node))
}

/// ✍️ Replaces the block `path` names with the XML of `block`; the inverse replaces it back with the exact previous XML node.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn set_block_plan(base: &DocxSnapshot, path: &DocxBlockPath, block: &DocxBlock) -> Result<PreparedDocxXmlMutation, ValueError> {
    let (container, slot) = docx_block_slot(base, path, false)?;
    let mut node_path = container.node_path.clone();
    node_path.push(slot);
    let address = docx_xml_address(base, &container.part_path, node_path)?;
    replacement_plan(base, &address, crate::standards::v_ecma_376::subsets::base::schema::construction::block_to_xml(block))
}

/// 🎨️ The styles root and the position of style `id` among its children.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn style_position(base: &DocxSnapshot, id: &str) -> Result<(DocxXmlAddress, Vec<Option<String>>, Option<usize>), ValueError> {
    let (root, ids) = docx_styles_root(base)?;
    let at = ids.iter().position(|candidate| candidate.as_deref() == Some(id));
    Ok((root, ids, at))
}

/// ➕️ Appends the XML of `style`; an id that already exists is refused.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn insert_style_plan(base: &DocxSnapshot, style: &DocxStyle) -> Result<PreparedDocxXmlMutation, ValueError> {
    let (root, ids, at) = style_position(base, &style.id)?;
    if at.is_some() {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("DOCX style {:?} already exists", style.id)));
    }
    child_insert_plan(base, &root, ids.len(), crate::standards::v_ecma_376::subsets::base::schema::construction::style_to_xml(style))
}

/// ➖️ Removes style `id`; the inverse re-inserts that exact XML child at its original position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn remove_style_plan(base: &DocxSnapshot, id: &str) -> Result<PreparedDocxXmlMutation, ValueError> {
    let (root, _, at) = style_position(base, id)?;
    let at = at.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, format!("DOCX style {id:?} does not exist")))?;
    let resolved = resolve_docx_xml_address(base, &root)?;
    let XmlNode::Element { children, .. } = &resolved.node else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "DOCX styles root is not an element")) };
    let node = &children[at];
    let expected_name = xml_address::child_identity(&resolved, node)?;
    child_remove_plan(base, &root, at, &expected_name, &docx_xml_subtree_revision(node))
}

/// 🏷️ Writes the `w:val` property element named `property` of style `id`: a present element is replaced in place, an absent one is inserted
/// at `insert_at` (`None` appends). `None` as `value` removes the element; removing an absent element changes nothing.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn style_property_plan(base: &DocxSnapshot, id: &str, property: &str, value: Option<&str>, insert_at: Option<usize>) -> Result<Option<PreparedDocxXmlMutation>, ValueError> {
    let (root, _, at) = style_position(base, id)?;
    let at = at.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, format!("DOCX style {id:?} does not exist")))?;
    let mut style_path = root.node_path.clone();
    style_path.push(at);
    let style = docx_xml_address(base, &root.part_path, style_path.clone())?;
    let resolved = resolve_docx_xml_address(base, &style)?;
    let XmlNode::Element { children, .. } = &resolved.node else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "DOCX style is not an element")) };
    let existing = children.iter().position(|child| matches!(child, XmlNode::Element { name, .. } if name == property));
    let element = value.map(|value| XmlNode::Element { name: property.into(), attrs: vec![XmlAttr { name: "w:val".into(), value: value.into() }], children: Vec::new() });
    Ok(Some(match (existing, element) {
        (Some(index), Some(element)) => {
            let mut path = style_path;
            path.push(index);
            replacement_plan(base, &docx_xml_address(base, &root.part_path, path)?, element)?
        }
        (Some(index), None) => child_remove_plan(base, &style, index, property, &docx_xml_subtree_revision(&children[index]))?,
        (None, Some(element)) => child_insert_plan(base, &style, insert_at.unwrap_or(children.len()), element)?,
        (None, None) => return Ok(None),
    }))
}

/// 🏷️ Renames style `id` through its `w:name` element.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn style_name_plan(base: &DocxSnapshot, id: &str, name: &str) -> Result<PreparedDocxXmlMutation, ValueError> {
    style_property_plan(base, id, "w:name", Some(name), Some(0))?.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "a style name always writes an element"))
}

/// 🌳 Sets or clears the `w:basedOn` element of style `id`; clearing an absent one is `None` (nothing to do).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn style_based_on_plan(base: &DocxSnapshot, id: &str, based_on: Option<&str>) -> Result<Option<PreparedDocxXmlMutation>, ValueError> {
    style_property_plan(base, id, "w:basedOn", based_on, None)
}

/// 🧾️ The outcome of an optional plan: `None` is the empty diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn optional_plan_outcome(plan: Result<Option<PreparedDocxXmlMutation>, ValueError>) -> protocol::MutationOutcome<DocxDiff> {
    match plan {
        Ok(Some(prepared)) => protocol::MutationOutcome::new(prepared.diff),
        Ok(None) => protocol::MutationOutcome::new(DocxDiff::default()),
        Err(error) => protocol::MutationOutcome::error("mutation.target-mismatch", error.into_message(), Vec::<String>::new()),
    }
}

/// ↩️ The exact inverse of an optional plan.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn optional_plan_inverse(plan: Result<Option<PreparedDocxXmlMutation>, ValueError>) -> Vec<DocxMutation> {
    plan.ok().flatten().map(|prepared| vec![prepared.inverse]).unwrap_or_default()
}

/// 📦️ The package part `path` holds in `base`, read back as the `set-part` payload that writes it, with its content type and (for a non-XML
/// part) its position among the package parts.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn current_part(base: &DocxSnapshot, path: &str) -> Result<Option<(String, set_part::DocxPartContent, Option<usize>)>, ValueError> {
    if let Some(part) = base.xml_part(path) {
        return Ok(Some((part.content_type.clone(), set_part::DocxPartContent::Xml { document: part.materialize_document_exact()? }, None)));
    }
    let opc = base.opc.materialize_package_exact()?;
    Ok(opc.parts.iter().position(|part| part.path == path).map(|at| (opc.parts[at].content_type.clone(), set_part::DocxPartContent::Binary { bytes: opc.parts[at].bytes.clone() }, Some(at))))
}

/// 🧾️ The `[Content_Types].xml` override write that makes `path` resolve to `content_type`: none when it already does.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn override_diff(opc: &semio_s_artifact_stdio_zip::opc::OpcPackage, path: &str, content_type: &str) -> Option<DocxOpcContentTypesDiff> {
    if opc.content_types.resolve(path) == Some(content_type) {
        return None;
    }
    let name = format!("/{path}");
    let overrides = if opc.content_types.overrides.iter().any(|(existing, _)| *existing == name) {
        NamedTripleDiff { modified: vec![NamedModified { key: name, diff: content_type.to_string() }], ..Default::default() }
    } else {
        NamedTripleDiff { added: vec![(name, content_type.to_string())], ..Default::default() }
    };
    Some(DocxOpcContentTypesDiff { defaults: None, overrides: Some(overrides) })
}

/// 📦️ The diff that writes part `path` -- inserting it (a non-XML part at `index` among the package parts, last by default) or replacing it
/// whole -- and keeps its content type resolvable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn set_part_diff(base: &DocxSnapshot, path: &str, content_type: &str, payload: &set_part::DocxPartContent, index: Option<usize>) -> Result<DocxDiff, ValueError> {
    let path = path.trim_start_matches('/');
    let opc = base.opc.materialize_package_exact()?;
    let content_types = override_diff(&opc, path, content_type);
    match payload {
        set_part::DocxPartContent::Xml { document } => {
            if !docx_part_is_xml(path, content_type) {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("{path} with content type {content_type} is not an XML part")));
            }
            let xml_parts = match base.xml_part(path) {
                Some(existing) => {
                    let unchanged = existing.document.materialize_exact()? == *document;
                    let replacement = DocxXmlPartDiff {
                        content_type: (existing.content_type != content_type).then(|| content_type.to_string()),
                        document: (!unchanged).then(|| xml_replace_document(document)),
                    };
                    (replacement.content_type.is_some() || replacement.document.is_some()).then(|| NamedTripleDiff { modified: vec![NamedModified { key: path.to_string(), diff: replacement }], ..Default::default() })
                }
                None => {
                    let added = DocxXmlPart::try_from_document(path.to_string(), content_type.to_string(), document.clone())?;
                    let mut keys: Vec<String> = base.xml_parts.iter().map(|part| part.path.clone()).collect();
                    keys.push(path.to_string());
                    let mut sorted = keys.clone();
                    sorted.sort_unstable();
                    Some(NamedTripleDiff { added: vec![added], order: if keys == sorted { Vec::new() } else { sorted }, ..Default::default() })
                }
            };
            Ok(DocxDiff { opc: content_types.map(|content_types| DocxOpcDiff { content_types: Some(content_types), ..Default::default() }), xml_parts })
        }
        set_part::DocxPartContent::Binary { bytes } => {
            if docx_part_is_xml(path, content_type) {
                return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("{path} with content type {content_type} is an XML part")));
            }
            let parts = match opc.parts.iter().find(|part| part.path == path) {
                Some(existing) => {
                    let replacement = DocxOpcPartDiff { content_type: (existing.content_type != content_type).then(|| content_type.to_string()), bytes: (existing.bytes != *bytes).then(|| bytes.clone()) };
                    (replacement.content_type.is_some() || replacement.bytes.is_some()).then(|| NamedTripleDiff { modified: vec![NamedModified { key: path.to_string(), diff: replacement }], ..Default::default() })
                }
                None => {
                    let mut keys: Vec<String> = opc.parts.iter().map(|part| part.path.clone()).collect();
                    let mut order = Vec::new();
                    if let Some(at) = index.filter(|at| *at < keys.len()) {
                        keys.insert(at, path.to_string());
                        order = keys;
                    }
                    Some(NamedTripleDiff { added: vec![semio_s_artifact_stdio_zip::opc::OpcPart { path: path.to_string(), content_type: content_type.to_string(), bytes: bytes.clone() }], order, ..Default::default() })
                }
            };
            Ok(DocxDiff { opc: (parts.is_some() || content_types.is_some()).then(|| DocxOpcDiff { parts, content_types, ..Default::default() }), xml_parts: None })
        }
    }
}

/// 🌳 The XML diff that replaces a whole document with `document`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xml_replace_document(document: &semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument) -> semio_s_artifact_stdio_xml::schema::diff::XmlDiff {
    semio_s_artifact_stdio_xml::schema::diff::XmlDiff {
        prolog: Some(document.prolog.clone()),
        epilog: Some(document.epilog.clone()),
        declaration: Some(document.declaration.clone()),
        doctype: Some(document.doctype.clone()),
        root: Some(XmlNodeDiff::Replace { node: document.root.clone() }),
    }
}

/// 📦️ The inverse of writing part `path`: the exact previous part, or its removal when the write created it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn set_part_inverse(base: &DocxSnapshot, path: &str) -> Result<Vec<DocxMutation>, ValueError> {
    let path = path.trim_start_matches('/');
    Ok(vec![match current_part(base, path)? {
        Some((content_type, payload, index)) => DocxMutation::SetPart(set_part::SetPart { path: path.to_string(), content_type, payload, index }),
        None => DocxMutation::RemovePart(remove_part::RemovePart { path: path.to_string() }),
    }])
}

/// 📦️ The diff that removes part `path` (and nothing else: its content-type override stays).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn remove_part_diff(base: &DocxSnapshot, path: &str) -> Result<DocxDiff, ValueError> {
    let path = path.trim_start_matches('/');
    if base.xml_part(path).is_some() {
        return Ok(DocxDiff { xml_parts: Some(NamedTripleDiff { removed: vec![path.to_string()], ..Default::default() }), ..Default::default() });
    }
    let opc = base.opc.materialize_package_exact()?;
    if opc.parts.iter().any(|part| part.path == path) {
        return Ok(DocxDiff { opc: Some(DocxOpcDiff { parts: Some(NamedTripleDiff { removed: vec![path.to_string()], ..Default::default() }), ..Default::default() }), ..Default::default() });
    }
    Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("DOCX package has no part {path}")))
}

/// 🧾️ The outcome of a part-level diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn part_outcome(diff: Result<DocxDiff, ValueError>) -> protocol::MutationOutcome<DocxDiff> {
    match diff {
        Ok(diff) => protocol::MutationOutcome::new(diff),
        Err(error) => protocol::MutationOutcome::error("mutation.target-mismatch", error.into_message(), Vec::<String>::new()),
    }
}
/// ▶️ Applies one revision-bound canonical XML edit in place on an OWNED snapshot -- the bounded retained-execution seam
/// (`✏️editor/📬️preparation`) that moves the document's parts into the edit instead of cloning them; every other caller goes through the
/// central diff apply.
pub fn apply_addressed_xml_mutation_in_place(snapshot: &mut DocxSnapshot, mutation: &DocxMutation) -> Result<PreparedDocxXmlMutation, ValueError> {
    let prepared = prepare_addressed_xml_mutation(snapshot, mutation)?;
    if prepared.changed {
        xml_address::replace_addressed_node(snapshot, &prepared.address, prepared.replacement.clone())?;
    }
    Ok(prepared)
}

/// 🌳 The XML diff that rewrites every attribute value equal to a member of `from` into `to` anywhere under `node` -- a namespace
/// declaration is an ordinary attribute, so one walk covers `xmlns`, `xmlns:r` and whatever prefixed alias a package uses. `None` when
/// nothing under `node` declares one of `from`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn retarget_attribute_values_diff(node: &XmlNode, from: &[&str], to: &str) -> Option<XmlNodeDiff> {
    let XmlNode::Element { attrs, children, .. } = node else { return None };
    let modified: Vec<semio_s_artifact_stdio_xml::schema::diff::XmlAttrModified> =
        attrs.iter().filter(|attr| from.contains(&attr.value.as_str()) && attr.value != to).map(|attr| semio_s_artifact_stdio_xml::schema::diff::XmlAttrModified { name: attr.name.clone(), value: to.to_string() }).collect();
    let attributes = (!modified.is_empty()).then(|| semio_s_artifact_stdio_xml::schema::diff::XmlAttributesDiff { order: attrs.iter().map(|attr| attr.name.clone()).collect(), removed: Vec::new(), modified, added: Vec::new() });
    let nested: Vec<semio_s_artifact_stdio_xml::schema::diff::XmlChildModified> =
        children.iter().enumerate().filter_map(|(index, child)| retarget_attribute_values_diff(child, from, to).map(|diff| semio_s_artifact_stdio_xml::schema::diff::XmlChildModified { index, diff })).collect();
    let children = (!nested.is_empty()).then(|| XmlChildrenDiff { removed: Vec::new(), modified: nested, added: Vec::new() });
    (attributes.is_some() || children.is_some()).then(|| XmlNodeDiff::Element(XmlElementDiff { name: None, attributes, children }))
}

/// 🧾️ The diff that applies one XML `root` edit to each named part: `edits` pairs a part path with the root diff it takes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn root_edits_diff(edits: Vec<(String, XmlNodeDiff)>) -> DocxDiff {
    if edits.is_empty() {
        return DocxDiff::default();
    }
    let modified = edits
        .into_iter()
        .map(|(key, root)| NamedModified {
            key,
            diff: DocxXmlPartDiff { content_type: None, document: Some(semio_s_artifact_stdio_xml::schema::diff::XmlDiff { root: Some(root), ..Default::default() }) },
        })
        .collect();
    DocxDiff { opc: None, xml_parts: Some(NamedTripleDiff { modified, ..Default::default() }) }
}

/// 🌿️ The attribute edit that sets (`Some`) or removes (`None`) attribute `name` on the ROOT element of `document`; `None` when nothing changes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn root_attribute_diff(document: &semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument, name: &str, value: Option<&str>) -> Option<XmlNodeDiff> {
    let Some(XmlNode::Element { attrs, .. }) = document.root.as_ref() else { return None };
    let existing = attrs.iter().find(|attr| attr.name == name);
    let mut order: Vec<String> = attrs.iter().map(|attr| attr.name.clone()).collect();
    let attributes = match (existing, value) {
        (Some(attr), Some(value)) if attr.value != value => semio_s_artifact_stdio_xml::schema::diff::XmlAttributesDiff { order, modified: vec![semio_s_artifact_stdio_xml::schema::diff::XmlAttrModified { name: name.into(), value: value.into() }], ..Default::default() },
        (None, Some(value)) => {
            order.push(name.to_string());
            semio_s_artifact_stdio_xml::schema::diff::XmlAttributesDiff { order, added: vec![semio_s_artifact_stdio_xml::schema::diff::XmlAttrAdded { name: name.into(), value: value.into() }], ..Default::default() }
        }
        (Some(_), None) => {
            order.retain(|attr| attr != name);
            semio_s_artifact_stdio_xml::schema::diff::XmlAttributesDiff { order, removed: vec![name.into()], ..Default::default() }
        }
        _ => return None,
    };
    Some(XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: Some(attributes), children: None }))
}

/// 🌿️ The children edit that appends `node` to (`Some`) or strips every child named `name` from (`None`) the ROOT element of `document`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn root_children_diff(document: &semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument, append: Option<XmlNode>, strip: Option<&str>) -> Option<XmlNodeDiff> {
    let Some(XmlNode::Element { children, .. }) = document.root.as_ref() else { return None };
    let removed: Vec<usize> = strip.map(|name| children.iter().enumerate().filter(|(_, child)| matches!(child, XmlNode::Element { name: actual, .. } if actual == name)).map(|(index, _)| index).collect()).unwrap_or_default();
    let added: Vec<XmlChildAdded> = append.map(|item| vec![XmlChildAdded { index: children.len(), item }]).unwrap_or_default();
    (!removed.is_empty() || !added.is_empty()).then(|| XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed, modified: Vec::new(), added }) }))
}
//#endregion 🔖️MutationTrait

//#region OpCodecs




















//#region 🔖️OpBinaryCodec
/// 🧪️ FG-wave: real recursive binary primitives backing the upgraded `OpBinary` impl below --
/// mirrors `📰️xml/…/🧬️mutations/🦀️.rs`'s own `enc_node_path_bin`/`enc_xml_snapshot_bin`
/// shape, reusing `store::pack_rt::write_varint_u64`/`store::ByteReader` plus `DocxDiff`'s own
/// `write_str_lp`/`read_str_lp`/`write_bytes_lp`/`read_bytes_lp`/`enc_block_bin`/`dec_block_bin`/
/// `enc_style_bin`/`dec_style_bin`/`enc_opc_part_bin`/`dec_opc_part_bin`/`enc_rel_bin`/
/// `dec_rel_bin` (`../🔺️diff/🦀️.rs`, `pub(crate)` to this artifact).













//#endregion 🔖️OpBinaryCodec




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
    let mut snapshot = crate::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_docx(DocxDocument {
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
    let XmlNode::Element { children: ref run_children, .. } = resolved_run.node else { panic!("demo run element") };
    let inserted = XmlNode::Comment { text: "inserted".into() };
    let removed = run_children.first().expect("demo run child").clone();
    let removed_name = xml_address::child_identity(&resolved_run, &removed).expect("demo child identity");
    let removed_revision = docx_xml_subtree_revision(&removed);
    vec![
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
        DocxMutation::SetPart(set_part::SetPart { path: "word/numbering.xml".into(), content_type: "application/xml".into(), payload: set_part::DocxPartContent::Xml { document: semio_s_artifact_stdio_xml::schema::snapshot::XmlDocument { root: Some(semio_s_artifact_stdio_xml::schema::snapshot::XmlNode::Element { name: "w:numbering".into(), attrs: Vec::new(), children: Vec::new() }), ..Default::default() } } , index: None }),
        DocxMutation::RemovePart(remove_part::RemovePart { path: "word/media/original.bin".into() }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
