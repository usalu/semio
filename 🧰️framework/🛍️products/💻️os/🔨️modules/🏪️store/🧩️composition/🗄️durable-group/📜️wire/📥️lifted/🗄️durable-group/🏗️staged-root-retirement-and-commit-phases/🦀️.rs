//! 🏗️ Staged root retirement and commit phases: the staged root retirement, the retained stacks, the three store commit phases and advances, the map recovery and the map assembly phases. See [`super`].

use super::*;
use crate::lifted_durable_group::outcome_and_journal_receipt_types::DurablePlanningOutputV1;

pub struct StagedRootRetirement<P, Mutation> {
        pub planning_output: Option<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>,
    pub diagnostics: Option<semio_framework_os_store_one_item_publication::diagnostics::ArtifactStoreDiagnosticStage>,
pub edit: Option<semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement<Mutation>>,
    pub external_strings: [Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>; 3],
    pub cursor: Option<semio_framework_os_store_read::ArtifactStoreCursorRetirement>,
    pub applied: Option<semio_framework_os_store_read::ArtifactStoreStringVectorRetirement>,
    pub redo: Option<semio_framework_os_store_read::ArtifactStoreStringVectorRetirement>,
    pub revision: Option<semio_framework_os_store_read::ArtifactStoreRevisionAccumulatorRetirement>,
    pub report: Option<semio_framework_os_store_ledger::ArtifactStorePendingReportRetirement>,
    pub tail_id: Option<semio_framework_os_store_read::ArtifactStoreStringRetirement>,
    pub authority: Option<semio_framework_os_store_one_item_publication::lifted_canonical_edit::ArtifactStoreOneItemAuthorityRetirement>,
    pub snapshots: [Option<Arc<P>>; 2],
    pub active: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    pub factory: Option<Arc<dyn semio_framework_value::SnapshotRetirementFactory<P>>>,
    pub factory_close: Option<semio_framework_value::FactoryAuthority>,
    pub phase: u8,
}

impl<P, Mutation> StagedRootRetirement<P, Mutation> {
    pub fn new(factory: Arc<dyn semio_framework_value::SnapshotRetirementFactory<P>>) -> Self {
        Self { planning_output: None, diagnostics: None, edit: None, external_strings: Default::default(), cursor: None, applied: None, redo: None, revision: None, report: None, tail_id: None, authority: None, snapshots: [None, None], active: None, factory: Some(factory), factory_close: None, phase: 0 }
    }

    fn inline_owner(&self) -> Option<&dyn semio_framework_value::ErasedSnapshotRetirement> {
        match self.phase {
            0 => self.cursor.as_ref().map(|owner| owner as _),
            1 => self.applied.as_ref().map(|owner| owner as _),
            2 => self.redo.as_ref().map(|owner| owner as _),
            3 => self.revision.as_ref().map(|owner| owner as _),
            4 => self.report.as_ref().map(|owner| owner as _),
            5 => self.tail_id.as_ref().map(|owner| owner as _),
            6 => self.authority.as_ref().map(|owner| owner as _),
            _ => None,
        }
    }

    fn inline_owner_mut(&mut self) -> Option<&mut dyn semio_framework_value::ErasedSnapshotRetirement> {
        match self.phase {
            0 => self.cursor.as_mut().map(|owner| owner as _),
            1 => self.applied.as_mut().map(|owner| owner as _),
            2 => self.redo.as_mut().map(|owner| owner as _),
            3 => self.revision.as_mut().map(|owner| owner as _),
            4 => self.report.as_mut().map(|owner| owner as _),
            5 => self.tail_id.as_mut().map(|owner| owner as _),
            6 => self.authority.as_mut().map(|owner| owner as _),
            _ => None,
        }
    }

    fn clear_inline(&mut self) {
        match self.phase {
            0 => { self.cursor = None; }
            1 => { self.applied = None; }
            2 => { self.redo = None; }
            3 => { self.revision = None; }
            4 => { self.report = None; }
            5 => { self.tail_id = None; }
            6 => { self.authority = None; }
            _ => {}
        }
    }
}

