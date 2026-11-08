//! 🔭️ Staged slice of the `maintenance-ladder` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! 🪜️ The live-maintenance rotation of one hosted app: priority and pressure units first, then a fair fixed round robin
//! in which an empty stage never consumes the call. Every unit quotes its next demand on all axes and spends the caller's
//! unchanged grant on exactly one owner; the same module drives the envelope-ingress, store-replacement and archive-load
//! frontiers for both the live rotation and the close ladder.
use semio_plugin_document_media::{ActiveArtifactStoreReplacementState, ActiveDocumentArchiveLoad, ActiveDocumentArchiveLoadPhase, ActiveDocumentArchiveLoadState, ArtifactEnvelopeDecodeOperationPoll, PendingDocumentArchiveMember};
use semio_plugin_decl_app::{ArtifactApp, DOCUMENT_LOAD_FAILED_CODE, DOCUMENT_LOAD_HISTORY_INVALID_CODE, document_load_fault};
use store::os_store::store_schema::ArtifactEnvelope;
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_artifact_reference::ArtifactRef;
use semio_plugin_instance_children::children_components::ChildComposition;
use semio_plugin_decl_emission::Emit;
use crate::docs_components::{EnvelopeJobs, MediaTransfers};
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_ownership::bounded_ownership::{PluginLifecycleStep, plugin_retirement_fault};
use semio_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_ui_contract::*;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};

/// 🚦️ The actual caller turn answers the parent history decoder's yield and cancellation checkpoints.
struct StepTurn<'a,'b>{actual:std::sync::Mutex<&'a mut semio_framework_job::StepContext<'b>>,closing:bool}

impl protocol::RetainedHistoryTurn for StepTurn<'_, '_> {
    fn should_yield(&self)->bool{self.actual.try_lock().map_or(true,|cx|if self.closing{cx.retirement_should_yield()}else{cx.should_yield()})}
    fn is_cancelled(&self)->bool{self.actual.try_lock().map_or(true,|cx|cx.is_cancelled())}
}
impl semio_plugin_document_media::ArchiveWorkTurn for StepTurn<'_,'_>{
    fn work_turn(&self)->Option<semio_framework_value_native_owned::native_decoding::OriginalSourceTurnIdentity>{self.actual.try_lock().ok().and_then(|cx|cx.original_source_turn_identity())}
    fn admits(&self,observed:u64,additional:u64)->Result<bool,ValueError>{
        let cx=self.actual.try_lock().map_err(|_|ValueError::invariant("original archive caller is already borrowed"))?;
        let paused=if self.closing{cx.retirement_should_yield()}else{cx.should_yield()};
        Ok(!paused&&(self.closing||!cx.is_cancelled())&&observed.checked_add(additional).is_some_and(|work|work<=cx.fuel_remaining()))
    }
}

/// 🧾️ Retains a producer's original failure beside any independent caller receiving refusal.
fn archive_retirement_failure(error:semio_framework_value::OriginalRetirementFailure,receiving:Option<semio_framework_job::OriginalTurnFailure>,progress:RetainedCloneProgress)->semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure{
 use semio_plugin_ownership::bounded_ownership::{PluginLifecycleFailure,PluginLifecyclePrimary};
 let primary=match receiving{
  None=>PluginLifecyclePrimary::Retirement(error),
  Some(receiving)=>{
   let semio_framework_job::OriginalTurnFailure{effects,primary,receipt_refusal,context_refusal,work_refusal}=receiving;
   let semio_framework_job::OriginalTurnPrimary::Value(receiving_primary)=primary else{unreachable!("native work reconciliation authors value refusals")};
   PluginLifecyclePrimary::RetirementReceiving(semio_framework_job::OriginalRetirementReceivingPrimary{retirement:error,receiving_primary,receiving_effects:effects,receiving_receipt_refusal:receipt_refusal,receiving_context_refusal:context_refusal,receiving_work_refusal:work_refusal})
  }
 };
 PluginLifecycleFailure{effects:progress,primary,receipt_refusal:None,context_refusal:None,work_refusal:None}
}
fn receive_archive<T>(cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant,fuel:u64,turn:semio_framework_value::RetainedWorkOutcome<T,semio_framework_value::OriginalRetirementFailure>,progress:RetainedCloneProgress,observed:u64)->Result<T,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
 use semio_framework_job_native_decode::{OriginalNativeDecodeSource,OriginalNativeDecodeIssued};
 let outcome=OriginalNativeDecodeSource::receive_observed_work(cx,grant,fuel,OriginalNativeDecodeIssued{value:turn.result,grant,progress,work:turn.work},observed);
 match (outcome.value,outcome.receiving_refusal) {
  (Ok(value),None)=>Ok(value),
  (Ok(_),Some(failure))=>{let mut failure:semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure=failure.into();failure.effects=progress;Err(failure)},
  (Err(error),receiving)=>Err(archive_retirement_failure(error,receiving,progress)),
 }
}

