//! 📬️ Strictly admitted canonical DOCX XML publication.

use super::*;
use crate::schema::mutations::{apply_addressed_xml_mutation_in_place, replace_xml_node};
use semio_framework_plugin::plugin_app_close_prelude::store as app_store;
use semio_s_artifact_stdio_contract::editing::NativeEditPreparationRoute;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};
use std::{mem::ManuallyDrop, sync::Arc};

const PREFIX: &str = "stdio-docx-base-set-page";
/// 📏️ The one-item store preparation copies the whole document in one step, so an owner (the document, or one mutation)
/// plus one turn fits the one-item store budget exactly — one measured item per 16 owned bytes — and larger owners refuse
/// typed (`paged-owner-required`) before the store ever sees an inadmissible footprint.
const OWNER_BYTES: usize = app_store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES - TURN_BYTES;
const OWNER_ITEMS: usize = OWNER_BYTES / 16;
const OWNER_DEPTH: usize = 32;
const TURN_BYTES: usize = 4_096;

pub(crate) fn route(_prefix: &'static str) -> Option<NativeEditPreparationRoute<DocxSnapshot, DocxMutation>> {
    Some(NativeEditPreparationRoute::new(recognizes, Arc::new(DocxPreparationFactory)))
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
    measure.add(snapshot.opc.parts.capacity().saturating_mul(std::mem::size_of::<semio_s_artifact_stdio_zip::opc::OpcPart>()))?;
    for part in &snapshot.opc.parts {
        measure.string(&part.path)?;
        measure.string(&part.content_type)?;
        measure.add(part.bytes.capacity())?;
    }
    measure.add(snapshot.opc.content_types.defaults.capacity().saturating_add(snapshot.opc.content_types.overrides.capacity()).saturating_mul(std::mem::size_of::<(String, String)>()))?;
    for (left, right) in snapshot.opc.content_types.defaults.iter().chain(&snapshot.opc.content_types.overrides) {
        measure.string(left)?;
        measure.string(right)?;
    }
    measure.add(snapshot.opc.relationships.len().saturating_mul(size_of::<(String, Vec<semio_s_artifact_stdio_zip::opc::OpcRelationship>)>() * 2))?;
    for (owner, relationships) in &snapshot.opc.relationships {
        measure.string(owner)?;
        measure.add(relationships.capacity().saturating_mul(std::mem::size_of::<semio_s_artifact_stdio_zip::opc::OpcRelationship>()))?;
        for relationship in relationships {
            measure.string(&relationship.id)?;
            measure.string(&relationship.rel_type)?;
            measure.string(&relationship.target)?;
        }
    }
    measure.string(&snapshot.opc.comment)?;
    measure.add(snapshot.xml_parts.capacity().saturating_mul(std::mem::size_of::<crate::schema::snapshot::DocxXmlPart>()))?;
    for part in &snapshot.xml_parts {
        measure.string(&part.path)?;
        measure.string(&part.content_type)?;
        measure_document(&part.document, &mut measure)?;
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
    let mutation = DocxMutation::SetRunText(crate::schema::mutations::set_run_text::SetRunText { address: address.clone(), text: text.to_owned() });
    let prepared = crate::schema::mutations::prepare_addressed_xml_mutation(snapshot, &mutation)?;
    Ok(prepared.changed.then_some(mutation))
}

struct DocxPreparationFactory;

impl app_store::ArtifactStoreOneItemPreparationFactory<DocxSnapshot, DocxMutation> for DocxPreparationFactory {
    fn preflight(&self, mutation: &DocxMutation, description: Option<&str>, lane: app_store::HistoryLane) -> Result<app_store::ArtifactStoreOneItemFootprint, String> {
        if lane != app_store::HistoryLane::Document || description.is_some_and(|value| value.len() > app_store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err(format!("{PREFIX}.lane"));
        }
        let mutation_bytes = measure_mutation(mutation)?;
        Ok(app_store::ArtifactStoreOneItemFootprint::for_one_invertible_item(TURN_BYTES.saturating_add(mutation_bytes)))
    }

    fn begin(
        &self,
        request: app_store::ArtifactStoreOneItemPreparationRequest<DocxSnapshot, DocxMutation>,
    ) -> Result<Box<dyn app_store::ArtifactStoreOneItemPreparation<DocxSnapshot, DocxMutation>>, app_store::ArtifactStoreOneItemPreparationRequest<DocxSnapshot, DocxMutation>> {
        let admitted = request.lane == app_store::HistoryLane::Document
            && request.operation == request.authority.operation()
            && request.generation == request.authority.generation()
            && request.base_revision == request.authority.base_revision()
            && measure_snapshot(request.base.get()).is_ok()
            && measure_mutation(&request.mutation).is_ok();
        if !admitted {
            return Err(request);
        }
        Ok(Box::new(DocxPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            inverse: None,
            post: None,
            sealer: None,
            external_retirement: None,
            checkpoint: Default::default(),
            seal_base: None,
            phase: 0,
            cancelled: false,
            closing: false,
        }))
    }
}