impl<P: Send + Sync, Mutation: Send + 'static> StagedRootRetirement<P, Mutation> {
    pub fn demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
        let nested = |mut demand: RetirementDemand| { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "staged root child depth overflow"))?; Ok(demand) };
                if let Some(owner)=self.planning_output.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<DurablePlanningOutputV1>>(),depth:1,..Default::default()});}return nested(owner.next_demand(maximum_body_bytes)?);}
        if let Some(owner)=self.diagnostics.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_os_store_one_item_publication::diagnostics::ArtifactStoreDiagnosticStage>(),depth:1,..Default::default()});}return nested(owner.close_demands(maximum_body_bytes)?);}
if let Some(active) = self.active.as_ref() { return nested(semio_framework_os_store_read::artifact_retirement_box_demands(active, maximum_body_bytes)?); }
        if self.edit.is_some() { return semio_framework_os_store_read::retirement_owner_demands(&self.edit, maximum_body_bytes); }
        if self.external_strings.iter().any(Option::is_some) { return Ok(RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>(), depth: 2, ..Default::default() }); }
        if let Some(owner) = self.inline_owner() { return nested(owner.next_demand(maximum_body_bytes)?); }
        if (7..9).contains(&self.phase) {
            if let Some(snapshot) = self.snapshots[usize::from(self.phase - 7)].as_ref() { return Ok(RetirementDemand { capacity_bytes: self.factory.as_ref().expect("staged root retains its snapshot factory").retirement_birth_bytes(snapshot), depth: 2, ..Default::default() }); }
        }
        if self.phase >= 9 && self.factory.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(factory) = self.factory_close.as_ref() { return nested(factory.demands(maximum_body_bytes)?); }
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }
}

impl<P: Send + Sync, Mutation: Send + 'static> semio_framework_value::ErasedSnapshotRetirement for StagedRootRetirement<P, Mutation> {
        fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.demands(body) }
fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(maximum_body_bytes)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(0)?.depth) }
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "staged root exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
                if let Some(owner)=self.planning_output.as_mut(){if owner.terminal_is_empty(){self.planning_output=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}return owner.step(child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if let Some(owner)=self.diagnostics.as_mut(){if owner.terminal_is_empty(){self.diagnostics=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}owner.begin_close();return owner.close_step(child).map(|step|RetainedCloneStep::Progress(step.progress()));}
if self.active.is_some() { return semio_framework_os_store_read::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.edit.is_some() { return semio_framework_os_store_read::retirement_owner_close(&mut self.edit, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(slot)=self.external_strings.iter_mut().find(|slot|slot.is_some()){let original=slot.take().unwrap();return match semio_framework_value::retirement::admit_owned_retirement(original,child){Ok((owner,receipt))=>{self.active=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original))=>{*slot=Some(original);Err(error)}};}
        if let Some(owner) = self.inline_owner_mut() {
            if !owner.terminal_is_empty() {
                let step = owner.close_step(child)?;
                return semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owner.terminal_is_empty(), "staged inline owner").map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            self.clear_inline();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if (7..9).contains(&self.phase) {
            let index = usize::from(self.phase - 7);
            if let Some(snapshot) = self.snapshots[index].take() {
                return match self.factory.as_ref().expect("staged root retains its snapshot factory").retire(snapshot, child) {
                    Ok((owner, progress)) => { self.active = Some(owner); semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "staged snapshot constructor")?; Ok(RetainedCloneStep::Progress(progress)) },
                    Err((error, snapshot)) => { self.snapshots[index] = Some(snapshot); Err(error) },
                };
            }
        }
        if self.phase < 9 { self.phase += 1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
        if let Some(factory) = self.factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_close = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), ..empty }));
        }
        if let Some(factory) = self.factory_close.as_mut() {
            let step = factory.step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "staged snapshot factory")?;
            if factory.terminal_is_empty() { self.factory_close = None; }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn terminal_is_empty(&self) -> bool { self.planning_output.is_none() && self.diagnostics.is_none() && self.edit.is_none() && self.external_strings.iter().all(Option::is_none) && self.cursor.is_none() && self.applied.is_none() && self.redo.is_none() && self.revision.is_none() && self.report.is_none() && self.tail_id.is_none() && self.authority.is_none() && self.snapshots.iter().all(Option::is_none) && self.active.is_none() && self.factory.is_none() && self.factory_close.is_none() }
}