#[doc(hidden)]
pub fn item() -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, ..Default::default() }
}

#[doc(hidden)]
pub fn item_demand() -> RetirementDemand {
    RetirementDemand { depth: 1, ..Default::default() }
}

fn nonzero(demand: RetirementDemand) -> RetirementDemand {
    if demand == RetirementDemand::default() { item_demand() } else { demand }
}

#[doc(hidden)]
pub fn idle() -> PluginLifecycleStep {
    PluginLifecycleStep::Progress(RetainedCloneProgress::default())
}

#[doc(hidden)]
pub fn progressed(step: RetainedCloneStep) -> PluginLifecycleStep {
    PluginLifecycleStep::Progress(step.progress())
}

#[doc(hidden)]
pub fn removed(progress: RetainedCloneProgress) -> PluginLifecycleStep {
    PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress })
}

/// 🧯️ Turns a refused owner-catalog admission into its fault, keeping the receipt the refused build actually spent.
fn owners_admission_fault<P, Mutation>(refusal: store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<P, Mutation>) -> Fault
where
    P: store::StoreState,
    Mutation: store::StoreMutationFor<P>,
{
    let store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError { error, owners, progress } = refusal;
    std::mem::forget(owners);
    plugin_retirement_fault(error).with_retained_progress(progress)
}

