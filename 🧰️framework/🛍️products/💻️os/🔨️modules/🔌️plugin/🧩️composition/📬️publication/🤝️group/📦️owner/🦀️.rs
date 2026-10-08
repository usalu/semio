use super::*;
use semio_framework_job::{Generation, OperationId, StepContext};
use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
use store::MemberOpenOperation;

pub(crate) const PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN: usize = 64;

pub(crate) struct PrivateChildGroupSource {
    pub(crate) parent: store::MemberStoreOwnedBatch,
    pub(crate) parent_touched: bool,
    pub(crate) children: Vec<OwnedChildEmit>,
    pub(crate) transaction: Option<protocol::TransactionRef>,
    pub(crate) group_id: String,
}

#[derive(Clone,Copy)]
pub(crate) struct PrivateChildPreparedReceiptIdentity {
    index: usize,
    publication: usize,
    edit: (usize,usize),
    group: (usize,usize),
}

pub(crate) struct PrivateChildPreparedReceipt<'a> {
    pub(crate) index: usize,
    pub(crate) edit_id: &'a str,
    pub(crate) reference: &'a ArtifactRef,
    pub(crate) key: Option<MemberKeyRef<'a>>,
    pub(crate) group_id: &'a str,
}

pub(crate) struct PrivateChildGroupRow<M: SpaceMember + MemberFactory> {
    input: PrivateChildPublicationInput,
    source: ManuallyDrop<Option<OwnedChildEmit>>,
    metadata: Option<PrivateChildMemberMetadata>,
    request: ManuallyDrop<Option<store::MemberOpenRequest>>,
    open: ManuallyDrop<Option<M::Open>>,
    member: ManuallyDrop<Option<M>>,
    raw: ManuallyDrop<Option<ChildEmit>>,
    lane: Option<PrivateOwnedPublicationLane>,
    identity: Option<PreparedChildContentIdentity>,
    entry: Option<PreparedChildContentEntry>,
    slot: Option<PreparedChildContentSlot>,
    admission: Option<ChildMemberAdmission>,
    input_ready: bool,
    new_member: bool,
    receipt_copied: bool,
}

