//! 🗄️ Document archive load: the archive load state and phase, the pending archive member, the member identity helpers and the active archive load with its drop guard. See [`super`].

use super::*;

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveDocumentArchiveLoadState {
    Pending,
    Running,
    Ready,
    Cancelled,
    Fault,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveDocumentArchiveLoadPhase {
    DecodeParent,
    MergeParent,
    HydrateParent,
    AwaitingMembers,
    BeginMember,
    FillMember,
    RetireMemberSource,
    AdmitMember,
    SealMembers,
    AwaitReplacement,
    Terminal,
}

/// 📜️ The History read policy of a whole-document archive: one parent SPR within the archive ceiling, one member envelope page directory per record.
const DOCUMENT_ARCHIVE_HISTORY_LIMITS: store::os_store::HistoryReadLimits = store::os_store::HistoryReadLimits {
    file_bytes: protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES as u32,
    frame_body_bytes: (store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES * 256) as u32,
    records: (protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS * 8) as u32,
    maximum_fields: 128,
    maximum_text_bytes: 1_048_576,
    maximum_binary_bytes: 1_048_576,
    maximum_json_bytes: 1_048_576,
    maximum_json_allocation_bytes: 8_388_608,
    maximum_json_items: 65_536,
    maximum_json_depth: 64,
};

/// 🧮️ The cumulative native allocation ceiling of one archive parent History decode.
const DOCUMENT_ARCHIVE_HISTORY_ALLOCATION_BYTES: usize = 8_388_608;

/// 📥️ Admits the retained parent History decoder over the whole of an archive's parent SPR bytes.
pub fn document_archive_history_decoder(parent_spr_bytes: usize) -> Result<protocol::RetainedHistoryDecode, ValueError> {
    protocol::RetainedHistoryDecode::new((0, parent_spr_bytes), DOCUMENT_ARCHIVE_HISTORY_LIMITS, true, DOCUMENT_ARCHIVE_HISTORY_ALLOCATION_BYTES)
}

#[doc(hidden)]
pub struct PendingDocumentArchiveMember {
    pub entry: Option<protocol::OwnedDocumentMemberPackEntry>,
    entry_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub pages: Option<store::os_store::owned_schema_decode::OwnedSchemaDecodePages>,
    pub copied: usize,
    pub identity_field: u8,
}

impl PendingDocumentArchiveMember {
    fn empty_ingress() -> OwnedDocumentMemberIngress {
        OwnedDocumentMemberIngress { identity: ManuallyDrop::new(None), request: ManuallyDrop::new(None), identity_field: 0,last_work:0 }
    }

    pub fn new(entry: protocol::OwnedDocumentMemberPackEntry) -> Result<Self, (Fault, protocol::OwnedDocumentMemberPackEntry)> {
        let page_count = entry.envelope_pack.len().div_ceil(store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES);
        let pages = match store::os_store::owned_schema_decode::OwnedSchemaDecodePages::try_with_credits(store::os_store::owned_schema_decode::OwnedSchemaDecodeCredits { maximum_pages: page_count, maximum_bytes: entry.envelope_pack.len() }) {
            Ok(pages) => pages,
            Err(fault) => return Err((document_load_fault(DOCUMENT_LOAD_FAILED_CODE, format!("document archive member page authority was rejected: {fault:?}")), entry)),
        };
        Ok(Self { entry: Some(entry), entry_retirement: None, pages: Some(pages), copied: 0, identity_field: 0 })
    }

    pub fn fill_one_page(&mut self, maximum_bytes: usize) -> Result<bool, Fault> {
        let entry = self.entry.as_ref().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member source owner is absent"))?;
        if self.copied == entry.envelope_pack.len() {
            return Ok(true);
        }
        let end = self.copied.saturating_add(store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES).min(entry.envelope_pack.len());
        let length = end - self.copied;
        if length > maximum_bytes {
            return Ok(false);
        }
        let page = store::os_store::owned_schema_decode::OwnedSchemaDecodePage::try_from_slice(&entry.envelope_pack[self.copied..end]).map_err(|_| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member page exceeded its fixed extent"))?;
        self.pages.as_mut().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member page owner is absent"))?.admit_page(page).map_err(|_| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member page was rejected by its exact authority"))?;
        self.copied = end;
        Ok(self.copied == entry.envelope_pack.len())
    }

    /// 📏️ The source bytes still owned: payload truncation, then their whole physical backing.
    pub fn source_retirement_demands(&self) -> RetirementDemand {
        self.entry.as_ref().map_or_else(RetirementDemand::default, |entry| plugin_bytes_retirement_demand(&entry.envelope_pack))
    }

    pub fn retire_source_step(&mut self, grant: RetainedCloneGrant) -> RetainedCloneStep {
        let empty = RetainedCloneProgress::default();
        let Some(entry) = self.entry.as_mut() else { return RetainedCloneStep::Complete(empty) };
        match plugin_bytes_close(&mut entry.envelope_pack, grant) {
            Some(progress) => RetainedCloneStep::Progress(progress),
            None => RetainedCloneStep::Complete(empty),
        }
    }
    pub fn take_ingress(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, actor: ActorId) -> Result<OwnedDocumentMemberIngress, (Fault, OwnedDocumentMemberIngress)> {
        let mut entry = self.entry.take().ok_or_else(|| (document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member identity owner is absent"), Self::empty_ingress()))?;
        if !entry.envelope_pack.is_empty() {
            self.entry = Some(entry);
            return Err((document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member source bytes remain live before identity transfer"), Self::empty_ingress()));
        }
        let mut pages = self.pages.take().ok_or_else(|| (document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive member page owner is absent"), Self::empty_ingress()))?;
        if let Err(fault) = pages.seal() {
            self.entry = Some(entry);
            self.pages = Some(pages);
            return Err((document_load_fault(DOCUMENT_LOAD_FAILED_CODE, format!("document archive member pages could not seal: {fault:?}")), Self::empty_ingress()));
        }
        let reference = ArtifactRef {
            artifact_id: std::mem::take(&mut entry.reference.artifact_id),
            dialect: ArtifactDialect { artifact_kind: std::mem::take(&mut entry.reference.artifact_kind), standard: std::mem::take(&mut entry.reference.standard), subset: std::mem::take(&mut entry.reference.subset) },
        };
        let owner = store::OwnerRef {
            parent: ArtifactRef {
                artifact_id: std::mem::take(&mut entry.owner.parent.artifact_id),
                dialect: ArtifactDialect { artifact_kind: std::mem::take(&mut entry.owner.parent.artifact_kind), standard: std::mem::take(&mut entry.owner.parent.standard), subset: std::mem::take(&mut entry.owner.parent.subset) },
            },
            slot: std::mem::take(&mut entry.owner.slot),
            child_id: std::mem::take(&mut entry.owner.child_id),
        };
        let ordinal = entry.ordinal as usize;
        drop(entry);
        let request = match store::MemberOpenRequest::new(handle.operation, handle.generation, u64::MAX, reference.clone(), Some(owner.clone()), pages, actor).admit(1) {
            Ok(request) => request,
            Err(rejected) => {
                let ingress = OwnedDocumentMemberIngress { identity: ManuallyDrop::new(Some((ordinal, reference, owner))), request: ManuallyDrop::new(Some(rejected.request)), identity_field: 0,last_work:0 };
                return Err((document_load_fault(DOCUMENT_LOAD_FAILED_CODE, format!("document archive member request was rejected: {:?}", rejected.diagnostic)), ingress));
            }
        };
        OwnedDocumentMemberIngress::try_new(ordinal, reference, owner, request)
    }

    /// 📏️ Borrows original member and whole page-directory demands before any transfer.
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(pages)=self.pages.as_ref() {
            return Ok(if let Some(bytes)=pages.close_page_bytes() { RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()} }
                else { RetirementDemand{release_bytes:pages.allocation_byte_demand(),depth:1,..Default::default()} });
        }
        if let Some(owner)=self.entry_retirement.as_ref() {
            let mut demand=store::os_store::snapshot_read::artifact_retirement_box_demands(owner,body)?;
            demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"archive member retirement parent depth overflow"))?;
            return Ok(demand);
        }
        Ok(if self.entry.is_some() { RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<protocol::OwnedDocumentMemberPackEntry>(),depth:2,..Default::default()} } else { Default::default() })
    }

    /// 🧹️ Pays inline payload work and releases each original physical allocation separately.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome {
        let mut work=0;
        let result=(||{
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty));}
        if let Some(pages)=self.pages.as_mut() {
            if let Some(page)=pages.close_take_page(){work=1;let bytes=page.len();drop(page);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..empty}));}
            if !pages.terminal_is_empty(){let result=pages.close_backing_step(grant);work=pages.last_backing_close_work();return result.map(|step|RetainedCloneStep::Progress(step.progress()));}
            work=1;drop(self.pages.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));
        }
        let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
        if self.entry_retirement.is_some(){let turn=store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.entry_retirement,child);work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress()));}
        let original=self.entry.take().expect("quoted original archive member");
        let turn=semio_framework_value::retirement::admit_owned_retirement(original,child);work=turn.work;match turn.result {
            Ok((owner,progress))=>{self.entry_retirement=Some(owner);semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original archive member birth")?;Ok(RetainedCloneStep::Progress(progress))}
            Err((error,original))=>{self.entry=Some(original);Err(error)}
        }
    })();
        semio_framework_value::RetainedWorkOutcome{result,work}
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.entry.is_none() && self.entry_retirement.is_none() && self.pages.is_none()
    }
}