impl<P, Mutation> Drop for StagedRootRetirement<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.planning_output.is_none() && self.diagnostics.is_none() && self.edit.is_none() && self.external_strings.iter().all(Option::is_none) && self.cursor.is_none() && self.applied.is_none() && self.redo.is_none() && self.revision.is_none() && self.report.is_none() && self.tail_id.is_none() && self.authority.is_none() && self.snapshots.iter().all(Option::is_none) && self.active.is_none() && self.factory.is_none() && self.factory_close.is_none()), "staged root retirement dropped before all exact outgoing owners were closed");
    }
}

pub fn retained_string_stack(source: impl IntoIterator<Item = String>, appended: Option<&str>) -> Result<semio_framework_os_vcs::HistoryPageStack<String>, DurableOwnedGroupDecisionError> {
    let mut values = semio_framework_os_vcs::HistoryPageStack::try_new().map_err(|error| DurableOwnedGroupDecisionError::Codec(error.into()))?;
    for value in source {
        values.try_push(value).map_err(|_| DurableOwnedGroupDecisionError::Codec("history catalog address space exhausted".into()))?;
    }
    if let Some(value) = appended {
        values.try_push(value.to_string()).map_err(|_| DurableOwnedGroupDecisionError::Codec("history catalog address space exhausted".into()))?;
    }
    Ok(values)
}

pub fn retained_revision_stack(source: impl IntoIterator<Item = semio_framework_os_store_ledger::CursorRevisionRecord>) -> Result<semio_framework_os_vcs::HistoryPageStack<semio_framework_os_store_ledger::CursorRevisionRecord>, DurableOwnedGroupDecisionError> {
    let mut values = semio_framework_os_vcs::HistoryPageStack::try_new().map_err(|error| DurableOwnedGroupDecisionError::Codec(error.into()))?;
    for value in source {
        values.try_push(value).map_err(|_| DurableOwnedGroupDecisionError::Codec("history catalog address space exhausted".into()))?;
    }
    Ok(values)
}

pub fn retain_displaced_owner(store: &mut semio_framework_os_store_read::ArtifactStoreDisplacedRetirements, reservation: &mut semio_framework_os_store_read::ArtifactStoreDisplacedOwnerReservation, owner: Box<dyn semio_framework_value::ErasedSnapshotRetirement>) {
    if let Err(owner) = store.push_owner_reserved(reservation, owner) {
        store.push_reserved(owner);
    }
}

/// 🚦️ Observable retained commit phase; no phase after `Journal` can regain abort authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DurableOwnedThreeStoreCommitPhaseV1 {
    StagingParent,
    StagingDrawing,
    StagingValue,
    StartingJournal,
    Journal,
    PublishingParentLease,
    PublishingDrawingLease,
    PublishingValueLease,
    AdoptingParent,
    AdoptingDrawing,
    AdoptingValue,
    ClearingParent,
    ClearingDrawing,
    ClearingValue,
    AwaitingAck,
    AbortingValue,
    AbortingDrawing,
    AbortingParent,
    ClosingJournal,
    Complete,
}

/// 📡️ One bounded coordinator result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DurableOwnedThreeStoreCommitAdvanceV1 {
    Progress(DurableOwnedThreeStoreCommitPhaseV1),
        DiagnosticProgress(DurableOwnedThreeStoreCommitPhaseV1,semio_framework_value::retained_clone::RetainedCloneProgress),
Blocked,
    AwaitingAck(DurableOwnedGroupJournalReceiptV1),
    Complete,
}

/// 📸️ One cross-store snapshot projection selected by one captured shared decision.
pub struct DurableOwnedThreeStoreSnapshotV1<ParentP, DrawingP, ValueP> {
    pub parent: Arc<ParentP>,
    pub drawing: Arc<DrawingP>,
    pub value: Arc<ValueP>,
    pub generations: [u64; 3],
    pub revisions: [[u8; 32]; 3],
}