impl<A: ArtifactApp> MediaTransfers<A> {
    /// 🧺️ Builds the app's exact owner catalog and hands a rejected envelope to its original retirement; the answer carries both receipts.
    pub(crate) fn retire_document_archive_envelope(envelope: ArtifactEnvelope<A::Snapshot, A::Mutation>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), Fault> {
        let (owners, built) = match A::build_document_store_owners(grant) {
            Some(Ok(built)) => built,
            Some(Err(refusal)) => {
                std::mem::forget(envelope);
                return Err(owners_admission_fault(refusal));
            }
            None => {
                std::mem::forget(envelope);
                return Err(document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive hydration lost its exact owner catalog while retiring a rejected envelope"));
            }
        };
        let remaining = RetainedCloneGrant {
            maximum_items: grant.maximum_items.saturating_sub(built.copied_items),
            maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(built.copied_bytes),
            maximum_capacity_bytes: grant.maximum_capacity_bytes.saturating_sub(built.retained_capacity_bytes),
            maximum_release_bytes: grant.maximum_release_bytes.saturating_sub(built.released_bytes),
            maximum_depth: grant.maximum_depth,
        };
        match owners.retire_envelope_uninstalled(envelope, remaining) {
            Ok((retirement, progress)) => Ok((retirement, built.checked_add(progress).map_err(plugin_retirement_fault)?)),
            Err((error, owners, envelope)) => {
                std::mem::forget((owners, envelope));
                Err(plugin_retirement_fault(error).with_retained_progress(built))
            }
        }
    }
}

impl<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static> EnvelopeJobs<A, M> {
    pub fn document_archive_load_demands(&self, active: &ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>, closing: bool, body: usize) -> Result<RetirementDemand, ValueError> {
        let item = item_demand();
        if active.terminal_target.is_some() || (closing && !active.terminal()) {
            let replacement_live = active.replacement.is_some_and(|handle| self.store_replacement_jobs.get(handle.operation.0).is_some());
            return Ok(if replacement_live || active.inputs_are_empty() { item } else { nonzero(active.retirement_demands(body)?) });
        }
        if active.state == ActiveDocumentArchiveLoadState::Pending {
            return Ok(item);
        }
        Ok(match active.phase {
            ActiveDocumentArchiveLoadPhase::DecodeParent => {
                let quote = active.history.as_ref().map_or_else(|| Ok(RetirementDemand::default()), |history| history.demands(body))?;
                RetirementDemand { copy_bytes: quote.copy_bytes.max(1), capacity_bytes: quote.capacity_bytes, release_bytes: quote.release_bytes, depth: quote.depth.max(item.depth) }
            }
            ActiveDocumentArchiveLoadPhase::MergeParent | ActiveDocumentArchiveLoadPhase::Terminal => RetirementDemand::default(),
            ActiveDocumentArchiveLoadPhase::HydrateParent => {
                if active.hydration.is_none() {
                    item
                } else {
                    RetirementDemand { capacity_bytes: store::os_store::snapshot_read::retire_document_envelope_birth_bytes::<A::Snapshot, A::Mutation>() + size_of::<store::os_store::document_store_owners::DocumentStoreOwners<A::Snapshot, A::Mutation>>(), depth: 4, ..Default::default() }
                }
            }
            ActiveDocumentArchiveLoadPhase::FillMember => {
                let length = active.member.as_ref().and_then(|member| member.entry.as_ref().map(|entry| entry.envelope_pack.len().saturating_sub(member.copied).min(store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES))).unwrap_or(0);
                RetirementDemand { copy_bytes: length, capacity_bytes: length, ..item }
            }
            ActiveDocumentArchiveLoadPhase::RetireMemberSource => active.member.as_ref().map_or(item, |member| nonzero(member.source_retirement_demands())),
            ActiveDocumentArchiveLoadPhase::AwaitingMembers | ActiveDocumentArchiveLoadPhase::BeginMember | ActiveDocumentArchiveLoadPhase::AdmitMember | ActiveDocumentArchiveLoadPhase::SealMembers | ActiveDocumentArchiveLoadPhase::AwaitReplacement => item,
        })
    }

