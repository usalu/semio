//! 🔁️ Stall witness and unit advance of the typed operation publication: the stall streak and the interaction write check, the advance of one publication unit and the window config refresh of a published receipt. See [`super`].

use super::*;
use semio_plugin_retained_command::typed_operation::note_publication_wait;

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
        && !emit.has_artifact_mutations()
        && emit.config_mutations.is_empty()
        && emit.draft_mutations.is_empty()
        && emit.child_emits.is_empty()
        && emit.owned_child_emits.is_empty()
        && ephemeral.presence.is_empty()
        && ephemeral.transient.is_empty()
}

/// 🩺️ Captures the selected operation's publication stage, ownership flags and store progress.
fn publication_witness<A: ArtifactApp>(operation: &MountedTypedCommandFullOperation<A>) -> (MountedTypedCommandFullOperationStage, u8, u64) {
    let flags = u8::from(operation.session.is_some())
        | u8::from(operation.session_rejected.is_some()) << 1
        | u8::from(operation.completion.is_some()) << 2
        | u8::from(operation.publication.is_some()) << 3
        | u8::from(operation.pending_artifact_publication.is_some()) << 4
        | u8::from(operation.worker_outcome_pending) << 5
        | u8::from(operation.terminal_seen) << 6
        | u8::from(operation.result_page.is_some()) << 7;
    (operation.stage, flags, operation.publication_progress)
}