struct DocxPreparation {
    base: Option<app_store::SnapshotRead<DocxSnapshot>>,
    mutation: Option<DocxMutation>,
    description: Option<String>,
    authority: Option<Arc<app_store::ArtifactStoreOneItemLiveAuthority>>,
    inverse: Option<DocxMutation>,
    post: Option<Arc<DocxSnapshot>>,
    sealer: Option<app_store::ArtifactStoreOneItemSealer<DocxSnapshot, DocxMutation>>,
    external_retirement: Option<Box<dyn app_store::ErasedSnapshotRetirement>>,
    checkpoint: app_store::ArtifactStoreOneItemCheckpoint,
    seal_base: Option<app_store::ArtifactStoreOneItemCheckpoint>,
    phase: u8,
    cancelled: bool,
    closing: bool,
}

impl DocxPreparation {
    fn progress(&mut self, bytes: usize) -> app_store::ArtifactStoreOneItemPreparationStep {
        self.checkpoint.cursor = self.checkpoint.cursor.saturating_add(1);
        self.checkpoint.completed_items = self.checkpoint.completed_items.saturating_add(1);
        self.checkpoint.completed_bytes = self.checkpoint.completed_bytes.saturating_add(bytes as u64);
        app_store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint)
    }
}

impl app_store::ArtifactStoreOneItemPreparation<DocxSnapshot, DocxMutation> for DocxPreparation {
    fn advance(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(app_store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if let Some(sealer) = self.sealer.as_mut() {
            let step = sealer.advance(grant)?;
            let checkpoint = match step {
                app_store::ArtifactStoreOneItemPreparationStep::Progress(value) | app_store::ArtifactStoreOneItemPreparationStep::Prepared(value) => value,
                app_store::ArtifactStoreOneItemPreparationStep::Blocked => return Ok(app_store::ArtifactStoreOneItemPreparationStep::Blocked),
            };
            let base = self.seal_base.ok_or_else(|| format!("{PREFIX}.seal-base"))?;
            self.checkpoint = app_store::ArtifactStoreOneItemCheckpoint {
                cursor: base.cursor.saturating_add(checkpoint.cursor),
                completed_items: base.completed_items.saturating_add(checkpoint.completed_items),
                completed_bytes: base.completed_bytes.saturating_add(checkpoint.completed_bytes),
                digest: checkpoint.digest,
            };
            if matches!(step, app_store::ArtifactStoreOneItemPreparationStep::Prepared(_)) {
                self.checkpoint.digest = sealer.prepared().ok_or_else(|| format!("{PREFIX}.prepared"))?.edit_digest();
                return Ok(app_store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
            }
            return Ok(app_store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        match self.phase {
            0 => {
                if grant.maximum_bytes < TURN_BYTES {
                    return Ok(app_store::ArtifactStoreOneItemPreparationStep::Blocked);
                }
                let base = self.base.as_ref().ok_or_else(|| format!("{PREFIX}.base"))?.get();
                measure_snapshot(base)?;
                let mutation = self.mutation.as_ref().ok_or_else(|| format!("{PREFIX}.mutation-owner"))?;
                let mut post = base.clone();
                let prepared = apply_addressed_xml_mutation_in_place(&mut post, mutation)?;
                if !prepared.changed {
                    return Err(format!("{PREFIX}.no-op"));
                }
                measure_snapshot(&post)?;
                measure_mutation(&prepared.inverse)?;
                self.inverse = Some(prepared.inverse);
                self.post = Some(Arc::new(post));
                self.phase = 1;
                Ok(self.progress(TURN_BYTES))
            }
            1 => {
                let mutation = self.mutation.take().ok_or_else(|| format!("{PREFIX}.mutation-owner"))?;
                let inverse = self.inverse.take().ok_or_else(|| format!("{PREFIX}.inverse-owner"))?;
                let post = self.post.take().ok_or_else(|| format!("{PREFIX}.post-owner"))?;
                let authority = self.authority.as_ref().ok_or_else(|| format!("{PREFIX}.authority-owner"))?;
                let id = authority.edit_id();
                let edit = protocol::Edit {
                    id: id.clone(),
                    actor: Some(authority.actor().to_string()),
                    forwards: vec![mutation],
                    inverse: vec![inverse],
                    mutation_meta: vec![protocol::MutationMeta {
                        mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
                        dependencies: Vec::new(),
                        base_version: authority.base_applied_edit_count() as u64,
                        author_id: Some(protocol::ActorId(authority.actor().to_string())),
                        timestamp: authority.next_clock(),
                        undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                        payload_hash: None,
                        semantic_kind: None,
                        label: None,
                        group_id: authority.group_id().map(str::to_owned),
                        origin: Default::default(),
                    }],
                    description: self.description.take(),
                    coalesce_key: None,
                    sequence_number: authority.next_sequence_number(),
                    started_at: String::new(),
                    finished_at: None,
                };
                self.sealer = Some(authority.begin_one_item_seal(edit, post, Arc::new(DocxMutationRetirementFactory), Arc::new(DocxSnapshotRetirementFactory)));
                self.seal_base = Some(self.checkpoint);
                self.phase = 2;
                Ok(self.progress(1))
            }
            _ => Err(format!("{PREFIX}.state")),
        }
    }

    fn checkpoint(&self) -> app_store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&app_store::ArtifactStoreOneItemPrepared<DocxSnapshot, DocxMutation>> {
        self.sealer.as_ref().and_then(app_store::ArtifactStoreOneItemSealer::prepared)
    }

    fn take_prepared(&mut self) -> Option<app_store::ArtifactStoreOneItemPrepared<DocxSnapshot, DocxMutation>> {
        self.sealer.as_mut().and_then(app_store::ArtifactStoreOneItemSealer::take_prepared)
    }

    fn cancel(&mut self) {
        self.cancelled = true;
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.cancel();
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(sealer) = self.sealer.as_mut() {
            sealer.begin_close();
        }
    }

    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::SnapshotRetirementStep, String> {
        if !self.closing || !grant.permits_one() {
            return Ok(app_store::SnapshotRetirementStep::Blocked);
        }
        if let Some(active) = self.external_retirement.as_mut() {
            let step = active.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
            if step == app_store::SnapshotRetirementStep::Complete {
                if !active.terminal_is_empty() {
                    return Err(format!("{PREFIX}.retirement-witness"));
                }
                self.external_retirement = None;
                return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(sealer) = self.sealer.as_mut() {
            let step = sealer.close_step(grant)?;
            if step == app_store::SnapshotRetirementStep::Complete {
                if !sealer.terminal_is_empty() {
                    return Err(format!("{PREFIX}.sealer-witness"));
                }
                self.sealer = None;
                return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(value) = self.inverse.take().or_else(|| self.mutation.take()) {
            self.external_retirement = Some(app_store::ArtifactOwnedValueRetirementFactory::retire_owned(&DocxMutationRetirementFactory, value));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(value) = self.post.take() {
            self.external_retirement = Some(app_store::SnapshotRetirementFactory::retire(&DocxSnapshotRetirementFactory, value));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(description) = self.description.take() {
            self.external_retirement = Some(app_store::retirement::owned_retirement(description));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err(format!("{PREFIX}.base-return"));
            }
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.take() {
            self.external_retirement = Some(authority.retire());
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(app_store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.inverse.is_none() && self.post.is_none() && self.sealer.is_none() && self.external_retirement.is_none()
    }
}

impl Drop for DocxPreparation {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || app_store::ArtifactStoreOneItemPreparation::terminal_is_empty(self), "DOCX preparation dropped with live owners");
    }
}

struct AdmittedRetirement<T> {
    value: ManuallyDrop<Option<T>>,
}

impl<T> AdmittedRetirement<T> {
    fn new(value: T) -> Self {
        Self { value: ManuallyDrop::new(Some(value)) }
    }
}

impl<T: Send + 'static> app_store::ErasedSnapshotRetirement for AdmittedRetirement<T> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<app_store::SnapshotRetirementStep, String> {
        if self.value.is_some() {
            if maximum_items == 0 || maximum_bytes < TURN_BYTES {
                return Ok(app_store::SnapshotRetirementStep::Blocked);
            }
            self.value.take();
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: TURN_BYTES });
        }
        Ok(app_store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none()
    }

    fn next_close_byte_demand(&self) -> usize {
        self.value.as_ref().map_or(1, |_| TURN_BYTES)
    }
}

impl<T> Drop for AdmittedRetirement<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.value.is_none(), "admitted DOCX owner dropped before terminal emptiness");
        unsafe { ManuallyDrop::drop(&mut self.value) };
    }
}

struct DocxMutationRetirementFactory;

impl app_store::ArtifactOwnedValueRetirementFactory<DocxMutation> for DocxMutationRetirementFactory {
    fn retire_owned(&self, value: DocxMutation) -> Box<dyn app_store::ErasedSnapshotRetirement> {
        Box::new(AdmittedRetirement::new(value))
    }
}

struct DocxSnapshotRetirementFactory;

impl app_store::SnapshotRetirementFactory<DocxSnapshot> for DocxSnapshotRetirementFactory {
    fn retire(&self, snapshot: Arc<DocxSnapshot>) -> Box<dyn app_store::ErasedSnapshotRetirement> {
        Box::new(AdmittedRetirement::new(snapshot))
    }
}

fn canonical_text(value: &str) -> app_store::ArtifactCanonicalJsonValue<'_> {
    app_store::ArtifactCanonicalJsonValue::Scalar(app_store::ArtifactCanonicalJsonNode::String(value))
}

fn canonical_static<'a>(value: &'static str) -> app_store::ArtifactCanonicalJsonValue<'a> {
    app_store::ArtifactCanonicalJsonValue::Scalar(app_store::ArtifactCanonicalJsonNode::String(value))
}

fn canonical_index<'a>(value: usize) -> app_store::ArtifactCanonicalJsonValue<'a> {
    app_store::ArtifactCanonicalJsonValue::Scalar(app_store::ArtifactCanonicalJsonNode::U64(value as u64))
}

fn canonical_bool<'a>(value: bool) -> app_store::ArtifactCanonicalJsonValue<'a> {
    app_store::ArtifactCanonicalJsonValue::Scalar(app_store::ArtifactCanonicalJsonNode::Bool(value))
}

fn canonical_optional_text(value: Option<&str>) -> app_store::ArtifactCanonicalJsonValue<'_> {
    value.map_or(app_store::ArtifactCanonicalJsonValue::Scalar(app_store::ArtifactCanonicalJsonNode::Null), canonical_text)
}