    pub(crate) fn drive_document_archive_terminal(&mut self, active: &mut ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>, grant: RetainedCloneGrant, cx:&mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let target = active.terminal_target.ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive terminal cleanup has no target"))?;
        if let Some(handle) = active.replacement {
            if self.store_replacement_jobs.get(handle.operation.0).is_some() {
                if target != ActiveDocumentArchiveLoadState::Ready {
                    self.cancel_artifact_store_replacement(handle)?;
                }
                let poll = self.poll_artifact_store_replacement(handle);
                if matches!(poll, ArtifactEnvelopeDecodeOperationPoll::Ready | ArtifactEnvelopeDecodeOperationPoll::Cancelled | ArtifactEnvelopeDecodeOperationPoll::Fault) && self.acknowledge_artifact_store_replacement(handle)? {
                    active.replacement = None;
                    return Ok(PluginLifecycleStep::Progress(item()));
                }
                return Ok(idle());
            }
            active.replacement = None;
            if target == ActiveDocumentArchiveLoadState::Ready {
                return Err(document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "ready recursive document archive lost its replacement acknowledgement authority").into());
            }
        }
        if active.inputs_are_empty() {
            if grant.maximum_items == 0 {
                return Ok(idle());
            }
            active.state = target;
            active.phase = ActiveDocumentArchiveLoadPhase::Terminal;
            return Ok(removed(RetainedCloneProgress::default()));
        }
        let fuel=cx.fuel_remaining();
        let actual=StepTurn{actual:std::sync::Mutex::new(&mut *cx),closing:true};
        let turn=active.close_step(grant,&actual);drop(actual);
        let progress=match &turn.result {Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
        let observed=active.last_turn_native_work();
        match receive_archive(cx,grant,fuel,turn,progress,observed)? {
            RetainedCloneStep::Complete(progress) => {
                if !active.inputs_are_empty() {
                    return Err(document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive cleanup returned Complete with a live owner").into());
                }
                active.state = target;
                active.phase = ActiveDocumentArchiveLoadPhase::Terminal;
                Ok(removed(progress))
            }
            step => Ok(progressed(step)),
        }
    }

    /// 🧵️ One granted unit of a whole-document archive load: decode, hydrate, admit every member, seal and await the replacement.
    ///
    /// Genesis and `try_begin_owned_document_members` happen in ONE step: yielding between them would let the replacement
    /// lane observe `AwaitingMembers` with no ingress and seal the roster before the archived members were admitted. A
    /// childless candidate is sealed by the replacement lane itself, so this load then only awaits its outcome.
    pub fn advance_document_archive_load(&mut self, stores: &mut InstanceStores<A>, child_composition: &mut ChildComposition<M>, active: &mut ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>, grant: RetainedCloneGrant,cx:&mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let empty = RetainedCloneProgress::default();
        let item = item();
        let missing = |message: &'static str| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, message);
        if active.terminal_target.is_some() {
            return self.drive_document_archive_terminal(active, grant,cx);
        }
        if active.state == ActiveDocumentArchiveLoadState::Pending {
            active.state = ActiveDocumentArchiveLoadState::Running;
            return Ok(PluginLifecycleStep::Progress(item));
        }
        match active.phase {
            ActiveDocumentArchiveLoadPhase::DecodeParent => {
                let archive = active.archive.as_ref().ok_or_else(|| missing("recursive document archive parent input owner is absent"))?;
                let decoder = active.history.as_mut().ok_or_else(|| missing("recursive document archive parent history decoder is absent"))?;
                let fuel=cx.fuel_remaining();
                let actual=StepTurn{actual:std::sync::Mutex::new(&mut *cx),closing:false};
                let turn=match active.work.with_loan(&mut (&actual as &dyn semio_plugin_document_media::ArchiveWorkTurn),|loan|decoder.step(&archive.parent_spr,&actual,grant,&mut active.decoded_history,loan)){Ok(turn)=>turn,Err(error)=>semio_framework_value::RetainedWorkOutcome{work:0,result:Err(semio_framework_value::OriginalRetirementFailure::from_work_refusal(error))}};drop(actual);
                let progress=decoder.take_receipt().map_or_else(||match &turn.result {Err(error)=>error.retained_progress(),_=>Default::default()},|(_,progress)|progress);
                let observed=decoder.last_turn_native_work();
                let step=receive_archive(cx,grant,fuel,turn,progress,observed)?;
                match step {
                    protocol::RetainedHistorySourceStep::Fault => Err(document_load_fault(DOCUMENT_LOAD_HISTORY_INVALID_CODE, format!("document archive parent SPR was rejected: {:?}", decoder.fault())).with_retained_progress(progress).into()),
                    protocol::RetainedHistorySourceStep::Ready => {
                        if active.decoded_history.is_none() {
                            return Err((missing("ready recursive document archive history retained no decoded owner")).into());
                        }
                        let decoder = active.history.take().ok_or_else(|| missing("ready recursive document archive history decoder changed before terminal transfer"))?;
                        if !decoder.terminal_is_empty() {
                            active.history = Some(decoder);
                            return Err((missing("ready recursive document archive history decoder retained an untransferred owner")).into());
                        }
                        drop(decoder);
                        active.phase = if active.merge { ActiveDocumentArchiveLoadPhase::MergeParent } else { ActiveDocumentArchiveLoadPhase::HydrateParent };
                        Ok(PluginLifecycleStep::Progress(progress))
                    }
                    protocol::RetainedHistorySourceStep::Closed => Err((missing("recursive document archive history decoder closed before its log was ready")).into()),
                    protocol::RetainedHistorySourceStep::Progress | protocol::RetainedHistorySourceStep::Blocked => {
                        let discovered = if active.merge { 0 } else { progress.copied_items as u64 };
                        active.decoded = active.decoded.saturating_add(progress.copied_items as u64);
                        active.completed = active.completed.saturating_add(discovered);
                        active.total = active.total.saturating_add(discovered);
                        Ok(PluginLifecycleStep::Progress(progress))
                    }
                }
            }
            ActiveDocumentArchiveLoadPhase::MergeParent => Ok(PluginLifecycleStep::Progress(empty)),
            ActiveDocumentArchiveLoadPhase::HydrateParent => {
                if active.hydration.is_none() {
                    let (owners, built) = A::build_document_store_owners(grant).ok_or_else(|| missing("document archive parent requires the app's exact document owner catalog"))?.map_err(owners_admission_fault)?;
                    let archive = active.archive.as_mut().ok_or_else(|| missing("recursive document archive parent input owner is absent"))?;
                    let pack = std::mem::take(&mut archive.parent_pack);
                    let history = active.decoded_history.take().ok_or_else(|| missing("recursive document archive decoded history owner is absent"))?;
                    let expected = ArtifactRef { artifact_id: history.doc_id.clone(), dialect: A::DIALECT.into() };
                    let owner = stores.store.envelope().owner.clone();
                    active.hydration = Some(store::RetainedPersistedDocumentHydration::from_pack(
                        pack,
                        history,
                        {let declared=expected;store::PersistedDocumentHydrationIdentity{id:declared.artifact_id,dialect:Some(declared.dialect)}},
                        owner,
                        A::DOCUMENT_SCHEMA.to_string(),
                        owners,
                        cx.operation(),
                        cx.generation(),
                        u64::MAX,
                        store::PersistedDocumentHydrationTarget::Envelope,
                        stores.store.local_actor_id().clone(),
                    ));
                    return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: built.copied_items.max(1), ..built }));
                }
                let before=cx.retained_progress();
                let owner=active.hydration.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("recursive document parent hydration is not retained"))?;
                owner.operation=cx.operation();
                owner.generation=cx.generation();
                let step=owner.step(cx,RetainedCloneGrant{maximum_items:1,..grant});
                let spent=semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?;
                let item=RetainedCloneProgress{copied_items:spent.copied_items.max(1),..spent};
                let envelope = match step {
                    store::PersistedDocumentHydrationStep::Pending(_) => return Ok(PluginLifecycleStep::Progress(item)),
                    store::PersistedDocumentHydrationStep::Rejected(diagnostic) => {
                        return Err((document_load_fault(DOCUMENT_LOAD_HISTORY_INVALID_CODE, format!("document archive parent Pack and SPR hydration was rejected: {diagnostic:?}")).with_retained_progress(spent)).into());
                    }
                    store::PersistedDocumentHydrationStep::Ready(store::PersistedDocumentHydrationOutput::Envelope(envelope)) => envelope,
                    store::PersistedDocumentHydrationStep::Ready(store::PersistedDocumentHydrationOutput::Store(_)) => {
                        return Err((missing("document archive parent hydration returned a store outside its requested envelope boundary")).into());
                    }
                };
                let hydration = active.hydration.take().ok_or_else(|| plugin_sdk_fault("ready recursive document parent hydration owner changed before handoff"))?;
                if !store::ErasedSnapshotRetirement::terminal_is_empty(&hydration) {
                    active.hydration = Some(hydration);
                    let (retained, retired) = MediaTransfers::<A>::retire_document_archive_envelope(envelope, grant)?;
                    active.retained = Some(retained);
                    return Err((plugin_sdk_fault("ready recursive document parent hydration retained nonterminal ownership").with_retained_progress(spent.checked_add(retired).map_err(plugin_retirement_fault)?)).into());
                }
                drop(hydration);
                match self.begin_persisted_document_store_replacement(stores, child_composition, envelope, cx) {
                    Ok(handle) => {
                        active.replacement = Some(handle);
                        active.completed = active.completed.saturating_add(1);
                        active.phase = ActiveDocumentArchiveLoadPhase::AwaitingMembers;
                        Ok(PluginLifecycleStep::Progress(item))
                    }
                    Err((fault, envelope)) => {
                        let (retained, retired) = MediaTransfers::<A>::retire_document_archive_envelope(envelope, grant)?;
                        active.retained = Some(retained);
                        Err((fault.with_retained_progress(spent.checked_add(retired).map_err(plugin_retirement_fault)?)).into())
                    }
                }
            }
            ActiveDocumentArchiveLoadPhase::AwaitingMembers => {
                let handle = active.replacement.ok_or_else(|| missing("recursive document archive replacement handle is absent"))?;
                let state = self.store_replacement_jobs.get(handle.operation.0).map(|replacement| replacement.state);
                if let Some(fold) = self.store_replacement_jobs.get(handle.operation.0).map(|replacement| replacement.fold_progress) {
                    active.fold = fold;
                }
                match state {
                    Some(ActiveArtifactStoreReplacementState::AwaitingMembers) => {
                        if !active.genesis_complete {
                            self.complete_document_archive_genesis(active, handle)?;
                            active.genesis_complete = true;
                        }
                        let expected = active.archive.as_ref().ok_or_else(|| missing("recursive document archive member roster owner is absent"))?.members.len();
                        if self.try_begin_owned_document_members(handle, expected, u64::MAX)? {
                            active.phase = ActiveDocumentArchiveLoadPhase::BeginMember;
                            Ok(PluginLifecycleStep::Progress(item))
                        } else {
                            Ok(PluginLifecycleStep::Progress(empty))
                        }
                    }
                    Some(ActiveArtifactStoreReplacementState::Complete) | None => Err((missing("document archive parent initialization failed before retained member admission")).into()),
                    Some(ActiveArtifactStoreReplacementState::Initializing) => Ok(PluginLifecycleStep::Progress(empty)),
                    Some(_) => {
                        if active.archive.as_ref().map_or(0, |archive| archive.members.len()) != 0 {
                            return Err((missing("document archive replacement sealed its member roster before the archived members were admitted")).into());
                        }
                        active.genesis_complete = true;
                        active.phase = ActiveDocumentArchiveLoadPhase::AwaitReplacement;
                        Ok(PluginLifecycleStep::Progress(item))
                    }
                }
            }
            ActiveDocumentArchiveLoadPhase::BeginMember => {
                let archive = active.archive.as_mut().ok_or_else(|| missing("recursive document archive member roster owner is absent"))?;
                let Some(entry) = archive.members.pop() else {
                    active.phase = ActiveDocumentArchiveLoadPhase::SealMembers;
                    return Ok(PluginLifecycleStep::Progress(item));
                };
                match PendingDocumentArchiveMember::new(entry) {
                    Ok(member) => {
                        active.member = Some(member);
                        active.phase = ActiveDocumentArchiveLoadPhase::FillMember;
                    }
                    Err((fault, entry)) => {
                        archive.members.push(entry);
                        return Err((fault).into());
                    }
                }
                Ok(PluginLifecycleStep::Progress(item))
            }
            ActiveDocumentArchiveLoadPhase::FillMember => {
                let member = active.member.as_mut().ok_or_else(|| missing("recursive document archive active member owner is absent"))?;
                let before = member.copied;
                let filled = member.fill_one_page(grant.maximum_copy_bytes.min(grant.maximum_capacity_bytes))?;
                let bytes = member.copied - before;
                if filled {
                    active.phase = ActiveDocumentArchiveLoadPhase::RetireMemberSource;
                }
                Ok(PluginLifecycleStep::Progress(if filled || bytes != 0 { RetainedCloneProgress { copied_bytes: bytes, retained_capacity_bytes: bytes, ..item } } else { empty }))
            }
            ActiveDocumentArchiveLoadPhase::RetireMemberSource => {
                let member = active.member.as_mut().ok_or_else(|| missing("recursive document archive active member owner is absent"))?;
                match member.retire_source_step(grant) {
                    RetainedCloneStep::Complete(_) => {
                        active.phase = ActiveDocumentArchiveLoadPhase::AdmitMember;
                        Ok(PluginLifecycleStep::Progress(item))
                    }
                    RetainedCloneStep::Progress(progress) => Ok(PluginLifecycleStep::Progress(progress)),
                }
            }
            ActiveDocumentArchiveLoadPhase::AdmitMember => {
                let handle = active.replacement.ok_or_else(|| missing("recursive document archive replacement handle is absent"))?;
                let ingress = match active.member.as_mut().ok_or_else(|| missing("recursive document archive active member owner is absent"))?.take_ingress(handle, stores.store.local_actor_id().clone()) {
                    Ok(ingress) => ingress,
                    Err((fault, ingress)) => {
                        if !ingress.terminal_is_empty() {
                            active.rejected_ingress = Some(ingress);
                        }
                        return Err((fault).into());
                    }
                };
                if let Err((fault, ingress)) = self.admit_owned_document_member(handle, ingress) {
                    active.rejected_ingress = Some(ingress);
                    return Err((fault).into());
                }
                let member = active.member.take().ok_or_else(|| missing("recursive document archive transferred member owner changed before exact removal"))?;
                if !member.terminal_is_empty() {
                    active.member = Some(member);
                    return Err((missing("recursive document archive admitted member retained a local owner")).into());
                }
                drop(member);
                active.completed = active.completed.saturating_add(1);
                active.phase = ActiveDocumentArchiveLoadPhase::BeginMember;
                Ok(PluginLifecycleStep::Progress(item))
            }
            ActiveDocumentArchiveLoadPhase::SealMembers => {
                let handle = active.replacement.ok_or_else(|| missing("recursive document archive replacement handle is absent"))?;
                self.seal_owned_document_members(handle)?;
                active.phase = ActiveDocumentArchiveLoadPhase::AwaitReplacement;
                Ok(PluginLifecycleStep::Progress(item))
            }
            ActiveDocumentArchiveLoadPhase::AwaitReplacement => {
                let handle = active.replacement.ok_or_else(|| missing("recursive document archive replacement handle is absent"))?;
                let target = match self.poll_artifact_store_replacement(handle) {
                    ArtifactEnvelopeDecodeOperationPoll::Pending | ArtifactEnvelopeDecodeOperationPoll::Progress => return Ok(PluginLifecycleStep::Progress(empty)),
                    ArtifactEnvelopeDecodeOperationPoll::Ready => ActiveDocumentArchiveLoadState::Ready,
                    ArtifactEnvelopeDecodeOperationPoll::Cancelled => ActiveDocumentArchiveLoadState::Cancelled,
                    ArtifactEnvelopeDecodeOperationPoll::Fault => ActiveDocumentArchiveLoadState::Fault,
                };
                let refusal = self.artifact_store_replacement_refusal(handle);
                if !self.acknowledge_artifact_store_replacement(handle)? {
                    return Ok(PluginLifecycleStep::Progress(empty));
                }
                active.replacement = None;
                if target == ActiveDocumentArchiveLoadState::Fault {
                    active.fault = semio_framework_diagnostic::encode_fault_bytes(&refusal.unwrap_or_else(|| missing("document archive replacement failed closure, authority, or retained publication validation before it recorded a leg")));
                }
                active.request_terminal(target);
                Ok(PluginLifecycleStep::Progress(item))
            }
            ActiveDocumentArchiveLoadPhase::Terminal => Ok(PluginLifecycleStep::Complete(empty)),
        }
    }
}