/// 🔢️ Counts consecutive attempts against one owner while its publishing frontier is unchanged.
fn observe_publication_stall(streak: &mut u64, attempted: bool, changed: bool, publishing: bool, genesis: bool) -> u64 {
    if !attempted { return 0 }
    if changed || !publishing || genesis {
        *streak = 0;
    } else {
        *streak = streak.saturating_add(1);
    }
    *streak
}

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    /// 🎟️ Batches the selected command's paid admission steps within the current host turn.
    pub async fn advance_typed_operation_publication_turn(&mut self, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let selected = self.tools.latest_wins_order.items.front().copied();
        let started = cx.now_us();
        for _ in 0..TYPED_OPERATION_PUBLICATION_PUMPS {
            let before = cx.retained_progress();
            self.advance_typed_operation_publication_unit(identity, cx).await?;
            if selected.is_none() || !self.tools.tool_operations.is_empty()
                || self.tools.latest_wins_order.items.front().copied() != selected
                || cx.retained_progress() == before || cx.should_yield() || cx.fuel_exhausted()
                || cx.now_us().zip(started).is_none_or(|(now, start)| now.saturating_sub(start) >= INTERACTIVE_TURN_WORKER_WALL_US) {
                break;
            }
        }
        Ok(())
    }

    pub async fn advance_typed_operation_publication_one(&mut self,identity:&mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        self.advance_typed_operation_publication_unit(identity,cx).await
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
    async fn advance_typed_operation_publication_unit(&mut self,identity:&mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let retained=cx.retained_grant();if cx.fuel_exhausted(){return Ok(())}
        if !cx.should_yield() && !self.tools.latest_wins_commands.is_empty() && (self.tools.latest_wins_turn || self.tools.tool_operations.is_empty()) {
            self.tools.latest_wins_turn = false;
            record_typed_operation_unit(TypedOperationUnitKind::LatestWins);
            return self.advance_latest_wins_command_one(retained, cx).await.map_err(Into::into);
        }
        self.tools.latest_wins_turn = true;
        let Some((index, operation_id)) = self.tools.next_advanceable_typed_operation() else {
            record_typed_operation_unit(TypedOperationUnitKind::Idle);
            return Ok(());
        };
        self.tools.typed_publication_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
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
        let before = publication_witness(&mounted);
        let outcome=if mounted.borrows_original_emission_authority(){self.publish_original_emission_run(&mut mounted,identity,cx,retained).await}else{self.publish_mounted_typed_operation_run(&mut mounted,cx,retained).await};
        let attempted = match outcome {
            Ok(attempted) => attempted,
            Err(failure) => {
                self.tools.tool_operations.insert_admitted(operation_id, mounted);
                return Err(failure)
            }
        };
        let driver_result=self.revalidate_mounted_publication(&mut mounted,identity,cx).await;
        let after = publication_witness(&mounted);
        let genesis = matches!(mounted.publication, Some(ArtifactToolCompletionValue::Genesis(_, Some(_))));
        let streak = observe_publication_stall(
            &mut mounted.publication_stall_streak, attempted, before != after,
            after.0 == MountedTypedCommandFullOperationStage::Publishing, genesis,
        );
        if streak >= TYPED_OPERATION_STALL_WITNESS_FLOOR && streak.is_power_of_two() {
            let pending = mounted.pending_artifact_publication.as_ref();
            let checkpoint = match pending {
                Some(PendingArtifactStorePublication::Artifact(publication)) => Some(publication.progress()),
                Some(PendingArtifactStorePublication::Config(publication)) => Some(publication.progress()),
                Some(PendingArtifactStorePublication::Draft(publication)) => Some(publication.progress()),
                _ => None,
            };
            semio_plugin_reactor_app::debug_runtime_line(format_args!(
                "[TRACE] typed-operation {} advanced {streak} units with no change: operation={operation_id} stage={:?} flags={:#010b} lane={:?} checkpoint={checkpoint:?} returned_reads={} fuel={} grant={:?}",
                mounted.verb, after.0, after.1, pending.map(|publication| publication.lane().0),
                self.stores.store.returned_snapshot_read_count(), cx.fuel_remaining(), cx.retained_grant(),
            ));
        }
        self.tools.tool_operations.insert_admitted(operation_id, mounted);
        driver_result?;
        self.tools.fault_refused_typed_operation_publication(operation_id)?;
        if streak >= TYPED_OPERATION_STALL_FAULT_CEILING {
            self.tools.fault_stalled_typed_operation_publication(operation_id, streak)?;
        }
        Ok(())
    }

}

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    /// 🧭️ Revalidates a published document through the same owned completion or original caller loan.
    async fn revalidate_mounted_publication(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>,identity:&mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        if !mounted.published_artifact||mounted.interaction_revalidated{return Ok(())}
        if cx.is_cancelled()||mounted.cancellation_lease.as_ref().is_some_and(|lease|lease.publication_is_cancelled()){return Ok(())}
        let borrowed=mounted.borrows_original_emission_authority();
        if borrowed{mounted.validate_original_emission_authoring(cx)?;}
        if borrowed{
            self.as_render().revalidate_interaction_on_document_change(identity).await?;
            mounted.interaction_revalidated=true;
            return Ok(())
        }
        let lease=mounted.cancellation_lease.as_ref().ok_or_else(||plugin_sdk_fault("mounted revalidation lacks its original cancellation lease"))?;
        let token=lease.token.clone();let publication=lease.publication_claim.clone();
        let receipt=mounted.identity.take().ok_or_else(||plugin_sdk_fault("mounted revalidation lacks its original authoring receipt"))?;
        let mut progress=mounted.identity_progress;
        let mut observer=|next:semio_framework_value::native_encoding::NativeEncodeProgress|{progress=next;!token.is_cancelled_now()&&!publication.is_cancelled()};
        let mut work=cx.native_encoder_work();
        let mut owned_identity=semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::resume(receipt,&mut observer as &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).with_work_lender(Box::new(semio_plugin_retained_command::typed_operation::MountedIdentityWork(&mut work)));
        let result=self.as_render().revalidate_interaction_on_document_change(&mut owned_identity).await;
        mounted.identity=Some(owned_identity.pause().map_err(|error|plugin_sdk_fault(error.to_string()))?);drop(observer);cx.consume_fuel(work.performed());mounted.identity_progress=progress;
        result?;mounted.interaction_revalidated=true;Ok(())
    }

    /// 🪟️ Continues the window-config authority refresh of a published receipt; once it is complete the extensions are told
    /// before the retained receipt's result page is queued.
    pub(crate) fn advance_mounted_window_config_refresh(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, retained: RetainedCloneGrant,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(),semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let op=mounted.operation.operation.0;
        let advanced = {
            let Some(((advanced, work), _)) = cx.with_original_effect(retained, |_, effect| { let advanced = self.stores.advance_window_config_refresh(mounted, effect); (advanced, effect.performed_work()) })? else { note_publication_wait(op,"the window config refresh was not admitted (items, copy bytes, fuel)",[retained.maximum_items as u64,retained.maximum_copy_bytes as u64,cx.fuel_remaining()]);return Ok(()) };
            semio_framework_job::receive_original_retained_work(cx, semio_framework_value::RetainedWorkOutcome { result: Ok::<_, semio_framework_value::ValueError>(semio_framework_value::RetainedCloneStep::Progress(Default::default())), work })?;
            advanced?
        };
        if !advanced{note_publication_wait(op,"the window config refresh is still in progress (items, copy bytes, fuel)",[retained.maximum_items as u64,retained.maximum_copy_bytes as u64,cx.fuel_remaining()]);}
        if advanced {
            if mounted.publication_ownership_progress.is_some(){return Ok(())}
            let receipt=mounted.pending_window_config_receipt.ok_or_else(||plugin_sdk_fault("window config refresh lost its accepted receipt"))?;
            if mounted.reserve_store_result(TypedOperationResultLane::WindowConfig,receipt,None,cx)?{X::window_config_published(&mut *self.time_travel,&mut *self.tool_runs);}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{observe_publication_stall, TYPED_OPERATION_STALL_FAULT_CEILING};

    #[test]
    fn selected_publication_streaks_match_the_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../🧪️tests/🧫️fixtures/publication-stall.json"
        )).unwrap();
        let rounds = fixture["rounds"].as_u64().unwrap();
        assert_eq!(fixture["faultCeiling"].as_u64(), Some(TYPED_OPERATION_STALL_FAULT_CEILING));
        let streams = fixture["streams"].as_array().unwrap();
        let mut outcomes = vec![(0, None); streams.len()];
        for turn in 1..=rounds {
            for (stream, (streak, first_fault)) in streams.iter().zip(&mut outcomes) {
                let attempted = turn > stream["skipBefore"].as_u64().unwrap();
                let every = stream["progressEvery"].as_u64().unwrap();
                let receipt_every = stream["receiptEvery"].as_u64().unwrap_or(0);
                let changed = (every != 0 && turn % every == 0) || (receipt_every != 0 && turn % receipt_every == 0);
                let genesis = stream["genesis"].as_bool().unwrap();
                let observed = observe_publication_stall(streak, attempted, changed, true, genesis);
                if observed >= TYPED_OPERATION_STALL_FAULT_CEILING {
                    first_fault.get_or_insert(turn);
                }
            }
        }
        for (stream, (streak, first_fault)) in streams.iter().zip(outcomes) {
            assert_eq!(Some(streak), stream["streak"].as_u64());
            assert_eq!(first_fault, stream["firstFault"].as_u64());
        }
        let mut streak = 7;
        assert_eq!(observe_publication_stall(&mut streak, false, false, true, false), 0);
        assert_eq!(streak, 7);
        assert_eq!(observe_publication_stall(&mut streak, true, false, false, false), 0);
        assert_eq!(streak, 0);
    }
}
