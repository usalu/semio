//! 🧾️ Typed operations of the mounted artifact instance: typed-operation publication, typed-command dispatch, configuration and the history panel.

use super::*;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
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
    fn drive_typed_operation_worker(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let retained=cx.retained_grant();let started_us=cx.now_us();
        let mut turn: Option<(bool, RetainedCloneGrant)> = None;
        let mut spent = RetainedCloneProgress::default();
        let mut failure = None;
        for _ in 0..INTERACTIVE_TURN_WORKER_PUMPS {
            let Some(operation)=self.tool_operations.get_mut(operation_id)else{break};
            if operation.stage!=MountedTypedCommandFullOperationStage::Worker{break;}
            let closing=operation.terminal_seen||operation.session_rejected.is_some()||operation.worker_outcome_pending;
            if turn.is_some_and(|(kind,_)|kind!=closing){break;}
            let policy=if closing{self.mounted_policy.close}else{self.mounted_policy.preparation};
            let (_,turn_grant)=*turn.get_or_insert((closing,crate::plugin_runtime::original_plugin_turn_grant(policy,retained)));
            let grant=crate::plugin_runtime::original_plugin_turn_grant(policy,crate::plugin_runtime::remaining_plugin_turn_grant(turn_grant,spent));
            let stepped=cx.with_retained_loan(grant,|same|operation.drive_worker_step(same));
            let progress=match &stepped{Ok(step)=>step.progress().unwrap_or_default(),Err(failure)=>failure.primary.retained_progress()};
            spent=spent.checked_add(progress)?;
            if let Err(fault)=stepped{failure=Some(fault);break;}
            if progress.copied_items==0||crate::plugin_runtime::remaining_plugin_turn_grant(turn_grant,spent).maximum_items==0{break;}
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

    /// 🔎️ Whether any live envelope decode still has worker close or step work this turn can run.
    fn has_runnable_artifact_envelope_decode(&self) -> bool {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).any(|index| self.envelope_decode_jobs.entry(index).is_some_and(|(_, active)| active.has_runnable_work()))
    }

    pub(super) fn has_runnable_artifact_envelope_decode_worker_step(&self) -> bool {
        self.has_runnable_artifact_envelope_decode()
    }

    /// 🎯️ The next mounted operation this turn's one publication unit can actually advance, taken from
    /// the rotating publication cursor. An `AwaitingAck` operation whose page is already out keeps the
    /// turn in `MoreWork` — the host owes it an acknowledgement event — but owns no step of its own
    /// until that event lands, so spending the turn's only unit on it starves every sibling that does
    /// have one (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B16).
    /// 🚫️async: E1 pure census over an already-owned fixed table — see R9.
    fn next_advanceable_typed_operation(&self) -> Option<(usize, u64)> {
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
    fn typed_operation_stall_witness(&self) -> Option<(u64, u8, u8, u64)> {
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

    /// 🩺️ How many consecutive publication units advanced the SAME operation without changing any of
    /// [`Self::typed_operation_stall_witness`]'s three facts — `Some(streak)` only on a power of two at
    /// or above [`TYPED_OPERATION_STALL_WITNESS_FLOOR`], so one stalled operation costs at most a
    /// handful of lines however long it spins.
    ///
    /// 🐛️ A mounted operation that never publishes its terminal keeps the actor in `MoreWork` with no
    /// effect, no patch and no fault, which every other instrument reads as silence: the host's own
    /// drain polls forever, the reactor's more-work streak names only `typed_operation` as its source,
    /// and nothing says WHICH arm of the publication ladder is not moving. Measured 2026-09-20 on the
    /// signed-in `s` Home, whose `applyDirectoryEventPage` operation spun for the whole session
    /// (ticket 26/09/18 S8).
    fn typed_operation_stall_streak(before: (u64, u8, u8, u64), after: (u64, u8, u8, u64)) -> Option<u64> {
        static WITNESS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
        static STREAK: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let key = (after.0.wrapping_shl(16) | u64::from(after.1).wrapping_shl(8) | u64::from(after.2)) ^ after.3.wrapping_mul(TYPED_OPERATION_STALL_PROGRESS_MIX);
        if before != after || WITNESS.swap(key, std::sync::atomic::Ordering::Relaxed) != key {
            STREAK.store(0, std::sync::atomic::Ordering::Relaxed);
            return None;
        }
        let streak = STREAK.fetch_add(1, std::sync::atomic::Ordering::Relaxed).saturating_add(1);
        (streak >= TYPED_OPERATION_STALL_WITNESS_FLOOR && streak.is_power_of_two()).then_some(streak)
    }

    pub(crate) async fn advance_typed_operation_publication_one(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let before = self.typed_operation_stall_witness();
        let advanced = self.advance_typed_operation_publication_unit(cx).await;
        if let (Some(before), Some(after)) = (before, self.typed_operation_stall_witness()) {
            if let Some(streak) = Self::typed_operation_stall_streak(before, after) {
                let verb = self.tool_operations.get(after.0).map_or("<released>", |operation| operation.verb.as_str());
                crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] typed-operation {verb} advanced {streak} units with no change: operation={} stage={} flags={:#010b}", after.0, after.1, after.2));
                if after.1 == TYPED_OPERATION_STALL_PUBLISHING_STAGE && streak >= TYPED_OPERATION_STALL_FAULT_CEILING {
                    self.fault_stalled_typed_operation_publication(after.0, streak)?;
                }
            }
        }
        advanced
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
    fn fault_typed_operation_worker(&mut self, operation_id: u64, fault: &Fault) -> Result<(), Fault> {
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
    fn fault_stalled_typed_operation_publication(&mut self, operation_id: u64, streak: u64) -> Result<(), Fault> {
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

    /// ♻️ A retiring operation answers `has_runnable_typed_operations`, so the turn driver that reports it
    /// runnable must be the one that releases its slot — see [`Self::retire_typed_operation_run`].
    ///
    /// 🕹️ "After EVERY artifact (document) dispatch, re-derive fresh topology and prune any
    /// selection/hover id no longer present" — the clause `dispatch_emit_inner` carries, which
    /// the migrated publication ladder never reaches (see the emit backstops above it). A
    /// migrated dispatch's document change lands on the Artifact lane, so this is where the
    /// revalidation belongs; `mounted` is out of the registry here, so the async pass is free
    /// to borrow the app, and the flag makes it exactly one pass per operation.
    async fn advance_typed_operation_publication_unit(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let retained=cx.retained_grant();if cx.should_yield(){return Ok(())}
        if !self.latest_wins_commands.is_empty() && (self.latest_wins_turn || self.tool_operations.is_empty()) {
            self.latest_wins_turn = false;
            return self.advance_latest_wins_command_one(retained, cx).await.map_err(Into::into);
        }
        self.latest_wins_turn = true;
        let Some((index, operation_id)) = self.next_advanceable_typed_operation() else {
            record_typed_operation_unit(TypedOperationUnitKind::Idle);
            return Ok(());
        };
        self.typed_publication_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        if self.tool_operations.get(operation_id).is_some_and(|mounted|mounted.cancellation_lease.is_none()&&mounted.cancellation_retirement.is_none()&&mounted.stage==MountedTypedCommandFullOperationStage::Publishing){
            let mounted=self.tool_operations.get_mut(operation_id).unwrap();
            if mounted.source_identity!=(cx.operation(),cx.generation()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original emission lease requires its mounted receiving caller").into())}
            let key=ToolOperationKey{app_instance_id:mounted.meta.instance_id,document:ArtifactDocumentAuthority(mounted.meta.instance_id),operation_id:mounted.operation.operation,base_revision:mounted.operation.base_revision,generation:mounted.operation.generation};
            mounted.cancellation_lease=self.tool_cancellations.begin_keyed(&key,cx)?;
            if let Some(lease)=mounted.cancellation_lease.as_ref(){
                if mounted.terminal_fault.is_some()||mounted.user_cancel_requested{lease.cancel();}
                let maximum=self.qualified_tool_proof(&self.tool_operations.get(operation_id).unwrap().verb)?.contract().max_output_bytes;
                let mounted=self.tool_operations.get_mut(operation_id).unwrap();let lease=mounted.cancellation_lease.as_ref().unwrap();
                let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|!lease.token.is_cancelled_now()&&!lease.publication_claim.is_cancelled();
                mounted.identity=Some(semio_framework_value::NativeEncodeControl::new(maximum,&mut observer).pause().map_err(|error|plugin_sdk_fault(error.to_string()))?);
            }
            return Ok(());
        }
        if self.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Worker) {
            record_typed_operation_unit(TypedOperationUnitKind::Worker);
            return self.drive_typed_operation_worker(operation_id,cx);
        }
        if self.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Retiring) {
            record_typed_operation_unit(TypedOperationUnitKind::Retirement);
            return self.retire_typed_operation_run(operation_id,cx);
        }
        if !self.tool_operations.get_mut(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Publishing) {
            record_typed_operation_unit(TypedOperationUnitKind::Idle);
            return Ok(());
        }
        let mut mounted =
            self.tool_operations.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.publication-authority"), "typed-operation publication owner changed before one-unit transfer"))?;
        let outcome = self.publish_mounted_typed_operation_run(&mut mounted,cx,retained).await;
        let mut driver_result = Ok(());
        if let Err(failure)=outcome{self.tool_operations.insert_admitted(operation_id,mounted);return Err(failure);}
        mounted.publication_attempt=0;
        if driver_result.is_ok() && mounted.published_artifact && !mounted.interaction_revalidated {
            mounted.interaction_revalidated = true;
            let meta = mounted.meta.clone();
            let receipt = mounted.identity.take().ok_or_else(|| plugin_sdk_fault("mounted revalidation lacks its original authoring receipt"))?;
            let lease = mounted.cancellation_lease.as_ref().ok_or_else(|| plugin_sdk_fault("mounted revalidation lacks its original cancellation lease"))?;
            let token = lease.token.clone();
            let publication = lease.publication_claim.clone();
            let mut progress = mounted.identity_progress;
            let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
            let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
            driver_result = self.revalidate_interaction_on_document_change(&meta, &mut identity).await;
            mounted.identity = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
            drop(observer);
            mounted.identity_progress = progress;
        }
        self.tool_operations.insert_admitted(operation_id, mounted);
        driver_result.map_err(Into::into)
    }

    /// 🏃️ Drives ONE mounted operation's publication ladder for a bounded slice of this continuation
    /// unit — the exact twin of the slice [`Self::drive_typed_operation_worker`] gives a Worker-stage
    /// operation and [`Self::retire_typed_operation_run`] gives a retiring one, for the publication
    /// state machine that sits between them.
    ///
    /// 🧾️ Every ladder step — install the taken completion, stage the batch, prepare one item, fold
    /// it, retire its preparation owner, validate the cursor, preflight the commit, commit, mint the
    /// receipt — used to be its own host round trip, because the caller is driven once per reactor
    /// turn and the store's grant pages one item per call. One `deleteSelection` on a ONE-object
    /// document therefore cost 104–235 round trips (wave B44 §2.3): a document-INDEPENDENT constant
    /// that alone burns seconds of a 30-second interaction budget, and the reason a `setCamera` on a
    /// document it never reads cost 2.3 s.
    ///
    /// 🛑️ The run stops on exactly the protocol's own bounds and on nothing else: a queued result page
    /// the host must acknowledge before the operation may continue (`stage` leaves `Publishing`), a
    /// completion that is not ready yet so nothing is staged to advance, [`TYPED_OPERATION_PUBLICATION_PUMPS`],
    /// and [`INTERACTIVE_TURN_WORKER_WALL_US`]. Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B54.
    pub(crate) async fn publish_mounted_typed_operation_run(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>,cx:&mut semio_framework_job::StepContext<'_>, retained:RetainedCloneGrant) -> Result<(),crate::PluginLifecycleFailure> {
        if mounted.identity.is_none() {
            let completion = mounted.completion.as_ref().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original effect completion"))?;
            let (receipt, progress) = completion.take_authoring_identity()?.ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original app-owned effect receipt"))?;
            mounted.identity = Some(receipt);
            mounted.identity_progress = progress;
        }
        let receipt = mounted.identity.take().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its admitted authoring receipt"))?;
        let lease = mounted.cancellation_lease.as_ref().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original cancellation lease"))?;
        let token = lease.token.clone();
        let publication = lease.publication_claim.clone();
        let mut progress = mounted.identity_progress;
        let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt, &mut observer as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>);
        let result = match self.publish_mounted_typed_operation_run_admitted(mounted, &mut identity,cx,retained).await {
            Ok(()) => self.follow_derivable_children(&mut identity).await.map_err(Into::into),
            Err(fault) => Err(fault),
        };
        mounted.identity = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
        drop(observer);
        mounted.identity_progress = progress;
        result
    }

    async fn publish_mounted_typed_operation_run_admitted(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>, retained:RetainedCloneGrant) -> Result<(),crate::PluginLifecycleFailure> {
        if mounted.publication_ownership_progress.is_some(){return Ok(());}
        let mut spent = RetainedCloneProgress::default();
        let outcome = self.publish_mounted_typed_operation_pump(mounted, identity, retained, &mut spent,cx).await;
        if spent != RetainedCloneProgress::default() {
            assert!(mounted.publication_ownership_progress.is_none(), "typed preparation cannot overwrite an unreturned original receipt");
            mounted.publication_ownership_progress = Some((crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation, retained), spent));
        }
        outcome
    }

    /// 🏃️ Spends ONE host turn's grant on consecutive publication units: each unit receives what the turn still owns, its receipt is summed into `spent`, and the pump hands the turn back as soon as the grant is exhausted — so a grant of one item still publishes one unit per turn, and a larger grant batches exactly as many units as it pays for.
    async fn publish_mounted_typed_operation_pump(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, retained:RetainedCloneGrant, spent: &mut RetainedCloneProgress,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),crate::PluginLifecycleFailure> {
        let started_us=cx.now_us();
        let turn = crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation, retained);
        for _ in 0..TYPED_OPERATION_PUBLICATION_PUMPS {
            if cx.should_yield(){return Ok(())}
            let before=cx.retained_progress();let fuel=cx.fuel_remaining();
            if mounted.owned_child_group.is_some() {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_owned_child_operation_unit(mounted,cx).await?;
            } else if mounted.pending_child_publication.is_some() {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_child_operation_unit(mounted, identity).await?;
            } else if Self::mounted_typed_interaction_writes_are_next(mounted) {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_interaction_unit(mounted, identity).await?;
            } else if self.mounted_typed_inline_interaction_verbs_are_next(mounted) {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_inline_interaction_unit(mounted, identity).await?;
            } else {
                let before = (mounted.publication_progress, mounted.publication.is_some(), mounted.pending_artifact_publication.is_some(), mounted.stage);
                self.publish_mounted_typed_operation_unit(mounted,crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.preparation,semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent))))?;
                let receipt = mounted.publication_ownership_progress.take();
                if let Some((_, unit)) = receipt {
                    cx.consume_retained(unit)?;
                    *spent = spent.checked_add(unit).map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}
                    if unit.copied_items == 0 || crate::plugin_runtime::remaining_plugin_turn_grant(turn,*spent).maximum_items == 0 { return Ok(()); }
                }
                if self.store.backbone_ref().is_some() {
                    self.store.flush_announcements().await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    if let Some(PendingArtifactStorePublication::Artifact(publication)) = mounted.pending_artifact_publication.as_mut() {
                        self.store.flush_published_apply_batch(publication).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    }
                }
                record_typed_operation_unit(if mounted.result_page.is_some() { TypedOperationUnitKind::Page } else { TypedOperationUnitKind::Store });
                if receipt.is_none() && before == (mounted.publication_progress, mounted.publication.is_some(), mounted.pending_artifact_publication.is_some(), mounted.stage) {
                    return Ok(());
                }
            }
            if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}
            if mounted.owned_child_group.is_some(){let delta=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,cx.retained_progress())?;*spent=spent.checked_add(delta)?;}
            if mounted.stage != MountedTypedCommandFullOperationStage::Publishing {
                return Ok(());
            }
            if mounted.publication.is_none() && mounted.pending_artifact_publication.is_none() && mounted.pending_child_publication.is_none() && mounted.owned_child_group.is_none() {
                return Ok(());
            }
            let Some(started_us) = started_us else { return Ok(()) };
            if cx.now_us().is_some_and(|now_us| now_us.saturating_sub(started_us) >= INTERACTIVE_TURN_WORKER_WALL_US) {
                return Ok(());
            }
        }
        Ok(())
    }

    /// 🕹️ Whether this operation's next publication unit is its [`InteractionWrite`] lane: every
    /// durable store lane of its emit is already drained (a selection may only name ids the
    /// document already holds), and there is at least one write left to apply. Sync and
    /// allocation-free so the async caller can branch on it before mounting anything.
    /// 🚫️async: E1 pure field census over an already-mounted publication owner — see R9.
    ///
    /// 🧺️ A batched durable lane empties its emit vector the moment it is admitted, so the
    /// in-flight publication itself — not the drained vector — is what still owes a turn.
    fn mounted_typed_interaction_writes_are_next(mounted: &MountedTypedCommandFullOperation<A>) -> bool {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral)) = mounted.publication.as_ref() else {
            return false;
        };
        !emit.interaction_writes.is_empty()
            && mounted.pending_artifact_publication.is_none()
            && mounted.owned_child_group.is_none()
            && emit.artifact_mutations.is_empty()
            && emit.config_mutations.is_empty()
            && emit.draft_mutations.is_empty()
            && emit.child_emits.is_empty()
            && emit.owned_child_emits.is_empty()
            && ephemeral.presence.is_empty()
            && ephemeral.transient.is_empty()
    }

    /// 🕹️ Publishes this operation's whole [`InteractionWrite`] lane as ONE unit through the same
    /// `apply_interaction_writes` single-writer path `dispatch_emit` uses — the retained-tool twin
    /// of that call. It is one unit rather than one-write-per-turn because a selection is a single
    /// state-machine transition per domain: draining it write-by-write would publish intermediate
    /// selections no user ever asked for. `MountedTypedCommandFullOperation` needs no extra field:
    /// the writes live in the retained emit until taken here, and the lane is only entered once
    /// every durable lane is drained (`mounted_typed_interaction_writes_are_next`).
    async fn publish_mounted_typed_interaction_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation interaction publication lost its emit owner"));
        };
        let writes = std::mem::take(&mut emit.interaction_writes);
        let meta = mounted.meta.clone();
        if let Err(fault) = self.apply_interaction_writes(&writes, &meta, identity).await {
            if let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() {
                emit.interaction_writes = writes;
            }
            return Err(fault);
        }
        let token = mounted.next_token();
        let page = TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Interaction, &("accepted", writes.len()))?;
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
    fn mounted_typed_inline_interaction_verbs_are_next(&self, mounted: &MountedTypedCommandFullOperation<A>) -> bool {
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

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the typed-operation twin of
    /// `dispatch_emit`'s inline interaction fold, for the ladder every app command
    /// (`dispatch_typed_command_inner`) and retained tool publishes through and that never enters
    /// `dispatch_emit`. Peels every interaction verb out of the retained emit's effect list as ONE
    /// unit (the same one-transition-per-domain reasoning as the `InteractionWrite` unit above),
    /// applies each through `apply_interaction_verb`, and widens the emit's `ui_scope` with the
    /// verbs' app-declared refresh scopes so the `Ui` page and the completion witness — published
    /// later in this same ladder — already carry them. No result page is queued: the host has
    /// nothing to acknowledge, because the verb it used to be asked to replay never leaves the
    /// reactor, and the ladder simply continues with the remaining effects next pump.
    ///
    /// 🩹 A verb that could not be applied is reported on the runtime's `[TRACE]` diagnostics line
    /// and dropped — the typed ladder has no `InvocationResult.diagnostics` lane (its only fault
    /// lane is terminal), and failing the carrying command for a pick that already happened is
    /// exactly what `fold_inline_interaction_verbs` exists to prevent.
    async fn publish_mounted_typed_inline_interaction_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation inline interaction publication lost its emit owner"));
        };
        let mut folded: Vec<(String, Option<DslValue>)> = Vec::new();
        if let Some(index) = self.typed_inline_interaction_verbs.iter().position(|(operation, _)| *operation == mounted.operation.operation.0) {
            folded = self.typed_inline_interaction_verbs.swap_remove(index).1;
        }
        folded.extend(take_inline_interaction_verbs(&mut emit.effects));
        let scope = emit.ui_scope.clone();
        let meta = mounted.meta.clone();
        let fold = self.fold_inline_interaction_verbs(&mounted.verb, folded, &meta, scope, identity).await;
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation inline interaction publication lost its emit owner after the fold"));
        };
        emit.ui_scope = fold.scope;
        if let Some(view) = fold.leftover {
            self.typed_inline_interaction_leftover = Some(TypedOperationLeftover { operation: mounted.operation.operation.0, view });
        }
        for diagnostic in fold.diagnostics {
            crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] {} {}", diagnostic.code.0, diagnostic.message));
        }
        Ok(())
    }

    pub(crate) async fn advance_latest_wins_command_one(&mut self,retained:RetainedCloneGrant, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), crate::PluginLifecycleFailure> {
        if retained.maximum_items==0||retained.maximum_depth==0{return Ok(());}
        let Some(operation_id) = self.latest_wins_order.items.front().copied() else { return Ok(()) };
        let mut pending = self.latest_wins_commands.remove(operation_id).expect("exact pending keyed command");
        if pending.receipt.is_some(){self.latest_wins_commands.insert_admitted(operation_id,pending);return Ok(());}
        if !pending.closing&&pending.lease.is_none(){
            let result=pending.admit_original_lease(&mut self.tool_cancellations,cx);
            self.latest_wins_commands.insert_admitted(operation_id,pending);
            result?;
            return Ok(());
        }
        let ready = match self.advance_latest_wins_admission_unit(&mut pending,retained) {
            Ok(ready) => ready,
            Err(fault) => {
                let result=if pending.receipt.is_some(){Ok(())}else{pending.record_original_receipt(retained,fault.retained_progress())};
                *pending.fault=Some(fault);
                if let Err(error)=result{self.latest_wins_commands.insert_admitted(operation_id,pending);return Err(error.into());}
                pending.closing = true;
                if let Some(lease) = pending.lease.as_ref() {
                    lease.cancel();
                }
                false
            }
        };
        let reserved = self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] == Some(operation_id);
        if !reserved || !self.tool_operations.can_insert(operation_id) {
            pending.closing = true;
            pending.lease.as_ref().expect("pending slot rejection retains its cancellation lease").cancel();
            if !self.tool_operations.can_insert(operation_id) || self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS].is_some_and(|owner| owner != operation_id) {
                self.latest_wins_commands.insert_admitted(operation_id, pending);
                return Ok(());
            }
        }
        if ready && !pending.closing {
            let command = pending.command.take().expect("accepted latest-wins command owner");
            let admission = pending.admission.take().expect("accepted latest-wins exact raw admission");
            let lease = pending.lease.take().expect("accepted latest-wins cancellation lease");
            assert_eq!(self.latest_wins_order.pop(), Some(operation_id));
            let started = self.start_typed_command_operation(command, admission, &pending.meta, pending.operation.operation, lease, cx).await;
            if started.is_err() {
                self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
            }
            started?;
            return Ok(());
        }
        if pending.closing && pending.command_owners_are_empty() {
            let mut mounted = MountedTypedCommandFullOperation::<A> {
                verb: "latest-wins-admission".into(),
                meta: pending.meta.clone(),
                operation: pending.operation,
                source_identity:pending.source_identity,
                canonical_revision: pending.revision,
                artifact_generation: pending.operation.generation.0,
                config_generation: 0,
                draft_generation: 0,
                presence_generation: 0,
                transient_generation: 0,
                window_config_authority: None,
                window_transient_authority: None,
                publication_lanes: &[],
                session: None,
                session_rejected: None,
                reserved_producer: None,
                completion: None,
                completion_retirement:None,
                publication_retirement:None,
                output_retirement:None,raw_input: None,
                output_chunks: None,
                cancellation_lease: pending.lease.take(),
                identity: None,
                identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 },
                worker_semantic_pending: false,
                worker_outcome_pending: false,
                worker_fault_capture: None,
                terminal_seen: true,
                worker_resume_pending: false,
                publication: None,
                pending_artifact_publication: None, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                pending_child_publication: None,
                owned_child_group: None,
                owned_child_committed: false,
                owned_child_result_pending: false,
                captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                captured_child_content_generation: 0,
                result_page: None,
                result_page_presented: false,
                result_sequence: 0,
                publication_progress: 0,
                publication_checkpoint: None,
                publication_ownership_progress: None,
                original_retirement_receipt:None,
                actor_capture: None,
                publication_attempt: 0,
                ui_pending: false,
                progress: None,
                progress_pending: false,
                user_cancel_requested: false,
                published_artifact: false,
                published_config: false,
                published_window_config: false,
                command_logged: false,
                interaction_revalidated: false,
                terminal_fault: None,
                retained_close_fault: None, retained_close_fault_retirement:None,retained_close_fault_refusal:None,
                stage: MountedTypedCommandFullOperationStage::Publishing,
            };
            mounted.queue_page(TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, b"latest-wins command admission cancelled or rejected before worker publication")?)?;
            self.tool_operations.insert_admitted(operation_id, mounted);
            self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
            assert_eq!(self.latest_wins_order.pop(), Some(operation_id));
        } else {
            self.latest_wins_commands.insert_admitted(operation_id, pending);
        }
        Ok(())
    }

    /// 🎟️ Advances the latest-wins key registry by exactly its own quoted turn.
    fn advance_latest_wins_keys(&mut self) -> Result<(), Fault> {
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
    fn advance_latest_wins_admission_unit(&mut self,pending:&mut PendingLatestWinsCommand<A>,retained:RetainedCloneGrant)->Result<bool,crate::PluginLifecycleFailure>{
        if retained.maximum_items==0||retained.maximum_depth==0{return Ok(false);}
        let operation=pending.operation.operation.0;
        if pending.lease.as_ref().is_none_or(|lease|lease.token.is_cancelled_now()){pending.closing=true;}
        if pending.closing{self.latest_wins_keys.cancel(operation);self.latest_wins_keys.take_outcome(operation);let step=if self.latest_wins_keys.active_operation==Some(operation){PluginLifecycleStep::Progress(self.latest_wins_keys.advance(retained)?.progress())}else{pending.close_step(retained)?};pending.record_original_receipt(retained,step.progress().unwrap_or_default())?;return Ok(false);}
        let live_revision=self.store.content_revision_now();if live_revision!=pending.revision||self.store.generation_now()!=pending.operation.generation.0{pending.restarting=true;}
        if pending.restarting{self.latest_wins_keys.cancel(operation);self.latest_wins_keys.take_outcome(operation);if !self.latest_wins_keys.can_begin(){let step=self.latest_wins_keys.advance(retained)?;pending.record_original_receipt(retained,step.progress())?;return Ok(false);}let base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(live_revision[..8].try_into().expect("original revision width")));let generation=semio_framework_job::Generation(self.store.generation_now());if !pending.lease.as_mut().expect("original keyed rebase lease").rebind_keyed(base_revision,generation)?{return Ok(false);}pending.lookup_started=false;pending.accepted=false;pending.restarting=false;pending.revision=live_revision;pending.operation.base_revision=base_revision;pending.operation.generation=generation;pending.record_original_receipt(retained,RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(false);}
        if pending.accepted{let step=pending.close_key_step(retained)?;pending.record_original_receipt(retained,step.progress().unwrap_or_default())?;return Ok(matches!(step,PluginLifecycleStep::Complete(_)));}
        if pending.lookup_started{if let Some(accepted)=self.latest_wins_keys.take_outcome(operation){pending.accepted=accepted;pending.closing=!accepted;if !accepted{pending.lease.as_ref().expect("original pending lease").cancel();}pending.record_original_receipt(retained,RetainedCloneProgress{copied_items:1,..Default::default()})?;}else{let step=self.latest_wins_keys.advance(retained)?;pending.record_original_receipt(retained,step.progress())?;}return Ok(false);}
        if let Some(key)=pending.key.as_ref(){if let Some(progress)=self.latest_wins_keys.begin(operation,key,pending.lease.as_ref().expect("original pending lease"),retained)?{pending.lookup_started=self.latest_wins_keys.active_operation==Some(operation);pending.record_original_receipt(retained,progress)?;}else{let step=self.latest_wins_keys.advance(retained)?;pending.record_original_receipt(retained,step.progress())?;}return Ok(false);}
        let admission=pending.admission.as_ref().expect("original pending admission");let QualifiedToolProof::AppOwned(registration)=&admission.proof else{return Err(plugin_sdk_fault("latest-wins admission lost its concrete factory").into());};let command=pending.command.as_ref().expect("original pending command");let target=(registration.latest_wins_target)(command.as_ref()).ok_or_else(||plugin_sdk_fault("latest-wins original command lost its borrowed target"))?;let parts=[&*self.store.envelope().id,&*registration.key.controller_id,&*registration.key.tool_id,target];
        if pending.key_copy.is_none(){if let Some((copy,progress))=ToolLatestWinsKeyCopy::admit_original(pending.meta.instance_id,parts,retained).map_err(|error|super::super::child_emit_preparation::retirement_refusal_fault(&error))?{pending.key_copy=Some(copy);pending.record_original_receipt(retained,progress)?;}return Ok(false);}
        let copy=pending.key_copy.as_mut().expect("original key byte cursor");
        if copy.part>4{if let Some((key,progress))=copy.take_key(retained).map_err(|error|super::super::child_emit_preparation::retirement_refusal_fault(&error))?{pending.key=Some(key);pending.record_original_receipt(retained,progress)?;}return Ok(false);}
        let step=copy.advance(parts,retained).map_err(|error|super::super::child_emit_preparation::retirement_refusal_fault(&error))?;pending.record_original_receipt(retained,step.progress())?;Ok(false)
    }

    /// 🧩️ The composed lane is PUBLISHED, not discarded: `dispatch_emit_group` has just
    /// built the parent's mutations, one per touched child under the child's own
    /// content-addressed handle, and the single `UndoGroup` that names them all. The
    /// three scalars of `receipt` are what the host ACKs; this is what the host READS.
    async fn publish_mounted_typed_child_operation_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if mounted.pending_child_publication.as_ref().is_some_and(|owner|matches!(owner.phase,PendingChildGroupPublicationPhase::Acknowledged|PendingChildGroupPublicationPhase::Closing)){return Ok(());}
        let Some(mut pending) = mounted.pending_child_publication.take() else {
            return Err(plugin_sdk_fault("typed-operation child publication lost its retained owner"));
        };
        match pending.phase {
            PendingChildGroupPublicationPhase::Acknowledged | PendingChildGroupPublicationPhase::Closing => {
                mounted.pending_child_publication=Some(pending);
                return Err(plugin_sdk_fault("original mounted child publication escaped its full-grant close frontier"));
            }
            PendingChildGroupPublicationPhase::Committed => {
                let receipt = pending.receipt.as_ref().cloned().ok_or_else(|| plugin_sdk_fault("committed child publication lost its exact result receipt"))?;
                mounted.published_artifact = true;
                self.record_typed_operation_lane(mounted);
                mounted.pending_child_publication = Some(pending);
                return mounted.queue_page(TypedOperationResultPage::try_serialize(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?);
            }
            PendingChildGroupPublicationPhase::Faulted => {
                let fault = pending.fault().unwrap_or_else(|| plugin_sdk_fault("typed-operation child publication fault lost its bounded detail"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
            PendingChildGroupPublicationPhase::Ready => {}
            PendingChildGroupPublicationPhase::Dispatching => {
                mounted.pending_child_publication = Some(pending);
                return Err(plugin_sdk_fault("typed-operation child publication re-entered a non-dispatchable phase"));
            }
        }
        match self.reclaim_child_retirements(1, 0) {
            Ok(true) => {}
            Ok(false) => {
                mounted.pending_child_publication = Some(pending);
                return Ok(());
            }
            Err(error) => {
                let fault = pending.reject_and_fault(&error);
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
        }
        let Some(_permit) = mounted.cancellation_lease.as_ref().and_then(ToolCancellationLease::try_claim_publication) else {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication claim is cancelled or occupied"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        };
        let live_revision = self.store.content_revision_now();
        let live_generation = self.store.generation_now();
        let Some(captured) = mounted.captured_child_content.as_ref() else {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication lost its captured immutable root"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        };
        let parent_fresh = typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation);
        let child_root_fresh = mounted.captured_child_content_generation == self.child_content_generation && captured.identity_digest() == self.child_content_root.identity_digest();
        if !parent_fresh || !child_root_fresh {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication rejected stale parent or child immutable roots"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        let mut seen = HashSet::with_capacity(pending.child_emits.len());
        for child in pending.child_emits.iter() {
            if !seen.insert((child.slot.clone(), child.child_id.clone())) {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication contains a duplicate exact target"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
            let expected_revision = match captured.revision(&child.slot, &child.child_id) {
                Ok(revision) => revision,
                Err(error) => {
                    let fault = pending.reject_and_fault(&error);
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
            };
            let Some(expected_dialect) = captured.dialect(&child.slot, &child.child_id) else {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication target is absent from its captured root"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            };
            let Some(entry) = self.children.get(&(child.slot.clone(), child.child_id.clone())) else {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication target is absent from the live member registry"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            };
            if entry.reference.dialect != expected_dialect || entry.member.content_revision().await != expected_revision {
                let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication rejected stale member identity, dialect, or revision"));
                mounted.pending_child_publication = Some(pending);
                return Err(fault);
            }
        }
        if mounted.cancellation_lease.as_ref().expect("claimed child publication retains its cancellation lease").is_cancelled().await {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication was cancelled before group linearization"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        if let Err(error) = pending.begin_dispatch() {
            let fault = pending.reject_and_fault(&error);
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        }
        let result = self.dispatch_emit_group(&mounted.verb, &pending.artifact_mutations, &pending.child_emits, Vec::new(), Vec::new(), UiDirtyScope::None, &mounted.meta, None, pending.transaction.as_ref().cloned(), identity).await;
        match result {
            Ok(result) => {
                mounted.artifact_generation = self.store.generation_now();
                mounted.canonical_revision = self.store.content_revision_now();
                mounted.operation.base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().expect("revision lane width")));
                mounted.operation.generation = semio_framework_job::Generation(mounted.artifact_generation);
                let receipt = ChildPublicationResultV1 { invocation_id: result.inverse_group.invocation_id.0.clone(), committed_members: result.inverse_group.member_edits.len(), child_content_generation: self.child_content_generation };
                if self.typed_composed_outbox.push(ComposedGestureResult { operation: mounted.operation.operation.0, mutations: result.mutations, inverse_group: result.inverse_group }).is_err() {
                    let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation composed gesture receiver is saturated"));
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
                if let Err(error) = pending.commit(receipt.clone()) {
                    let fault = pending.reject_and_fault(&error);
                    mounted.pending_child_publication = Some(pending);
                    return Err(fault);
                }
                mounted.pending_child_publication = Some(pending);
                mounted.queue_page(TypedOperationResultPage::try_serialize(mounted.next_token(), TypedOperationResultLane::Child, &receipt)?)
            }
            Err(fault) => {
                let fault = pending.reject_and_fault(&fault);
                mounted.pending_child_publication = Some(pending);
                Err(fault)
            }
        }
    }

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — THE point the reactor first owns a
    /// typed command's emit. The interaction verbs are peeled here, before this same call
    /// can reach the effect lane below: an emit with no durable lane (fem2d's
    /// `canvasPointerUp` is `effects: [interactionSelect], ui_scope: partial` and nothing
    /// else) used to fall straight through to `emit.effects.pop()` → `typed_effect_outbox`
    /// on the install pump, while `mounted_typed_inline_interaction_verbs_are_next` had
    /// never seen the emit (it gates on `mounted.publication`, which was still `None`
    /// when this pump's ladder branch was chosen). Per-turn law: no host-visible turn of a
    /// typed operation ever carries one of the six interaction verbs as an effect.
    ///
    /// 🪜️ Hand the pump back to the ladder so its own lane order picks the first unit: the
    /// `InteractionWrite` lane and the inline fold both precede the effect/`Ui`/terminal
    /// pages of this same emit, and the fold must land BEFORE the `Ui` page is minted so
    /// that page already carries the verbs' widened scope and their leftover view.
    ///
    /// 🔒️ The three emit-time backstops of contract §2.3 clause 2, §5.10 and the tool-run
    /// freeze live in `dispatch_emit_inner`, which the migrated publication ladder never
    /// calls for the single-store artifact lane (FP5 §3.4 measured the same thing for the
    /// kind guard above). A migrated dispatch answers with an admission receipt and its
    /// mutations only exist HERE, so this is the only point at which they can be refused.
    ///
    /// 🔀️ Composite-mutation proposal (contract §5.1): apply NOTHING and stash a
    /// `TransactionProposalDraft` instead, drained once by `plugin_exchange`.
    ///
    /// 🧺️ ONE gesture is ONE batched publication: the whole artifact lane is drained
    /// into a single staged `Edit` (one history ledger slot, one undo step), instead
    /// of one edit per mutation per turn — see `store::begin_apply_batch`.
    ///
    /// 🧵️ A config lane is one plain edit per emission (design §20.1): a continuous view/config gesture rides its press
    /// in the window transient and commits its ONE config edit at gesture end (`settle_press`), so nothing here folds
    /// ticks into an earlier edit.
    ///
    /// 🎚️ The authority was captured when this command was DISPATCHED, and a window's
    /// config partition is its OWN exact-base document: any earlier command that
    /// settled against the same window has already moved that partition's generation
    /// and revision on, so a second window-config command dispatched in the same turn
    /// carries a stale base by the time its batch begins. `begin_apply_batch` then
    /// refuses it — and because the mutation was already POPPED out of `emit`, the
    /// retry `advance_typed_operation_publication_unit` grants on a publication fault
    /// found nothing left to publish and the operation completed clean: no page, no
    /// fault, the amend lost (measured on flow as one window-config page for two
    /// commands, ticket 26/09/19 `📓️flow.md` §5.2). Two repairs, both of which every
    /// sibling lane of this ladder already had: re-read the authority before beginning
    /// (the window-TRANSIENT arm below always did), and hand the mutation BACK to
    /// `emit` when admission refuses it, so a retry is a real retry and an exhausted
    /// one refuses by name on the Fault lane instead of vanishing.
    ///
    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (design-abi.md §4) — the
    /// task lane, ONE task per publication unit. The unit spends itself here
    /// and hands the pump straight back to the ladder without a host page of
    /// its own: a spawn is actor-local bookkeeping, and the task's eventual
    /// `TaskResolution` re-enters as its own follow-up dispatch
    /// (`drain_task_resumes`), never as this operation's result. `remove(0)`
    /// because spawn order is observable — a same-key respawn cancels the
    /// live task under that key, so the LAST declared task under a key must
    /// be the one that survives. A refused spawn (quota, key supersession,
    /// executor capacity) is this operation's own `Fault`: the ladder above
    /// gives the slot a chance to free and then mints its terminal fault page.
    fn handoff_mounted_durable_publication(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>) -> Result<bool, Fault> {
        let Some(pending) = mounted.pending_artifact_publication.take() else {
            return Ok(false);
        };
        let (result, restore) = match pending {
            PendingArtifactStorePublication::Artifact(publication) => {
                let mut owner = Some(publication);
                let result = self.store.handoff_batch_publication_close(&mut owner).map_err(|error| plugin_sdk_fault(error.to_string()));
                (result, owner.map(PendingArtifactStorePublication::Artifact))
            }
            PendingArtifactStorePublication::Config(publication) => {
                let mut owner = Some(publication);
                let result = self.config_store.handoff_batch_publication_close(&mut owner).map_err(|error| plugin_sdk_fault(error.to_string()));
                (result, owner.map(PendingArtifactStorePublication::Config))
            }
            PendingArtifactStorePublication::Draft(publication) => {
                let mut owner = Some(publication);
                let result = self.draft_store.handoff_batch_publication_close(&mut owner).map_err(|error| plugin_sdk_fault(error.to_string()));
                (result, owner.map(PendingArtifactStorePublication::Draft))
            }
            pending => {
                mounted.pending_artifact_publication = Some(pending);
                return Ok(false);
            }
        };
        if let Some(owner) = restore {
            mounted.pending_artifact_publication = Some(owner);
        }
        result
    }

    /// 🪟️ Continues the window-config authority refresh a published receipt retains until the displaced snapshot has admitted custody, then queues the retained receipt's result page; every unit reports its actual progress to the host turn.
    fn advance_mounted_window_config_refresh(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, retained: RetainedCloneGrant) -> Result<(), Fault> {
        let receipt = mounted.pending_window_config_receipt.ok_or_else(|| plugin_sdk_fault("window config refresh lost its retained publication receipt"))?;
        let authority = mounted.window_config_authority.as_mut().ok_or_else(|| plugin_sdk_fault("window config receipt lost its captured window authority"))?;
        self.window_config_store.refresh_demands(authority, retained.maximum_copy_bytes).map_err(plugin_retirement_fault)?;
        let step = self.window_config_store.refresh(authority, retained)?;
        let (super::super::window_mutation::WindowAuthorityRefreshStep::Pending(progress) | super::super::window_mutation::WindowAuthorityRefreshStep::Ready(progress)) = step;
        if progress != RetainedCloneProgress::default() {
            assert!(mounted.publication_ownership_progress.is_none(), "window config refresh cannot overwrite an unreturned original receipt");
            mounted.publication_ownership_progress = Some((retained, progress));
            mounted.publication_progress = mounted.publication_progress.saturating_add(1);
        }
        if matches!(step, super::super::window_mutation::WindowAuthorityRefreshStep::Pending(_)) { return Ok(()); }
        mounted.pending_window_config_receipt = None;
        self.tool_runs.note_window_config_published();
        let token = mounted.next_token();
        mounted.queue_page(TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::WindowConfig, &receipt)?)
    }

    pub(crate) fn publish_mounted_typed_operation_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, retained:RetainedCloneGrant) -> Result<(), Fault> {
        if mounted.reject_cancelled_publication()? {
            self.handoff_mounted_durable_publication(mounted)?;
            return Ok(());
        }
        let _permit = mounted.cancellation_lease.as_ref().and_then(ToolCancellationLease::try_claim_publication).ok_or_else(|| plugin_sdk_fault("typed-operation publication claim is cancelled or occupied"))?;
        if mounted.publication.is_none() {
            let Some(completion) = mounted.completion.as_ref() else { return Err(plugin_sdk_fault("typed-operation completion owner was retired before publication")) };
            let Some(mut publication) = completion.take()? else { return Ok(()) };
            mounted.ui_pending = matches!(publication, ArtifactToolCompletionValue::Emit(Ok(_), _));
            self.settle_tool_operation(mounted, &mut publication)?;
            if let ArtifactToolCompletionValue::Emit(Ok(emit), _) = &mut publication {
                let verbs = take_inline_interaction_verbs(&mut emit.effects);
                if !verbs.is_empty() {
                    self.typed_inline_interaction_verbs.retain(|(operation, _)| *operation != mounted.operation.operation.0);
                    self.typed_inline_interaction_verbs.push((mounted.operation.operation.0, verbs));
                }
            }
            mounted.publication = Some(publication);
            if Self::mounted_typed_interaction_writes_are_next(mounted) || self.mounted_typed_inline_interaction_verbs_are_next(mounted) {
                return Ok(());
            }
        }
        let live_revision = self.store.content_revision_now();
        let live_generation = self.store.generation_now();
        if !typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation) {
            if self.handoff_mounted_durable_publication(mounted)? {
                return Err(plugin_sdk_fault("typed-operation pending publication rejected a stale immutable document root"));
            }
            if let Some(pending)=mounted.pending_artifact_publication.as_mut(){pending.begin_close();}
            return Err(plugin_sdk_fault("typed-operation pending publication rejected a stale immutable document root"));
        }
        if mounted.pending_artifact_publication.is_none() && matches!(mounted.publication.as_ref(),Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))if !emit.artifact_mutations.is_empty()||!emit.config_mutations.is_empty()||!emit.draft_mutations.is_empty()||!emit.window_config_mutations.is_empty()) && mounted.admit_actor_capture(retained)? {return Ok(());}
        if let Some(pending) = mounted.pending_artifact_publication.as_mut() {
            let grant=store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(retained);
            let (pending_lane, _) = pending.lane();
            if pending.is_closing(){return Ok(());}
            if mounted.pending_window_config_receipt.is_some() { return self.advance_mounted_window_config_refresh(mounted, retained); }
            let pending = mounted.pending_artifact_publication.as_mut().expect("pending publication checked above");
            if matches!(pending, PendingArtifactStorePublication::Artifact(_))
                && self.store.returned_snapshot_read_count() > PUBLICATION_RETURNED_ROOT_ALLOWANCE
                && !self.reclaim_document_snapshot_read_returns()?
            {
                return Ok(());
            }
            let pending = mounted.pending_artifact_publication.as_mut().expect("pending publication checked above");
            let advance = match pending {
                PendingArtifactStorePublication::Artifact(publication) => self.store.advance_apply_batch(publication, grant).map_err(store_publication_fault)?,
                PendingArtifactStorePublication::Config(publication) => self.config_store.advance_apply_batch(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Draft(publication) => self.draft_store.advance_apply_batch(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Presence(publication) => self.presence_store.advance_publish_one(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Transient(publication) => self.transient_store.advance_publish_one(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::WindowConfig(publication) => self.window_config_store.advance(publication.as_mut(), grant)?,
                PendingArtifactStorePublication::WindowTransient(publication) => self.window_transient_store.advance(publication.as_mut(), grant)?,
            };
            return match advance {
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(receipt) => {
                    match pending_lane {
                        TypedOperationResultLane::Artifact => {
                            mounted.published_artifact = true;
                            self.record_typed_operation_lane(mounted);
                            mounted.artifact_generation = receipt.generation_after;
                            mounted.canonical_revision = self.store.content_revision_now();
                            mounted.operation.base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().expect("revision lane width")));
                            mounted.operation.generation = semio_framework_job::Generation(receipt.generation_after);
                        }
                        TypedOperationResultLane::Config => {
                            mounted.published_config = true;
                            mounted.config_generation = receipt.generation_after;
                        }
                        TypedOperationResultLane::Draft => mounted.draft_generation = receipt.generation_after,
                        TypedOperationResultLane::Presence => mounted.presence_generation = receipt.generation_after,
                        TypedOperationResultLane::Transient => mounted.transient_generation = receipt.generation_after,
                        TypedOperationResultLane::WindowConfig => {
                            mounted.published_window_config = true;
                            mounted.pending_window_config_receipt = Some(receipt);
                            return self.advance_mounted_window_config_refresh(mounted, retained);
                        }
                        TypedOperationResultLane::WindowTransient => {
                            let authority = mounted.window_transient_authority.as_mut().ok_or_else(|| plugin_sdk_fault("window transient receipt lost its captured window authority"))?;
                            authority.generation = receipt.generation_after;
                        }
                        _ => unreachable!("pending artifact publication has one durable store lane"),
                    }
                    let token = mounted.next_token();
                    mounted.queue_page(TypedOperationResultPage::try_serialize(token, pending_lane, &receipt)?)
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Progress(checkpoint) => {
                    mounted.note_publication_checkpoint(checkpoint);
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(checkpoint, ownership) => {
                    if !ownership.fits(grant.retained_grant())||grant.maximum_depth==0&&ownership!=RetainedCloneProgress::default(){return Err(plugin_sdk_fault("typed preparation violated its original full grant"));}
                    assert!(mounted.publication_ownership_progress.is_none(),"typed preparation cannot overwrite an unreturned original receipt");
                    mounted.publication_ownership_progress=Some((retained,ownership));
                    mounted.note_publication_checkpoint(checkpoint);
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationRefused => {
                    let reason=pending.preparation_refusal().ok_or_else(||plugin_sdk_fault("typed preparation refused without its original retained failure"))?;
                    let ownership=reason.retained_progress();
                    assert!(mounted.publication_ownership_progress.is_none(),"typed refusal cannot overwrite its unreturned actual receipt");
                    mounted.publication_ownership_progress=Some((retained,ownership));
                    Ok(())
                }
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Blocked => Ok(()),
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::AwaitingAck(_) => Err(plugin_sdk_fault("artifact-store publication awaits an ACK without its retained host page")),
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Complete => Err(plugin_sdk_fault("artifact-store publication completed before result ACK")),
            };
        }
        let token = mounted.next_token();
        let publication_lanes = mounted.publication_lanes;
        let page = match mounted.publication.as_mut().expect("publication owner was installed") {
            ArtifactToolCompletionValue::Emit(Err(fault), _) | ArtifactToolCompletionValue::Download(Err(fault), _) => {
                let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];
                let len = fault.framed_page_bytes(&mut framed);
                TypedOperationResultPage::try_new(token, TypedOperationResultLane::Fault, &framed[..len])?
            }
            ArtifactToolCompletionValue::Download(Ok(download), ephemeral) => {
                if (!ephemeral.presence.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Presence))
                    || (!ephemeral.transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Transient))
                    || (!ephemeral.window_transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowTransient))
                {
                    return Err(plugin_sdk_fault("typed-operation emitted an undeclared ephemeral publication lane"));
                }
                if let Some(mutation) = ephemeral.presence.pop() {
                    match self.presence_store.begin_publish_one(mounted.operation.operation, mounted.presence_generation, mutation, self.presence_one_item_factory.as_deref(), self.presence_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Presence(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.presence.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.transient.pop() {
                    match self.transient_store.begin_publish_one(mounted.operation.operation, mounted.transient_generation, mutation, self.transient_one_item_factory.as_deref(), self.transient_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Transient(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.transient.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.window_transient.pop() {
                    let authority = match mounted.window_transient_authority.as_mut() {
                        Some(authority) => authority,
                        None => {
                            ephemeral.window_transient.push(mutation);
                            return Err(plugin_sdk_fault("window transient emission requires one exact captured ViewModel window authority"));
                        }
                    };
                    let grant=self.mounted_policy.maintenance;
                    let refresh=match self.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>self.window_transient_store.refresh(authority,grant),Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault);}}
                    match self.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowTransient(publication));
                            return Ok(());
                        }
                        Err(super::super::window_transient::RejectedWindowTransientEmission { mutation, fault }) => {
                            ephemeral.window_transient.push(mutation);
                            return Err(fault);
                        }
                    }
                } else {
                    TypedOperationResultPage::try_serialize(
                        token,
                        TypedOperationResultLane::Download,
                        &DownloadResultPayload { filename: download.filename.as_str(), mime_type: download.mime_type.as_str(), encoding: download.encoding.as_deref(), bytes: download.chunks.bytes() },
                    )?
                }
            }
            ArtifactToolCompletionValue::Emit(Ok(emit), ephemeral) => {
                let preparation=match emit.prepare_child_one(retained){Ok(step)=>step,Err(fault)=>{
                    let progress=fault.retained_progress();if !progress.fits(retained){return Err(plugin_sdk_fault("original child preparation error exceeded mounted caller policy"));}
                    if progress!=RetainedCloneProgress::default(){assert!(mounted.publication_ownership_progress.is_none(),"child preparation error cannot overwrite an original receipt");mounted.publication_ownership_progress=Some((retained,progress));}return Err(fault);
                }};
                let progress=preparation.progress();
                if !progress.fits(retained){return Err(plugin_sdk_fault("original emitted child preparation exceeded mounted caller policy"));}
                if progress!=RetainedCloneProgress::default(){
                    assert!(mounted.publication_ownership_progress.is_none(),"child preparation cannot overwrite an original receipt");mounted.publication_ownership_progress=Some((retained,progress));return Ok(());
                }
                match preparation{ChildEmitPreparationStep::Ready(_)=>{},ChildEmitPreparationStep::Pending(_)=>return Ok(()),ChildEmitPreparationStep::Refused(fault,_)=>return Err(fault)}
                if (!emit.artifact_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Artifact))
                    || (!emit.config_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Config))
                    || (!emit.window_config_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowConfig))
                    || (!emit.draft_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Draft))
                    || (!ephemeral.presence.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Presence))
                    || (!ephemeral.transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Transient))
                    || (!ephemeral.window_transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::WindowTransient))
                    || ((!emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty()) && !publication_lanes.contains(&ArtifactToolPublicationLane::Child))
                    || (!emit.interaction_writes.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Interaction))
                {
                    return Err(plugin_sdk_fault("typed-operation emitted a store lane absent from its exact factory publication contract"));
                }
                if !emit.artifact_mutations.is_empty() || !emit.owned_child_emits.is_empty() {
                    self.require_operation_emitting_kind(&mounted.verb, self.declared_dispatch_kind(&mounted.verb))?;
                    if A::ROLE == AppRole::Viewer {
                        return Err(viewer_read_only_fault(&mounted.verb));
                    }
                    if self.tool_runs.freezes_local_emits() {
                        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("toolRun.busy"), format!("verb {:?} would emit artifact mutations while a freezing tool run is non-terminal on this instance", mounted.verb)));
                    }
                    if let Some(pending_txn_id) = self.pending_transaction.as_ref().map(|pending| pending.txn_id.clone()) {
                        return Err(Self::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", format!("verb {:?} would emit artifact mutations while transaction {pending_txn_id:?} is pending on this instance", mounted.verb)));
                    }
                    if emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
                        let mut running = self.store.snapshot().map_err(|error| error.into_fault())?;
                        let foreign = Vec::new();
                        for op in emit.artifact_mutations.iter() {
                            if op.foreign_step_source(&running, 0).map_err(|error| error.into_fault())?.is_some() { return Err(plugin_sdk_fault("foreign transaction output awaits its original staged preparation recipient")); }
                            let outcome = op.diff(&running);
                            if !outcome.is_applicable(protocol::MergePolicy::default()) {
                                return Err(Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                            }
                            running = protocol::apply_diff(outcome.diff(), &running).map_err(|error| Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
                        }
                        if !foreign.is_empty() {
                            let mut local_ops = Vec::with_capacity(emit.artifact_mutations.len());
                            for op in emit.artifact_mutations.iter() {
                                local_ops.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
                            }
                            self.pending_transaction_proposal = Some(TransactionProposalDraft { local_ops, foreign });
                            emit.artifact_mutations.clear();
                            return Ok(());
                        }
                    }
                }
                if let Some(fault) = tool_transaction_shape_fault(
                    &mounted.verb,
                    emit.transaction.as_ref(),
                    emit.transaction_phase,
                    !emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty(),
                    !emit.artifact_mutations.is_empty(),
                    emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps),
                ) {
                    return Err(fault);
                }
                if (!emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty()) && self.time_travel.freezes_local_emits() {
                    return Err(time_travel_frozen_fault(&mounted.verb));
                }
                self.close_streamed_transaction_unit(emit)?;
                if !emit.owned_child_emits.is_empty() {
                    if !emit.child_emits.is_empty() || emit.owned_child_emits.len() > PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN { return Err(plugin_sdk_fault("mounted owned child group requires its one exact admitted typed source family")); }
                    let operation = mounted.operation.operation.0;
                    let captured_millis = i64::try_from(store::os_store::document_materialize::now_ms()).map_err(|_|plugin_sdk_fault("mounted child publication requires its original UTC clock capture"))?;
                    let grant=retained;
                    let step=self.admit_private_child_group_frame(operation,captured_millis,u64::MAX,grant)?;
                    let progress=step.progress();
                    if !progress.fits(grant){return Err(plugin_sdk_fault("mounted child frame exceeded its original preparation grant"));}
                    if progress!=RetainedCloneProgress::default(){
                        assert!(mounted.publication_ownership_progress.is_none(),"mounted child frame cannot overwrite its original unreturned receipt");
                        mounted.publication_ownership_progress=Some((retained,progress));
                    }
                    if self.private_child_groups.get(operation).is_some(){mounted.owned_child_group=Some(operation);}
                    return Ok(());
                }
                if emit.child_emits.is_empty() && emit.owned_child_emits.is_empty() && !emit.artifact_mutations.is_empty() {
                    if let Some(fault) = self.open_transaction_admission_fault(emit.transaction.as_ref()) {
                        return Err(fault);
                    }
                    if let Err(error) = store::os_store::one_item::ArtifactStoreOneItemFootprint::for_gesture::<A::Snapshot, A::Mutation>(&emit.artifact_mutations) {
                        return Err(error.into_fault());
                    }
                    let streams=emit.transaction_phase==TransactionPhase::Stream;
                    let outbound=self.store.backbone_ref().is_some();
                    if let Some((mut publication,ownership))=self.store.admit_apply_batch(mounted.operation.operation,mounted.artifact_generation,mounted.canonical_revision,&mut mounted.actor_capture,&mut emit.artifact_mutations,HistoryLane::Document,self.artifact_one_item_factory.as_ref(),&mut emit.transaction,Some(&mounted.verb),outbound,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        publication.set_transaction_open(streams);
                        emit.transaction_phase=TransactionPhase::Commit;
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Artifact(publication));
                    }
                    return Ok(());
                }
                if !emit.config_mutations.is_empty() {
                    let revision=self.config_store.content_revision_now();
                    let mut transaction=None;
                    if let Some((publication,ownership))=self.config_store.admit_apply_batch(mounted.operation.operation,mounted.config_generation,revision,&mut mounted.actor_capture,&mut emit.config_mutations,HistoryLane::Document,self.config_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Config(publication));
                    }
                    return Ok(());
                } else if !emit.window_config_mutations.is_empty() {
                    let authority = mounted.window_config_authority.as_ref().ok_or_else(|| plugin_sdk_fault("window config emission requires one exact captured ViewModel window authority"))?;
                    if let Some((publication,ownership))=self.window_config_store.admit_begin(mounted.operation.operation,&mut mounted.actor_capture,&mut emit.window_config_mutations,authority,retained).map_err(|error|error.into_fault())? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::WindowConfig(publication));
                    }
                    return Ok(());
                } else if !emit.draft_mutations.is_empty() {
                    let revision=self.draft_store.content_revision_now();
                    let mut transaction=None;
                    if let Some((publication,ownership))=self.draft_store.admit_apply_batch(mounted.operation.operation,mounted.draft_generation,revision,&mut mounted.actor_capture,&mut emit.draft_mutations,HistoryLane::Document,self.draft_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Draft(publication));
                    }
                    return Ok(());
                } else if let Some(mutation) = ephemeral.presence.pop() {
                    match self.presence_store.begin_publish_one(mounted.operation.operation, mounted.presence_generation, mutation, self.presence_one_item_factory.as_deref(), self.presence_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Presence(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.presence.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.transient.pop() {
                    match self.transient_store.begin_publish_one(mounted.operation.operation, mounted.transient_generation, mutation, self.transient_one_item_factory.as_deref(), self.transient_local_root_retirement_factory.clone()) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Transient(publication));
                            return Ok(());
                        }
                        Err(rejected) => {
                            let (reason, mutation) = rejected.into_owners();
                            ephemeral.transient.push(mutation);
                            return Err(plugin_sdk_fault(reason));
                        }
                    }
                } else if let Some(mutation) = ephemeral.window_transient.pop() {
                    let authority = match mounted.window_transient_authority.as_mut() {
                        Some(authority) => authority,
                        None => {
                            ephemeral.window_transient.push(mutation);
                            return Err(plugin_sdk_fault("window transient emission requires one exact captured ViewModel window authority"));
                        }
                    };
                    let grant=self.mounted_policy.maintenance;
                    let refresh=match self.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>self.window_transient_store.refresh(authority,grant),Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault);}}
                    match self.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
                        Ok(publication) => {
                            mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::WindowTransient(publication));
                            return Ok(());
                        }
                        Err(super::super::window_transient::RejectedWindowTransientEmission { mutation, fault }) => {
                            ephemeral.window_transient.push(mutation);
                            return Err(fault);
                        }
                    }
                } else if !emit.child_emits.is_empty() {
                    if mounted.pending_child_publication.is_some() {
                        return Err(plugin_sdk_fault("typed-operation retained more than one child-group publication owner"));
                    }
                    mounted.pending_child_publication = Some(PendingChildGroupPublication::new(std::mem::take(&mut emit.artifact_mutations), std::mem::take(&mut emit.child_emits), emit.transaction.take())?);
                    return Ok(());
                } else {
                    Self::mint_extension_invocations(mounted.meta.instance_id, emit)?;
                    if let Some(effect) = emit.effects.pop() {
                        let effect = self.stamp_load_document_identity(effect)?;
                        if let Err(effect) = self.typed_effect_outbox.push(effect) {
                            emit.effects.push(effect);
                            return Err(plugin_sdk_fault("typed-operation effect receiver is saturated"));
                        }
                        TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Effect, &("accepted", self.typed_effect_outbox.len()))?
                    } else if let Some(event) = emit.events.pop() {
                        if let Err(event) = self.typed_event_outbox.push(event) {
                            emit.events.push(event);
                            return Err(plugin_sdk_fault("typed-operation event receiver is saturated"));
                        }
                        TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Event, &("accepted", self.typed_event_outbox.len()))?
                    } else {
                        match () {
                            () if !emit.tasks.is_empty() => {
                                let task = emit.tasks.remove(0);
                                crate::reactor::spawn_task(mounted.meta.instance_id, &mounted.meta, task)?;
                                return Ok(());
                            }
                            () if mounted.ui_pending => {
                                if let Err(_scope) = self.typed_ui_outbox.push(emit.ui_scope.clone()) {
                                    return Err(plugin_sdk_fault("typed-operation UI receiver is saturated"));
                                }
                                mounted.ui_pending = false;
                                TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Ui, &emit.ui_scope)?
                            }
                            () => {
                                if self.typed_completion_outbox.push(TypedOperationCompletionWitness { operation: token.operation, ui_scope: emit.ui_scope.clone() }).is_err() {
                                    return Err(plugin_sdk_fault("typed-operation completion receiver is saturated"));
                                }
                                TypedOperationResultPage::try_new(token, TypedOperationResultLane::Terminal, b"typed-operation-complete")?
                            }
                        }
                    }
                }
            }
        };
        mounted.queue_page(page)
    }

    pub(super) fn require_tool_operation_authority(&self, admission: &AdmittedToolCommand) -> Result<(), Fault> {
        let contract = admission.proof.contract();
        let exact = admission.decoded_items <= contract.max_decoded_items
            && contract.max_work_units_per_step != 0
            && contract.max_output_bytes != 0
            && contract.max_step_micros != 0
            && contract.max_step_micros < semio_framework::ToolExecutionContract::INTERACTIVE_MAX_STEP_MICROS
            && matches!(contract.cancellation, semio_framework::ToolCancellationPolicy::PerOperation)
            && matches!(contract.freshness, semio_framework::ToolFreshnessPolicy::ValidateImmediatelyBeforeExposure)
            && matches!(contract.shape, semio_framework::ToolExecutionShape::Resumable | semio_framework::ToolExecutionShape::BoundedFirstStep);
        if !exact {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.incomplete-operation-authority"), format!("typed command '{}' has no complete bounded prepare/reducer/commit authority", admission.verb)));
        }
        Ok(())
    }

    pub(super) fn require_complete_tool_operation_pipeline(&self, admission: &AdmittedToolCommand) -> Result<(), Fault> {
        self.require_tool_operation_authority(admission)?;
        match admission.proof {
            QualifiedToolProof::AppOwned(_) => Ok(()),
            QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::Bounded(_) => {
                Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-reducer"), format!("typed command '{}' has no exact app-owned retained decoder/reducer factory", admission.verb)))
            }
        }
    }

    pub(crate) fn can_admit_typed_operation(&self, operation: u64) -> bool {
        self.tool_operations.can_insert(operation) && self.typed_operation_reservations[operation as usize % ARTIFACT_LIVE_OUTPUT_SLOTS].is_none()
    }

    /// 🎰️ Whether every fixed typed-operation and segmented-output authority owns nothing in this one
    /// residue class of `id % ARTIFACT_LIVE_OUTPUT_SLOTS`.
    pub(crate) fn typed_operation_slot_is_vacant(&self, slot: usize) -> bool {
        self.tool_operations.slot_is_vacant(slot)
            && self.typed_operation_reservations[slot].is_none()
            && self.latest_wins_commands.slot_is_vacant(slot)
            && self.pending_reserved.slot_is_vacant(slot)
            && self.segmented_downloads.slot_is_vacant(slot)
            && self.segmented_closures.slot_is_vacant(slot)
    }

    /// 🎰️ How many of the fixed residue classes currently own a live typed operation or segmented
    /// output — the exact complement of what [`Self::admit_typed_operation_slot`] can still hand out,
    /// so `0` is the only value that proves every admitted slot reached its release path.
    pub(super) fn live_typed_operation_slots(&self) -> usize {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).filter(|slot| !self.typed_operation_slot_is_vacant(*slot)).count()
    }

    /// 🎫️ Pre-admits the exact operation slot BEFORE an operation id exists. Every typed authority is a
    /// direct-mapped table addressed by `id % ARTIFACT_LIVE_OUTPUT_SLOTS`, so minting a shared-counter id
    /// first and then testing its one residue class refuses an action whenever an unrelated live
    /// operation happens to share that class — a hover/tick/camera storm then fails while 63 slots sit
    /// empty. Ingress therefore chooses the vacant slot and mints the id for it, and refusal now means
    /// actual saturation of all fixed slots.
    pub(crate) fn admit_typed_operation_slot(&self) -> Option<semio_framework_job::OperationId> {
        if !self.latest_wins_order.allocation_admitted || self.latest_wins_order.len() >= ARTIFACT_LIVE_OUTPUT_SLOTS {
            return None;
        }
        let slot = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot| self.typed_operation_slot_is_vacant(*slot))?;
        Some(semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, slot as u64))
    }

    pub(crate) async fn dispatch_typed_command_inner(&mut self, command: Box<A::Command>, admission: AdmittedToolCommand, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if self.live_runtime_instance_id != Some(meta.instance_id) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.live-instance"), format!("typed command '{}' does not belong to the mounted live app instance", admission.verb)));
        }
        self.require_complete_tool_operation_pipeline(&admission)?;
        self.deliver_utility_host_events(meta, identity, cx).await?;
        let verb = A::command_id(&command).await.to_string();
        if admission.verb != verb {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.command-identity"), format!("exact admitted command '{}' decoded as '{verb}'", admission.verb)));
        }
        if let Some(fault) = self.document_loading_refusal(&verb) {
            return Err(fault);
        }
        let Some(operation_id) = self.admit_typed_operation_slot() else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-capacity"), "every fixed typed-operation and segmented-output slot already owns a live operation"));
        };
        if self.delivering_host_event {
            self.host_event_operations.insert(operation_id.0);
        }
        if let Some(tag) = self.tool_machines.ingress.take() {
            self.tool_machines.bind(operation_id.0, tag);
        }
        if let QualifiedToolProof::AppOwned(registration) = &admission.proof {
            {
                let ordinary = (registration.latest_wins_target)(command.as_ref()).is_none();
                let disposer = (registration.latest_wins_command_disposer)()
                    .and_then(|owner| owner.downcast::<Box<dyn ArtifactOwnedDisposer<A::Command>>>().ok())
                    .map(|owner| *owner)
                    .ok_or_else(|| plugin_sdk_fault("latest-wins target requires its exact compiler-owned command retirement factory"))?;
                let revision = self.store.content_revision_now();
                let generation = semio_framework_job::Generation(self.store.generation_now());
                let base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().expect("revision lane width")));
                let operation = semio_framework_job::Operation::new(operation_id, base_revision, generation, operation_id.0);
                self.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = Some(operation_id.0);
                self.latest_wins_commands.insert_admitted(
                    operation_id.0,
                    PendingLatestWinsCommand {
                        command: ManuallyDrop::new(Some(command)),
                        disposer,
                        admission: Some(admission),
                        meta: meta.clone(),
                        operation,
                        source_identity:(cx.operation(),cx.generation()),
                        revision,
                        lease: None,
                        key_copy: None,
                        key: None,
                        retired_key: None,
                        lookup_started: false,
                        accepted: ordinary,
                        restarting: false,
                        closing: false,
                        receipt:None,
                        fault:ManuallyDrop::new(None),fault_retirement:None,
                    },
                );
                self.latest_wins_order.push(operation_id.0).expect("exact FIFO command slot was pre-admitted");
                let mut started = Self::empty_result(&verb, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
                started.output = DslValue::Object(vec![("operationId".into(), DslValue::String(operation_id.0.to_string())), ("generation".into(), DslValue::String(generation.0.to_string()))]);
                return Ok(started);
            }
        }
        Err(plugin_sdk_fault("typed admission requires its exact retained command owner"))
    }

    /// 📸️ The roots one typed command runs against, captured once and identically for the shell lane
    /// that mounts the operation ([`Self::start_typed_command_operation`]) and for the agent lane's prepare
    /// phase that only previews it ([`Self::preview_addressed_action`]) — so the two lanes can never run
    /// the same verb against different state.
    pub(super) async fn capture_typed_command_roots(&mut self, command: &A::Command, meta: &ActionMeta) -> Result<TypedCommandRoots<A>, Fault> {
        self.refresh_cache().await?;
        let (snapshot, config, history) = self.command_cache_inputs();
        let canonical_base_revision = self.store.content_revision();
        let window_config_authority = self.window_config_store.capture(meta.view_state.as_ref()).await?;
        let targeted_window_transient_view = match A::retained_window_transient_target(command) {
            Some((window_id, expected_kind)) => {
                let view = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an exact ViewModel roster"))?;
                let window = view.window_instances.iter().find(|window| window.id == window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an attached window instance"))?;
                if window.window_kind_id != expected_kind {
                    return Err(plugin_sdk_fault("targeted window transient capture does not match the command owner's expected window kind"));
                }
                Some(view.for_window_instance(window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture lost its attached window instance"))?)
            }
            None => None,
        };
        let window_transient_authority = self.window_transient_store.capture(targeted_window_transient_view.as_ref().or(meta.view_state.as_ref()))?;
        Ok(TypedCommandRoots {
            snapshot,
            config,
            history,
            children: std::sync::Arc::new(ChildContentView::clone(&self.child_content_root)),
            draft_snapshot: self.draft_store.snapshot_root(),
            interaction_state: self.interaction_store.snapshot_root(),
            interaction_hover: self.interaction_hover.clone(),
            presence_peers: self.presence_store.peers_root(),
            transient: self.transient_store.current_root(),
            peer_presence: std::sync::Arc::clone(&self.peer_presence),
            canonical_base_revision,
            base_revision: semio_framework_job::RevisionId(u64::from_be_bytes(canonical_base_revision[..8].try_into().expect("revision lane width"))),
            generation: semio_framework_job::Generation(self.store.generation()),
            config_generation: self.config_store.generation(),
            draft_generation: self.draft_store.generation(),
            presence_generation: self.presence_store.generation().await,
            transient_generation: self.transient_store.generation().await,
            window_config_authority,
            window_transient_authority,
        })
    }

    /// 🧪️ Observes actual capture authority without exposing production dispatch preparation.
    #[cfg(test)]
    pub(crate) async fn test_capture_presence_roots(&mut self, command: &A::Command, meta: &ActionMeta) -> Result<(std::sync::Arc<PeerPresenceRoot>, std::sync::Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>), Fault> {
        let TypedCommandRoots { peer_presence, presence_peers, .. } = self.capture_typed_command_roots(command, meta).await?;
        Ok((peer_presence, presence_peers))
    }

    /// 🧪️ Mounts an exact typed admission so laws can observe the boundary before producer completion.
    #[cfg(test)]
    pub(crate) async fn test_mount_typed_command(&mut self, command: A::Command, verb: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let payload = ::protocol::OpBinary::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.admit_command_wire(verb, &payload, 1).await?;
        self.dispatch_typed_command_inner(Box::new(command), admission, meta, identity, cx).await
    }

    /// 🧵️ Captures immutable roots and mounts one admitted worker turn. Ephemeral producers
    /// execute inside their app-owned job and publish through its declared completion lanes.
    async fn start_typed_command_operation(
        &mut self,
        command: Box<A::Command>,
        admission: AdmittedToolCommand,
        meta: &ActionMeta,
        operation_id: semio_framework_job::OperationId,
        cancellation_lease: ToolCancellationLease,
     cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        assert!(self.tool_operations.can_insert(operation_id.0), "typed worker handoff retains its exact empty operation slot");
        assert_eq!(self.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS], Some(operation_id.0), "worker handoff owns its exact pending reservation");
        let verb = admission.verb.clone();
        let TypedCommandRoots {
            snapshot,
            config,
            history,
            children,
            draft_snapshot,
            interaction_state,
            interaction_hover,
            presence_peers,
            transient,
            peer_presence,
            canonical_base_revision,
            base_revision,
            generation,
            config_generation,
            draft_generation,
            presence_generation,
            transient_generation,
            window_config_authority,
            window_transient_authority,
        } = self.capture_typed_command_roots(command.as_ref(), meta).await?;
        let parent_document_id = self.store.envelope().id.clone();
        let seed_handle = artifact_handle_of(&format!("{}/{}/{verb}/{}", meta.instance_id, self.tool_job_controller_id, base_revision.0)).await;
        let operation = semio_framework_job::Operation::new(operation_id, base_revision, generation, (seed_handle.0 as u64) ^ ((seed_handle.0 >> 64) as u64));
        assert_eq!(cancellation_lease.key.base_revision, base_revision, "fresh worker receives a revision-rebound cancellation lease");
        assert_eq!(cancellation_lease.key.generation, generation, "fresh worker receives a generation-rebound cancellation lease");
        let payload_schema_id = admission.proof.schema_id();
        let qualified_key = admission.proof.key();
        let publication_lanes = admission.proof.publication_lanes();
        let completion = ArtifactToolCompletion::<A>::new();
        let mut identity_observer = |_: semio_framework_value::native_encoding::NativeEncodeProgress| !cancellation_lease.token.is_cancelled_now() && !cancellation_lease.publication_claim.is_cancelled();
        let identity = semio_framework_value::NativeEncodeControl::new(admission.proof.contract().max_output_bytes, &mut identity_observer).pause().map_err(|error| plugin_sdk_fault(error.to_string()))?;
        drop(identity_observer);
        completion.install_authoring_identity(identity, &cancellation_lease)?;
        let retained_wire_admission = admission.wire_admission.clone();
        let app_owned_retained_route = matches!(&admission.proof, QualifiedToolProof::AppOwned(_));
        let mut retained_wire = Some(admission.raw_wire);
        let raw_input = if app_owned_retained_route { ArtifactToolRawInput::transferred_to_factory() } else { ArtifactToolRawInput::new(retained_wire.take().expect("generic retained wire owner remains mounted")) };
        let output_chunks = ArtifactOutputChunks::new(admission.proof.contract().max_output_bytes);
        let mut request_snapshot_read = if app_owned_retained_route { Some(self.store.snapshot_read().map_err(FaultFrom::into_fault)?) } else { None };
        let mut request_window_config = if app_owned_retained_route { window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())? } else { None };
        let owned_context = if app_owned_retained_route {
            let presence_local = self.presence_store.local_read().map_err(Fault::from)?;
            let gesture = self.admit_gesture_slot(operation_id.0, &canonical_base_revision, meta,artifact_owned_tool_birth_grant(),artifact_owned_tool_handoff_grant())?;
            Some(ArtifactOwnedContextHandle::admit(
            ArtifactOwnedToolJobContext::new(
                meta.instance_id,
                meta.view_state.clone(),
                canonical_base_revision,
                draft_generation,
                transient_generation,
                ArtifactOwnedToolJobSnapshots {
                    children: std::sync::Arc::clone(&children),
                    draft: std::sync::Arc::clone(&draft_snapshot),
                    transient: std::sync::Arc::clone(&transient),
                    window_config: window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                    window_transient: window_transient_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                },
            )
            .with_source_issuers(&self.draft_store, &self.transient_store)
            .with_presence(presence_local, std::sync::Arc::clone(&presence_peers), self.presence_store.peer_retirement_factory())
            .with_tool_run(self.tool_runs.view_for(meta.view_state.as_ref().and_then(|view| view.window_id.as_deref())))
            .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation())
            .with_gesture(gesture),
            artifact_owned_tool_birth_grant(),
        ).0) } else { None };
        let operation_spec = match admission.proof.clone() {
            QualifiedToolProof::Bounded(_) => {
                let job = TypedCommandFullOperationJob::<A> {
                    operation: None,
                    parent_document_id: Some(parent_document_id.clone()),
                    command: Some(std::sync::Arc::from(command)),
                    snapshot: Some(snapshot),
                    config: Some(config),
                    history: Some(history),
                    children: Some(std::sync::Arc::clone(&children)),
                    draft: Some(draft_snapshot),
                    interaction_state: Some(interaction_state),
                    interaction_hover: Some(std::sync::Arc::new(interaction_hover)),
                    peer_presence: Some(peer_presence),
                    presence_local: Some(self.presence_store.local_read().map_err(Fault::from)?),
                    presence_peers: Some(presence_peers),
                    transient: Some(transient),
                    contract: admission.proof.contract(),
                    decoded_items: admission.decoded_items,
                    phase: TypedCommandFullOperationPhase::Prepare,
                    prepare_cursor: 0,
                    reducer_cursor: 0,
                    output: Some(completion.clone()),
                    state: [0; 3],
                    fault_message: b"",
                    pending: None,
                    outcomes: crate::tool_contract::ReservedJobOutcomes::new(),
                    closing: false,
                };
                semio_framework::ToolOperationSpec::new(self.tool_job_controller_id.clone(), verb.clone(), payload_schema_id, job, operation)
            }
            QualifiedToolProof::AppOwned(_) => A::build_tool_job(ArtifactOwnedToolJobRequest {
                retained:self.mounted_policy.maintenance,
                command,
                raw_wire: raw_input.clone(),
                operation,
                controller_id: qualified_key.controller_id,
                tool_id: qualified_key.tool_id,
                payload_schema_id,
                contract: admission.proof.contract(),
                decoded_items: admission.decoded_items,
                app_instance_id: meta.instance_id,
                parent_document_id,
                canonical_base_revision,
                authoring_seed: self.authoring_seed(&meta.actor),
                snapshot,
                snapshot_read: request_snapshot_read.take().expect("app-owned route retains its request snapshot read"),
                config,
                window_config: request_window_config.take(),
                history,
                interaction_state,
                interaction_hover: std::sync::Arc::new(interaction_hover),
                context: owned_context.expect("app-owned route retains its original admitted context"),
                instance_operation_owner: self.instance_operation_owner.clone(),
                output_chunks: output_chunks.clone(),
                completion: completion.clone(),
            })
            .await?
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-builder"), format!("app-owned tool '{verb}' registered a factory but supplied no exact payload builder")))?,
            QualifiedToolProof::FrameworkOwned(_) => {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.framework-route"), format!("framework-owned tool '{verb}' cannot enter the application typed-command reducer")));
            }
        };
        let dispatch = if app_owned_retained_route {
            self.tool_jobs
                .dispatch_wire_retained_with_spec(&retained_wire_admission, retained_wire.take().expect("app-owned retained wire owner transfers once"), None, operation_spec)
                .map_err(|rejected| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), rejected.error.to_string()))?
        } else {
            self.tool_jobs.dispatch(operation_spec).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?
        };
        let session=None;let session_rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(dispatch.job,(cx.operation(),cx.generation())));
        self.tool_operations.insert_admitted(
            operation_id.0,
            MountedTypedCommandFullOperation {
                verb: verb.clone(),
                meta: meta.clone(),
                operation,
                source_identity:(cx.operation(),cx.generation()),
                canonical_revision: canonical_base_revision,
                artifact_generation: generation.0,
                config_generation,
                draft_generation,
                presence_generation,
                transient_generation,
                window_config_authority,
                window_transient_authority,
                publication_lanes,
                session,
                session_rejected,
                reserved_producer: None,
                completion: Some(completion),
                completion_retirement:None,
                publication_retirement:None,
                output_retirement:None,raw_input: (!app_owned_retained_route).then_some(raw_input),
                output_chunks: Some(output_chunks),
                cancellation_lease: Some(cancellation_lease),
                identity: None,
                identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 },
                worker_semantic_pending: false,
                worker_outcome_pending: false,
                worker_fault_capture: None,
                terminal_seen: false,
                worker_resume_pending: false,
                publication: None,
                pending_artifact_publication: None, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                pending_child_publication: None,
                owned_child_group: None,
                owned_child_committed: false,
                owned_child_result_pending: false,
                captured_child_content: Some(std::sync::Arc::clone(&children)),
                captured_child_content_generation: self.child_content_generation,
                result_page: None,
                result_page_presented: false,
                result_sequence: 0,
                publication_progress: 0,
                publication_checkpoint: None,
                publication_ownership_progress: None,
                original_retirement_receipt:None,
                actor_capture: None,
                publication_attempt: 0,
                ui_pending: false,
                progress: None,
                progress_pending: false,
                user_cancel_requested: false,
                published_artifact: false,
                published_config: false,
                published_window_config: false,
                command_logged: false,
                interaction_revalidated: false,
                terminal_fault: None,
                retained_close_fault: None, retained_close_fault_retirement:None,retained_close_fault_refusal:None,
                stage: MountedTypedCommandFullOperationStage::Worker,
            },
        );
        self.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
        let mut started = Self::empty_result(&verb, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
        started.output = DslValue::Object(vec![("operationId".into(), DslValue::String(operation_id.0.to_string())), ("generation".into(), DslValue::String(generation.0.to_string()))]);
        Ok(started)
    }

    /// 🎯️ B1: the shared body behind both `dispatch_typed_command` (wire bytes, decoded above)
    /// 🧪️ Test-only typed-value helper. Production callers must enter with an exact
    /// owner-qualified retained raw-page admission and cannot infer identity from a bare command.
    pub fn dispatch_typed<'a, 'b, 'context>(&'a mut self, command: A::Command, meta: &'a ActionMeta, identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'a mut semio_framework_job::StepContext<'context>) -> impl Future<Output = Result<InvocationResult, Fault>> + 'a + use<'a, 'b, 'context, A, M> {
        let command = Box::new(command);
        async move {
            let log_generation_before = self.log_generation;
            let verb = A::command_id(command.as_ref()).await;
            let payload = ::protocol::OpBinary::encode_op(command.as_ref()).map_err(|error| error.into_fault())?;
            let admission = self.admit_command_wire(verb, &payload, 1).await?;
            let result = self.dispatch_typed_command_inner(command, admission, meta, identity, cx).await?;
            Ok(self.finish_recorded(log_generation_before, verb, result).await)
        }
    }

    /// 🕰️ The actual body of `PluginApp::import_media` — see `dispatch_action`'s doc.
    ///
    /// 🔒️ Contract §2.3 clause 1 — `import` is not a string action (see `VIEWER_REJECTED_ACTION_IDS`'s
    /// own doc), so it is guarded here instead of `dispatch_action`.
    pub(super) async fn dispatch_import_media(&mut self, port: &str, media: Media, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if A::ROLE == AppRole::Viewer {
            return Err(viewer_read_only_fault("import"));
        }
        let proof = self.qualified_tool_proof("import-media")?;
        let key = proof.key();
        let admission = self.tool_jobs.admit_exact_wire(key.controller_id, key.tool_id, proof.schema_id(), &[]).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        if !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.factory-identity"), "media import resolved outside its exact controller/owner/factory/tool/schema proof"));
        }
        let (authority,source)=self.pending_media_import_permit.take().ok_or_else(||plugin_sdk_fault("media source requires its original pre-admitted import permit"))?;
        if (source.0,source.1)!=(cx.operation(),cx.generation()){*self.pending_media_import_permit=Some((authority,source));return Err(plugin_sdk_fault("media import requires its unchanged original caller"))}
        let (job, completion) = match self.build_artifact_reserved_media_job(port, media, meta).await { Ok(producer) => producer, Err(fault) => { self.finish_reserved_emit_producer(authority); return Err(fault); } };
        let raw = Vec::new();
        self.dispatch_reserved_original_producer("import-media",&raw,1,job,completion,meta,Some(admission),authority, cx).await
    }

    /// 🧮️ B1: dispatches a binary-encoded `store::os_store::store_schema::ArtifactCommand<A::ConfigMutation>` against
    /// the config store — real work for `AppCommand::ConfigCommand` (replaces the deleted
    /// `apply_config_bytes` whole-record-replace legacy path).
    pub(super) async fn dispatch_config_command_inner(&mut self,command_bytes:&mut Vec<u8>,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        if self.receive_configuration_failure(cx)?{return Ok(None)}
        if self.pending_configuration_permit.is_some()&&self.pending_configuration_completion.is_some(){return self.finish_configuration_command(meta,identity,cx).await}
        let proof=self.qualified_tool_proof("configuration-binary")?;let key=proof.key();
        let admission=match self.tool_jobs.admit_exact_wire(key.controller_id,key.tool_id,proof.schema_id(),command_bytes){Ok(admission)=>admission,Err(source)=>{*self.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Admission(source));self.pending_configuration_identity=Some((cx.operation(),cx.generation()));let _=self.receive_configuration_failure(cx)?;return Ok(None)}};
        if self.pending_configuration_permit.is_none(){let Some(authority)=self.admit_reserved_emit_producer("configuration-binary",meta,cx)?else{return Ok(None)};self.pending_configuration_permit=Some(authority);self.pending_configuration_identity=Some((cx.operation(),cx.generation()));return Ok(None)}
        if self.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration admission requires its unchanged original receiving caller").into())}
        type Completion<M>=std::sync::Mutex<Option<Result<ArtifactCommand<M>,semio_framework_job::OriginalTurnFailure>>>;
        let bytes=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<Completion<A::ConfigMutation>>();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes{return Ok(None)}
        let decoded=std::sync::Arc::new(std::sync::Mutex::new(None));cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}).expect("exact original configuration completion preflight remains exclusive");cx.consume_fuel(1);
        self.pending_configuration_identity=Some((cx.operation(),cx.generation()));
        self.pending_configuration_completion=Some(crate::reserved_jobs::ConfigurationDecodeRetirement{original:ManuallyDrop::new(Some(decoded.clone())),payload:None});
        let job=FrameworkConfigurationBinaryJob::<A>{raw:std::mem::take(command_bytes),cursor:0,stage:FrameworkConfigurationBinaryStage::Envelope,operation:None,decoded:ManuallyDrop::new(Some(decoded)),decoded_close:None,receiving:ManuallyDrop::new(None),native:semio_framework_job::OriginalNativeDecodeSource::empty(semio_framework_job::OriginalNativeDecodePolicy{maximum_owned_bytes:crate::reserved_jobs::FRAMEWORK_CONFIGURATION_NATIVE_OWNED_BYTES}),failure:ManuallyDrop::new(None),state:[0;8],pending:None,outcomes:crate::tool_contract::ReservedJobOutcomes::new(),closing:false};
        let authority=self.pending_configuration_permit.take().unwrap();
        let permit=match self.run_framework_reserved_job("configuration-binary",&[],1,job,meta,Some(admission),authority,cx).await{Ok(permit)=>permit,Err((source,permit))=>{self.pending_configuration_permit=Some(permit);*self.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.receive_configuration_failure(cx)?;return Ok(None)}};
        self.pending_configuration_permit=Some(permit);
        let command=self.pending_configuration_completion.as_ref().unwrap().original.as_ref().unwrap().lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().ok_or_else(||Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.missing-output"),"configuration binary job completed without a decoded command"))?;
        match command{Ok(command)=>*self.pending_configuration_command=Some(command),Err(source)=>{*self.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.receive_configuration_failure(cx)?;return Ok(None)}}
        self.finish_configuration_command(meta,identity,cx).await
    }

    async fn finish_configuration_command(&mut self,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        if self.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration commit requires its unchanged original caller").into())}
        if let Some(owner)=self.pending_configuration_completion.as_mut(){
            let demand=if owner.terminal_is_empty(){RetirementDemand{depth:1,..Default::default()}}else{owner.demand(cx.retained_grant().maximum_copy_bytes)?};
            let terminal=owner.terminal_is_empty();let mut removed=false;
            semio_framework_job::close_original_retained_step(cx,demand,|grant|{if terminal{removed=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}else{owner.step(grant)}})?;
            if removed{self.pending_configuration_completion.take();}return Ok(None)
        }
        let permit=self.pending_configuration_permit.as_ref().expect("configuration commit retains its admitted permit");self.validate_framework_reserved_commit("configuration-binary",permit).await?;
        if permit.is_cancelled().await{return Err(Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.cancelled"),"configuration binary was cancelled before config-store commit").into())}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        let command=self.pending_configuration_command.take().expect("configuration commit retains its original typed command");cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
        let dispatched=self.config_store.dispatch(command,identity).await;let permit=self.pending_configuration_permit.take().unwrap();self.finish_reserved_emit_producer(permit);if let Err(source)=dispatched{*self.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Store(source));let _=self.receive_configuration_failure(cx)?;return Ok(None)}self.pending_configuration_identity=None;self.cache=None;Ok(Some(Self::empty_result("configCommand",meta,Vec::new(),Vec::new(),UiDirtyScope::Full).await))
    }

    fn receive_configuration_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if self.pending_configuration_failure.is_none(){return Ok(false)}if self.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration failure requires its unchanged original caller").into())}if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.pending_configuration_failure,cx)?{self.pending_configuration_identity=None;return Err(failure)}Ok(true)}
    pub(crate) fn configuration_failure_terminal_is_empty(&self)->bool{self.pending_configuration_completion.is_none()&&self.pending_configuration_command.is_none()&&self.pending_configuration_command_close.is_none()&&self.pending_configuration_permit.is_none()&&self.pending_configuration_failure.is_none()&&self.pending_configuration_retirement.is_none()&&self.pending_configuration_identity.is_none()}
    pub(crate) fn configuration_failure_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.pending_configuration_completion.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.demand(body)}}if let Some(owner)=self.pending_configuration_command_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)}}self.pending_configuration_retirement.as_ref().map_or(Ok(RetirementDemand{depth:usize::from(!self.configuration_failure_terminal_is_empty()),..Default::default()}),|owner|if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.selected_demand(body)})}
    pub(crate) fn close_configuration_failure(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{let demand=self.configuration_failure_demands(grant.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|_,grant|{if let Some(owner)=self.pending_configuration_completion.as_mut(){if owner.terminal_is_empty(){self.pending_configuration_completion.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(owner)=self.pending_configuration_command_close.as_mut(){if owner.terminal_is_empty(){self.pending_configuration_command_close.take();return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(command)=self.pending_configuration_command.take(){match ControlledRetirement::new(command){Ok(owner)=>self.pending_configuration_command_close=Some(owner),Err((error,command))=>{*self.pending_configuration_command=Some(command);return Err(error.into())}}return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}if let Some(permit)=self.pending_configuration_permit.take(){self.finish_reserved_emit_producer(permit);return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}if let Some(owner)=self.pending_configuration_retirement.as_mut(){if owner.terminal_is_empty(){self.pending_configuration_retirement.take();return Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}),true))}let step=owner.step(grant)?;return Ok((PluginLifecycleStep::Progress(step.progress()),false))}if let Some(source)=self.pending_configuration_failure.take(){match ControlledRetirement::new(source){Ok(owner)=>self.pending_configuration_retirement=Some(owner),Err((error,source))=>{*self.pending_configuration_failure=Some(source);return Err(error.into())}}return Ok((PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}),false))}self.pending_configuration_identity=None;Ok((PluginLifecycleStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}),true))})}

    /// 🧾️ Whether a verb of this declared kind may dirty the history panel. `dispatch_emit` records a
    /// command-log row for EVERY dispatch, view actions included (`ActionKind::View` is also the
    /// fallback for an unresolved verb), so the log generation advancing is not by itself evidence that
    /// anything a user would call history happened. A `View` verb paints, it does not command; an
    /// `Interaction` verb moves selection/hover, which `dispatch_interaction_action` has always
    /// declared as kept out of the panel. Handing either one a history patch made every background
    /// tick a history update — measured 2026-09-09 20:55 as 90 host `readHistory` round trips beside
    /// 252 `fillBuildTick`s in 35 s.
    /// 🚫️async: E1 pure classification consumed by sync-only `matches!` call sites — see R9
    fn verb_dirties_history_panel(kind: Option<ActionKind>) -> bool {
        !matches!(kind, Some(ActionKind::View | ActionKind::Interaction))
    }

    /// 🧾️ Attaches the command-log delta directly to the accepted invocation response. History is
    /// no longer a plugin-rendered panel body, so this deliberately leaves `ui_scope` untouched.
    /// `verb` is the DECLARED action/command id — every caller must pass the real one, never a generic
    /// placeholder, or the kind lookup below silently resolves to nothing and the skip is unreachable.
    pub(super) async fn finish_recorded(&mut self, log_generation_before: u64, verb: &str, mut result: InvocationResult) -> InvocationResult {
        if self.log_generation == log_generation_before {
            return result;
        }
        let kind = self.registry.get(verb).map(|definition| definition.kind).or_else(|| self.registry.get_command(verb).map(|command| command.kind));
        if !Self::verb_dirties_history_panel(kind) {
            return result;
        }
        result.history_patch = self.history_patch(false).await.ok();
        result
    }

    pub(super) fn retained_fields_terminal_is_empty(&self) -> bool {
        self.retained_fields_own_terminal_is_empty() && self.children.is_empty() && self.local_interaction_query.is_none() && self.snapshot_read_returns_terminal_is_empty()
    }

    /// 🪪️ The immutable identity this instance's live document revision is observed under — the ONE
    /// derivation every entry point that hands a plugin an [`ArtifactView`] stamps into it
    /// (`render`, `pending_effects`, `window_engagements`, `window_measures`, `tool_measures`,
    /// `context_menu`). Chrome that reaches for its own per-instance session (puzzle 3d's
    /// `puzzle3d_view_session_key`) read `None` on the four view-only lanes and ran cold against a
    /// session-less app; deriving the identity here — not per call site — makes that key total.
    /// 🚫️async: E1 pure identity projection over already-owned store fields — see R9.
    pub(super) fn live_render_operation(&self) -> AppRenderOperationContext {
        let canonical_base_revision = self.store.content_revision();
        let [lane_0, lane_1, lane_2, lane_3, lane_4, lane_5, lane_6, lane_7, ..] = canonical_base_revision;
        AppRenderOperationContext {
            app_instance_id: self.live_runtime_instance_id.unwrap_or_default(),
            base_revision: semio_framework_job::RevisionId(u64::from_be_bytes([lane_0, lane_1, lane_2, lane_3, lane_4, lane_5, lane_6, lane_7])),
            generation: semio_framework_job::Generation(self.store.generation()),
            canonical_base_revision,
            mounted_policy: self.mounted_policy,
        }
    }

    #[cfg(test)]
    pub(crate) fn typed_operation_result_state_for_test(&self, operation_id: u64) -> Option<(TypedOperationResultToken, TypedOperationResultLane, bool, bool)> {
        let operation = self.tool_operations.get(operation_id)?;
        let page = operation.result_page.as_ref()?;
        Some((page.token, page.lane, operation.result_page_presented, operation.stage == MountedTypedCommandFullOperationStage::AwaitingAck))
    }
}
