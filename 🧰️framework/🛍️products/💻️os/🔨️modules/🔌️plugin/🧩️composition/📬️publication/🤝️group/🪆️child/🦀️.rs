use super::*;
use crate::component::private_child_input::PrivateChildGenesisInput;
use semio_framework_job::{Generation, OperationId};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};

pub(crate) struct PrivateChildPublicationInput {
    source: ManuallyDrop<Option<OwnedChildEmit>>,
    issuer: PrivateChildMemberMetadataIssuer,
    metadata: Option<PrivateChildMemberMetadata>,
    genesis: PrivateChildGenesisInput,
    request: ManuallyDrop<Option<store::MemberOpenRequest>>,
    declaration: usize,
    declaration_count: Option<usize>,
    declaration_pointer: Option<usize>,
    schema: Option<&'static str>,
    framed: bool,
    ready: bool,
    closing: bool,
}
pub(crate) struct PrivateChildPublicationInputParts {
    pub(crate) source: OwnedChildEmit,
    pub(crate) metadata: PrivateChildMemberMetadata,
    pub(crate) request: Option<store::MemberOpenRequest>,
}
impl PrivateChildPublicationInput {
    /// 🪆️ Keeps original child metadata and its typed source inline until exact private admission.
    pub(crate) fn new(source: OwnedChildEmit) -> Self {
        Self { source: ManuallyDrop::new(Some(source)), issuer: PrivateChildMemberMetadataIssuer::new(), metadata: None, genesis: PrivateChildGenesisInput::default(), request: ManuallyDrop::new(None), declaration: 0, declaration_count: None, declaration_pointer: None, schema: None, framed: false, ready: false, closing: false }
    }
    pub(crate) fn source(&self) -> Option<&ChildEmit> { self.source.as_ref()?.metadata() }
    pub(crate) fn ready(&self) -> bool { self.ready && !self.closing }
    fn metadata_source<'a>(&'a self, parent_id: &'a str, parent_dialect: &'a ArtifactDialect, existing: Option<&'a ArtifactRef>, actor: &'a str, transaction: Option<&'a protocol::TransactionRef>, group_id: Option<&'a str>) -> Result<PrivateChildMemberMetadataSource<'a>, ValueError> {
        let source = self.source().ok_or_else(|| input_error("private child source has already transferred"))?;
        let expected = source.genesis.as_ref().map(|genesis| &genesis.reference).or(existing).ok_or_else(|| input_error("private child requires its declared existing member or exact genesis"))?;
        Ok(PrivateChildMemberMetadataSource { expected, parent_id, parent_dialect, key: MemberKeyRef { owner: &source.owner, slot: &source.slot, child_id: &source.child_id }, actor, transaction, group_id })
    }
    /// 📐️ Borrows each next complete metadata, URI, or page allocation before any input transfer.
    pub(crate) fn next_capacity_byte_demand(&self, parent_id: &str, parent_dialect: &ArtifactDialect, existing: Option<&ArtifactRef>, actor: &str, transaction: Option<&protocol::TransactionRef>, group_id: Option<&str>) -> Result<usize, ValueError> {
        if self.ready || self.closing { return Ok(0); }
        if self.metadata.is_none() {
            return self.issuer.next_capacity_byte_demand(self.metadata_source(parent_id, parent_dialect, existing, actor, transaction, group_id)?).ok_or_else(|| input_error("private metadata lost its next field birth"));
        }
        let Some(genesis) = self.source().and_then(|source| source.genesis.as_ref()) else { return Ok(0) };
        if self.schema.is_none() || self.declaration_count.is_none_or(|count| self.declaration < count) || self.framed { return Ok(0); }
        let parts = self.metadata.as_ref().and_then(PrivateChildMemberMetadata::parts).ok_or_else(|| input_error("private metadata lost its original fields"))?;
        self.genesis.next_capacity_byte_demand(store::MemberGenesisEnvelopeSource { schema: self.schema.unwrap(), expected: &parts.expected, owner: &parts.owner, initial_pack: &genesis.initial_pack })
    }
    /// 🪜️ Processes one original metadata or canonical genesis input phase under independent work and release grants.
    pub(crate) fn advance(&mut self, parent_id: &str, parent_dialect: &ArtifactDialect, existing: Option<&ArtifactRef>, actor: &str, transaction: Option<&protocol::TransactionRef>, group_id: Option<&str>, declarations: &'static [store::MemberOpenDeclaration], operation: OperationId, generation: Generation, expires_at_us: u64, now_us: u64, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let grant = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), maximum_copy_bytes: grant.maximum_copy_bytes.min(64), ..grant };
        let paused = || RetainedCloneStep::Progress(Default::default());
        if grant.maximum_items == 0 { return Ok(paused()); }
        if self.closing { return Err(input_error("private child input cannot advance after cancellation")); }
        if self.ready { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if declarations.len() > store::CHILD_RESTORE_MAXIMUM_REFERENCES { return Err(input_error("private child factory declarations exceed their fixed bound")); }
        let pointer = declarations.as_ptr() as usize;
        if self.declaration_count.is_some_and(|count| count != declarations.len()) || self.declaration_pointer.is_some_and(|original| original != pointer) { return Err(input_error("private child factory declarations changed their original roster")); }
        self.declaration_count = Some(declarations.len());
        self.declaration_pointer = Some(pointer);
        if self.metadata.is_none() {
            if self.issuer.ready() { self.metadata = self.issuer.take_ready(grant)?; return Ok(structural()); }
            let source = self.source.as_ref().and_then(OwnedChildEmit::metadata).ok_or_else(|| input_error("private child retains its original metadata"))?;
            let expected = source.genesis.as_ref().map(|genesis| &genesis.reference).or(existing).ok_or_else(|| input_error("private child requires its exact declared target"))?;
            return self.issuer.advance(PrivateChildMemberMetadataSource { expected, parent_id, parent_dialect, key: MemberKeyRef { owner: &source.owner, slot: &source.slot, child_id: &source.child_id }, actor, transaction, group_id }, grant).map(input_pending);
        }
        if self.source().is_some_and(|source| source.genesis.is_none()) { self.ready = true; return Ok(structural()); }
        if self.declaration < declarations.len() {
            let parts = self.metadata.as_ref().and_then(PrivateChildMemberMetadata::parts).ok_or_else(|| input_error("private child identity remains owned until request handoff"))?;
            let declaration = declarations[self.declaration];
            if declaration.kind == parts.expected.dialect.artifact_kind && declaration.standard == parts.expected.dialect.standard && declaration.subset == parts.expected.dialect.subset {
                if self.schema.is_some() { return Err(input_error("private child dialect has multiple concrete factory declarations")); }
                self.schema = Some(declaration.schema);
            }
            self.declaration += 1;
            return Ok(structural());
        }
        let schema = self.schema.ok_or_else(|| input_error("private child has no concrete declared factory"))?;
        if !self.framed {
            let source = self.source.as_ref().and_then(OwnedChildEmit::metadata).and_then(|source| source.genesis.as_ref()).ok_or_else(|| input_error("private child genesis remains original"))?;
            let parts = self.metadata.as_ref().and_then(PrivateChildMemberMetadata::parts).ok_or_else(|| input_error("private child input requires retained metadata"))?;
            let step = self.genesis.advance(store::MemberGenesisEnvelopeSource { schema, expected: &parts.expected, owner: &parts.owner, initial_pack: &source.initial_pack }, grant)?;
            if !matches!(step, RetainedCloneStep::Complete(_)) { return Ok(step); }
            let pages = self.genesis.take_ready().ok_or_else(|| input_error("private genesis completion lost its admitted pages"))?;
            let parts = self.metadata.as_mut().and_then(PrivateChildMemberMetadata::parts_mut).unwrap();
            let expected = take_reference(&mut parts.expected);
            let owner = store::OwnerRef { parent: take_reference(&mut parts.owner.parent), slot: std::mem::take(&mut parts.owner.slot), child_id: std::mem::take(&mut parts.owner.child_id) };
            let request = store::MemberOpenRequest::new(operation, generation, expires_at_us, expected, Some(owner), pages, protocol::ActorId(std::mem::take(&mut parts.actor.0)));
            match request.admit(now_us) {
                Ok(request) => *self.request = Some(request),
                Err(rejected) => { *self.request = Some(rejected.request); return Err(input_error("private child input failed exact semantic request admission")); }
            }
            self.framed = true;
            self.genesis.begin_close();
            return Ok(structural());
        }
        if !self.genesis.terminal_is_empty() { return self.genesis.close_granted(grant).map(input_pending); }
        self.ready = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }
    /// 🧳️ Hands off only original ready owners; opening remains a separately funded factory phase.
    pub(crate) fn take_ready(&mut self, grant: RetainedCloneGrant) -> Option<PrivateChildPublicationInputParts> {
        if grant.maximum_items == 0 || !self.ready || self.closing || !self.genesis.terminal_is_empty() { return None; }
        Some(PrivateChildPublicationInputParts { source: self.source.take()?, metadata: self.metadata.take()?, request: self.request.take() })
    }
    pub(crate) fn next_close_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(request) = self.request.as_ref() { return Ok(request.next_close_byte_demand()); }
        if !self.genesis.terminal_is_empty() { return Ok(self.genesis.next_close_byte_demand()); }
        if let Some(metadata) = self.metadata.as_ref() { return Ok(metadata.next_close_byte_demand()); }
        if !self.issuer.terminal_is_empty() { return Ok(self.issuer.next_close_byte_demand()); }
        self.source.as_ref().map_or(Ok(0), OwnedChildEmit::next_close_byte_demand)
    }
    /// ♻️ Retires the exact retained request before metadata, frame, and original typed source owners.
    pub(crate) fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let grant = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), maximum_copy_bytes: grant.maximum_copy_bytes.min(64), ..grant };
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.closing = true;
        if let Some(request) = self.request.as_mut() {
            if request.terminal_is_empty() { self.request.take(); return Ok(structural()); }
            return request.close_step(1, grant.maximum_release_bytes).map(snapshot_progress);
        }
        if !self.genesis.terminal_is_empty() { self.genesis.begin_close(); return self.genesis.close_granted(grant).map(input_pending); }
        if let Some(metadata) = self.metadata.as_mut() {
            let step = metadata.close_granted(grant)?;
            if metadata.terminal_is_empty() { self.metadata = None; }
            return Ok(input_pending(step));
        }
        if !self.issuer.terminal_is_empty() { return self.issuer.close_granted(grant).map(input_pending); }
        if let Some(source) = self.source.as_mut() {
            let step = source.close_granted(grant)?;
            if source.terminal_is_empty() { self.source.take(); }
            return Ok(if self.terminal_is_empty() { step } else { input_pending(step) });
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.source.is_none() && self.metadata.is_none() && self.request.is_none() && self.issuer.terminal_is_empty() && self.genesis.terminal_is_empty() }
}
impl Drop for PrivateChildPublicationInput { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "private child input retains each exact original owner until guarded handoff or bounded close"); } }
fn structural() -> RetainedCloneStep { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }) }
fn input_error(message: &str) -> ValueError { ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, message) }
fn take_reference(value: &mut ArtifactRef) -> ArtifactRef { ArtifactRef { artifact_id: std::mem::take(&mut value.artifact_id), dialect: ArtifactDialect { artifact_kind: std::mem::take(&mut value.dialect.artifact_kind), standard: std::mem::take(&mut value.dialect.standard), subset: std::mem::take(&mut value.dialect.subset) } } }
fn snapshot_progress(step: store::SnapshotRetirementStep) -> RetainedCloneStep { match step { store::SnapshotRetirementStep::Complete => structural(), store::SnapshotRetirementStep::Pending { released_items, released_bytes } => RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, released_bytes, ..Default::default() }), store::SnapshotRetirementStep::Blocked => RetainedCloneStep::Progress(Default::default()) } }

fn input_pending(step: RetainedCloneStep) -> RetainedCloneStep { match step { RetainedCloneStep::Complete(progress) => RetainedCloneStep::Progress(progress), progress => progress } }
