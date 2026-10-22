//! 🔁️ Stall witness and unit advance of the typed operation publication: the stall streak and the interaction write check, the advance of one publication unit and the window config refresh of a published receipt. See [`super`].

use super::*;

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

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    pub async fn advance_typed_operation_publication_one(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let before = self.tools.typed_operation_stall_witness();
        let advanced = self.advance_typed_operation_publication_unit(cx).await;
        if let (Some(before), Some(after)) = (before, self.tools.typed_operation_stall_witness()) {
            if let Some(streak) = typed_operation_stall_streak::<A>(before, after) {
                let verb = self.tools.tool_operations.get(after.0).map_or("<released>", |operation| operation.verb.as_str());
                semio_framework_plugin_reactor_app::debug_runtime_line(format_args!("[TRACE] typed-operation {verb} advanced {streak} units with no change: operation={} stage={} flags={:#010b}", after.0, after.1, after.2));
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
    async fn advance_typed_operation_publication_unit(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
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
            if !mounted.source_identity.matches(cx){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original emission lease requires its mounted receiving caller").into())}
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

}

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
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