fn canonical_object<'a, const N: usize>(mut fields: [(&'a str, app_store::ArtifactCanonicalJsonValue<'a>); N]) -> app_store::ArtifactCanonicalJsonValue<'a> {
    fields.sort_unstable_by(|left, right| left.0.cmp(right.0));
    app_store::ArtifactCanonicalJsonValue::Object(app_store::ArtifactCanonicalJsonObject::new(fields.into_iter()))
}

fn canonical_address(address: &DocxXmlAddress) -> app_store::ArtifactCanonicalJsonValue<'_> {
    let node_path = app_store::ArtifactCanonicalJsonValue::Array(app_store::ArtifactCanonicalJsonArray::new(address.node_path.iter().copied().map(canonical_index)));
    canonical_object([("expectedName", canonical_text(&address.expected_name)), ("nodePath", node_path), ("partPath", canonical_text(&address.part_path)), ("revision", canonical_text(&address.revision))])
}

fn canonical_attr(attr: &XmlAttr) -> app_store::ArtifactCanonicalJsonValue<'_> {
    canonical_object([("name", canonical_text(&attr.name)), ("value", canonical_text(&attr.value))])
}

fn canonical_node(node: &XmlNode) -> app_store::ArtifactCanonicalJsonValue<'_> {
    match node {
        XmlNode::Element { name, attrs, children } => canonical_object([
            ("attrs", app_store::ArtifactCanonicalJsonValue::Array(app_store::ArtifactCanonicalJsonArray::new(attrs.iter().map(canonical_attr)))),
            ("children", app_store::ArtifactCanonicalJsonValue::Array(app_store::ArtifactCanonicalJsonArray::new(children.iter().map(canonical_node)))),
            ("kind", canonical_static("element")),
            ("name", canonical_text(name)),
        ]),
        XmlNode::Text { text } => canonical_object([("kind", canonical_static("text")), ("text", canonical_text(text))]),
        XmlNode::CData { text } => canonical_object([("kind", canonical_static("cData")), ("text", canonical_text(text))]),
        XmlNode::Comment { text } => canonical_object([("kind", canonical_static("comment")), ("text", canonical_text(text))]),
        XmlNode::ProcessingInstruction { target, data } => canonical_object([("data", canonical_text(data)), ("kind", canonical_static("processingInstruction")), ("target", canonical_text(target))]),
    }
}

