//! 🏃️ Mounted run and pump of the typed operation publication: the mounted run with its admitted variant, the publication pump and the mounted interaction unit. See [`super`].

use super::*;
use semio_os_kernel::os_vcs::io::binary::entity_identity::control::{EntityIdentityAuthority, Observer};
use semio_plugin_retained_command::typed_operation::pending_result_page::{PublicationPagePlan,PublicationPageIntent};
use semio_plugin_retained_command::typed_operation::{note_publication_wait,MountedIdentityWork};

fn publication_unit_advanced(
    receipt_pending: bool,
    progress: RetainedCloneProgress,
    checkpoint_advanced: bool,
) -> bool {
    progress != RetainedCloneProgress::default() || (!receipt_pending && checkpoint_advanced)
}

/// ⏸️ A pump whose last unit stood still only because its turn deadline or fuel had passed was paused, not stalled: only a still-running turn counts a standing unit toward the stall streak. See [`publication_unit_advanced`].
fn publication_pump_counts_as_attempt(attempted: bool, moved: bool, yielded: bool) -> bool {
    attempted && (moved || !yielded)
}

/// 🪙️ A step that receives store work needs one phase item beyond the cost of its receiving scope.
fn publication_turn_funds_step(phase_items: usize, receiving_scope_items: Option<usize>) -> bool {
    receiving_scope_items.is_none_or(|scope_items| phase_items > scope_items)
}

#[cfg(test)]
mod tests {
    use super::{publication_pump_counts_as_attempt, publication_turn_funds_step, publication_unit_advanced, RetainedCloneProgress};

    #[test]
    fn a_turn_without_items_beyond_the_scope_headroom_ends_unfunded_and_uncounted() {
        for scope in [0usize, 1, 10] {
            assert!(!publication_turn_funds_step(0, Some(scope)));
            assert!(!publication_turn_funds_step(scope, Some(scope)));
            assert!(publication_turn_funds_step(scope + 1, Some(scope)));
        }
        assert!(!publication_pump_counts_as_attempt(true, false, true));
        assert!(publication_pump_counts_as_attempt(true, true, true));
    }

    #[test]
    fn page_and_retirement_steps_can_use_items_below_receiving_scope_headroom() {
        assert!(publication_turn_funds_step(1, None));
        assert!(!publication_turn_funds_step(1, Some(10)));
    }

    #[test]
    fn a_standing_unit_past_its_turn_deadline_is_paused_not_counted() {
        assert!(!publication_pump_counts_as_attempt(true, false, true));
        assert!(publication_pump_counts_as_attempt(true, false, false));
        assert!(publication_pump_counts_as_attempt(true, true, true));
        assert!(!publication_pump_counts_as_attempt(false, false, false));
        assert!(!publication_pump_counts_as_attempt(false, true, true));
    }

    #[test]
    fn publication_batches_paid_units_at_the_same_checkpoint() {
        for progress in [
            RetainedCloneProgress { copied_items: 1, ..Default::default() },
            RetainedCloneProgress { copied_bytes: 1, ..Default::default() },
            RetainedCloneProgress { retained_capacity_bytes: 1, ..Default::default() },
            RetainedCloneProgress { released_bytes: 1, ..Default::default() },
        ] {
            assert!(publication_unit_advanced(false, progress, false), "{progress:?}");
            assert!(publication_unit_advanced(true, progress, false), "{progress:?}");
        }
    }

