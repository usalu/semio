//! 🪒️ Staged slice of the `typed-operations` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! 🧾️ Typed operations of the mounted artifact instance: typed-operation publication, typed-command dispatch, configuration and the history panel.
use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, PluginLifecycleStep, plugin_demand_grant, plugin_retirement_fault};
use semio_framework_plugin_decl_views::AppRenderOperationContext;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactBoundedToolFault, ArtifactToolCompletionValue, TYPED_OPERATION_FAULT_PAGE_BYTES, TypedOperationResultLane};
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin};
use std::future::Future;
use semio_framework_plugin_job::{INTERACTIVE_TURN_WORKER_PUMPS, INTERACTIVE_TURN_WORKER_WALL_US, TypedOperationResultPage, TypedOperationResultToken};
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use store::{MemberFactory, Mutation, SpaceMember};
use semio_framework_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation;
use semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage;
use protocol::OpText;
use semio_framework_plugin_command::command_admission::PendingLatestWinsCommand;
use crate::jobs_components::{PendingReserved, ToolOperations};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_job::reserved_jobs::QualifiedToolProof;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_plugin_command::command_admission::original_latest_key::ToolLatestWinsKeyCopy;
use semio_framework_plugin_interaction::interaction_topology::is_inline_interaction_verb;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;

impl<A: ArtifactApp> PendingReserved<A> {
    pub fn receive_configuration_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if self.pending_configuration_failure.is_none(){return Ok(false)}if self.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration failure requires its unchanged original caller").into())}if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.pending_configuration_failure,cx)?{return Err(failure)}Ok(true)}

    pub fn configuration_failure_terminal_is_empty(&self)->bool{!self.pending_configuration_rejected&&self.pending_configuration_source.is_none()&&self.pending_configuration_worker.is_none()&&self.pending_configuration_completion.is_none()&&self.pending_configuration_command.is_none()&&self.pending_configuration_command_close.is_none()&&self.pending_configuration_permit.is_none()&&self.pending_configuration_failure.is_none()&&self.pending_configuration_retirement.is_none()&&self.pending_configuration_identity.is_none()}

    pub fn configuration_failure_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(worker)=self.pending_configuration_worker.as_ref(){return worker.close_demands(body)}if let Some(owner)=self.pending_configuration_completion.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.demand(body)}}if let Some(owner)=self.pending_configuration_command_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)}}self.pending_configuration_retirement.as_ref().map_or(Ok(RetirementDemand{depth:usize::from(!self.configuration_failure_terminal_is_empty()),..Default::default()}),|owner|if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)})}
}

