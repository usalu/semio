//! 📬️ Paged structural edit for strictly admitted DOCX XML publication.

use super::*;
use crate::schema::mutations::xml_address::{resolve_docx_xml_address, validate_replacement_identity};
use crate::schema::mutations::{prepare_addressed_xml_mutation, DocxXmlAddress, PreparedDocxXmlMutation};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_contract::editing::NativeEditPreparationRoute;
use semio_framework_plugin::plugin_app_close_prelude::store::{self as app_store, PagedOneItemEdit, PagedOneItemEditStep};
use std::sync::Arc;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};

/// 🧭️ The element at `path` below `node`, reopened for the one in-place replacement of the retained-execution seam.
fn node_at_path_mut<'a>(node: &'a mut XmlNode, path: &[usize]) -> Option<&'a mut XmlNode> {
    let Some((&index, rest)) = path.split_first() else { return Some(node) };
    let XmlNode::Element { children, .. } = node else { return None };
    node_at_path_mut(children.get_mut(index)?, rest)
}

/// ✍️ Writes `replacement` over the addressed element of an OWNED snapshot, moving the part's document through the edit instead of cloning the package.
fn replace_addressed_node(snapshot: &mut DocxSnapshot, address: &DocxXmlAddress, replacement: XmlNode) -> Result<(), ValueError> {
    let resolved = resolve_docx_xml_address(snapshot, address)?;
    validate_replacement_identity(&resolved, &replacement)?;
    let part_index = resolved.part_index;
    let mut document = snapshot.xml_parts[part_index].materialize_document_exact()?;
    let root = document.root.as_mut().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, format!("DOCX XML part {} has no root", address.part_path)))?;
    let node = node_at_path_mut(root, &address.node_path).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "DOCX XML address became stale during apply".to_string()))?;
    *node = replacement;
    semio_s_artifact_stdio_xml::schema::snapshot::validate_xml_document_boundaries(&document).map_err(|message| ValueError::new(ValueRefusalKind::InvalidValue, message))?;
    snapshot.xml_parts[part_index].replace_document(document)?;
    snapshot.validate_authority().map_err(crate::standards::v_ecma_376::subsets::base::schema::refusal::DocxError::into_value_error)
}

/// ▶️ Applies one revision-bound canonical XML edit in place on an OWNED snapshot -- the bounded retained-execution seam that moves the document's parts into the
/// edit instead of cloning them; every other caller goes through the central diff apply.
fn apply_addressed_xml_mutation_in_place(snapshot: &mut DocxSnapshot, mutation: &DocxMutation) -> Result<PreparedDocxXmlMutation, ValueError> {
    let prepared = prepare_addressed_xml_mutation(snapshot, mutation)?;
    if prepared.changed {
        replace_addressed_node(snapshot, &prepared.address, prepared.replacement.clone())?;
    }
    Ok(prepared)
}

const PREFIX: &str = "stdio-docx-base-set-page";
/// 📏️ The one-item store preparation copies the whole document in one step, so an owner (the document, or one mutation)
/// plus one turn fits the one-item store budget exactly — one measured item per 16 owned bytes — and larger owners refuse
/// typed (`paged-owner-required`) before the store ever sees an inadmissible footprint.
const OWNER_BYTES: usize = app_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES - TURN_BYTES;
const OWNER_ITEMS: usize = OWNER_BYTES / 16;
const OWNER_DEPTH: usize = 32;
const TURN_BYTES: usize = 4_096;

pub(crate) fn route(_prefix: &'static str) -> Option<NativeEditPreparationRoute<DocxSnapshot, DocxMutation>> {
    Some(NativeEditPreparationRoute::new(DocxXmlEdit::recognizes, Arc::new(app_store::PagedOneItemPreparationFactory::<DocxSnapshot, DocxMutation, DocxXmlEdit>::default())))
}