impl app_store::ArtifactCanonicalJson for DocxMutation {
    fn canonical_json_borrowed_root(&self) -> Result<Option<app_store::ArtifactCanonicalJsonValue<'_>>, String> {
        Ok(Some(match self {
            DocxMutation::SetRunText(value) => canonical_object([("address", canonical_address(&value.address)), ("mutation", canonical_static("setRunText")), ("text", canonical_text(&value.text))]),
            DocxMutation::ReplaceXmlNode(replace_xml_node::ReplaceXmlNode { address, node }) => canonical_object([("address", canonical_address(address)), ("mutation", canonical_static("replaceXmlNode")), ("node", canonical_node(node))]),
            DocxMutation::SetRunFormatting(value) => canonical_object([
                ("address", canonical_address(&value.address)),
                ("bold", canonical_bool(value.bold)),
                ("italic", canonical_bool(value.italic)),
                ("mutation", canonical_static("setRunFormatting")),
                ("underline", canonical_bool(value.underline)),
            ]),
            DocxMutation::SetParagraphStyle(value) => canonical_object([("address", canonical_address(&value.address)), ("mutation", canonical_static("setParagraphStyle")), ("styleId", canonical_optional_text(value.style_id.as_deref()))]),
            DocxMutation::InsertTableRow(value) => canonical_object([
                ("address", canonical_address(&value.address)),
                ("cells", app_store::ArtifactCanonicalJsonValue::Array(app_store::ArtifactCanonicalJsonArray::new(value.cells.iter().map(|cell| canonical_text(cell))))),
                ("index", canonical_index(value.index)),
                ("mutation", canonical_static("insertTableRow")),
            ]),
            DocxMutation::RemoveTableRow(value) => canonical_object([("address", canonical_address(&value.address)), ("index", canonical_index(value.index)), ("mutation", canonical_static("removeTableRow"))]),
            DocxMutation::InsertXmlNode(value) => canonical_object([("index", canonical_index(value.index)), ("mutation", canonical_static("insertXmlNode")), ("node", canonical_node(&value.node)), ("parent", canonical_address(&value.parent))]),
            DocxMutation::RemoveXmlNode(value) => canonical_object([
                ("expectedName", canonical_text(&value.expected_name)),
                ("index", canonical_index(value.index)),
                ("mutation", canonical_static("removeXmlNode")),
                ("parent", canonical_address(&value.parent)),
                ("revision", canonical_text(&value.revision)),
            ]),
            _ => return Err(format!("{PREFIX}.canonical-mutation")),
        }))
    }
}
