//! 📮️ Mailbox impl: the single runtime mailbox implementation.
use super::*;

#[path = "🏗️mailbox-construction-and-authorities/🦀️.rs"]
mod mailbox_construction_and_authorities;

#[path = "📥️renderer-asset-steps/🦀️.rs"]
mod renderer_asset_steps;

#[path = "🖼️asset-decode-pump-and-reference-images/🦀️.rs"]
mod asset_decode_pump_and_reference_images;

#[path = "🌐️native-asset-pumps/🦀️.rs"]
mod native_asset_pumps;

    #[cfg(not(target_arch = "wasm32"))]
    pub fn recover_original_frame_step(&self, runtime: &mut R, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let Ok(mut slot) = self.0.original_recovery.try_lock() else { return Ok(false); };
        if slot.is_none() {
            if cx.retirement_should_yield() || cx.retained_grant().maximum_items == 0 || cx.retained_grant().maximum_depth == 0 { return Ok(false); }
            let original = if let Some(original) = runtime.pending_frame_maintenance_refusal_mut().take() { Some(FrameOriginalRecovery::Maintenance(original)) }
                else if self.0.frame_maintenance_executions.has_returned_owner() { self.try_take_returned_frame_maintenance().map(FrameOriginalRecovery::Maintenance) }
                else { self.0.frame_deferred_executions.try_take_returned().map(|(_, owner)| FrameOriginalRecovery::Deferred(owner)) };
            if original.is_none() { return Ok(false); }
            *slot = original;
            cx.consume_retained(semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
            cx.consume_fuel(1);
            return Ok(true);
        }
        if slot.as_ref().unwrap().empty() {
            let demand = slot.as_ref().unwrap().release_demand();
            let step = semio_framework_job::close_original_retained_step(cx, demand, |grant| FrameOriginalRecovery::release_empty(&mut slot, grant))?;
            return Ok(step.progress() != Default::default());
        }
        let original = slot.as_mut().unwrap();
        if let FrameOriginalRecovery::Maintenance(original) = original {
            if original.reservation_live {
                if cx.retirement_should_yield() || cx.retained_grant().maximum_items == 0 || cx.retained_grant().maximum_depth == 0 { return Ok(false); }
                if !self.cancel_interaction_future_reservation() { return Ok(false); }
                original.reservation_live = false;
                cx.consume_retained(semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                cx.consume_fuel(1);
                return Ok(true);
            }
        }
        let mut restore = |interaction: &mut Option<AppInteractionState>, cursor: &mut Option<FrameDeferredCursor>, maintenance: bool, cx: &mut semio_framework_job::StepContext<'_>| -> Result<bool, semio_framework_job::OriginalTurnFailure> {
            if !runtime.interaction_available() && interaction.is_some() {
                if !frame_action_metadata(cx)? { return Ok(false); }
                runtime.return_interaction(interaction.take().unwrap());
                return Ok(true);
            }
            if runtime.pending_frame_deferred().is_none() && cursor.is_some() {
                if !frame_action_metadata(cx)? { return Ok(false); }
                if maintenance { cursor.as_mut().unwrap().return_shell_maintenance(); }
                *runtime.pending_frame_deferred_mut() = cursor.take();
                return Ok(true);
            }
            Ok(false)
        };
        match original {
            FrameOriginalRecovery::Maintenance(original) => original.owner.try_borrow_mut(|owner| {
                let result = restore(&mut owner.interaction, &mut owner.cursor, owner.restart, cx);
                if owner.cursor.is_none() { owner.restart = false; }
                result
            }).unwrap_or(Ok(false)),
            FrameOriginalRecovery::Deferred(original) => original.try_borrow_mut(|owner| {
                if owner.work.is_some() {
                    if !frame_action_metadata(cx)? { return Ok(false); }
                    owner.cancel_current();
                    return Ok(owner.work.is_none());
                }
                restore(&mut owner.interaction, &mut owner.cursor, false, cx)
            }).unwrap_or(Ok(false)),
        }
    }

#[path = "⏳️pending-work-deadlines-and-spawns/🦀️.rs"]
mod pending_work_deadlines_and_spawns;

impl RuntimeMailbox<AppRuntime> {
    /// 📮️ Every completion remains queue-owned across child pending turns and complete original failures.
    pub fn apply_pending_step(&self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if cx.should_yield() || cx.retained_grant().maximum_items == 0 || cx.retained_grant().maximum_depth == 0 { return Ok(false); }
        let Ok(mut runtime) = self.try_lock() else { return Ok(false) };
        #[cfg(not(target_arch = "wasm32"))]
        if self.recover_original_frame_step(&mut *runtime, cx)? { return Ok(true); }
        if cx.should_yield() { return Ok(false); }
        let Ok(mut queue) = self.0.completions.try_lock() else { return Ok(false) };
        let available = runtime.interaction_available();
        let head = queue.head_requires_interaction();
        let owner = queue.interaction_owner_outstanding();
        let admission = runtime.checkout.admit(head, available, owner);
        let Some(index) = queue.first_applicable(available) else {
            if admission == runtime_mailbox_core::InteractionCheckoutStep::Abandoned {
                return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original interaction checkout has no return owner").into());
            }
            return Ok(false);
        };
        let selected = &queue.ready[index];
        if selected.key.is_some() && !(selected.key == Some("window-metrics") && matches!(&selected.apply, RuntimeApply::Resize { .. } | RuntimeApply::Applied)) {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "runtime completion key has no declared scalar revision owner").into());
        }
        let removing = matches!(&selected.apply, RuntimeApply::Applied);
        let child_controls_work = matches!(&selected.apply, RuntimeApply::DispatchEvents(_) | RuntimeApply::ResumeDispatch { .. } | RuntimeApply::ResumeFrameDeferred { .. });
        #[cfg(not(target_arch = "wasm32"))]
        let child_controls_work = child_controls_work || matches!(&selected.apply, RuntimeApply::RestoreInteraction(_));
        if removing || !child_controls_work {
            cx.consume_retained(semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
            cx.consume_fuel(1);
        }
        let (mut completion, checkout) = queue.checkout_at(index).expect("selected original completion");
        drop(queue);
        if removing {
            if completion.key == Some("window-metrics") { self.0.applied_window_metrics_revision.fetch_max(completion.revision, Ordering::AcqRel); }
            self.0.completions.lock().unwrap_or_else(std::sync::PoisonError::into_inner).finish_checkout(checkout);
            return Ok(true);
        }
        let stale = completion.key == Some("window-metrics") && completion.revision <= self.0.applied_window_metrics_revision.load(Ordering::Acquire);
        let result = if stale { Ok(true) } else { completion.apply.apply_step(&mut *runtime, &self.downgrade(), cx) };
        completion.restores_interaction = completion.apply.restores_interaction();
        if matches!(result, Ok(true)) { completion.apply = RuntimeApply::Applied; }
        self.0.completions.lock().unwrap_or_else(std::sync::PoisonError::into_inner).return_checkout(index, completion, checkout);
        result.map(|_| true)
    }

    pub fn pump_pending_applies(&self, credits: u32, cx: &mut semio_framework_job::StepContext<'_>) -> Result<u32, semio_framework_job::OriginalTurnFailure> {
        let mut applied = 0;
        while applied < credits && self.apply_pending_step(cx)? { applied += 1; }
        Ok(applied)
    }

    /// 👁️ Observes checkout custody without moving a completion or issuing another original caller.
    pub fn presenter_interaction_status(&self) -> PresenterInteractionStep {
        let Ok(runtime) = self.try_lock() else { return PresenterInteractionStep::RuntimeLock };
        if runtime.interaction_available() { return PresenterInteractionStep::Ready; }
        let Ok(queue) = self.0.completions.try_lock() else { return PresenterInteractionStep::RuntimeLock };
        let owner = queue.interaction_owner_outstanding();
        #[cfg(not(target_arch = "wasm32"))]
        let owner = owner || !self.0.original_recovery.try_lock().is_ok_and(|owner| owner.is_none()) || runtime.pending_frame_maintenance_refusal.is_some() || self.0.frame_maintenance_executions.state.load(Ordering::Acquire) != 0 || self.0.frame_deferred_executions.state.load(Ordering::Acquire) != 0;
        if owner { PresenterInteractionStep::Waiting { site: runtime.checkout.site().unwrap_or("unknown"), request: semio_framework_os_renderer_wgpu_host_base::host_io_request_trace_label() } } else { PresenterInteractionStep::Abandoned }
    }

    /// 🤝️ Restores the actual interaction in place while its remaining cursor stays in the same bounded queue.
    pub fn restore_presenter_interaction_step(&self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<PresenterInteractionStep, semio_framework_job::OriginalTurnFailure> {
        if cx.should_yield() || cx.retained_grant().maximum_items == 0 || cx.retained_grant().maximum_depth == 0 { return Ok(self.presenter_interaction_status()); }
        let Ok(mut runtime) = self.try_lock() else { return Ok(PresenterInteractionStep::RuntimeLock) };
        if runtime.interaction_available() { return Ok(PresenterInteractionStep::Ready); }
        let Ok(mut queue) = self.0.completions.try_lock() else { return Ok(PresenterInteractionStep::RuntimeLock) };
        if let Some(index) = queue.first_interaction_restoration() {
            let completion = &mut queue.ready[index];
            if let RuntimeApply::ResumeDispatch { cursor: Some(cursor), .. } = &completion.apply { cursor.validate_original(cx)?; }
            cx.consume_retained(semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
            cx.consume_fuel(1);
            completion.apply.restore_interaction_only(&mut *runtime);
            completion.restores_interaction = completion.apply.restores_interaction();
            return Ok(PresenterInteractionStep::Restored);
        }
        drop(queue);
        #[cfg(not(target_arch = "wasm32"))]
        if self.recover_original_frame_step(&mut *runtime, cx)? && runtime.interaction_available() { return Ok(PresenterInteractionStep::Restored); }
        drop(runtime);
        Ok(self.presenter_interaction_status())
    }

}