impl Drop for PendingDocumentArchiveMember {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "recursive document archive member reached Drop before its source and page owners were terminal empty");
    }
}

fn document_archive_member_identity(entry: &protocol::OwnedDocumentMemberPackEntry, field: u8) -> Option<&String> {
    match field {
        0 => Some(&entry.reference.artifact_id),
        1 => Some(&entry.reference.artifact_kind),
        2 => Some(&entry.reference.standard),
        3 => Some(&entry.reference.subset),
        4 => Some(&entry.owner.parent.artifact_id),
        5 => Some(&entry.owner.parent.artifact_kind),
        6 => Some(&entry.owner.parent.standard),
        7 => Some(&entry.owner.parent.subset),
        8 => Some(&entry.owner.slot),
        9 => Some(&entry.owner.child_id),
        _ => None,
    }
}

fn document_archive_member_identity_mut(entry: &mut protocol::OwnedDocumentMemberPackEntry, field: u8) -> Option<&mut String> {
    match field {
        0 => Some(&mut entry.reference.artifact_id),
        1 => Some(&mut entry.reference.artifact_kind),
        2 => Some(&mut entry.reference.standard),
        3 => Some(&mut entry.reference.subset),
        4 => Some(&mut entry.owner.parent.artifact_id),
        5 => Some(&mut entry.owner.parent.artifact_kind),
        6 => Some(&mut entry.owner.parent.standard),
        7 => Some(&mut entry.owner.parent.subset),
        8 => Some(&mut entry.owner.slot),
        9 => Some(&mut entry.owner.child_id),
        _ => None,
    }
}