#[cfg(test)]
mod archive_retirement_cause_tests {
 use super::*;
 use semio_framework_value::{OriginalRetirementFailure,ValueRefusalKind};
 use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
 use semio_plugin_ownership::bounded_ownership::PluginLifecyclePrimary;
 fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:256}}
 #[test]fn original_archive_failures_preserve_every_cause_and_retire_through_the_caller(){
  let child=ValueError::new(ValueRefusalKind::InvalidValue,"archive child owns its prose");let child_ptr=child.message.as_ptr();let work=ValueError::new(ValueRefusalKind::WorkLimit,"archive work owns its prose");let work_ptr=work.message.as_ptr();let mut outer:semio_framework_job::OriginalTurnFailure=ValueError::literal(ValueRefusalKind::InvariantViolated,"outer quote").into();outer.receipt_refusal=Some(ValueError::literal(ValueRefusalKind::OwnershipLimit,"receipt"));outer.context_refusal=Some(ValueError::literal(ValueRefusalKind::DepthLimit,"context"));outer.work_refusal=Some(ValueError::literal(ValueRefusalKind::WorkLimit,"outer work"));outer.effects=RetainedCloneProgress{copied_items:1,copied_bytes:5,..Default::default()};
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(1771),Generation(1),grant(),||Some(1),u64::MAX,100_000);let mut failure=None;let mut probe=None;
  caller.turn(grant(),|cx|cx.with_original_work_loan(4096,cx.deadline_us(),|cx|{let (bytes,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Vec::<u8>::with_capacity(73));assert_eq!(heap.requested_bytes,73);let progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:heap.requested_bytes,..Default::default()};cx.consume_retained(progress)?;cx.consume_fuel(1);let (out,fold)=semio_framework_trace::observe_heap_allocations_on_this_thread(||archive_retirement_failure(OriginalRetirementFailure::from_primary_and_work(child,work),Some(outer),progress));assert_eq!((fold.requested_bytes,fold.released_bytes),(0,0));failure=Some(out);probe=Some(bytes);Ok(())})).unwrap();
  let owned=failure.as_ref().unwrap();assert_eq!(owned.effects.retained_capacity_bytes,73);match &owned.primary{PluginLifecyclePrimary::RetirementReceiving(value)=>{assert_eq!(value.retirement.primary().unwrap().message.as_ptr(),child_ptr);assert_eq!(value.retirement.work_refusal().unwrap().message.as_ptr(),work_ptr);assert_eq!(value.receiving_primary.message,"outer quote");assert!(value.receiving_receipt_refusal.is_some());assert!(value.receiving_context_refusal.is_some());assert!(value.receiving_work_refusal.is_some());assert_eq!(value.receiving_effects.copied_bytes,5);},_=>panic!("original combined owner required")};
  let mut owner=None;caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{let (turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_value::retirement::admit_owned_retirement(failure.take().unwrap(),grant()));let (value,progress)=turn.result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));cx.consume_retained(progress)?;cx.consume_fuel(turn.work);owner=Some(value);Ok(())})).unwrap();
  for _ in 0..100_000{if owner.is_none(){break}caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{let (turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut owner,grant()));let progress=turn.result.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));cx.consume_retained(progress)?;cx.consume_fuel(turn.work);turn.result?;Ok(())})).unwrap();}assert!(owner.is_none());
  caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(probe.take().unwrap()));assert_eq!(heap.released_bytes,73);cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:heap.released_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(())})).unwrap();caller.retire(grant());
  let work_only=archive_retirement_failure(OriginalRetirementFailure::from_work_refusal(ValueError::literal(ValueRefusalKind::WorkLimit,"work only")),None,Default::default());assert!(matches!(work_only.primary,PluginLifecyclePrimary::Retirement(_)));
 }
}