    #[test]
    fn publication_yields_to_an_empty_receipt_or_unchanged_frontier() {
        let empty = RetainedCloneProgress::default();
        assert!(publication_unit_advanced(false, empty, true));
        assert!(!publication_unit_advanced(false, empty, false));
        assert!(!publication_unit_advanced(true, empty, true));
        assert!(!publication_unit_advanced(true, empty, false));
    }
}

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
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
    pub async fn publish_mounted_typed_operation_run(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>,cx:&mut semio_framework_job::StepContext<'_>, retained:RetainedCloneGrant) -> Result<bool,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        if mounted.identity.is_none() {
            let completion = mounted.completion.as_ref().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original effect completion"))?;
            let (receipt, progress) = completion.take_authoring_identity()?.ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original app-owned effect receipt"))?;
            mounted.identity = Some(receipt);
            mounted.identity_progress = progress;
        }
        let attempted = self.publish_mounted_typed_operation_run_admitted(mounted, None, cx, retained).await?;
        if cx.is_cancelled() || mounted.cancellation_lease.as_ref().is_some_and(|lease| lease.publication_is_cancelled()) {
            return Ok(attempted);
        }
        Self::with_mounted_publication_authoring(mounted, None, cx, async |_, identity| {
            self.child_composition.follow_derivable_children(&mut *self.stores, identity).await
        }).await?;
        Ok(attempted)
    }

    /// 🪪️ Lends current-turn Work to one authoring call and returns the original identity before another publication unit starts.
    async fn with_mounted_publication_authoring<T>(
        mounted: &mut MountedTypedCommandFullOperation<A>,
        borrowed: Option<&mut EntityIdentityAuthority<'_>>,
        cx: &mut semio_framework_job::StepContext<'_>,
        run: impl AsyncFnOnce(&mut MountedTypedCommandFullOperation<A>, &mut EntityIdentityAuthority<'_>) -> Result<T, Fault>,
    ) -> Result<T, Fault> {
        if let Some(identity) = borrowed {
            return run(mounted, identity).await;
        }
        let lease = mounted.cancellation_lease.as_ref().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its original cancellation lease"))?;
        let token = lease.token.clone();
        let publication = lease.publication_claim.clone();
        let receipt = mounted.identity.take().ok_or_else(|| plugin_sdk_fault("mounted publication lacks its admitted authoring receipt"))?;
        let mut progress = mounted.identity_progress;
        let mut observer = |next: semio_framework_value::native_encoding::NativeEncodeProgress| { progress = next; !token.is_cancelled_now() && !publication.is_cancelled() };
        let mut work = cx.native_encoder_work();
        let mut identity = EntityIdentityAuthority::resume(receipt, &mut observer as &mut Observer<'_>)
            .with_work_lender(Box::new(MountedIdentityWork(&mut work)));
        let result = run(mounted, &mut identity).await;
        let receipt = identity.pause();
        drop(observer);
        cx.consume_fuel(work.performed());
        mounted.identity_progress = progress;
        mounted.identity = Some(receipt.map_err(|error| plugin_sdk_fault(error.to_string()))?);
        result
    }

    /// 🤲️ Publishes a direct emission through the actual caller loan without creating another authoring ceiling.
    pub async fn publish_original_emission_run(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>,identity:&mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>,retained:RetainedCloneGrant)->Result<bool,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        mounted.validate_original_emission_authoring(cx)?;
        if cx.is_cancelled(){if let Some(lease)=mounted.cancellation_lease.as_ref(){lease.cancel();}}
        let attempted = self.publish_mounted_typed_operation_run_admitted(mounted,Some(&mut *identity),cx,retained).await?;
        if cx.is_cancelled(){if let Some(lease)=mounted.cancellation_lease.as_ref(){lease.cancel();}}
        if mounted.cancellation_lease.as_ref().is_some_and(|lease|lease.publication_is_cancelled()){return Ok(attempted)}
        self.child_composition.follow_derivable_children(&mut *self.stores,identity).await.map(|()| attempted).map_err(Into::into)
    }

    async fn publish_mounted_typed_operation_run_admitted(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: Option<&mut EntityIdentityAuthority<'_>>,cx:&mut semio_framework_job::StepContext<'_>, retained:RetainedCloneGrant) -> Result<bool,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        if mounted.publication_ownership_progress.is_some(){return Ok(false);}
        let mut spent = RetainedCloneProgress::default();
        let outcome = self.publish_mounted_typed_operation_pump(mounted, identity, retained, &mut spent,cx).await;
        if spent != RetainedCloneProgress::default() {
            assert!(mounted.publication_ownership_progress.is_none(), "typed preparation cannot overwrite an unreturned original receipt");
            mounted.publication_ownership_progress = Some((semio_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation, retained), spent));
        }
        outcome
    }

    /// 🏃️ Spends ONE host turn's grant on consecutive publication units: each unit receives what the turn still owns, its receipt is summed into `spent`, and the pump hands the turn back as soon as the grant is exhausted — so a grant of one item still publishes one unit per turn, and a larger grant batches exactly as many units as it pays for.
    async fn publish_mounted_typed_operation_pump(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, mut identity: Option<&mut EntityIdentityAuthority<'_>>, retained:RetainedCloneGrant, spent: &mut RetainedCloneProgress,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let started_us=cx.now_us();
        let turn = semio_plugin_ownership::bounded_ownership::original_plugin_turn_grant(self.mounted_policy.preparation, retained);
        let mut attempted = false;
        let mut moved = false;
        for _ in 0..TYPED_OPERATION_PUBLICATION_PUMPS {
            if cx.is_cancelled(){if let Some(lease)=mounted.cancellation_lease.as_ref(){lease.cancel();}}
            let acknowledged_page = mounted.result_page_retirement_pending();
            let pending_store = mounted.pending_artifact_publication.as_ref();
            let custody = mounted.page_publication_pending() || pending_store.is_some_and(|owner| owner.accepted_receipt().is_some());
            if cx.fuel_exhausted() || (cx.should_yield() && !acknowledged_page && !custody) {
                note_publication_wait(
                    mounted.operation.operation.0,
                    "the turn yielded before the publication unit (fuel, deadline exceeded, retained items)",
                    [cx.fuel_remaining(), u64::from(cx.should_yield()), cx.retained_grant().maximum_items as u64],
                );
                return Ok(attempted);
            }
            let cancelled = mounted.cancellation_lease.as_ref().is_some_and(|lease| lease.publication_is_cancelled());
            let needs_receiving = !acknowledged_page && !custody && !cancelled
                && mounted.owned_child_group.is_none()
                && mounted.pending_child_publication.is_none()
                && mounted.publication_failure.terminal_is_empty()
                && pending_store.is_some_and(|owner| !owner.is_closing());
            let phase_items = cx.retained_grant().maximum_items;
            let receiving_scope_items = needs_receiving.then(|| {
                if cx.original_receiving_scope_held() { 0 } else { semio_framework_job::original_receiving_scope_headroom().0 }
            });
            if !publication_turn_funds_step(phase_items, receiving_scope_items) {
                note_publication_wait(
                    mounted.operation.operation.0,
                    "the turn phase items cannot fund one receiving step (phase items, scope held, fuel)",
                    [phase_items as u64, u64::from(cx.original_receiving_scope_held()), cx.fuel_remaining()],
                );
                return Ok(publication_pump_counts_as_attempt(attempted, moved, true));
            }
            attempted = true;
            let before = cx.retained_progress();
            let reserved_before = cx.reserved_return_progress();
            let fuel = cx.fuel_remaining();
            let owned_child_route = !acknowledged_page && !custody && !cancelled && mounted.owned_child_group.is_some();
            let mut staged = false;
            if owned_child_route {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                let remaining=semio_plugin_reactor_app::remaining_plugin_turn_grant(turn,*spent);
                self.publish_mounted_owned_child_operation_unit(mounted,cx,remaining).await?;
            } else if !acknowledged_page && !custody && !cancelled && mounted.pending_child_publication.is_some() {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_child_operation_unit(mounted, cx).await?;
            } else if !acknowledged_page && !custody && !cancelled && mounted_typed_interaction_writes_are_next::<A>(mounted) {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                self.publish_mounted_typed_interaction_unit(mounted, identity.as_deref_mut(), cx).await?;
            } else if !acknowledged_page && !custody && !cancelled && self.tools.mounted_typed_inline_interaction_verbs_are_next(mounted) {
                record_typed_operation_unit(TypedOperationUnitKind::Store);
                Self::with_mounted_publication_authoring(mounted, identity.as_deref_mut(), cx, async |mounted, identity| {
                    self.publish_mounted_typed_inline_interaction_unit(mounted, identity).await
                }).await?;
            } else {
                staged = true;
                let before = (mounted.publication_progress, mounted.publication.is_some(), mounted.pending_artifact_publication.is_some(), mounted.stage);
                let remaining = semio_plugin_reactor_app::remaining_plugin_turn_grant(turn, *spent);
                let (receipt, unit) = cx.with_retained_loan(remaining, |cx| {
                    let original_before = cx.retained_progress();
                    let grant = cx.retained_grant();
                    self.publish_mounted_typed_operation_unit(mounted, grant, cx).map_err(semio_framework_job::OriginalTurnFailure::from)?;
                    let receipt = mounted.publication_ownership_progress.take();
                    if receipt.is_none() {
                        cx.settle_receiving_scope()?;
                    }
                    let after = semio_plugin_ownership::bounded_ownership::original_lifecycle_net_after(cx.retained_progress(), reserved_before, cx.reserved_return_progress());
                    let unit = if let Some((_, unit)) = receipt {
                        let unposted = semio_plugin_ownership::bounded_ownership::original_lifecycle_unposted(unit, original_before, after)?;
                        cx.consume_retained(unposted)?;
                        unit
                    } else {
                        semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(original_before, after)?
                    };
                    Ok((receipt, unit))
                })?;
                *spent=spent.checked_add(unit)?;
                if self.stores.store.backbone_ref().is_some() {
                    self.stores.store.flush_announcements().await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    if let Some(PendingArtifactStorePublication::Artifact(publication)) = mounted.pending_artifact_publication.as_mut() {
                        self.stores.store.flush_published_apply_batch(publication).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
                    }
                }
                record_typed_operation_unit(if mounted.result_page.is_some() { TypedOperationUnitKind::Page } else { TypedOperationUnitKind::Store });
                let after = (mounted.publication_progress, mounted.publication.is_some(), mounted.pending_artifact_publication.is_some(), mounted.stage);
                if !publication_unit_advanced(receipt.is_some(), unit, before != after) {
                    if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}
                    return Ok(publication_pump_counts_as_attempt(attempted,moved,cx.should_yield()));
                }
            }
            moved=true;
            if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}
            if !staged{let after=semio_plugin_ownership::bounded_ownership::original_lifecycle_net_after(cx.retained_progress(),reserved_before,cx.reserved_return_progress());let delta=semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before,after)?;*spent=spent.checked_add(delta)?;}
            if semio_plugin_reactor_app::remaining_plugin_turn_grant(turn,*spent).maximum_items==0{return Ok(attempted)}
            if mounted.stage != MountedTypedCommandFullOperationStage::Publishing {
                return Ok(attempted);
            }
            if mounted.publication.is_none() && mounted.pending_artifact_publication.is_none() && mounted.pending_child_publication.is_none() && mounted.owned_child_group.is_none() {
                return Ok(attempted);
            }
            let Some(started_us) = started_us else { return Ok(attempted) };
            if cx.now_us().is_some_and(|now_us| now_us.saturating_sub(started_us) >= INTERACTIVE_TURN_WORKER_WALL_US) {
                return Ok(attempted);
            }
        }
        Ok(attempted)
    }

    /// 🕹️ Publishes this operation's whole [`InteractionWrite`] lane as ONE unit through the same
    /// `apply_interaction_writes` single-writer path `dispatch_emit` uses — the retained-tool twin
    /// of that call. It is one unit rather than one-write-per-turn because a selection is a single
    /// state-machine transition per domain: draining it write-by-write would publish intermediate
    /// selections no user ever asked for. `MountedTypedCommandFullOperation` needs no extra field:
    /// the writes live in the retained emit until taken here, and the lane is only entered once
    /// every durable lane is drained (`mounted_typed_interaction_writes_are_next`).
    async fn publish_mounted_typed_interaction_unit(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>, identity: Option<&mut EntityIdentityAuthority<'_>>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() else {
            return Err(plugin_sdk_fault("typed-operation interaction publication lost its emit owner"));
        };
        let d=semio_framework_value::RetirementDemand{copy_bytes:2*std::mem::size_of_val(&emit.interaction_writes)+std::mem::size_of::<Option<PublicationPagePlan>>()+2*std::mem::size_of_val(&mounted.page_publication.interaction_close),depth:1,..Default::default()};
        if !cx.retained_work_can_enter(d){return Ok(())}
        let writes = std::mem::take(&mut emit.interaction_writes);
        let result = Self::with_mounted_publication_authoring(mounted, identity, cx, async |mounted, identity| {
            self.as_render().apply_interaction_writes(&writes, &mounted.meta, identity).await
        }).await;
        if let Err(fault) = result {
            if let Some(ArtifactToolCompletionValue::Emit(Ok(emit), _)) = mounted.publication.as_mut() {
                emit.interaction_writes = writes;
            }
            return Err(fault);
        }
        let token = mounted.next_token();let count=writes.len()as u64;
        mounted.page_publication.interaction_close=Some(semio_framework_value::retirement::controlled::ControlledRetirement::retain(writes));
        mounted.page_publication.plan=Some(PublicationPagePlan{token,intent:PublicationPageIntent::Accepted{lane:TypedOperationResultLane::Interaction,count}});
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}).map_err(plugin_retirement_fault)?;cx.consume_fuel(1);
        Ok(())
    }

}