#[doc(hidden)]
pub struct ActiveDocumentArchiveLoad<P, Mutation>
where
    P: ::semio_os_kernel::StoreState,
    Mutation: ::semio_os_kernel::StoreMutationFor<P>,
{
    pub operation: u64,
    pub archive: Option<Box<protocol::DocumentArchivePack>>,
    archive_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub history: Option<protocol::RetainedHistoryDecode>,
    history_custody: Option<protocol::HistoryLog>,
    pub decoded_history: Option<protocol::HistoryLog>,
    pub member: Option<PendingDocumentArchiveMember>,
    pub rejected_ingress: Option<OwnedDocumentMemberIngress>,
    pub retained: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub hydration: Option<store::RetainedPersistedDocumentHydration<P, Mutation>>,
    pub(crate) hydration_sequence: u64,
    pub replacement: Option<ArtifactEnvelopeDecodeOperationHandle>,
    pub genesis_complete: bool,
    pub phase: ActiveDocumentArchiveLoadPhase,
    pub terminal_target: Option<ActiveDocumentArchiveLoadState>,
    close_identity_field: u8,
    last_native_work:u64,
    pub state: ActiveDocumentArchiveLoadState,
    pub completed: u64,
    pub total: u64,
    pub fold: (u64, u64),
    pub decoded: u64,
    pub merge: bool,
    pub ahead: u64,
    pub fault: Vec<u8>,
}

