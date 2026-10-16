//! 🏈️ Staged `publication` members of the `instance-typed-ops` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 📬️ Typed operation publication: the mounted publication units, the latest-wins ladder and the stall witness.
use super::*;
use super::docs_instance_root::tool_transaction_shape_fault;
use super::docs_instance_root::typed_operation_document_is_fresh;
use super::docs_instance_root::viewer_read_only_fault;
use super::typed_ops_instance_root::store_publication_fault;
use super::typed_ops_plugin_app_impl::PUBLICATION_RETURNED_ROOT_ALLOWANCE;

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
pub(crate) fn typed_operation_stall_streak<A: ArtifactApp>(before: (u64, u8, u8, u64), after: (u64, u8, u8, u64)) -> Option<u64> {
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

/// 🕹️ Whether this operation's next publication unit is its [`InteractionWrite`] lane: every
/// durable store lane of its emit is already drained (a selection may only name ids the
/// document already holds), and there is at least one write left to apply. Sync and
/// allocation-free so the async caller can branch on it before mounting anything.
/// 🚫️async: E1 pure field census over an already-mounted publication owner — see R9.
///
/// 🧺️ A batched durable lane empties its emit vector the moment it is admitted, so the
/// in-flight publication itself — not the drained vector — is what still owes a turn.
pub(crate) fn mounted_typed_interaction_writes_are_next<A: ArtifactApp>(mounted: &MountedTypedCommandFullOperation<A>) -> bool {
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

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    pub(crate) async fn advance_typed_operation_publication_one(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let before = self.tools.typed_operation_stall_witness();
        let advanced = self.advance_typed_operation_publication_unit(cx).await;
        if let (Some(before), Some(after)) = (before, self.tools.typed_operation_stall_witness()) {
            if let Some(streak) = typed_operation_stall_streak::<A>(before, after) {
                let verb = self.tools.tool_operations.get(after.0).map_or("<released>", |operation| operation.verb.as_str());
                crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] typed-operation {verb} advanced {streak} units with no change: operation={} stage={} flags={:#010b}", after.0, after.1, after.2));
                if after.1 == TYPED_OPERATION_STALL_PUBLISHING_STAGE && streak >= TYPED_OPERATION_STALL_FAULT_CEILING {
                    self.tools.fault_stalled_typed_operation_publication(after.0, streak)?;
                }
            }
        }
        advanced
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
        if !self.tools.latest_wins_commands.is_empty() && (self.tools.latest_wins_turn || self.tools.tool_operations.is_empty()) {
            self.tools.latest_wins_turn = false;
            return self.advance_latest_wins_command_one(retained, cx).await.map_err(Into::into);
        }
        self.tools.latest_wins_turn = true;
        let Some((index, operation_id)) = self.tools.next_advanceable_typed_operation() else {
            record_typed_operation_unit(TypedOperationUnitKind::Idle);
            return Ok(());
        };
        self.tools.typed_publication_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        if self.tools.tool_operations.get(operation_id).is_some_and(|mounted|mounted.cancellation_lease.is_none()&&mounted.cancellation_retirement.is_none()&&mounted.stage==MountedTypedCommandFullOperationStage::Publishing){
            let mounted=self.tools.tool_operations.get_mut(operation_id).unwrap();
            if mounted.source_identity!=(cx.operation(),cx.generation()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original emission lease requires its mounted receiving caller").into())}
            let key=ToolOperationKey{app_instance_id:mounted.meta.instance_id,document:ArtifactDocumentAuthority(mounted.meta.instance_id),operation_id:mounted.operation.operation,base_revision:mounted.operation.base_revision,generation:mounted.operation.generation};
            mounted.cancellation_lease=self.tools.tool_cancellations.begin_keyed(&key,cx)?;
            if let Some(lease)=mounted.cancellation_lease.as_ref(){
                if mounted.terminal_fault.is_some()||mounted.user_cancel_requested{lease.cancel();}
                let maximum=({ let __a0 = &self.tools.tool_operations.get(operation_id).unwrap().verb; self.tools.qualified_tool_proof(&*self.stores, __a0) })?.contract().max_output_bytes;
                let mounted=self.tools.tool_operations.get_mut(operation_id).unwrap();let lease=mounted.cancellation_lease.as_ref().unwrap();
                let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|!lease.token.is_cancelled_now()&&!lease.publication_claim.is_cancelled();
                mounted.identity=Some(semio_framework_value::NativeEncodeControl::new(maximum,&mut observer).pause().map_err(|error|plugin_sdk_fault(error.to_string()))?);
            }
            return Ok(());
        }
        if self.tools.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Worker) {
            record_typed_operation_unit(TypedOperationUnitKind::Worker);
            return self.tools.drive_typed_operation_worker(&self.mounted_policy, operation_id, cx);
        }
        if self.tools.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Retiring) {
            record_typed_operation_unit(TypedOperationUnitKind::Retirement);
            return self.retire_typed_operation_run(operation_id,cx);
        }
        if !self.tools.tool_operations.get_mut(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Publishing) {
            record_typed_operation_unit(TypedOperationUnitKind::Idle);
            return Ok(());
        }
        let mut mounted =
            self.tools.tool_operations.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.publication-authority"), "typed-operation publication owner changed before one-unit transfer"))?;
        let outcome = self.publish_mounted_typed_operation_run(&mut mounted,cx,retained).await;
        let mut driver_result = Ok(());
        if let Err(failure)=outcome{self.tools.tool_operations.insert_admitted(operation_id,mounted);return Err(failure);}
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
            driver_result = self.as_render().revalidate_interaction_on_document_change(&meta, &mut identity).await;
            mounted.identity = Some(identity.pause().map_err(|error| plugin_sdk_fault(error.to_string()))?);
            drop(observer);
            mounted.identity_progress = progress;
        }
        self.tools.tool_operations.insert_admitted(operation_id, mounted);
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
            Ok(()) => self.child_composition.follow_derivable_children(&mut *self.stores, &mut identity).await.map_err(Into::into),
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
            } else if mounted_typed_interaction_writes_are_next::<A>(mounted) {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_interaction_unit(mounted, identity).await?;
            } else if self.tools.mounted_typed_inline_interaction_verbs_are_next(mounted) {
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
                if self.stores.store.backbone_ref().is_some() {
                    self.stores.store.flush_announcements().await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    if let Some(PendingArtifactStorePublication::Artifact(publication)) = mounted.pending_artifact_publication.as_mut() {
                        self.stores.store.flush_published_apply_batch(publication).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
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
        if let Err(fault) = self.as_render().apply_interaction_writes(&writes, &meta, identity).await {
            if let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() {
                emit.interaction_writes = writes;
            }
            return Err(fault);
        }
        let token = mounted.next_token();
        let page = TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Interaction, &("accepted", writes.len()))?;
        mounted.queue_page(page)
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
        if let Some(index) = self.tools.typed_inline_interaction_verbs.iter().position(|(operation, _)| *operation == mounted.operation.operation.0) {
            folded = self.tools.typed_inline_interaction_verbs.swap_remove(index).1;
        }
        folded.extend(take_inline_interaction_verbs(&mut emit.effects));
        let scope = emit.ui_scope.clone();
        let meta = mounted.meta.clone();
        let fold = self.as_render().fold_inline_interaction_verbs(&mounted.verb, folded, &meta, scope, identity).await;
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation inline interaction publication lost its emit owner after the fold"));
        };
        emit.ui_scope = fold.scope;
        if let Some(view) = fold.leftover {
            self.tools.typed_inline_interaction_leftover = Some(TypedOperationLeftover { operation: mounted.operation.operation.0, view });
        }
        for diagnostic in fold.diagnostics {
            crate::plugin_runtime::debug_runtime_line(format_args!("[TRACE] {} {}", diagnostic.code.0, diagnostic.message));
        }
        Ok(())
    }

    pub(crate) async fn advance_latest_wins_command_one(&mut self,retained:RetainedCloneGrant, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), crate::PluginLifecycleFailure> {
        if retained.maximum_items==0||retained.maximum_depth==0{return Ok(());}
        let Some(operation_id) = self.tools.latest_wins_order.items.front().copied() else { return Ok(()) };
        let mut pending = self.tools.latest_wins_commands.remove(operation_id).expect("exact pending keyed command");
        if pending.receipt.is_some(){self.tools.latest_wins_commands.insert_admitted(operation_id,pending);return Ok(());}
        if !pending.closing&&pending.lease.is_none(){
            let result=pending.admit_original_lease(&mut self.tools.tool_cancellations,cx);
            self.tools.latest_wins_commands.insert_admitted(operation_id,pending);
            result?;
            return Ok(());
        }
        let ready = match self.tools.advance_latest_wins_admission_unit(&mut *self.stores, &mut pending, retained) {
            Ok(ready) => ready,
            Err(fault) => {
                let result=if pending.receipt.is_some(){Ok(())}else{pending.record_original_receipt(retained,fault.retained_progress())};
                *pending.fault=Some(fault);
                if let Err(error)=result{self.tools.latest_wins_commands.insert_admitted(operation_id,pending);return Err(error.into());}
                pending.closing = true;
                if let Some(lease) = pending.lease.as_ref() {
                    lease.cancel();
                }
                false
            }
        };
        let reserved = self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] == Some(operation_id);
        if !reserved || !self.tools.tool_operations.can_insert(operation_id) {
            pending.closing = true;
            pending.lease.as_ref().expect("pending slot rejection retains its cancellation lease").cancel();
            if !self.tools.tool_operations.can_insert(operation_id) || self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS].is_some_and(|owner| owner != operation_id) {
                self.tools.latest_wins_commands.insert_admitted(operation_id, pending);
                return Ok(());
            }
        }
        if ready && !pending.closing {
            let command = pending.command.take().expect("accepted latest-wins command owner");
            let admission = pending.admission.take().expect("accepted latest-wins exact raw admission");
            let lease = pending.lease.take().expect("accepted latest-wins cancellation lease");
            assert_eq!(self.tools.latest_wins_order.pop(), Some(operation_id));
            let started = self.as_dispatch().start_typed_command_operation(command, admission, &pending.meta, pending.operation.operation, lease, cx).await;
            if started.is_err() {
                self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
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
            self.tools.tool_operations.insert_admitted(operation_id, mounted);
            self.tools.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
            assert_eq!(self.tools.latest_wins_order.pop(), Some(operation_id));
        } else {
            self.tools.latest_wins_commands.insert_admitted(operation_id, pending);
        }
        Ok(())
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
                self.as_dispatch().record_typed_operation_lane(mounted);
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
        match self.child_composition.reclaim_child_retirements(1, 0) {
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
        let live_revision = self.stores.store.content_revision_now();
        let live_generation = self.stores.store.generation_now();
        let Some(captured) = mounted.captured_child_content.as_ref() else {
            let fault = pending.reject_and_fault(&plugin_sdk_fault("typed-operation child publication lost its captured immutable root"));
            mounted.pending_child_publication = Some(pending);
            return Err(fault);
        };
        let parent_fresh = typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation);
        let child_root_fresh = mounted.captured_child_content_generation == self.child_composition.child_content_generation && captured.identity_digest() == self.child_composition.child_content_root.identity_digest();
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
            let Some(entry) = self.child_composition.children.get(&(child.slot.clone(), child.child_id.clone())) else {
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
        let result = self.as_dispatch().dispatch_emit_group(&mounted.verb, &pending.artifact_mutations, &pending.child_emits, Vec::new(), Vec::new(), UiDirtyScope::None, &mounted.meta, None, pending.transaction.as_ref().cloned(), identity).await;
        match result {
            Ok(result) => {
                mounted.artifact_generation = self.stores.store.generation_now();
                mounted.canonical_revision = self.stores.store.content_revision_now();
                mounted.operation.base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().expect("revision lane width")));
                mounted.operation.generation = semio_framework_job::Generation(mounted.artifact_generation);
                let receipt = ChildPublicationResultV1 { invocation_id: result.inverse_group.invocation_id.0.clone(), committed_members: result.inverse_group.member_edits.len(), child_content_generation: self.child_composition.child_content_generation };
                if self.tools.typed_composed_outbox.push(ComposedGestureResult { operation: mounted.operation.operation.0, mutations: result.mutations, inverse_group: result.inverse_group }).is_err() {
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

    pub(crate) fn publish_mounted_typed_operation_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, retained:RetainedCloneGrant) -> Result<(), Fault> {
        if mounted.reject_cancelled_publication()? {
            self.stores.handoff_mounted_durable_publication(mounted)?;
            return Ok(());
        }
        let _permit = mounted.cancellation_lease.as_ref().and_then(ToolCancellationLease::try_claim_publication).ok_or_else(|| plugin_sdk_fault("typed-operation publication claim is cancelled or occupied"))?;
        if mounted.publication.is_none() {
            let Some(completion) = mounted.completion.as_ref() else { return Err(plugin_sdk_fault("typed-operation completion owner was retired before publication")) };
            let Some(mut publication) = completion.take()? else { return Ok(()) };
            mounted.ui_pending = matches!(publication, ArtifactToolCompletionValue::Emit(Ok(_), _));
            self.as_render().settle_tool_operation(mounted, &mut publication)?;
            if let ArtifactToolCompletionValue::Emit(Ok(emit), _) = &mut publication {
                let verbs = take_inline_interaction_verbs(&mut emit.effects);
                if !verbs.is_empty() {
                    self.tools.typed_inline_interaction_verbs.retain(|(operation, _)| *operation != mounted.operation.operation.0);
                    self.tools.typed_inline_interaction_verbs.push((mounted.operation.operation.0, verbs));
                }
            }
            mounted.publication = Some(publication);
            if mounted_typed_interaction_writes_are_next::<A>(mounted) || self.tools.mounted_typed_inline_interaction_verbs_are_next(mounted) {
                return Ok(());
            }
        }
        let live_revision = self.stores.store.content_revision_now();
        let live_generation = self.stores.store.generation_now();
        if !typed_operation_document_is_fresh(&mounted.operation, mounted.canonical_revision, live_revision, live_generation) {
            if self.stores.handoff_mounted_durable_publication(mounted)? {
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
                && self.stores.store.returned_snapshot_read_count() > PUBLICATION_RETURNED_ROOT_ALLOWANCE
                && !self.ingest.reclaim_document_snapshot_read_returns(&*self.stores)?
            {
                return Ok(());
            }
            let pending = mounted.pending_artifact_publication.as_mut().expect("pending publication checked above");
            let advance = match pending {
                PendingArtifactStorePublication::Artifact(publication) => self.stores.store.advance_apply_batch(publication, grant).map_err(store_publication_fault)?,
                PendingArtifactStorePublication::Config(publication) => self.stores.config_store.advance_apply_batch(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Draft(publication) => self.stores.draft_store.advance_apply_batch(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Presence(publication) => self.stores.presence_store.advance_publish_one(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::Transient(publication) => self.stores.transient_store.advance_publish_one(publication, grant).map_err(|error| plugin_sdk_fault(error.to_string()))?,
                PendingArtifactStorePublication::WindowConfig(publication) => self.stores.window_config_store.advance(publication.as_mut(), grant)?,
                PendingArtifactStorePublication::WindowTransient(publication) => self.stores.window_transient_store.advance(publication.as_mut(), grant)?,
            };
            return match advance {
                store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(receipt) => {
                    match pending_lane {
                        TypedOperationResultLane::Artifact => {
                            mounted.published_artifact = true;
                            self.as_dispatch().record_typed_operation_lane(mounted);
                            mounted.artifact_generation = receipt.generation_after;
                            mounted.canonical_revision = self.stores.store.content_revision_now();
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
                    match self.stores.presence_store.begin_publish_one(mounted.operation.operation, mounted.presence_generation, mutation, self.stores.presence_one_item_factory.as_deref(), self.stores.presence_local_root_retirement_factory.clone()) {
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
                    match self.stores.transient_store.begin_publish_one(mounted.operation.operation, mounted.transient_generation, mutation, self.stores.transient_one_item_factory.as_deref(), self.stores.transient_local_root_retirement_factory.clone()) {
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
                    let refresh=match self.stores.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>self.stores.window_transient_store.refresh(authority,grant),Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault);}}
                    match self.stores.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
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
                    self.media.require_operation_emitting_kind(&mounted.verb, self.tools.declared_dispatch_kind(&*self.host_events, &mounted.verb))?;
                    if A::ROLE == AppRole::Viewer {
                        return Err(viewer_read_only_fault(&mounted.verb));
                    }
                    if let Some(fault) = X::mutation_refusal(&*self.time_travel, &*self.tool_runs, &mounted.verb) {
                        return Err(fault);
                    }
                    if let Some(pending_txn_id) = self.txn.pending_transaction.as_ref().map(|pending| pending.txn_id.clone()) {
                        return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", format!("verb {:?} would emit artifact mutations while transaction {pending_txn_id:?} is pending on this instance", mounted.verb)));
                    }
                    if emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
                        let mut running = self.stores.store.snapshot().map_err(|error| error.into_fault())?;
                        let foreign = Vec::new();
                        for op in emit.artifact_mutations.iter() {
                            if op.foreign_step_source(&running, 0).map_err(|error| error.into_fault())?.is_some() { return Err(plugin_sdk_fault("foreign transaction output awaits its original staged preparation recipient")); }
                            let outcome = op.diff(&running);
                            if !outcome.is_applicable(protocol::MergePolicy::default()) {
                                return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                            }
                            running = protocol::apply_diff(outcome.diff(), &running).map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
                        }
                        if !foreign.is_empty() {
                            let mut local_ops = Vec::with_capacity(emit.artifact_mutations.len());
                            for op in emit.artifact_mutations.iter() {
                                local_ops.push(::protocol::OpBinary::encode_op(op).unwrap_or_default());
                            }
                            self.txn.pending_transaction_proposal = Some(TransactionProposalDraft { local_ops, foreign });
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
                if !emit.artifact_mutations.is_empty() || !emit.child_emits.is_empty() || !emit.owned_child_emits.is_empty() {
                    if let Some(fault) = X::document_refusal(&*self.time_travel, &*self.tool_runs, &mounted.verb) {
                        return Err(fault);
                    }
                }
                self.history.close_streamed_transaction_unit(&mut *self.stores, emit)?;
                if !emit.owned_child_emits.is_empty() {
                    if !emit.child_emits.is_empty() || emit.owned_child_emits.len() > PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN { return Err(plugin_sdk_fault("mounted owned child group requires its one exact admitted typed source family")); }
                    let operation = mounted.operation.operation.0;
                    let captured_millis = i64::try_from(store::os_store::document_materialize::now_ms()).map_err(|_|plugin_sdk_fault("mounted child publication requires its original UTC clock capture"))?;
                    let grant=retained;
                    let step=self.child_composition.admit_private_child_group_frame(operation, captured_millis, u64::MAX, grant)?;
                    let progress=step.progress();
                    if !progress.fits(grant){return Err(plugin_sdk_fault("mounted child frame exceeded its original preparation grant"));}
                    if progress!=RetainedCloneProgress::default(){
                        assert!(mounted.publication_ownership_progress.is_none(),"mounted child frame cannot overwrite its original unreturned receipt");
                        mounted.publication_ownership_progress=Some((retained,progress));
                    }
                    if self.child_composition.private_child_groups.get(operation).is_some(){mounted.owned_child_group=Some(operation);}
                    return Ok(());
                }
                if emit.child_emits.is_empty() && emit.owned_child_emits.is_empty() && !emit.artifact_mutations.is_empty() {
                    if let Some(fault) = self.stores.open_transaction_admission_fault(emit.transaction.as_ref()) {
                        return Err(fault);
                    }
                    if let Err(error) = store::os_store::one_item::ArtifactStoreOneItemFootprint::for_gesture::<A::Snapshot, A::Mutation>(&emit.artifact_mutations) {
                        return Err(error.into_fault());
                    }
                    let streams=emit.transaction_phase==TransactionPhase::Stream;
                    let outbound=self.stores.store.backbone_ref().is_some();
                    if let Some((mut publication,ownership))=self.stores.store.admit_apply_batch(mounted.operation.operation,mounted.artifact_generation,mounted.canonical_revision,&mut mounted.actor_capture,&mut emit.artifact_mutations,HistoryLane::Document,self.stores.artifact_one_item_factory.as_ref(),&mut emit.transaction,Some(&mounted.verb),outbound,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        publication.set_transaction_open(streams);
                        emit.transaction_phase=TransactionPhase::Commit;
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Artifact(publication));
                    }
                    return Ok(());
                }
                if !emit.config_mutations.is_empty() {
                    let revision=self.stores.config_store.content_revision_now();
                    let mut transaction=None;
                    if let Some((publication,ownership))=self.stores.config_store.admit_apply_batch(mounted.operation.operation,mounted.config_generation,revision,&mut mounted.actor_capture,&mut emit.config_mutations,HistoryLane::Document,self.stores.config_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Config(publication));
                    }
                    return Ok(());
                } else if !emit.window_config_mutations.is_empty() {
                    let authority = mounted.window_config_authority.as_ref().ok_or_else(|| plugin_sdk_fault("window config emission requires one exact captured ViewModel window authority"))?;
                    if let Some((publication,ownership))=self.stores.window_config_store.admit_begin(mounted.operation.operation,&mut mounted.actor_capture,&mut emit.window_config_mutations,authority,retained).map_err(|error|error.into_fault())? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::WindowConfig(publication));
                    }
                    return Ok(());
                } else if !emit.draft_mutations.is_empty() {
                    let revision=self.stores.draft_store.content_revision_now();
                    let mut transaction=None;
                    if let Some((publication,ownership))=self.stores.draft_store.admit_apply_batch(mounted.operation.operation,mounted.draft_generation,revision,&mut mounted.actor_capture,&mut emit.draft_mutations,HistoryLane::Document,self.stores.draft_one_item_factory.as_ref(),&mut transaction,Some(&mounted.verb),false,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {
                        mounted.publication_ownership_progress=Some((retained,ownership));
                        mounted.pending_artifact_publication=Some(PendingArtifactStorePublication::Draft(publication));
                    }
                    return Ok(());
                } else if let Some(mutation) = ephemeral.presence.pop() {
                    match self.stores.presence_store.begin_publish_one(mounted.operation.operation, mounted.presence_generation, mutation, self.stores.presence_one_item_factory.as_deref(), self.stores.presence_local_root_retirement_factory.clone()) {
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
                    match self.stores.transient_store.begin_publish_one(mounted.operation.operation, mounted.transient_generation, mutation, self.stores.transient_one_item_factory.as_deref(), self.stores.transient_local_root_retirement_factory.clone()) {
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
                    let refresh=match self.stores.window_transient_store.refresh_demands(authority,grant.maximum_copy_bytes){Ok(_)=>self.stores.window_transient_store.refresh(authority,grant),Err(error)=>Err(plugin_retirement_fault(error))};
                    match refresh{Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Ready(_))=>{},Ok(super::super::window_mutation::WindowAuthorityRefreshStep::Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(());},Err(fault)=>{ephemeral.window_transient.push(mutation);return Err(fault);}}
                    match self.stores.window_transient_store.begin(mounted.operation.operation, authority, mutation) {
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
                    MediaTransfers::<A>::mint_extension_invocations(mounted.meta.instance_id, emit)?;
                    if let Some(effect) = emit.effects.pop() {
                        let effect = self.stores.stamp_load_document_identity(effect)?;
                        if let Err(effect) = self.tools.typed_effect_outbox.push(effect) {
                            emit.effects.push(effect);
                            return Err(plugin_sdk_fault("typed-operation effect receiver is saturated"));
                        }
                        TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Effect, &("accepted", self.tools.typed_effect_outbox.len()))?
                    } else if let Some(event) = emit.events.pop() {
                        if let Err(event) = self.tools.typed_event_outbox.push(event) {
                            emit.events.push(event);
                            return Err(plugin_sdk_fault("typed-operation event receiver is saturated"));
                        }
                        TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Event, &("accepted", self.tools.typed_event_outbox.len()))?
                    } else {
                        match () {
                            () if !emit.tasks.is_empty() => {
                                let task = emit.tasks.remove(0);
                                crate::reactor::spawn_task(mounted.meta.instance_id, &mounted.meta, task)?;
                                return Ok(());
                            }
                            () if mounted.ui_pending => {
                                if let Err(_scope) = self.tools.typed_ui_outbox.push(emit.ui_scope.clone()) {
                                    return Err(plugin_sdk_fault("typed-operation UI receiver is saturated"));
                                }
                                mounted.ui_pending = false;
                                TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::Ui, &emit.ui_scope)?
                            }
                            () => {
                                if self.tools.typed_completion_outbox.push(TypedOperationCompletionWitness { operation: token.operation, ui_scope: emit.ui_scope.clone() }).is_err() {
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
}

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    /// 🪟️ Continues the window-config authority refresh of a published receipt; once it is complete the extensions are told
    /// before the retained receipt's result page is queued.
    pub(crate) fn advance_mounted_window_config_refresh(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, retained: RetainedCloneGrant) -> Result<(), Fault> {
        if self.stores.advance_window_config_refresh(mounted, retained)? {
            X::window_config_published(&mut *self.time_travel, &mut *self.tool_runs);
            InstanceStores::<A>::queue_window_config_receipt(mounted)?;
        }
        Ok(())
    }
}