pub(super) fn recognizes(mutation: &DocxMutation) -> bool {
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

#[derive(Default)]
struct Measure {
    bytes: usize,
    items: usize,
}

impl Measure {
    fn add(&mut self, bytes: usize) -> Result<(), String> {
        self.bytes = self.bytes.checked_add(bytes).ok_or_else(|| format!("{PREFIX}.bytes-overflow"))?;
        self.items = self.items.checked_add(1).ok_or_else(|| format!("{PREFIX}.items-overflow"))?;
        if self.bytes > OWNER_BYTES || self.items > OWNER_ITEMS {
            return Err(format!("{PREFIX}.paged-owner-required"));
        }
        Ok(())
    }

    fn text(&mut self, value: &str) -> Result<(), String> {
        self.add(value.len())
    }

    fn string(&mut self, value: &String) -> Result<(), String> {
        self.add(value.capacity())
    }
}

fn measure_node(node: &XmlNode, depth: usize, measure: &mut Measure) -> Result<(), String> {
    if depth > OWNER_DEPTH {
        return Err(format!("{PREFIX}.depth-limit"));
    }
    match node {
        XmlNode::Element { name, attrs, children } => {
            measure.string(name)?;
            measure.add(attrs.capacity().saturating_mul(std::mem::size_of::<XmlAttr>()))?;
            for attr in attrs {
                measure.string(&attr.name)?;
                measure.string(&attr.value)?;
            }
            measure.add(children.capacity().saturating_mul(std::mem::size_of::<XmlNode>()))?;
            for child in children {
                measure_node(child, depth + 1, measure)?;
            }
        }
        XmlNode::Text { text } | XmlNode::CData { text } | XmlNode::Comment { text } => measure.string(text)?,
        XmlNode::ProcessingInstruction { target, data } => {
            measure.string(target)?;
            measure.string(data)?;
        }
    }
    Ok(())
}

fn measure_document(document: &XmlDocument, measure: &mut Measure) -> Result<(), String> {
    measure.add(std::mem::size_of::<XmlDocument>())?;
    if document.doctype.is_some() || !document.prolog.is_empty() || !document.epilog.is_empty() {
        return Err(format!("{PREFIX}.paged-xml-envelope-required"));
    }
    if let Some(declaration) = &document.declaration {
        measure.string(&declaration.version)?;
        if let Some(encoding) = &declaration.encoding {
            measure.string(encoding)?;
        }
    }
    if let Some(root) = &document.root {
        measure_node(root, 0, measure)?;
    }
    Ok(())
}

fn measure_snapshot(snapshot: &DocxSnapshot) -> Result<usize, String> {
    let mut measure = Measure::default();
    measure.add(std::mem::size_of::<DocxSnapshot>())?;
    measure.string(&snapshot.schema)?;
    for part in snapshot.xml_parts.iter() {
        measure.string(&part.path)?;
        measure.string(&part.content_type)?;
    }
    Ok(measure.bytes)
}

fn measure_address(address: &DocxXmlAddress, measure: &mut Measure) -> Result<(), String> {
    measure.string(&address.part_path)?;
    measure.add(address.node_path.capacity().saturating_mul(std::mem::size_of::<usize>()))?;
    measure.string(&address.expected_name)?;
    measure.string(&address.revision)
}

pub(super) fn measure_mutation(mutation: &DocxMutation) -> Result<usize, String> {
    let mut measure = Measure::default();
    measure.add(std::mem::size_of::<DocxMutation>())?;
    match mutation {
        DocxMutation::SetRunText(value) => {
            measure_address(&value.address, &mut measure)?;
            measure.string(&value.text)?;
        }
        DocxMutation::ReplaceXmlNode(value) => {
            measure_address(&value.address, &mut measure)?;
            measure_node(&value.node, 0, &mut measure)?;
        }
        DocxMutation::SetRunFormatting(value) => measure_address(&value.address, &mut measure)?,
        DocxMutation::SetParagraphStyle(value) => {
            measure_address(&value.address, &mut measure)?;
            if let Some(style_id) = &value.style_id {
                measure.string(style_id)?;
            }
        }
        DocxMutation::InsertTableRow(value) => {
            measure_address(&value.address, &mut measure)?;
            measure.add(value.cells.capacity().saturating_mul(std::mem::size_of::<String>()))?;
            for cell in &value.cells {
                measure.string(cell)?;
            }
        }
        DocxMutation::RemoveTableRow(value) => measure_address(&value.address, &mut measure)?,
        DocxMutation::InsertXmlNode(value) => {
            measure_address(&value.parent, &mut measure)?;
            measure_node(&value.node, 0, &mut measure)?;
        }
        DocxMutation::RemoveXmlNode(value) => {
            measure_address(&value.parent, &mut measure)?;
            measure.string(&value.expected_name)?;
            measure.string(&value.revision)?;
        }
        _ => return Err(format!("{PREFIX}.mutation")),
    }
    Ok(measure.bytes)
}

pub(super) fn set_run_text_is_admitted(address: &DocxXmlAddress, text: &str) -> Result<(), String> {
    let mut measure = Measure::default();
    measure.add(std::mem::size_of::<DocxMutation>())?;
    measure_address(address, &mut measure)?;
    measure.text(text)
}

pub(crate) fn prepare_set_run_text(snapshot: &DocxSnapshot, address: &DocxXmlAddress, text: &str) -> Result<Option<DocxMutation>, String> {
    set_run_text_is_admitted(address, text)?;
    let mutation = if address.expected_name.ends_with("}p") {
        let Some(mutation) = crate::schema::mutations::xml_address::empty_paragraph_text_mutation(snapshot, address, text).map_err(semio_framework_value::ValueError::into_message)? else { return Ok(None) };
        measure_mutation(&mutation)?;
        mutation
    } else {
        DocxMutation::SetRunText(crate::schema::mutations::set_run_text::SetRunText { address: address.clone(), text: text.to_owned() })
    };
    let prepared = crate::schema::mutations::prepare_addressed_xml_mutation(snapshot, &mutation).map_err(semio_framework_value::ValueError::into_message)?;
    Ok(prepared.changed.then_some(mutation))
}

#[derive(Default)]
pub(super) struct DocxXmlEdit {
    reserve: Option<usize>,
}

impl PagedOneItemEdit<DocxSnapshot, DocxMutation> for DocxXmlEdit {
    const PREFIX: &'static str = PREFIX;

    fn recognizes(mutation: &DocxMutation) -> bool {
        recognizes(mutation)
    }

    fn preflight(mutation: &DocxMutation) -> Result<usize, String> {
        measure_mutation(mutation)
    }

    fn advance(&mut self, post: &mut DocxSnapshot, mutation: &DocxMutation, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<DocxMutation>, ValueError> {
        let Some(reserve) = self.reserve else {
            measure_snapshot(post).map_err(|message| ValueError::new(ValueRefusalKind::InvalidValue, message))?;
            let reserve = measure_mutation(mutation).map_err(|message| ValueError::new(ValueRefusalKind::InvalidValue, message))?.saturating_add(TURN_BYTES);
            self.reserve = Some(reserve);
            return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        };
        if grant.maximum_copy_bytes < TURN_BYTES || grant.maximum_capacity_bytes < reserve || grant.maximum_release_bytes < TURN_BYTES {
            return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress::default()));
        }
        let prepared = apply_addressed_xml_mutation_in_place(post, mutation)?;
        if !prepared.changed {
            return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("{PREFIX}.no-op")));
        }
        measure_snapshot(post).map_err(|message| ValueError::new(ValueRefusalKind::InvalidValue, message))?;
        let retained = measure_mutation(&prepared.inverse).map_err(|message| ValueError::new(ValueRefusalKind::InvalidValue, message))?;
        if retained > grant.maximum_capacity_bytes {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "stdio-docx-base-set-page.inverse-exceeds-capacity-grant"));
        }
        Ok(PagedOneItemEditStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: TURN_BYTES, retained_capacity_bytes: retained, released_bytes: 0 }, prepared.inverse))
    }
}