impl<P, Mutation> ActiveDocumentArchiveLoad<P, Mutation>
where
    P: ArtifactPack + ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot + ::semio_os_kernel::StoreState,
    Mutation: ::semio_os_kernel::StoreMutationFor<P>,
{
    pub fn new(operation: u64, archive: protocol::DocumentArchivePack, history: protocol::RetainedHistoryDecode) -> Self {
        let total = archive.members.len() as u64 + 1;
        Self {
            operation,
            archive: Some(Box::new(archive)),
            archive_retirement: None,
            history: Some(history),
            history_custody: None,
            decoded_history: None,
            member: None,
            rejected_ingress: None,
            retained: None,
            hydration: None,
            hydration_sequence: 0,
            replacement: None,
            genesis_complete: false,
            phase: ActiveDocumentArchiveLoadPhase::DecodeParent,
            terminal_target: None,
            close_identity_field: 0,
            last_native_work:0,
            state: ActiveDocumentArchiveLoadState::Pending,
            completed: 0,
            total,
            fold: (0, 0),
            decoded: 0,
            merge: false,
            ahead: 0,
            fault: Vec::new(),
        }
    }

    pub fn status(&self) -> protocol::DocumentArchiveLoadStatus {
        let state = match self.state {
            ActiveDocumentArchiveLoadState::Pending => protocol::DocumentArchiveLoadState::Pending,
            ActiveDocumentArchiveLoadState::Running => protocol::DocumentArchiveLoadState::Running,
            ActiveDocumentArchiveLoadState::Ready => protocol::DocumentArchiveLoadState::Ready,
            ActiveDocumentArchiveLoadState::Cancelled => protocol::DocumentArchiveLoadState::Cancelled,
            ActiveDocumentArchiveLoadState::Fault => protocol::DocumentArchiveLoadState::Fault,
        };
        protocol::DocumentArchiveLoadStatus { operation: self.operation, state, completed: self.completed.saturating_add(self.fold.0), total: self.total.saturating_add(self.fold.1), ahead: self.ahead, fault: self.fault.clone() }
    }

    pub fn terminal(&self) -> bool {
        matches!(self.state, ActiveDocumentArchiveLoadState::Ready | ActiveDocumentArchiveLoadState::Cancelled | ActiveDocumentArchiveLoadState::Fault)
    }

    /// ⏳️ Whether the document is still being loaded: not once the load ended, not while a cancelled, faulted or
    /// committed load only retires what it held — the document is then already the one it will stay — and never for a
    /// read-back merge, which replaces nothing.
    pub fn loading(&self) -> bool {
        !self.merge && !self.terminal() && self.terminal_target.is_none()
    }

    /// 🫙️ Every original input owner is returned; only the encoded fault diagnostic the terminal Fault status carries may remain.
    pub fn inputs_are_empty(&self) -> bool {
        self.archive.is_none() && self.archive_retirement.is_none() && self.history.is_none() && self.history_custody.is_none() && self.decoded_history.is_none() && self.member.is_none() && self.rejected_ingress.is_none() && self.retained.is_none() && self.hydration.is_none() && self.replacement.is_none()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.inputs_are_empty() && self.fault.capacity() == 0
    }

    pub fn request_terminal(&mut self, target: ActiveDocumentArchiveLoadState) {
        if self.terminal_target.is_none() {
            self.terminal_target = Some(target);
        }
    }

    /// 🔀️ Whether a read-back merge decoded its log and waits for the host's poll turn to merge it.
    pub fn awaits_merge(&self) -> bool {
        self.phase == ActiveDocumentArchiveLoadPhase::MergeParent && self.terminal_target.is_none()
    }

    pub fn request_fault(&mut self, fault: &Fault) {
        if self.terminal_target.is_none() {
            self.fault = semio_framework_diagnostic::encode_fault_bytes(fault);
            self.terminal_target = Some(ActiveDocumentArchiveLoadState::Fault);
        }
    }
    /// 📏️ Quotes complete original child currency before history or archive ownership moves.
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let parent=|mut demand:RetirementDemand|->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"archive retirement parent depth overflow"))?;Ok(demand)};
        if self.replacement.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"archive replacement awaits its original operation recipient"));}
        if let Some(owner)=self.retained.as_ref(){return parent(store::os_store::snapshot_read::artifact_retirement_box_demands(owner,body)?);}
        if let Some(owner)=self.archive_retirement.as_ref(){return parent(store::os_store::snapshot_read::artifact_retirement_box_demands(owner,body)?);}
        if let Some(hydration)=self.hydration.as_ref(){
            if store::ErasedSnapshotRetirement::terminal_is_empty(hydration){return Ok(RetirementDemand{depth:1,..Default::default()});}
            return parent(RetirementDemand{copy_bytes:store::ErasedSnapshotRetirement::next_copy_byte_demand(hydration)?,capacity_bytes:store::ErasedSnapshotRetirement::next_capacity_byte_demand(hydration,body)?,release_bytes:store::ErasedSnapshotRetirement::next_release_byte_demand(hydration)?,depth:store::ErasedSnapshotRetirement::next_depth_demand(hydration)?});
        }
        if let Some(ingress)=self.rejected_ingress.as_ref(){return if ingress.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{parent(ingress.retirement_demands(body)?)};}
        if let Some(member)=self.member.as_ref(){return if member.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{parent(member.retirement_demands(body)?)};}
        if let Some(history)=self.history.as_ref(){return parent(history.demands(body)?);}
        if self.history_custody.is_some()||self.decoded_history.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<protocol::HistoryLog>(),depth:2,..Default::default()});}
        if self.archive.is_some(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<Box<protocol::DocumentArchivePack>>(),depth:2,..Default::default()});}
        Ok(Default::default())
    }

    /// 🎟️ Retains all original history and physical archive backing until full admission succeeds.
    pub fn close_step(&mut self, grant: RetainedCloneGrant,recipient:&mut dyn semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient) -> semio_framework_value::RetainedWorkOutcome<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure> {
        self.last_native_work=0;let mut work=0;let result:Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>=(||{
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty));}
        let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
        if self.retained.is_some(){let turn=store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retained,child);work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into);}
        if self.archive_retirement.is_some(){let turn=store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.archive_retirement,child);work=turn.work;return turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into);}
        if let Some(hydration)=self.hydration.as_mut(){
            if store::ErasedSnapshotRetirement::terminal_is_empty(hydration){work=1;drop(self.hydration.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
            let turn=store::ErasedSnapshotRetirement::close_step(hydration,child);work=turn.work;let step=turn.result?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child,step,store::ErasedSnapshotRetirement::terminal_is_empty(hydration),"original archive hydration")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(ingress)=self.rejected_ingress.as_mut(){
            if ingress.terminal_is_empty(){work=1;drop(self.rejected_ingress.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
            let result=ingress.close_step(child);work=ingress.last_turn_work();let step=result?;semio_framework_value::retained_clone::admit_retained_clone_close(child,step,ingress.terminal_is_empty(),"original archive ingress")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(member)=self.member.as_mut(){
            if member.terminal_is_empty(){work=1;drop(self.member.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
            let turn=member.close_step(child);work=turn.work;let step=turn.result?;semio_framework_value::retained_clone::admit_retained_clone_close(child,step,member.terminal_is_empty(),"original archive pending member")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(history)=self.history.as_mut(){
            if history.terminal_is_empty(){work=1;drop(self.history.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
            let source=&self.archive.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"archive history decoder lost its original source"))?.parent_spr;
            let turn=history.close_step(source,&protocol::RetainedHistoryUninterrupted,child,recipient);work=turn.work;self.last_native_work=history.last_turn_native_work();let step=turn.result?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child,step,history.terminal_is_empty(),"original archive history decoder")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.history_custody.is_none(){self.history_custody=self.decoded_history.take();}
        if let Some(original)=self.history_custody.take(){
            let turn=semio_framework_value::retirement::admit_owned_retirement(original,child);work=turn.work;match turn.result {
                Ok((owner,progress))=>{self.retained=Some(owner);semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original archive history birth")?;return Ok(RetainedCloneStep::Progress(progress));}
                Err((error,original))=>{self.history_custody=Some(original);return Err(error.into());}
            }
        }
        if let Some(original)=self.archive.take(){
            let turn=semio_framework_value::retirement::admit_owned_retirement(original,child);work=turn.work;return match turn.result {
                Ok((owner,progress))=>{self.archive_retirement=Some(owner);semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original physical archive birth")?;Ok(RetainedCloneStep::Progress(progress))}
                Err((error,original))=>{self.archive=Some(original);Err(error.into())}
            };
        }
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"archive close lost its quoted original frontier").into())
    })();semio_framework_value::RetainedWorkOutcome{result,work}
    }

    /// 🧮️ Borrows actual native decoder work independently of delegated reported work.
    pub fn last_turn_native_work(&self)->u64{self.last_native_work}

}

impl<P, Mutation> Drop for ActiveDocumentArchiveLoad<P, Mutation>
where
    P: ::semio_os_kernel::StoreState,
    Mutation: ::semio_os_kernel::StoreMutationFor<P>,
{
    fn drop(&mut self) {
        assert!(
            std::thread::panicking()
                || (self.archive.is_none()
                    && self.archive_retirement.is_none()
                    && self.history.is_none()
                    && self.history_custody.is_none()
                    && self.decoded_history.is_none()
                    && self.member.is_none()
                    && self.rejected_ingress.is_none()
                    && self.retained.is_none()
                    && self.hydration.is_none()
                    && self.replacement.is_none()
                    && self.fault.capacity() == 0),
            "recursive document archive operation reached Drop before its retained input owner was terminal empty",
        );
    }
}

/// 🧭️ Retains the exact borrowed archive and its bounded validation frontier before registry transfer.
#[derive(Clone,Copy,Default,semio_framework_value::RetireOwned)]
pub struct OriginalDocumentArchiveAdmission{source:Option<(u64,u64,u64)>,donor:usize,operation:u64,members:usize,phase:u8,row:usize,search:usize,field:usize,offset:usize,payload:usize,ready:bool}
impl OriginalDocumentArchiveAdmission{
 pub const fn empty()->Self{Self{source:None,donor:0,operation:0,members:0,phase:0,row:0,search:0,field:0,offset:0,payload:0,ready:false}}
 pub fn completed(&self)->bool{self.phase==6}
 pub fn validate_step(&mut self,operation:u64,archive:&mut protocol::DocumentArchivePack,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  let identity=(cx.operation().0,cx.generation().0,cx.original_source_parent()?.source_identity()?);let donor=archive as*mut _ as usize;
  if self.source.is_some_and(|source|source!=identity)||self.source.is_some()&&(self.operation!=operation||self.donor!=donor||self.phase!=6&&self.members!=archive.members.len()){return Err(archive_admission_fault("archive admission requires its unchanged Source and donor").into())}
  if self.phase==6{return if archive.parent_pack.is_empty()&&archive.parent_spr.is_empty()&&archive.members.is_empty(){Ok(true)}else{Err(archive_admission_fault("transferred archive donor must remain empty").into())}}if cx.is_cancelled(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"archive admission observed cancellation").into())}
  if !archive_admission_charge(cx,std::mem::size_of::<Self>())?{return Ok(false)}
  if self.source.is_none(){if archive.members.len()>protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS||archive.parent_pack.is_empty()||archive.parent_spr.is_empty(){return Err(archive_admission_fault("archive count and parent pair must satisfy the declared archive bounds").into())}self.source=Some(identity);self.donor=donor;self.operation=operation;self.members=archive.members.len();self.payload=archive.parent_pack.len().checked_add(archive.parent_spr.len()).ok_or_else(||archive_admission_fault("archive byte count overflow"))?;self.phase=1;return Ok(false)}
  match self.phase{
   1=>{if self.row<self.members{self.payload=self.payload.checked_add(archive.members[self.row].envelope_pack.len()).ok_or_else(||archive_admission_fault("archive byte count overflow"))?;self.row+=1;}else{if self.payload>protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES{return Err(archive_admission_fault("archive exceeds its declared byte bound").into())}self.row=0;self.search=0;self.phase=2;}},
   2=>{if self.row==self.members{self.row=0;self.field=0;self.offset=0;self.phase=3;}else if self.search==self.members{return Err(archive_admission_fault("archive member ordinals must be unique and contiguous").into())}else if archive.members[self.search].ordinal as usize==self.row{let bytes=std::mem::size_of::<protocol::OwnedDocumentMemberPackEntry>()*2;if !archive_admission_charge(cx,bytes)?{return Ok(false)}archive.members.swap(self.row,self.search);self.row+=1;self.search=self.row;}else{self.search+=1;}},
   3=>{if self.row==self.members{self.row=0;self.phase=4;}else{let entry=&archive.members[self.row];if entry.envelope_pack.is_empty(){return Err(archive_admission_fault("archive member envelope must be present").into())}let fields=[entry.reference.artifact_id.as_str(),entry.reference.artifact_kind.as_str(),entry.reference.standard.as_str(),entry.reference.subset.as_str(),entry.owner.parent.artifact_id.as_str(),entry.owner.parent.artifact_kind.as_str(),entry.owner.parent.standard.as_str(),entry.owner.parent.subset.as_str(),entry.owner.slot.as_str(),entry.owner.child_id.as_str()];let field=fields[self.field];if field.is_empty()||field.len()>store::MEMBER_OPEN_IDENTITY_BYTES{return Err(archive_admission_fault("archive member identity exceeds its declared bounds").into())}while self.offset<field.len()&&!cx.should_yield(){let character=field[self.offset..].chars().next().ok_or_else(||archive_admission_fault("archive member identity lost its character frontier"))?;cx.consume_fuel(1);if character.is_control(){return Err(archive_admission_fault("archive member identity contains a control character").into())}self.offset+=character.len_utf8();}if self.offset==field.len(){self.offset=0;self.field+=1;if self.field==fields.len(){self.field=0;self.row+=1;}}}},
   4=>{if self.row<self.members/2{if !archive_admission_charge(cx,std::mem::size_of::<protocol::OwnedDocumentMemberPackEntry>()*2)?{return Ok(false)}archive.members.swap(self.row,self.members-1-self.row);self.row+=1;}else{self.phase=5;self.ready=true;}},
   5=>return Ok(self.ready),
   _=>return Err(archive_admission_fault("archive admission frontier is invalid").into())
  }Ok(false)
 }
 pub fn transfer_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,transfer:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if self.phase!=5||!self.ready{return Err(archive_admission_fault("archive transfer requires completed validation").into())}if !archive_admission_charge(cx,std::mem::size_of::<Self>())?{return Ok(false)}transfer(cx)?;self.phase=6;self.ready=false;Ok(true)}
}
fn archive_admission_charge(cx:&mut semio_framework_job::StepContext<'_>,bytes:usize)->Result<bool,semio_framework_job::OriginalTurnFailure>{let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
fn archive_admission_fault(message:&'static str)->ValueError{ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,message)}

#[cfg(test)]
mod original_archive_admission_laws{
 use super::*;use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};use semio_framework_value::{RetainedCloneGrant,ValueRefusalKind};fn clock()->Option<u64>{Some(1000)}
 #[test]
 fn borrowed_archive_keeps_its_actual_donor_on_denial_foreign_source_cancel_and_failed_transfer(){
  let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:65536,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(9801),Generation(1),grant,clock,1000000,1000000);let mut donor=protocol::DocumentArchivePack{parent_pack:vec![1],parent_spr:vec![2],members:Vec::new()};let mut cursor=OriginalDocumentArchiveAdmission::empty();
  let denied=RetainedCloneGrant{maximum_copy_bytes:0,..grant};assert!(!caller.turn(denied,|cx|cursor.validate_step(17,&mut donor,cx)).unwrap());assert!(cursor.source.is_none());assert_eq!(caller.receipt(),Default::default());
  let mut ready=false;for _ in 0..32{ready=caller.turn(grant,|cx|cursor.validate_step(17,&mut donor,cx)).unwrap();if ready{break}}assert!(ready);assert_eq!(donor.parent_pack,[1]);assert_eq!(donor.parent_spr,[2]);
  let mut foreign=OriginalJobFixtureCaller::admit(OperationId(9801),Generation(1),grant,clock,1000000,1000000);assert!(foreign.turn(grant,|cx|cursor.validate_step(17,&mut donor,cx)).is_err());assert_eq!(foreign.receipt(),Default::default());foreign.retire(grant);
  assert!(caller.turn(grant,|cx|cursor.transfer_step(cx,|_|Err(ValueError::literal(ValueRefusalKind::WorkLimit,"retained recipient refused").into()))).is_err());assert!(!cursor.completed());assert_eq!(donor.parent_pack,[1]);assert_eq!(donor.parent_spr,[2]);
  caller.cancel().cancel_now();assert!(caller.turn(grant,|cx|cursor.validate_step(17,&mut donor,cx)).is_err());assert!(!cursor.completed());assert_eq!(donor.parent_pack,[1]);caller.retire(grant);
 }
 #[test]
 fn actual_source_validation_matches_the_portable_ordinal_corpus_without_payload_allocation(){
  let law:serde_json::Value=serde_json::from_str(include_str!("admission.json")).unwrap();let grant=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:65536,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
  for row in law["cases"].as_array().unwrap(){
   let reference=||protocol::DocumentArchiveArtifactRef{artifact_id:"id".into(),artifact_kind:"kind".into(),standard:"standard".into(),subset:"subset".into()};
   let members=row["ordinals"].as_array().unwrap().iter().map(|value|protocol::OwnedDocumentMemberPackEntry{ordinal:value.as_u64().unwrap()as u32,reference:reference(),owner:protocol::DocumentArchiveOwnerRef{parent:reference(),slot:"slot".into(),child_id:"child".into()},envelope_pack:vec![3]}).collect();
   let mut donor=protocol::DocumentArchivePack{parent_pack:vec![1],parent_spr:vec![2],members};let mut cursor=OriginalDocumentArchiveAdmission::empty();let mut caller=OriginalJobFixtureCaller::admit(OperationId(9802),Generation(1),grant,clock,1000000,1000000);let mut accepted=false;let mut refused=false;
   for _ in 0..4096{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|cursor.validate_step(18,&mut donor,cx)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));match result{Ok(true)=>{accepted=true;break},Ok(false)=>{},Err(_)=>{refused=true;break}}}
   assert_eq!(accepted,row["accepted"].as_bool().unwrap());if accepted{assert_eq!(donor.members.iter().map(|member|u64::from(member.ordinal)).collect::<Vec<_>>(),row["expected"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()).collect::<Vec<_>>());}else{assert!(refused)}assert_eq!(donor.parent_pack,[1]);assert_eq!(donor.parent_spr,[2]);caller.retire(grant);
  }
 }

}