impl<M: SpaceMember + MemberFactory> PrivateChildGroupRow<M> {
    fn new(source: OwnedChildEmit) -> Self {
        let new_member = source.metadata().is_some_and(|metadata| metadata.genesis.is_some());
        Self { input: PrivateChildPublicationInput::new(source), source: ManuallyDrop::new(None), metadata: None, request: ManuallyDrop::new(None), open: ManuallyDrop::new(None), member: ManuallyDrop::new(None), raw: ManuallyDrop::new(None), lane: None, identity: None, entry: None, slot: None, admission: None, input_ready: false, new_member, receipt_copied: false }
    }
    fn key(&self) -> MemberKeyRef<'_> {
        if let Some(parts) = self.metadata.as_ref().and_then(PrivateChildMemberMetadata::parts) { return parts.key.borrowed(); }
        let source = self.input.source().expect("private row retains its original child address");
        MemberKeyRef { owner: &source.owner, slot: &source.slot, child_id: &source.child_id }
    }
    fn member<'a>(&'a self, registry: &'a ChildMemberRegistry<M>) -> Result<&'a M, Fault> {
        if let Some(member) = self.member.as_ref() { return Ok(member); }
        registry.member(self.key()).map(|entry| &entry.member).ok_or_else(|| plugin_sdk_fault("private row lost its exact retained existing member"))
    }
    fn existing<'a>(&'a self, registry: &'a ChildMemberRegistry<M>) -> Option<&'a ArtifactRef> {
        (!self.new_member).then(|| registry.member(self.key()).map(|entry| &entry.reference)).flatten()
    }
    fn input_capacity(&self, registry: &ChildMemberRegistry<M>, parent_id: &str, dialect: &ArtifactDialect, actor: &str, transaction: Option<&protocol::TransactionRef>, group_id: &str) -> Result<usize, ValueError> {
        self.input.next_capacity_byte_demand(parent_id, dialect, self.existing(registry), actor, transaction, Some(group_id))
    }
    fn advance_input(&mut self, registry: &ChildMemberRegistry<M>, parent_id: &str, dialect: &ArtifactDialect, actor: &str, transaction: Option<&protocol::TransactionRef>, group_id: &str, operation: OperationId, generation: Generation, expires_at_us: u64, now_us: u64, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.input_ready { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if self.input.ready() {
            let Some(parts) = self.input.take_ready(grant) else { return Ok(RetainedCloneStep::Progress(Default::default())); };
            *self.source = Some(parts.source); self.metadata = Some(parts.metadata); *self.request = parts.request; self.input_ready = true;
            return Ok(group_structural());
        }
        let key = self.input.source().map(|source| MemberKeyRef { owner: &source.owner, slot: &source.slot, child_id: &source.child_id }).expect("private input source remains retained");
        let existing = registry.member(key).map(|entry| &entry.reference);
        self.input.advance(parent_id, dialect, if self.new_member { None } else { existing }, actor, transaction, Some(group_id), M::OPEN_DECLARATIONS, operation, generation, expires_at_us, now_us, grant)
    }
    fn open_capacity(&self) -> Result<usize, Fault> {
        self.request.as_ref().map_or(Ok(0), |request| M::open_birth_bytes(request).map_err(|_| plugin_sdk_fault("private child factory refused its original request birth")))
    }
    fn advance_open(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant, begin: fn(&mut Option<store::MemberOpenRequest>, RetainedCloneGrant) -> Result<Option<M::Open>, store::MemberOpenDiagnostic>) -> Result<bool, Fault> {
        if !self.new_member || self.member.is_some() { return Ok(true); }
        if self.request.is_some() {
            *self.open = begin(&mut self.request, grant).map_err(|_| plugin_sdk_fault("private child factory refused its exact original open authority"))?;
            return Ok(false);
        }
        let open = self.open.as_mut().ok_or_else(|| plugin_sdk_fault("private child opening lost its admitted owner"))?;
        if !open.terminal_is_empty() {
            match open.step(cx) {
                store::MemberOpenStep::Pending(_) => return Ok(false),
                store::MemberOpenStep::Rejected(_) => return Err(plugin_sdk_fault("private child open rejected its declared request")),
                store::MemberOpenStep::Ready(member) => { *self.member = Some(member); return Ok(false); }
            }
        }
        Ok(false)
    }
    fn retire_open(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let Some(open) = self.open.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        if !open.terminal_is_empty() { return open.close_step(grant.maximum_items.min(1), grant.maximum_release_bytes).map(group_snapshot); }
        let bytes = open.terminal_drop_byte_demand().expect("terminal private open retains its exact physical frame extent");
        if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.open.take();
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }))
    }
    fn make_lane(&mut self, registry: &ChildMemberRegistry<M>, operation: OperationId, grant: RetainedCloneGrant) -> Result<bool, Fault> {
        if self.lane.is_some() { return Ok(true); }
        if grant.maximum_items == 0 { return Ok(false); }
        let (expected_generation, expected_revision) = self.member(registry)?.one_item_publication_identity();
        let (raw, mutations) = self.source.as_mut().and_then(OwnedChildEmit::take_parts).ok_or_else(|| plugin_sdk_fault("private child lane requires its original typed source"))?;
        *self.raw = Some(raw); self.source.take();
        let parts = self.metadata.as_mut().and_then(PrivateChildMemberMetadata::parts_mut).expect("private lane retains prebuilt publication metadata");
        self.identity = Some(std::mem::replace(&mut parts.prepared_identity, PreparedChildContentIdentity { key: MemberKey { owner: String::new(), slot: String::new(), child_id: String::new() }, reference: ArtifactRef { artifact_id: String::new(), dialect: ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } } }));
        self.lane = Some(PrivateOwnedPublicationLane::new(store::MemberStoreOwnedBatchRequest { operation, expected_generation, expected_revision, actor: std::mem::take(&mut parts.publication_actor), group_id: parts.group_id.take(), transaction: parts.transaction.take(), mutations }));
        Ok(false)
    }
    fn advance_lane(&mut self, registry: &mut ChildMemberRegistry<M>, visibility: &Arc<vcs::ArtifactGroupVisibility>, grant: store::ArtifactStoreOneItemGrant) -> Result<bool, Fault> {
        let key = self.metadata.as_ref().and_then(PrivateChildMemberMetadata::parts).expect("private lane retains its exact member key").key.borrowed();
        let member = if let Some(member) = self.member.as_mut() { member } else { &mut registry.member_mut(key).ok_or_else(|| plugin_sdk_fault("private lane lost its original member"))?.member };
        let lane = self.lane.as_mut().expect("private lane is admitted before preparation");
        if lane.staged() { return Ok(true); }
        if lane.prepared_for_staging() && !self.receipt_copied { return Ok(false); }
        lane.advance(member, visibility, grant).map_err(plugin_sdk_fault)?;
        Ok(false)
    }
    fn terminal_is_empty(&self) -> bool { self.input.terminal_is_empty() && self.source.is_none() && self.metadata.is_none() && self.request.is_none() && self.open.is_none() && self.member.is_none() && self.raw.is_none() && self.lane.is_none() && self.identity.is_none() && self.entry.is_none() && self.slot.is_none() && self.admission.is_none() }
    fn close_demand(&self) -> Result<usize, ValueError> {
        if !self.input.terminal_is_empty() { return self.input.next_close_byte_demand(); }
        if let Some(request) = self.request.as_ref() { return Ok(request.next_close_byte_demand()); }
        if let Some(open) = self.open.as_ref() { return Ok(open.terminal_drop_byte_demand().unwrap_or_else(|| open.next_close_byte_demand())); }
        if let Some(entry) = self.entry.as_ref() { return Ok(entry.next_close_byte_demand()); }
        if self.slot.is_some() { return Ok(ChildContentView::prepared_entry_release_bytes()); }
        if let Some(lane) = self.lane.as_ref() { return lane.next_close_byte_demand(); }
        if let Some(source) = self.source.as_ref() { return source.next_close_byte_demand(); }
        if let Some(member) = self.member.as_ref() { return Ok(if member.close_owned_terminal_is_empty() { member.member_frame_bytes() } else { member.close_owned_byte_demand() }); }
        if let Some(raw) = self.raw.as_ref() { return Ok(raw.next_close_byte_demand()); }
        if let Some(metadata) = self.metadata.as_ref() { return Ok(metadata.next_close_byte_demand()); }
        Ok(0)
    }
    fn close(&mut self, registry: &mut ChildMemberRegistry<M>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        if !self.input.terminal_is_empty() { return self.input.close_granted(grant).map_err(|error| plugin_sdk_fault(error.to_string())); }
        if let Some(request) = self.request.as_mut() {
            if request.terminal_is_empty() { self.request.take(); return Ok(group_structural()); }
            return request.close_step(1, grant.maximum_release_bytes).map(group_snapshot).map_err(|error| plugin_sdk_fault(error.to_string()));
        }
        if self.open.is_some() { return self.retire_open(grant).map_err(|error| plugin_sdk_fault(error.to_string())); }
        if self.admission.is_some() { registry.cancel_admission(self.admission.as_ref().unwrap()); self.admission.take(); return Ok(group_structural()); }
        if let Some(lane) = self.lane.as_mut() {
            let key = self.metadata.as_ref().and_then(PrivateChildMemberMetadata::parts).unwrap().key.borrowed();
            let member = if let Some(member) = self.member.as_mut() { member } else { &mut registry.member_mut(key).ok_or_else(|| plugin_sdk_fault("private cancellation lost its exact member owner"))?.member };
            let demand = lane.next_close_byte_demand().map_err(|error| plugin_sdk_fault(error.to_string()))?;
            if demand > grant.maximum_capacity_bytes || demand > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let step = lane.close_step(member, store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: demand.max(1) }).map_err(|error| plugin_sdk_fault(error.to_string()))?;
            if lane.terminal_is_empty() { self.lane = None; }
            return Ok(group_plugin(step));
        }
        if let Some(source) = self.source.as_mut() {
            let step = source.close_granted(grant).map_err(|error| plugin_sdk_fault(error.to_string()))?;
            if source.terminal_is_empty() { self.source.take(); }
            return Ok(step);
        }
        if let Some(member) = self.member.as_mut() {
            if !member.close_owned_terminal_is_empty() { return member.close_owned_step(1, grant.maximum_release_bytes).map(group_snapshot).map_err(|error| plugin_sdk_fault(error.to_string())); }
            let bytes = member.member_frame_bytes();
            if grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.member.take();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
        }
        if let Some(raw) = self.raw.as_mut() { let step = raw.close_one(1, grant.maximum_release_bytes); if step == PluginCloseStep::Complete { self.raw.take(); } return Ok(group_plugin(step)); }
        if self.identity.is_some() { self.metadata.as_mut().and_then(PrivateChildMemberMetadata::parts_mut).unwrap().prepared_identity = self.identity.take().unwrap(); return Ok(group_structural()); }
        if let Some(metadata) = self.metadata.as_mut() { let step = metadata.close_granted(grant).map_err(|error| plugin_sdk_fault(error.to_string()))?; if metadata.terminal_is_empty() { self.metadata = None; } return Ok(step); }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}