/// ⏭️ One bounded lower restoration turn; it deliberately exposes no receipt or durable approval acknowledgement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DurableOwnedMapRecoveryAdvanceV1 {
    Progress(DurableOwnedThreeStoreCommitPhaseV1),
        DiagnosticProgress(DurableOwnedThreeStoreCommitPhaseV1, semio_framework_value::retained_clone::RetainedCloneProgress),
Blocked,
    Fault(DurableOwnedGroupDecisionError),
    AwaitingAcknowledgement,
    Complete,
}

/// 🎫 One exact unbound Store admission. No group identity can enter before the private bind.
pub struct DurableOwnedMapMemberAdmissionV1<Mutation> {
    pub operation: semio_framework_job::OperationId,
    pub expected_generation: u64,
    pub expected_revision: [u8; 32],
    pub actor: semio_framework_value::SharedUtf8,
    pub mutation: Option<Mutation>,
}

impl<Mutation> DurableOwnedMapMemberAdmissionV1<Mutation> {
    pub fn new(operation: semio_framework_job::OperationId, expected_generation: u64, expected_revision: [u8; 32], actor: semio_framework_value::SharedUtf8, mutation: Mutation) -> Self {
        Self { operation, expected_generation, expected_revision, actor, mutation: Some(mutation) }
    }
}

/// 🚫 Terminal pre-journal assembly outcome. Every variant still returns all three Stores and the sink.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DurableOwnedThreeStoreMapAssemblyFailureV1 {
    Cancelled,
    Admission { role: &'static str, reason: String },
    Preparation { role: &'static str, reason: String },
    Binding(DurableOwnedGroupDecisionError),
}

/// 🧭 Observable retained assembly phase. `Mounted` contains the existing fixed host and no loose Store.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DurableOwnedThreeStoreMapAssemblyPhaseV1 {
    AdmittingParent,
    AdmittingDrawing,
    AdmittingValue,
    PreparingParent,
    PreparingDrawing,
    PreparingValue,
        TakingPrepared,
ClosingValue,
    ClosingDrawing,
    ClosingParent,
    Binding,
    Mounted,
    Terminal,
    Empty,
}

/// ⏭️ One bounded assembly turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DurableOwnedThreeStoreMapAssemblyAdvanceV1 {
    Progress(DurableOwnedThreeStoreMapAssemblyPhaseV1),
        OwnershipProgress(DurableOwnedThreeStoreMapAssemblyPhaseV1,semio_framework_value::retained_clone::RetainedCloneProgress),
Blocked,
    Mounted,
    Terminal,
}

pub fn close_assembly_publication<P: Send + Sync + 'static, Mutation: Send + 'static>(publication: &mut Option<semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication<P, Mutation>>, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<bool, DurableOwnedGroupDecisionError> {
    use semio_framework_value::retained_clone::{RetainedCloneStep, admit_retained_clone_close};
    let Some(owner) = publication.as_mut() else { return Ok(true) };
    let grant = grant.retained_grant();
    let codec = DurableOwnedGroupDecisionError::Retirement;
    let step = owner.close_step(grant).map_err(codec)?;
    match admit_retained_clone_close(grant, step, owner.terminal_is_empty(), "durable group publication").map_err(codec)? {
        RetainedCloneStep::Complete(_) => {
            drop(publication.take());
            Ok(true)
        }
        RetainedCloneStep::Progress(_) => Ok(false),
    }
}

pub fn take_assembly_prepared<P, Mutation>(publication: &mut Option<semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication<P, Mutation>>, grant: semio_framework_os_store_one_item::ArtifactStoreOneItemGrant) -> Result<(ArtifactStoreOneItemPrepared<P, Mutation>,semio_framework_value::retained_clone::RetainedCloneProgress), DurableOwnedGroupDecisionError> {
    let publication = publication.as_mut().ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)?;
    if publication.phase() != semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPublicationPhase::Publishing {
        return Err(DurableOwnedGroupDecisionError::InvalidOutcome);
    }
    publication.take_staged_prepared(grant.retained_grant()).ok_or(DurableOwnedGroupDecisionError::InvalidOutcome)
}