impl<A: ArtifactApp> ToolOperations<A> {
    /// 📬️ Advances exactly one retained publication unit. Store lanes return one-item
    /// receipts; host lanes remain in the operation until their fixed result page is ACKed.
    /// 🏃️ Drives one Worker-stage typed operation for a bounded slice of this turn. The mounted
    /// worker session only submits a job step to the process pool; on wasm that pool has no threads
    /// and runs a step solely when it is pumped, and until now the only pump was the
    /// cooperative-maintenance cadence (`maintenance_step` stage 0 of 21, one call per ~8 reactor
    /// turns) — measured 2026-09-09: 3 interactive job steps in 4 096 reactor turns, every action
    /// exhausting the host's continuation budget with `more-work`. Bounded by
    /// [`INTERACTIVE_TURN_WORKER_PUMPS`] and [`INTERACTIVE_TURN_WORKER_WALL_US`].
    ///
    /// 🛑️ ONE operation's structural fault is that operation's fault, never the turn's:
    /// see [`Self::fault_typed_operation_worker`].
    /// 🏃️ Spends ONE host turn's grant on consecutive worker steps of one mounted operation: every step receives what the turn still owns, the receipts are summed into the one original receipt the host takes, and the drive hands the turn back when the grant is exhausted, a step made no progress, the operation left the worker stage or its close mode changed.
    pub fn drive_typed_operation_worker(&mut self, mounted_policy: &semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let retained=cx.retained_grant();let started_us=cx.now_us();
        let mut turn: Option<(bool, RetainedCloneGrant)> = None;
        let mut spent = RetainedCloneProgress::default();
        let mut failure = None;
        for _ in 0..INTERACTIVE_TURN_WORKER_PUMPS {
            let Some(operation)=self.tool_operations.get_mut(operation_id)else{break};
            if operation.stage!=MountedTypedCommandFullOperationStage::Worker{break;}
            let closing=operation.terminal_seen||operation.user_cancel_requested||cx.is_cancelled()||operation.producer_request_retirement.is_some()||operation.session_rejected.is_some()||operation.worker_outcome_pending;
            if turn.is_some_and(|(kind,_)|kind!=closing){break;}
            let policy=if closing{mounted_policy.close}else{mounted_policy.preparation};
            let (_,turn_grant)=*turn.get_or_insert((closing,semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(policy,retained)));
            let grant=semio_framework_plugin_ownership::bounded_ownership::original_plugin_turn_grant(policy,semio_framework_plugin_reactor_app::remaining_plugin_turn_grant(turn_grant,spent));
            let stepped=cx.with_retained_loan(grant,|same|operation.drive_worker_step(same));
            let progress=match &stepped{Ok(step)=>step.progress().unwrap_or_default(),Err(failure)=>failure.retained_progress()};
            spent=spent.checked_add(progress)?;
            if let Err(fault)=stepped{failure=Some(fault);break;}
            if progress.copied_items==0||semio_framework_plugin_reactor_app::remaining_plugin_turn_grant(turn_grant,spent).maximum_items==0{break;}
            if cx.should_yield()||started_us.is_some_and(|started_us|cx.now_us().is_some_and(|now_us|now_us.saturating_sub(started_us)>=INTERACTIVE_TURN_WORKER_WALL_US)){break;}
        }
        if spent!=RetainedCloneProgress::default(){
            let (closing,grant)=turn.expect("a worker turn that spent anything was granted");
            let operation=self.tool_operations.get_mut(operation_id).ok_or_else(||plugin_sdk_fault("worker receipt lost its mounted operation"))?;
            if closing{assert!(operation.original_retirement_receipt.is_none(),"worker Close receipt awaits its original recipient");operation.original_retirement_receipt=Some((grant,spent));}
            else{assert!(operation.publication_ownership_progress.is_none(),"worker preparation receipt awaits its original recipient");operation.publication_ownership_progress=Some((grant,spent));}
        }
        #[cfg(target_arch="wasm32")]
        if let Some(now_ms)=semio_framework_job::default_now_ms(){semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get))).pump(now_ms);}
        match failure{Some(failure)=>Err(failure.into()),None=>Ok(())}
    }

    /// 🎯️ The next mounted operation this turn's one publication unit can actually advance, taken from
    /// the rotating publication cursor. An `AwaitingAck` operation whose page is already out keeps the
    /// turn in `MoreWork` — the host owes it an acknowledgement event — but owns no step of its own
    /// until that event lands, so spending the turn's only unit on it starves every sibling that does
    /// have one (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B16).
    /// 🚫️async: E1 pure census over an already-owned fixed table — see R9.
    pub fn next_advanceable_typed_operation(&self) -> Option<(usize, u64)> {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset| {
            let index = (self.typed_publication_cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            self.tool_operations.entry(index).filter(|(_, operation)| operation.stage != MountedTypedCommandFullOperationStage::AwaitingAck).map(|(id, _)| (index, *id))
        })
    }

    /// 🩺️ What the publication cursor is about to advance, as the four facts that decide whether the
    /// ladder can move at all AND whether it did: the operation it selected, that operation's stage,
    /// the ownership flags the arms branch on, and the publication's own monotone progress.
    /// Compared before and after one unit by [`Self::typed_operation_stall_streak`].
    ///
    /// 🐛️ The progress term is the fourth fact because the first three are constant for a HEALTHY
    /// publication: an edit of 5 745 mutations (puzzle5d's `capsule-dream` example switch) folds one
    /// bounded mutation per unit for ~19 000 units with the same operation id, the same `Publishing`
    /// stage and the same ownership flags, so the streak counter could not tell it from an owner that
    /// can never advance and terminated it at [`TYPED_OPERATION_STALL_FAULT_CEILING`] — about a fifth
    /// of the way through — with `interactive-job.publication-stalled`. Measured by the
    /// "Semio-tech play" session on 2026-09-21. Raising the ceiling would only move the cliff to the
    /// next document size and re-open the silent spins the guard exists to catch.
    /// 🚫️async: E1 pure census over an already-owned fixed table — see R9.
    pub fn typed_operation_stall_witness(&self) -> Option<(u64, u8, u8, u64)> {
        let (_, operation_id) = self.next_advanceable_typed_operation()?;
        let operation = self.tool_operations.get(operation_id)?;
        let stage = match operation.stage {
            MountedTypedCommandFullOperationStage::Worker => 0,
            MountedTypedCommandFullOperationStage::Publishing => 1,
            MountedTypedCommandFullOperationStage::AwaitingAck => 2,
            MountedTypedCommandFullOperationStage::Retiring => 3,
        };
        let flags = u8::from(operation.session.is_some())
            | u8::from(operation.session_rejected.is_some()) << 1
            | u8::from(operation.completion.is_some()) << 2
            | u8::from(operation.publication.is_some()) << 3
            | u8::from(operation.pending_artifact_publication.is_some()) << 4
            | u8::from(operation.worker_outcome_pending) << 5
            | u8::from(operation.terminal_seen) << 6
            | u8::from(operation.result_page.is_some()) << 7;
        Some((operation_id, stage, flags, operation.publication_progress))
    }

    /// 🛑️ Terminates the ONE `Worker`-stage operation whose own step answered a structural
    /// `Err` — an operation that lost its persistent worker session, or whose mounted session
    /// reported a state its own ladder cannot continue from — with that operation's terminal
    /// fault page, and lets the turn carry on.
    ///
    /// 🎯️ Why it is not a `?`: `drive_typed_operation_worker` is called from
    /// [`Self::advance_typed_operation_publication_unit`], the actor's ONE publication unit per
    /// turn. Propagating the `Err` aborted that unit for the WHOLE actor, so a single
    /// structurally faulted operation stopped every other mounted operation from ever advancing
    /// again — with no fault page, no effect and no patch to say why. The `Publishing` lane has
    /// answered this correctly since wave B16 (`advance_typed_operation_publication_unit`'s
    /// retry ladder, and [`Self::fault_stalled_typed_operation_publication`]'s own doc: a
    /// per-operation failure becomes that operation's fault, never the turn's); the `Worker`
    /// lane was the one lane that did not honour it.
    ///
    /// 🔒️ The operation is terminal the moment its worker ladder refuses: the lease is
    /// cancelled so no publication claim can be taken again, the fault page is minted against
    /// the operation's own token, and the stage moves to `Publishing` so the ordinary
    /// cancelled-publication close (`reject_cancelled_publication`) retires everything it still
    /// owns. Nothing is dropped here — the retained owners leave through the close ladder they
    /// always did.
    pub(crate) fn fault_typed_operation_worker(&mut self, operation_id: u64, fault: &Fault) -> Result<(), Fault> {
        let Some(mounted) = self.tool_operations.get_mut(operation_id) else { return Ok(()) };
        if mounted.stage != MountedTypedCommandFullOperationStage::Worker || mounted.result_page.is_some() {
            return Ok(());
        }
        let bounded = ArtifactBoundedToolFault::from_fault(fault);
        if let Some(lease) = mounted.cancellation_lease.as_ref() {
            lease.cancel();
        }
        let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];
        let framed_len = bounded.framed_page_bytes(&mut framed);
        mounted.stage = MountedTypedCommandFullOperationStage::Publishing;
        let page = TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, &framed[..framed_len])?;
        mounted.terminal_fault = Some(bounded);
        mounted.queue_page(page)
    }

    /// 🛑️ Terminates a `Publishing`-stage operation whose ladder has not moved for
    /// [`TYPED_OPERATION_STALL_FAULT_CEILING`] units with a real fault page instead of leaving it to
    /// spin. The publication ladder's non-advancing answers are all `Ok(())` by design — a completion
    /// that is not ready yet, and `ArtifactStoreOneItemAdvance::Blocked` from a store owner that
    /// refuses the granted page — so an owner that can NEVER advance pins its actor in `MoreWork`
    /// with no effect, no patch and no fault, which no instrument reads as a failure.
    ///
    /// Only the `Publishing` stage is bounded this way: a `Worker`-stage operation's progress is its
    /// own job's business and a long tool run legitimately leaves this witness unchanged for as long
    /// as it runs. Measured first on the signed-in `s` Home, whose `applyDirectoryEventPage` config
    /// publication answered `Blocked` on every unit (ticket 26/09/18 S8).
    pub fn fault_stalled_typed_operation_publication(&mut self, operation_id: u64, streak: u64) -> Result<(), Fault> {
        let Some(mounted) = self.tool_operations.get_mut(operation_id) else { return Ok(()) };
        if mounted.stage != MountedTypedCommandFullOperationStage::Publishing || mounted.result_page.is_some() {
            return Ok(());
        }
        let fault = ArtifactBoundedToolFault::from_fault(&Fault::new(
            FaultOrigin::Framework,
            FaultCode::new("interactive-job.publication-stalled"),
            format!("typed operation '{}' advanced {streak} publication units without moving its ladder", mounted.verb),
        ));
        if let Some(lease) = mounted.cancellation_lease.as_ref() {
            lease.cancel();
        }
        let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];
        let framed_len = fault.framed_page_bytes(&mut framed);
        let page = TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, &framed[..framed_len])?;
        mounted.terminal_fault = Some(fault);
        mounted.queue_page(page)
    }

    /// 🔀️ Whether this operation's next publication unit is the inline fold of its emit's
    /// guest-emitted interaction verbs (`Effect::ReplayShellCommand { action_id ∈
    /// INTERACTION_ACTION_IDS }` or `Effect::DispatchAction { action ∈ INTERACTION_ACTION_IDS }`,
    /// see [`is_inline_interaction_verb`]): every durable store lane is drained (a pick may only name ids
    /// the document already holds), the `InteractionWrite` lane went first (it precedes the verb
    /// in the emit's own order, exactly as it did when the verb still took a host round trip),
    /// and at least one such verb is still in the effect list. Sync and allocation-free like its
    /// `mounted_typed_interaction_writes_are_next` twin.
    /// 🚫️async: E1 pure field census over an already-mounted publication owner — see R9.
    ///
    /// 🅿️ The verbs were parked at install (`typed_inline_interaction_verbs`, keyed by this
    /// operation); the effect-list scan stays as the belt to that brace, for an emit whose
    /// effects were (re)filled after install.
    pub fn mounted_typed_inline_interaction_verbs_are_next(&self, mounted: &MountedTypedCommandFullOperation<A>) -> bool {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral)) = mounted.publication.as_ref() else {
            return false;
        };
        let parked = self.typed_inline_interaction_verbs.iter().any(|(operation, verbs)| *operation == mounted.operation.operation.0 && !verbs.is_empty());
        (parked || emit.effects.iter().any(is_inline_interaction_verb))
            && emit.interaction_writes.is_empty()
            && mounted.pending_artifact_publication.is_none()
            && mounted.owned_child_group.is_none()
            && emit.artifact_mutations.is_empty()
            && emit.config_mutations.is_empty()
            && emit.window_config_mutations.is_empty()
            && emit.draft_mutations.is_empty()
            && emit.child_emits.is_empty()
            && emit.owned_child_emits.is_empty()
            && ephemeral.presence.is_empty()
            && ephemeral.transient.is_empty()
            && ephemeral.window_transient.is_empty()
    }

    /// 🎟️ Advances the latest-wins key registry by exactly its own quoted turn.
    pub(crate) fn advance_latest_wins_keys(&mut self) -> Result<(), Fault> {
        let demand = self.latest_wins_keys.advance_demands().map_err(plugin_retirement_fault)?;
        self.latest_wins_keys.advance(plugin_demand_grant(demand))?;
        Ok(())
    }

    /// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B21: a rebase re-bases the LEASE, it does not
    /// change the key. The key is `(instance, envelope id, controller id, tool id, target)` — not one
    /// of which depends on the document revision or generation — so retiring it here only made the
    /// restart throw away an in-flight `ToolLatestWinsKeyCopy` one character per host continuation and
    /// rebuild it from scratch, several hundred round trips per rebase. And the registry's in-flight
    /// unit is driven here for ANY owner, not only for this one: `active_operation` is cleared solely
    /// by `ToolLatestWinsRegistry::advance`, and a sibling that was accepted and started leaves its
    /// closing update behind, so gating the drive on `active_operation == Some(operation)` let a
    /// restarting front-of-FIFO command wait forever on a registry no one would advance.
    pub fn advance_latest_wins_admission_unit(&mut self, stores: &mut InstanceStores<A>,pending:&mut PendingLatestWinsCommand<A>,retained:RetainedCloneGrant)->Result<bool,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        if retained.maximum_items==0||retained.maximum_depth==0{return Ok(false);}
        let operation=pending.operation.operation.0;
        if pending.lease.as_ref().is_none_or(|lease|lease.token.is_cancelled_now()){pending.closing=true;}
        if pending.closing{self.latest_wins_keys.cancel(operation);self.latest_wins_keys.take_outcome(operation);let step=if self.latest_wins_keys.active_operation==Some(operation){PluginLifecycleStep::Progress(self.latest_wins_keys.advance(retained)?.progress())}else{pending.close_step(retained)?};pending.record_original_receipt(retained,step.progress().unwrap_or_default())?;return Ok(false);}
        let live_revision=stores.store.content_revision_now();if live_revision!=pending.revision||stores.store.generation_now()!=pending.operation.generation.0{pending.restarting=true;}
        if pending.restarting{self.latest_wins_keys.cancel(operation);self.latest_wins_keys.take_outcome(operation);if !self.latest_wins_keys.can_begin(){let step=self.latest_wins_keys.advance(retained)?;pending.record_original_receipt(retained,step.progress())?;return Ok(false);}let base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(live_revision[..8].try_into().expect("original revision width")));let generation=semio_framework_job::Generation(stores.store.generation_now());if !pending.lease.as_mut().expect("original keyed rebase lease").rebind_keyed(base_revision,generation)?{return Ok(false);}pending.lookup_started=false;pending.accepted=false;pending.restarting=false;pending.revision=live_revision;pending.operation.base_revision=base_revision;pending.operation.generation=generation;pending.record_original_receipt(retained,RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(false);}
        if pending.accepted{let step=pending.close_key_step(retained)?;pending.record_original_receipt(retained,step.progress().unwrap_or_default())?;return Ok(matches!(step,PluginLifecycleStep::Complete(_)));}
        if pending.lookup_started{if let Some(accepted)=self.latest_wins_keys.take_outcome(operation){pending.accepted=accepted;pending.closing=!accepted;if !accepted{pending.lease.as_ref().expect("original pending lease").cancel();}pending.record_original_receipt(retained,RetainedCloneProgress{copied_items:1,..Default::default()})?;}else{let step=self.latest_wins_keys.advance(retained)?;pending.record_original_receipt(retained,step.progress())?;}return Ok(false);}
        if let Some(key)=pending.key.as_ref(){if let Some(progress)=self.latest_wins_keys.begin(operation,key,pending.lease.as_ref().expect("original pending lease"),retained)?{pending.lookup_started=self.latest_wins_keys.active_operation==Some(operation);pending.record_original_receipt(retained,progress)?;}else{let step=self.latest_wins_keys.advance(retained)?;pending.record_original_receipt(retained,step.progress())?;}return Ok(false);}
        let admission=pending.admission.as_ref().expect("original pending admission");let QualifiedToolProof::AppOwned(registration)=&admission.proof else{return Err(plugin_sdk_fault("latest-wins admission lost its concrete factory").into());};let command=pending.command.as_ref().expect("original pending command");let target=(registration.latest_wins_target)(command.as_ref()).ok_or_else(||plugin_sdk_fault("latest-wins original command lost its borrowed target"))?;let parts=[&*stores.store.envelope().id,&*registration.key.controller_id,&*registration.key.tool_id,target];
        if pending.key_copy.is_none(){if let Some((copy,progress))=ToolLatestWinsKeyCopy::admit_original(pending.meta.instance_id,parts,retained).map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?{pending.key_copy=Some(copy);pending.record_original_receipt(retained,progress)?;}return Ok(false);}
        let copy=pending.key_copy.as_mut().expect("original key byte cursor");
        if copy.part>4{if let Some((key,progress))=copy.take_key(retained).map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?{pending.key=Some(key);pending.record_original_receipt(retained,progress)?;}return Ok(false);}
        let step=copy.advance(parts,retained).map_err(|error|semio_framework_plugin_decl_emission::retirement_refusal_fault(&error))?;pending.record_original_receipt(retained,step.progress())?;Ok(false)
    }

    pub fn can_admit_typed_operation(&self, operation: u64) -> bool {
        self.tool_operations.can_insert(operation) && self.typed_operation_reservations[operation as usize % ARTIFACT_LIVE_OUTPUT_SLOTS].is_none()
    }

    pub fn close_configuration_failure(&mut self, pending: &mut PendingReserved<A>,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{if pending.pending_configuration_source.is_some_and(|source|!source.matches(cx)){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration retirement refused different original source roots or clock").into())}if let Some(worker)=pending.pending_configuration_worker.as_mut(){worker.cancel();let permit=pending.pending_configuration_permit.as_ref().expect("configuration close retains its original permit");let done=cx.with_retained_loan(grant,|same|worker.close_step(same,permit.lease.cancel_token())).map_err(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from)?;if done{let mut worker=pending.pending_configuration_worker.take().unwrap();if let Some(failure)=worker.take_failure(){assert!(pending.pending_configuration_failure.is_none());*pending.pending_configuration_failure=Some(semio_framework_plugin_job::reserved_jobs::OriginalConfigurationFailure::Original(failure));}assert!(worker.terminal_is_empty());}return Ok(PluginLifecycleStep::Progress(Default::default()))}let demand=pending.configuration_failure_demands(grant.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|{if let Some(owner)=pending.pending_configuration_completion.as_mut(){if owner.terminal_is_empty(){pending.pending_configuration_completion.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(owner)=pending.pending_configuration_command_close.as_mut(){if owner.terminal_is_empty(){pending.pending_configuration_command_close.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(command)=pending.pending_configuration_command.take(){match ControlledRetirement::new(command){Ok(owner)=>pending.pending_configuration_command_close=Some(owner),Err((error,command))=>{*pending.pending_configuration_command=Some(command);return Err(error.into())}}return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}if let Some(permit)=pending.pending_configuration_permit.take(){self.finish_reserved_emit_producer(permit);return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}if let Some(owner)=pending.pending_configuration_retirement.as_mut(){if owner.terminal_is_empty(){pending.pending_configuration_retirement.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(source)=pending.pending_configuration_failure.take(){match ControlledRetirement::new(source){Ok(owner)=>pending.pending_configuration_retirement=Some(owner),Err((error,source))=>{*pending.pending_configuration_failure=Some(source);return Err(error.into())}}return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}pending.pending_configuration_identity=None;pending.pending_configuration_source=None;pending.pending_configuration_rejected=false;Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}),true))})}

    /// 🪪️ The immutable identity this instance's live document revision is observed under — the ONE
    /// derivation every entry point that hands a plugin an [`ArtifactView`] stamps into it
    /// (`render`, `pending_effects`, `window_engagements`, `window_measures`, `tool_measures`,
    /// `context_menu`). Chrome that reaches for its own per-instance session (puzzle 3d's
    /// `puzzle3d_view_session_key`) read `None` on the four view-only lanes and ran cold against a
    /// session-less app; deriving the identity here — not per call site — makes that key total.
    /// 🚫️async: E1 pure identity projection over already-owned store fields — see R9.
    pub fn live_render_operation(&self, mounted_policy: &semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, stores: &InstanceStores<A>) -> AppRenderOperationContext {
        let canonical_base_revision = stores.store.content_revision();
        let [lane_0, lane_1, lane_2, lane_3, lane_4, lane_5, lane_6, lane_7, ..] = canonical_base_revision;
        AppRenderOperationContext {
            app_instance_id: self.live_runtime_instance_id.unwrap_or_default(),
            base_revision: semio_framework_job::RevisionId(u64::from_be_bytes([lane_0, lane_1, lane_2, lane_3, lane_4, lane_5, lane_6, lane_7])),
            generation: semio_framework_job::Generation(stores.store.generation()),
            canonical_base_revision,
            mounted_policy: *mounted_policy,
        }
    }

    #[cfg(test)]
    pub fn typed_operation_result_state_for_test(&self, operation_id: u64) -> Option<(TypedOperationResultToken, TypedOperationResultLane, bool, bool)> {
        let operation = self.tool_operations.get(operation_id)?;
        let page = operation.result_page.as_ref()?;
        Some((page.token, page.lane, operation.result_page_presented, operation.stage == MountedTypedCommandFullOperationStage::AwaitingAck))
    }
}