fn group_structural() -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }) }
fn group_snapshot(step: store::SnapshotRetirementStep) -> RetainedCloneStep { match step { store::SnapshotRetirementStep::Complete => group_structural(), store::SnapshotRetirementStep::Pending { released_items, released_bytes } => RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, ..Default::default() }), store::SnapshotRetirementStep::Blocked => RetainedCloneStep::Progress(Default::default()) } }

impl<M: SpaceMember + MemberFactory> Drop for PrivateChildGroupRow<M> { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "private child row retains all original and prepared owners until bounded terminal close"); } }

pub(crate) struct PrivateChildPublicationGroup<M: SpaceMember + MemberFactory> {
    parent_source: ManuallyDrop<Option<store::MemberStoreOwnedBatch>>,
    parent_touched: bool,
    parent_identity: Option<(u64, [u8; 32])>,
    sources: ManuallyDrop<Option<std::collections::VecDeque<OwnedChildEmit>>>,
    original_metadata: Option<PrivatePublicationMetadata>,
    parent_issuer: PrivatePublicationMetadataIssuer,
    parent_metadata: Option<PrivatePublicationMetadata>,
    parent_lane: Option<PrivateOwnedPublicationLane>,
    parent_receipt_copied: bool,
    rows: ManuallyDrop<[Option<Box<PrivateChildGroupRow<M>>>; PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN]>,
    count: usize,
    input_cursor: usize,
    open_cursor: usize,
    lane_cursor: usize,
    entry_cursor: usize,
    graph_cursor: usize,
    visibility_owner: Option<vcs::ArtifactGroupVisibilityOwner>,
    visibility: Option<Arc<vcs::ArtifactGroupVisibility>>,
    graph: Option<store::GroupOwnsPreparation>,
    parent_read: ManuallyDrop<Option<store::ErasedSnapshotRead>>,
    projection_cursor: usize,
    projection_row: usize,
    projection_matched: bool,
    projection_ready: bool,
    future: ManuallyDrop<Option<ChildContentView>>,
    displaced: ManuallyDrop<[Option<ChildContentView>; PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN]>,
    committed: bool,
    closing: bool,
    complete: bool,
}

impl<M: SpaceMember + MemberFactory> PrivateChildPublicationGroup<M> {
    pub(crate) fn parent_touched(&self) -> bool { self.parent_touched }


    pub(crate) fn frame_birth_bytes() -> usize { std::mem::size_of::<Self>() }
    pub(crate) fn row_birth_bytes() -> usize { std::mem::size_of::<PrivateChildGroupRow<M>>() }
    /// 📦️ Moves exact source backing and original transaction owners into one inline private group.
    pub(crate) fn try_new(source: PrivateChildGroupSource) -> Result<Self, PrivateChildGroupSource> {
        if source.children.is_empty() || source.children.len() > PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN { return Err(source); }
        let count = source.children.len();
        Ok(Self { parent_source: ManuallyDrop::new(Some(source.parent)), parent_touched: source.parent_touched, parent_identity: None, sources: ManuallyDrop::new(Some(source.children.into())), original_metadata: Some(PrivatePublicationMetadata::from_parts(PrivatePublicationMetadataParts { actor: String::new(), transaction: source.transaction, group_id: Some(source.group_id) })), parent_issuer: PrivatePublicationMetadataIssuer::new(), parent_metadata: None, parent_lane: None, parent_receipt_copied: false, rows: ManuallyDrop::new(std::array::from_fn(|_| None)), count, input_cursor: 0, open_cursor: 0, lane_cursor: 0, entry_cursor: 0, graph_cursor: 0, visibility_owner: None, visibility: None, graph: None, parent_read: ManuallyDrop::new(None), projection_cursor: 0, projection_row: 0, projection_matched: false, projection_ready: false, future: ManuallyDrop::new(None), displaced: ManuallyDrop::new(std::array::from_fn(|_| None)), committed: false, closing: false, complete: false })
    }
    fn transaction(&self) -> Option<&protocol::TransactionRef> { self.original_metadata.as_ref().and_then(PrivatePublicationMetadata::parts).and_then(|parts| parts.transaction.as_ref()) }
    pub(crate) fn group_id(&self) -> &str { self.original_metadata.as_ref().and_then(PrivatePublicationMetadata::parts).and_then(|parts| parts.group_id.as_deref()).expect("private group retains its original common identity") }
    pub(crate) fn prepared_publication(&self, index: usize) -> Option<&dyn store::ErasedMemberStoreOneItemPublication> {
        if self.closing || self.committed { return None; }
        if index == 0 { return self.parent_lane.as_ref()?.prepared_publication(); }
        self.rows.get(index.checked_sub(1)?)?.as_ref()?.lane.as_ref()?.prepared_publication()
    }
    /// 🧾️ Borrows the exact original prepared edit before its candidate moves into final staging.
    pub(crate) fn prepared_operation_schema_parts(&self, index: usize, operation_index: usize) -> Option<(&str, &str)> {
        self.prepared_publication(index)?.prepared_operation_schema_parts(operation_index)
    }
    pub(crate) fn prepared_operation_wire_source(&self, index: usize, inverse: bool, operation_index: usize) -> Option<store::ArtifactPreparedOperationSource<'_>> {
        self.prepared_publication(index)?.prepared_operation_wire_source(inverse, operation_index)
    }

    pub(crate) fn prepared_receipt<'a>(&'a self, index: usize, parent_reference: &'a ArtifactRef, registry: &'a ChildMemberRegistry<M>) -> Option<PrivateChildPreparedReceipt<'a>> {
        if self.closing || self.committed { return None; }
        let group_id = self.group_id();
        if index == 0 {
            if !self.parent_touched || self.parent_receipt_copied { return None; }
            return Some(PrivateChildPreparedReceipt { index, edit_id: self.parent_lane.as_ref()?.prepared_edit_id()?, reference: parent_reference, key: None, group_id });
        }
        let row = self.rows.get(index.checked_sub(1)?)?.as_ref()?;
        if row.receipt_copied { return None; }
        let parts = row.metadata.as_ref()?.parts()?;
        let reference = row.raw.as_ref().and_then(|raw| raw.genesis.as_ref()).map(|genesis| &genesis.reference).or_else(|| row.existing(registry))?;
        Some(PrivateChildPreparedReceipt { index, edit_id: row.lane.as_ref()?.prepared_edit_id()?, reference, key: Some(parts.key.borrowed()), group_id })
    }
    /// 🧿️ Borrows the actual prepared child artifact identity without constructing a parent reference.
    pub(crate) fn prepared_child_artifact_id<'a>(&'a self, index: usize, registry: &'a ChildMemberRegistry<M>) -> Option<&'a str> {
        if self.closing || self.committed || index == 0 { return None; }
        self.prepared_publication(index)?;
        let row = self.rows.get(index - 1)?.as_ref()?;
        if row.receipt_copied { return None; }
        let reference = row.raw.as_ref().and_then(|raw| raw.genesis.as_ref()).map(|genesis| &genesis.reference).or_else(||row.existing(registry))?;
        Some(reference.artifact_id.as_str())
    }

    /// 🪪️ Seals the exact original publication and edit addresses for bounded acknowledgment.
    pub(crate) fn prepared_receipt_identity(&self, index: usize) -> Option<PrivateChildPreparedReceiptIdentity> {
        let publication = self.prepared_publication(index)?;
        let edit = publication.prepared_edit_id()?;
        if index == 0 && (!self.parent_touched || self.parent_receipt_copied) { return None; }
        if index > 0 && self.rows.get(index - 1)?.as_ref()?.receipt_copied { return None; }
        let group = self.group_id();
        Some(PrivateChildPreparedReceiptIdentity { index, publication: publication as *const dyn store::ErasedMemberStoreOneItemPublication as *const () as usize, edit: (edit.as_ptr() as usize, edit.len()), group: (group.as_ptr() as usize, group.len()) })
    }

    /// 🔐️ Acknowledges only the still-retained exact original addresses without comparing copied strings.
    pub(crate) fn acknowledge_prepared_receipt_identity(&mut self, identity: PrivateChildPreparedReceiptIdentity, grant: RetainedCloneGrant) -> bool {
        if grant.maximum_items == 0 { return false; }
        let Some(current) = self.prepared_receipt_identity(identity.index) else { return false; };
        if current.publication != identity.publication || current.edit != identity.edit || current.group != identity.group { return false; }
        if identity.index == 0 { self.parent_receipt_copied = true; }
        else { self.rows[identity.index - 1].as_mut().unwrap().receipt_copied = true; }
        true
    }

    /// ✅️ Records one receipt's completed bounded copy while its original edit identity is still retained.
    pub(crate) fn acknowledge_prepared_receipt(&mut self, index: usize, edit_id: &str, grant: RetainedCloneGrant) -> bool {
        if self.closing || self.committed || grant.maximum_items == 0 { return false; }
        if index == 0 {
            if !self.parent_touched || self.parent_receipt_copied || self.parent_lane.as_ref().and_then(PrivateOwnedPublicationLane::prepared_edit_id) != Some(edit_id) { return false; }
            self.parent_receipt_copied = true;
            return true;
        }
        let Some(row) = index.checked_sub(1).and_then(|index| self.rows.get_mut(index)).and_then(Option::as_mut) else { return false; };
        if row.receipt_copied || row.lane.as_ref().and_then(PrivateOwnedPublicationLane::prepared_edit_id) != Some(edit_id) { return false; }
        row.receipt_copied = true;
        true
    }
    pub(crate) fn child_count(&self) -> usize { self.count }
    pub(crate) fn parent_reference(&self) -> Option<&ArtifactRef> { self.rows.first()?.as_ref()?.metadata.as_ref()?.parts().map(|parts| &parts.registry_owner.parent) }
    pub(crate) fn inputs_ready(&self) -> bool { self.input_cursor == self.count && self.parent_metadata.is_some() }
    pub(crate) fn openings_ready(&self) -> bool { self.open_cursor == self.count }
    pub(crate) fn publications_ready(&self) -> bool { self.lane_cursor == self.count && (!self.parent_touched || self.parent_lane.as_ref().is_some_and(PrivateOwnedPublicationLane::staged)) }
    pub(crate) fn entries_ready(&self) -> bool { self.entry_cursor == self.count }
    pub(crate) fn projection_ready(&self) -> bool { self.projection_ready }
    pub(crate) fn committed(&self) -> bool { self.committed }
    /// 🧮️ Queries only the next full metadata or separately allocated row frame.
    pub(crate) fn next_input_capacity_byte_demand(&self, registry: &ChildMemberRegistry<M>, parent_id: &str, dialect: &ArtifactDialect, actor: &str) -> Result<usize, ValueError> {
        if self.parent_metadata.is_none() { return Ok(self.parent_issuer.next_capacity_byte_demand(PrivatePublicationMetadataSource { actor, transaction: self.transaction(), group_id: Some(self.group_id()) }).unwrap_or(0)); }
        if self.input_cursor == self.count { return Ok(0); }
        self.rows[self.input_cursor].as_ref().map_or(Ok(Self::row_birth_bytes()), |row| row.input_capacity(registry, parent_id, dialect, actor, self.transaction(), self.group_id()))
    }
    /// 🧵️ Advances one funded parent field, row frame, or original child input phase.
    pub(crate) fn advance_inputs(&mut self, registry: &ChildMemberRegistry<M>, parent_id: &str, dialect: &ArtifactDialect, actor: &str, operation: OperationId, generation: Generation, expires_at_us: u64, now_us: u64, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let grant = group_grant(grant);
        if self.closing || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.parent_metadata.is_none() {
            if self.parent_issuer.ready() { self.parent_metadata = self.parent_issuer.take_ready(grant)?; return Ok(group_structural()); }
            let transaction = self.original_metadata.as_ref().and_then(PrivatePublicationMetadata::parts).and_then(|parts| parts.transaction.as_ref());
            let group_id = self.original_metadata.as_ref().and_then(PrivatePublicationMetadata::parts).and_then(|parts| parts.group_id.as_deref()).unwrap();
            return self.parent_issuer.advance(PrivatePublicationMetadataSource { actor, transaction, group_id: Some(group_id) }, grant);
        }
        if self.input_cursor == self.count { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if self.rows[self.input_cursor].is_none() {
            let bytes = Self::row_birth_bytes();
            if grant.maximum_capacity_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let source = self.sources.as_mut().unwrap().pop_front().expect("fixed group retains every original child source");
            self.rows[self.input_cursor] = Some(Box::new(PrivateChildGroupRow::new(source)));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: bytes, ..Default::default() }));
        }
        let transaction = self.original_metadata.as_ref().and_then(PrivatePublicationMetadata::parts).and_then(|parts| parts.transaction.as_ref());
        let group_id = self.original_metadata.as_ref().and_then(PrivatePublicationMetadata::parts).and_then(|parts| parts.group_id.as_deref()).unwrap();
        let row = self.rows[self.input_cursor].as_mut().unwrap();
        let step = row.advance_input(registry, parent_id, dialect, actor, transaction, group_id, operation, generation, expires_at_us, now_us, grant)?;
        if row.input_ready { self.input_cursor += 1; }
        Ok(step)
    }
    pub(crate) fn next_open_capacity_byte_demand(&self) -> Result<usize, Fault> { self.rows.get(self.open_cursor).and_then(Option::as_ref).map_or(Ok(0), |row| row.open_capacity()) }
    /// 🚪️ Drives only the exact funded factory selected by the retained original request.
    pub(crate) fn advance_openings(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant, begin: fn(&mut Option<store::MemberOpenRequest>, RetainedCloneGrant) -> Result<Option<M::Open>, store::MemberOpenDiagnostic>) -> Result<RetainedCloneStep, Fault> {
        let grant = group_grant(grant);
        if self.closing || grant.maximum_items == 0 || !self.inputs_ready() { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.open_cursor == self.count { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let row = self.rows[self.open_cursor].as_mut().unwrap();
        if row.member.is_some() && row.open.is_some() { return row.retire_open(grant).map_err(|error| plugin_sdk_fault(error.to_string())); }
        if row.advance_open(cx, grant, begin)? { self.open_cursor += 1; return Ok(group_structural()); }
        Ok(group_structural())
    }
    pub(crate) fn next_open_release_byte_demand(&self) -> usize { self.rows.get(self.open_cursor).and_then(Option::as_ref).and_then(|row| row.open.as_ref()).map_or(0, |open| open.terminal_drop_byte_demand().unwrap_or_else(|| open.next_close_byte_demand())) }
    pub(crate) fn visibility_birth_bytes() -> usize { 2 * std::mem::size_of::<usize>() + std::mem::size_of::<vcs::ArtifactGroupVisibility>().next_multiple_of(std::mem::align_of::<usize>()) }
    pub(crate) fn next_publication_capacity_byte_demand(&self, parent: &impl SpaceMember, registry: &ChildMemberRegistry<M>) -> Result<usize, Fault> {
        if self.visibility.is_none() { return Ok(Self::visibility_birth_bytes()); }
        if let Some(lane) = self.parent_lane.as_ref().filter(|lane| !lane.staged()) { return lane.next_capacity_byte_demand(parent).map_err(plugin_sdk_fault); }
        if self.lane_cursor == self.count { return Ok(0); }
        let row = self.rows[self.lane_cursor].as_ref().unwrap();
        row.lane.as_ref().map_or(Ok(0), |lane| lane.next_capacity_byte_demand(row.member(registry)?).map_err(plugin_sdk_fault))
    }
    /// 🪜️ Retains parent and child typed lanes under one pending common visibility owner.
    pub(crate) fn advance_publications(&mut self, parent: &mut impl SpaceMember, registry: &mut ChildMemberRegistry<M>, operation: OperationId, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let grant = group_grant(grant);
        if self.closing || grant.maximum_items == 0 || !self.openings_ready() { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.parent_identity.is_none() { self.parent_identity = Some(parent.one_item_publication_identity()); return Ok(group_structural()); }
        if self.visibility.is_none() {
            let bytes = Self::visibility_birth_bytes();
            if grant.maximum_capacity_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let owner = vcs::ArtifactGroupVisibilityOwner::new(); self.visibility = Some(owner.view()); self.visibility_owner = Some(owner);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: bytes, ..Default::default() }));
        }
        if self.parent_touched && self.parent_lane.is_none() {
            let (expected_generation, expected_revision) = parent.one_item_publication_identity();
            let parts = self.parent_metadata.as_mut().and_then(PrivatePublicationMetadata::parts_mut).expect("group retains prebuilt parent metadata");
            self.parent_lane = Some(PrivateOwnedPublicationLane::new(store::MemberStoreOwnedBatchRequest { operation, expected_generation, expected_revision, actor: std::mem::take(&mut parts.actor), group_id: parts.group_id.take(), transaction: parts.transaction.take(), mutations: self.parent_source.take().unwrap() }));
            return Ok(group_structural());
        }
        let item = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_capacity_bytes };
        let visibility = self.visibility.as_ref().unwrap();
        if let Some(parent_lane) = self.parent_lane.as_mut() {
            if parent_lane.prepared_for_staging() && !self.parent_receipt_copied { return Ok(RetainedCloneStep::Progress(Default::default())); }
            if !parent_lane.staged() { parent_lane.advance(parent, visibility, item).map_err(plugin_sdk_fault)?; return Ok(group_structural()); }
        }
        if self.lane_cursor == self.count { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let row = self.rows[self.lane_cursor].as_mut().unwrap();
        if !row.make_lane(registry, operation, grant)? { return Ok(group_structural()); }
        if row.advance_lane(registry, visibility, item)? { self.lane_cursor += 1; }
        Ok(group_structural())
    }

    pub(crate) fn next_projection_capacity_byte_demand(&self) -> usize { if self.parent_touched && self.parent_read.is_none() && !self.projection_ready { store::ErasedSnapshotRead::LEASE_ALLOCATION_BYTES } else { 0 } }
    pub(crate) fn next_projection_release_byte_demand(&self) -> usize { if self.parent_read.is_some() && self.projection_row == self.count { store::ErasedSnapshotRead::LEASE_ALLOCATION_BYTES } else { 0 } }
    /// 🔎️ Checks one exact local-child and target tuple against the staged parent before returning its private lease.
    pub(crate) fn advance_projection(&mut self, parent: &impl SpaceMember, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let grant = group_grant(grant);
        if self.closing || grant.maximum_items == 0 || !self.publications_ready() { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.projection_ready { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let visibility = self.visibility.as_ref().unwrap();
        if !self.parent_touched && self.parent_identity != Some(parent.one_item_publication_identity()) { return Err(plugin_sdk_fault("untouched parent authority changed during private child projection")); }
        if self.parent_touched && self.parent_read.is_none() {
            let publication = self.parent_lane.as_mut().unwrap().publication_mut().unwrap();
            let bytes = store::ErasedSnapshotRead::LEASE_ALLOCATION_BYTES;
            if grant.maximum_capacity_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let (_, _, read) = parent.snapshot_read_erased_for_publication(publication, visibility, store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).map_err(plugin_sdk_fault)?;
            *self.parent_read = Some(read);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: bytes, ..Default::default() }));
        }
        if self.projection_row == self.count {
            if !self.parent_touched { self.projection_ready = true; return Ok(group_structural()); }
            let publication = self.parent_lane.as_mut().unwrap().publication_mut().unwrap();
            let bytes = store::ErasedSnapshotRead::LEASE_ALLOCATION_BYTES;
            if grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let read = self.parent_read.take().unwrap();
            match parent.return_prepared_snapshot_read_erased(publication, visibility, read, store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }) {
                Ok(step) => { self.projection_ready = true; return Ok(group_snapshot(step)); }
                Err(rejected) => { *self.parent_read = Some(rejected.snapshot); return Err(plugin_sdk_fault(rejected.reason)); }
            }
        }
        let projection = if self.parent_touched { parent.child_restore_projection_for_read(self.parent_read.as_ref().unwrap()) } else { parent.child_restore_projection() }.map_err(|error| plugin_sdk_fault(error.to_string()))?;
        let row = self.rows[self.projection_row].as_ref().unwrap();
        let identity = row.identity.as_ref().expect("private projection retains its exact prebuilt identity until entry handoff");
        if let Some((slot, fields)) = projection.get(self.projection_cursor) {
            if slot == identity.key.slot && fields.child_id == identity.key.child_id {
                if self.projection_matched || fields.artifact_id != identity.reference.artifact_id || fields.artifact_kind != identity.reference.dialect.artifact_kind || fields.standard != identity.reference.dialect.standard || fields.subset != identity.reference.dialect.subset { return Err(plugin_sdk_fault("staged parent has a duplicate or different exact child target")); }
                self.projection_matched = true;
            }
            self.projection_cursor += 1;
            return Ok(group_structural());
        }
        if !self.projection_matched || !identity.key.owner.is_empty() { return Err(plugin_sdk_fault("staged parent does not declare its original local child identity")); }
        self.projection_row += 1; self.projection_cursor = 0; self.projection_matched = false;
        Ok(group_structural())
    }
    pub(crate) fn next_entry_capacity_byte_demand(&self) -> usize {
        if self.entry_cursor == self.count || self.future.is_none() { return 0; }
        let row = self.rows[self.entry_cursor].as_ref().unwrap();
        if row.entry.is_none() { store::ErasedSnapshotRead::LEASE_ALLOCATION_BYTES } else { ChildContentView::prepared_entry_birth_bytes() }
    }
    /// 🪞️ Prebuilds one candidate lease, fixed registry slot, or immutable view insertion before publication.
    pub(crate) fn advance_entries(&mut self, registry: &mut ChildMemberRegistry<M>, live: &ChildContentView, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let grant = group_grant(grant);
        if self.closing || grant.maximum_items == 0 || !self.projection_ready { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.entries_ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if self.future.is_none() { *self.future = Some(ChildContentView::clone(live)); return Ok(group_structural()); }
        let row = self.rows[self.entry_cursor].as_mut().unwrap();
        let item = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_capacity_bytes };
        if row.entry.is_none() {
            let key = row.metadata.as_ref().and_then(PrivateChildMemberMetadata::parts).unwrap().key.borrowed();
            let member = row.member.as_ref().or_else(|| registry.member(key).map(|entry| &entry.member)).ok_or_else(|| plugin_sdk_fault("prepared view requires its original member"))?;
            row.entry = PreparedChildContentEntry::prepare(&mut row.identity, member, row.lane.as_mut().unwrap().publication_mut().unwrap(), self.visibility.as_ref().unwrap(), item)?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(row.entry.is_some()), retained_capacity_bytes: if row.entry.is_some() { store::ErasedSnapshotRead::LEASE_ALLOCATION_BYTES } else { 0 }, ..Default::default() }));
        }
        if row.new_member && row.admission.is_none() {
            row.admission = Some(registry.admit_identity(row.key())?);
            return Ok(group_structural());
        }
        if let Some((next, slot)) = self.future.as_ref().unwrap().insert_prepared_entry(row.entry.as_mut().unwrap(), item)? {
            self.displaced[self.entry_cursor] = self.future.take(); *self.future = Some(next); row.slot = Some(slot); row.entry = None; self.entry_cursor += 1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: ChildContentView::prepared_entry_birth_bytes(), ..Default::default() }));
        }
        Ok(RetainedCloneStep::Progress(Default::default()))
    }
    pub(crate) fn next_displaced_view_release_byte_demand(&self) -> usize { self.displaced.iter().flatten().next().map_or(0, ChildContentView::prepared_structure_close_byte_demand) }
    /// 🪵️ Retires one intermediate frontier against the two known immutable owners without live recapture.
    pub(crate) fn close_displaced_view_step(&mut self, live: &ChildContentView, grant: RetainedCloneGrant) -> Result<PluginCloseStep, Fault> {
        if grant.maximum_items == 0 { return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }); }
        let Some(view) = self.displaced.iter_mut().flatten().next() else { return Ok(PluginCloseStep::Complete); };
        let future = self.future.as_ref().unwrap_or(live);
        let retained = view.prepared_structure_retainer(live, future).ok_or_else(|| plugin_sdk_fault("private intermediate view lacks its exact known frontier retainer"))?;
        let step = view.close_prepared_structure_step(retained, store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_release_bytes })?;
        if view.root.is_none() { self.displaced.iter_mut().find(|view| view.as_ref().is_some_and(|view| view.root.is_none())).unwrap().take(); }
        Ok(step)
    }
    fn graph_edge<'a>(&'a self, index: usize, registry: &'a ChildMemberRegistry<M>) -> Result<(&'a str, &'a str, &'a str), Fault> {
        let row = self.rows[index].as_ref().unwrap();
        let parts = row.metadata.as_ref().and_then(PrivateChildMemberMetadata::parts).unwrap();
        let target = if let Some(genesis) = row.raw.as_ref().and_then(|raw| raw.genesis.as_ref()) { &genesis.reference } else { &registry.member(parts.key.borrowed()).ok_or_else(|| plugin_sdk_fault("private graph lost its original existing target"))?.reference };
        Ok((&parts.registry_owner.parent.artifact_id, &parts.registry_owner.slot, &target.artifact_id))
    }
    pub(crate) fn next_graph_capacity_byte_demand(&self, graph: &store::CompositionGraph, registry: &ChildMemberRegistry<M>) -> Result<usize, Fault> {
        let Some(preparation) = self.graph.as_ref() else { return Ok(0); };
        if self.graph_cursor < self.count { let (parent, slot, child) = self.graph_edge(self.graph_cursor, registry)?; return preparation.next_edge_byte_demand(graph, parent, slot, child).map_err(|_| plugin_sdk_fault("private graph rejected its exact edge authority")); }
        Ok(preparation.next_byte_demand(graph))
    }
    /// 🌳️ Builds one sorted ownership row or ancestor hop under its original phase allocation grant.
    pub(crate) fn advance_graph(&mut self, graph: &mut store::CompositionGraph, registry: &ChildMemberRegistry<M>, grant: RetainedCloneGrant) -> Result<bool, Fault> {
        if self.closing || grant.maximum_items == 0 || !self.entries_ready() || self.displaced.iter().any(Option::is_some) { return Ok(false); }
        if self.graph.is_none() { self.graph = Some(graph.begin_owns_group(self.visibility.as_ref().unwrap(), self.count).map_err(|_| plugin_sdk_fault("private graph group refused its pending visibility"))?); return Ok(false); }
        let item = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_capacity_bytes };
        if self.graph_cursor < self.count {
            let row = self.rows[self.graph_cursor].as_ref().unwrap();
            let parts = row.metadata.as_ref().and_then(PrivateChildMemberMetadata::parts).unwrap();
            let target = if let Some(genesis) = row.raw.as_ref().and_then(|raw| raw.genesis.as_ref()) { &genesis.reference } else { &registry.member(parts.key.borrowed()).ok_or_else(|| plugin_sdk_fault("private graph retains its original existing target"))?.reference };
            let step = graph.prepare_owns_group_edge(self.graph.as_mut().unwrap(), &parts.registry_owner.parent.artifact_id, &parts.registry_owner.slot, &target.artifact_id, item).map_err(|_| plugin_sdk_fault("private graph refused its declared owner edge"))?;
            if matches!(step, store::GroupOwnsStep::EdgePrepared) { self.graph_cursor += 1; }
            return Ok(false);
        }
        graph.seal_owns_group(self.graph.as_mut().unwrap(), item).map_err(|_| plugin_sdk_fault("private graph refused its prebuilt ownership root"))?;
        Ok(graph.owns_group_ready(self.graph.as_ref().unwrap()))
    }
    pub(crate) fn ready(&self, graph: &store::CompositionGraph) -> bool { !self.closing && !self.committed && self.count <= PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN && self.publications_ready() && self.projection_ready && self.entries_ready() && self.displaced.iter().all(Option::is_none) && self.graph.as_ref().is_some_and(|preparation| graph.owns_group_ready(preparation)) }
    /// 🔓️ Transfers at most sixty-five already staged roots and their prebuilt view before any host output.
    pub(crate) fn commit(&mut self, parent: &mut impl SpaceMember, registry: &mut ChildMemberRegistry<M>, graph: &mut store::CompositionGraph, grant: store::ArtifactStoreOneItemGrant) -> Result<Option<ChildContentView>, Fault> {
        if !grant.permits_one() || grant.maximum_bytes < 4096 || !self.ready(graph) { return Ok(None); }
        if !self.parent_touched && self.parent_identity != Some(parent.one_item_publication_identity()) { return Err(plugin_sdk_fault("untouched parent authority changed before private child decision")); }
        let visibility = self.visibility.as_ref().unwrap();
        if !self.visibility_owner.as_mut().unwrap().commit() { return Err(plugin_sdk_fault("private group lost its single common decision authority")); }
        let item = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 };
        if let Some(lane) = self.parent_lane.as_mut() { lane.adopt(parent, visibility, item).map_err(plugin_sdk_fault)?; }
        for row in self.rows[..self.count].iter_mut().flatten() {
            let key = row.metadata.as_ref().and_then(PrivateChildMemberMetadata::parts).unwrap().key.borrowed();
            let member = if let Some(member) = row.member.as_mut() { member } else { &mut registry.member_mut(key).expect("prepared existing member remains owned throughout the decision").member };
            row.lane.as_mut().unwrap().adopt(member, visibility, item).map_err(plugin_sdk_fault)?;
            if row.new_member {
                let raw = row.raw.as_mut().unwrap();
                let reference = group_take_reference(&mut raw.genesis.as_mut().unwrap().reference);
                let parts = row.metadata.as_mut().and_then(PrivateChildMemberMetadata::parts_mut).unwrap();
                let owner = store::OwnerRef { parent: group_take_reference(&mut parts.registry_owner.parent), slot: std::mem::take(&mut parts.registry_owner.slot), child_id: std::mem::take(&mut parts.registry_owner.child_id) };
                registry.insert_admitted(row.admission.take().unwrap(), reference, owner, row.member.take().unwrap());
            }
            row.slot.take();
        }
        graph.commit_owns_group(self.graph.as_mut().unwrap()).map_err(|_| plugin_sdk_fault("private common decision lost its prepared graph root"))?;
        self.committed = true;
        Ok(self.future.take())
    }
    /// 📏️ Exposes the entire next close allocation, preserving each original owner on undergrant.
    pub(crate) fn next_close_byte_demand(&self) -> Result<usize, ValueError> {
        if self.parent_read.is_some() { return Ok(store::ErasedSnapshotRead::LEASE_ALLOCATION_BYTES); }
        if let Some(view) = self.displaced.iter().flatten().next() { return Ok(view.prepared_structure_close_byte_demand()); }
        if self.future.is_some() {
            if let Some(row) = self.rows[..self.count].iter().flatten().find(|row| row.entry.is_some() || row.slot.is_some()) { return row.close_demand(); }
            return Ok(self.future.as_ref().unwrap().prepared_structure_close_byte_demand());
        }
        if let Some(lane) = self.parent_lane.as_ref() { return lane.next_close_byte_demand(); }
        if let Some(row) = self.rows[..self.count].iter().flatten().next() { return if row.terminal_is_empty() { Ok(Self::row_birth_bytes()) } else { row.close_demand() }; }
        if let Some(batch) = self.parent_source.as_ref() { let (capacity, release) = batch.next_demands()?; return Ok(capacity.max(release).max(batch.next_copy_byte_demand())); }
        if let Some(sources) = self.sources.as_ref() { return if let Some(source) = sources.front() { source.next_close_byte_demand() } else { Ok(sources.capacity() * std::mem::size_of::<OwnedChildEmit>()) }; }
        if let Some(graph) = self.graph.as_ref() { return Ok(graph.next_close_byte_demand()); }
        if let Some(metadata) = self.parent_metadata.as_ref() { return Ok(metadata.next_close_byte_demand()); }
        if !self.parent_issuer.terminal_is_empty() { return Ok(self.parent_issuer.next_close_byte_demand()); }
        if let Some(metadata) = self.original_metadata.as_ref() { return Ok(metadata.next_close_byte_demand()); }
        if self.visibility.is_some() { return Ok(Self::visibility_birth_bytes()); }
        Ok(0)
    }
    /// ♻️ Returns every private read before staged abort and retires whole source, member, row, and visibility extents.
    pub(crate) fn close_step(&mut self, parent: &mut impl SpaceMember, registry: &mut ChildMemberRegistry<M>, graph: &mut store::CompositionGraph, live: &ChildContentView, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let grant = group_grant(grant);
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.closing = true;
        if !self.committed { if let Some(owner) = self.visibility_owner.as_mut() { owner.abort(); } }
        let item = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: grant.maximum_release_bytes };
        if let Some(read) = self.parent_read.take() {
            if grant.maximum_release_bytes < store::ErasedSnapshotRead::LEASE_ALLOCATION_BYTES { *self.parent_read = Some(read); return Ok(RetainedCloneStep::Progress(Default::default())); }
            match parent.return_prepared_snapshot_read_erased(self.parent_lane.as_mut().unwrap().publication_mut().unwrap(), self.visibility.as_ref().unwrap(), read, item) {
                Ok(step) => return Ok(group_snapshot(step)),
                Err(rejected) => { *self.parent_read = Some(rejected.snapshot); return Err(plugin_sdk_fault(rejected.reason)); }
            }
        }
        if self.displaced.iter().any(Option::is_some) { return self.close_displaced_view_step(live, grant).map(group_plugin); }
        if self.future.is_some() {
            if let Some(row) = self.rows[..self.count].iter_mut().flatten().find(|row| row.entry.is_some() || row.slot.is_some()) {
                if let Some(slot) = row.slot.as_mut() {
                    if let Some((entry, step)) = self.future.as_mut().unwrap().take_prepared_entry(slot, item)? { row.entry = Some(entry); row.slot = None; return Ok(group_plugin(step)); }
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                let key = row.metadata.as_ref().and_then(PrivateChildMemberMetadata::parts).unwrap().key.borrowed();
                let member = row.member.as_ref().or_else(|| registry.member(key).map(|entry| &entry.member)).ok_or_else(|| plugin_sdk_fault("private entry cancellation lost its exact original member"))?;
                let entry = row.entry.as_mut().unwrap();
                let step = entry.close_step(member, row.lane.as_mut().unwrap().publication_mut().unwrap(), item)?;
                if entry.terminal_is_empty() { row.entry = None; }
                return Ok(group_plugin(step));
            }
            let future = self.future.as_mut().unwrap();
            let step = future.close_prepared_structure_step(live, item)?;
            if future.root.is_none() { self.future.take(); }
            return Ok(group_plugin(step));
        }
        if let Some(lane) = self.parent_lane.as_mut() {
            let demand = lane.next_close_byte_demand().map_err(|error| plugin_sdk_fault(error.to_string()))?;
            if demand > grant.maximum_capacity_bytes || demand > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let step = lane.close_step(parent, store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: demand.max(1) }).map_err(|error| plugin_sdk_fault(error.to_string()))?;
            if lane.terminal_is_empty() { self.parent_lane = None; }
            return Ok(group_plugin(step));
        }
        if let Some(index) = self.rows[..self.count].iter().position(Option::is_some) {
            let row = self.rows[index].as_mut().unwrap();
            if !row.terminal_is_empty() { return row.close(registry, grant); }
            let bytes = Self::row_birth_bytes();
            if grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.rows[index].take();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
        }
        if let Some(batch) = self.parent_source.as_mut() { let step = batch.close_granted(grant).map_err(|error| plugin_sdk_fault(error.to_string()))?; if batch.terminal_is_empty() { self.parent_source.take(); } return Ok(step); }
        if let Some(sources) = self.sources.as_mut() {
            if let Some(source) = sources.front_mut() { let step = source.close_granted(grant).map_err(|error| plugin_sdk_fault(error.to_string()))?; if source.terminal_is_empty() { sources.pop_front(); } return Ok(step); }
            let bytes = sources.capacity() * std::mem::size_of::<OwnedChildEmit>();
            if grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.sources.take();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
        }
        if let Some(preparation) = self.graph.as_mut() { let step = graph.close_owns_group(preparation, item); if preparation.terminal_is_empty() { self.graph = None; } return Ok(group_snapshot(step)); }
        if let Some(metadata) = self.parent_metadata.as_mut() { let step = metadata.close_granted(grant).map_err(|error| plugin_sdk_fault(error.to_string()))?; if metadata.terminal_is_empty() { self.parent_metadata = None; } return Ok(step); }
        if !self.parent_issuer.terminal_is_empty() { return self.parent_issuer.close_granted(grant).map_err(|error| plugin_sdk_fault(error.to_string())); }
        if let Some(metadata) = self.original_metadata.as_mut() { let step = metadata.close_granted(grant).map_err(|error| plugin_sdk_fault(error.to_string()))?; if metadata.terminal_is_empty() { self.original_metadata = None; } return Ok(step); }
        if self.visibility.is_some() {
            let bytes = Self::visibility_birth_bytes();
            if grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
            if Arc::strong_count(self.visibility.as_ref().unwrap()) != 2 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.visibility_owner.take(); self.visibility.take();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
        }
        self.complete = true;
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.complete && self.parent_source.is_none() && self.sources.is_none() && self.original_metadata.is_none() && self.parent_metadata.is_none() && self.parent_lane.is_none() && self.parent_issuer.terminal_is_empty() && self.rows.iter().all(Option::is_none) && self.graph.is_none() && self.parent_read.is_none() && self.future.is_none() && self.displaced.iter().all(Option::is_none) && self.visibility.is_none() && self.visibility_owner.is_none() }
}

fn group_grant(grant: RetainedCloneGrant) -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), maximum_copy_bytes: grant.maximum_copy_bytes.min(64), ..grant } }
fn group_take_reference(value: &mut ArtifactRef) -> ArtifactRef { ArtifactRef { artifact_id: std::mem::take(&mut value.artifact_id), dialect: ArtifactDialect { artifact_kind: std::mem::take(&mut value.dialect.artifact_kind), standard: std::mem::take(&mut value.dialect.standard), subset: std::mem::take(&mut value.dialect.subset) } } }
fn group_plugin(step: PluginCloseStep) -> RetainedCloneStep { match step { PluginCloseStep::Pending { released_items, released_bytes } => RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, ..Default::default() }), PluginCloseStep::Complete => group_structural(), PluginCloseStep::Blocked { .. } | PluginCloseStep::AwaitingInput { .. } => RetainedCloneStep::Progress(Default::default()) } }

impl<M: SpaceMember + MemberFactory> Drop for PrivateChildPublicationGroup<M> { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "private child group retains original sources and staged authorities until bounded terminal close"); } }
