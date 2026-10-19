//! ⏳️ Pending work, deadlines and spawns: the waker, the apply and dispatch queues, the pending work queries, the fault takes, the control deadlines, the future reservations and the frame spawns. See [`super`].

use super::*;

//#region 📮️RuntimeMailbox
impl<R: MailboxRuntime> RuntimeMailbox<R> {
    pub fn set_waker(&self, waker: RuntimeHostWaker) {
        *self.0.waker.lock().expect("runtime completion waker lock") = Some(waker);
    }

    pub fn enqueue_apply(&self, key: Option<&'static str>, requires_interaction: bool, apply: RuntimeApply) -> bool {
        self.0.enqueue(key, requires_interaction, apply)
    }

    pub fn enqueue_dispatch_on_original_caller(&self,events:&mut ui_host::EventQueue,generation:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let mut queue=self.0.completions.lock().expect("runtime completion mailbox lock");if queue.len()>=RUNTIME_COMPLETION_CAPACITY-1||events.is_empty(){return Ok(false)}let Some(original)=RuntimeDispatchSource::admit(cx)?else{return Ok(false)};
        let input_generation=events.current_generation();let drained=events.drain_page(ui_host::WorkerContext::new(input_generation));let cursor=RuntimeDispatchCursor::new_for_generation(drained,generation,original);let completion=self.0.completion(None,true,RuntimeApply::DispatchEvents(Some(cursor)));assert!(queue.enqueue(completion),"same locked queue admits the original dispatch owner");drop(queue);self.0.presentation_authority.mark_scene_changed();if let Some(waker)=self.0.waker.lock().expect("runtime completion waker lock").as_ref(){waker()}Ok(true)
    }

    fn has_lossless_capacity(&self) -> bool {
        self.0.completions.lock().expect("runtime completion mailbox lock").len() < RUNTIME_COMPLETION_CAPACITY - 1
    }

    /// 📮️ Whether the mailbox still owes the runtime an apply — the per-frame predicate that turns a
    /// queued `DispatchEvents`/`Resize` into the next `request_frame` on an event-driven shell.
    #[cfg(target_arch = "wasm32")]
    pub fn has_pending_applies(&self) -> bool {
        !self.0.completions.lock().expect("runtime completion mailbox lock").ready.is_empty()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn has_pending_text_work(&self) -> bool {
        self.try_lock().ok().and_then(|runtime| runtime.interaction().map(AppInteractionState::has_pending_text_work)).unwrap_or(false)
    }

    /// 🌍️ Whether any World3d surface still owes its own frame.
    ///
    /// ⚖️ The world's existing wake authority (`World3dBuildContext::request_cursor_wake`) is only
    /// reachable from `render_world_3d`, i.e. from a PAINT — and a retained window republishes its
    /// cached paint while its revision is unchanged, so the paint that would ask for the next frame
    /// runs once per DOCUMENT, not once per frame. A mesh ingest that needs more frames than the
    /// document that started it therefore stalled forever on a settled, event-driven shell: measured
    /// on 6118 as `procedural-preview` parked at `apply-page=0/3 apply-item=1 rebuild=true
    /// state-meshes=2 draws=0` with `extrude@solid` sitting unconsumed in its mesh lane
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). This is the same shape as
    /// `has_pending_text_work`: a per-frame predicate the browser tick turns into `request_frame`,
    /// so the work the FRAME TRANSACTION drives keeps its own frames coming.
    #[cfg(target_arch = "wasm32")]
    pub fn has_pending_world3d_work(&self) -> bool {
        self.try_lock()
            .ok()
            .and_then(|runtime| {
                runtime.interaction().map(|interaction| interaction.shell.surfaces.world3d_states.values().any(|state| infinite_world::world::world3d_cursor_work_pending(state) || infinite_world::world::world3d_brush_mesh_announce_pending(&state.scene, &state.assets)))
            })
            .unwrap_or(false)
    }

    /// 📡️ Reports only work owned by the private decoder continuation.
    pub(super) fn has_pending_asset_decode(&self) -> bool {
        if self.0.asset_decode_session.try_lock().map_or(true, |owner| owner.is_some()) {
            return true;
        }
        if self.0.asset_probe.try_lock().map_or(true, |owner| owner.is_some()) {
            return true;
        }
        if renderer_asset_io().try_lock().map_or(true, |authority| authority.has_completed_step()) {
            return true;
        }
        self.try_lock().map_or(true, |runtime| runtime.interaction().is_some_and(|interaction| interaction.shell.surfaces.world3d_states.values().any(|world| infinite_world::world::world3d_asset_decode_pending(&world.assets))))
    }

    /// 🫀️ Whether the shell's settle lane still owes a step — the per-frame predicate that keeps an
    /// event-driven shell ticking through a convergence nobody is touching.
    ///
    /// ⚖️ This is what makes the settle pump real on this target. `settle_boot` now ARMS the lane and
    /// returns instead of driving the chain to a fixed point before the first paint, and every input
    /// gesture only declares; without this term the browser shell would settle the instant the last
    /// apply drained, and a booted example would converge only for a user who kept moving the mouse —
    /// which is exactly the hole `refresh_ui`'s own measurements named
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-progress-visibility-2026-09-14.md` §8.1).
    ///
    /// A checked-out interaction answers `false` on purpose: its owner is a live apply, and
    /// [`Self::has_pending_applies`] already owes the frame.
    #[cfg(target_arch = "wasm32")]
    pub fn has_pending_settle(&self) -> bool {
        self.try_lock().ok().and_then(|runtime| runtime.interaction().map(|interaction| semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx::of(&mut interaction.shell).settle_pump_pending())).unwrap_or(false)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn take_text_fault(&self) -> Option<String> {
        self.try_lock().ok()?.interaction_mut()?.text_fault.take()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn take_frame_fault(&self) -> Option<String> {
        if let Some(fault) = self.0.frame_fault.lock().expect("runtime frame fault lock").take() {
            return Some(fault);
        }
        self.try_lock().ok()?.interaction_mut()?.frame_fault.take()
    }

    pub fn record_frame_fault(&self, fault: &'static str) {
        log_debug(&format!("[TRACE] frame fault recorded: {fault}"));
        let mut slot = self.0.frame_fault.lock().expect("runtime frame fault lock");
        if slot.is_none() {
            *slot = Some(fault.to_string());
        }
    }

    pub fn frame_inputs(&self, now_ms: f64) -> FrameBuildInputs {
        let mut inputs = self.0.frame_inputs.try_lock().map(|inputs| inputs.clone()).unwrap_or_default();
        inputs.now_ms = now_ms;
        inputs
    }

    pub fn publish_retained_control_deadline(&self, deadline: Option<f64>) {
        let due_us = deadline.filter(|seconds| seconds.is_finite() && *seconds >= 0.0).map_or(u64::MAX, |seconds| (seconds * 1_000_000.0).ceil() as u64);
        self.0.retained_control_deadline_us.store(due_us, Ordering::Release);
    }

    pub fn publish_shell_clock_deadline(&self, deadline: Option<f64>) {
        let due_us = deadline.filter(|seconds| seconds.is_finite() && *seconds >= 0.0).map_or(u64::MAX, |seconds| (seconds * 1_000_000.0).ceil() as u64);
        self.0.shell_clock_deadline_us.store(due_us, Ordering::Release);
    }

    pub fn control_deadline_us(&self) -> Option<u64> {
        let due_us = self.0.retained_control_deadline_us.load(Ordering::Acquire).min(self.0.shell_clock_deadline_us.load(Ordering::Acquire));
        (due_us != u64::MAX).then_some(due_us)
    }

    pub fn retained_control_deadline(&self, host_seconds: f64) -> Option<ui_render::Deadline> {
        semio_framework_os_renderer_wgpu_host_plumbing::deadlines::retained_control_deadline(self.control_deadline_us(), semio_framework_job::default_now_us(), host_seconds)
    }

    pub fn update_frame_inputs(&self, runtime: &R) {
        let Some(interaction) = runtime.interaction() else {
            return;
        };
        *self.0.frame_inputs.lock().expect("runtime frame inputs lock") = FrameBuildInputs { wheel_zoom_deadline_ms: interaction.wheel_zoom_deadline_ms, now_ms: app_now_ms() };
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn reserve_future(&self, key: Option<&'static str>) -> bool {
        self.0.completions.lock().expect("runtime completion mailbox lock").reserve(key)
    }

    pub fn reserve_interaction_future(&self) -> bool {
        self.0.completions.lock().expect("runtime completion mailbox lock").reserve_interaction()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn cancel_interaction_future_reservation(&self) -> bool {
        match self.0.completions.try_lock() {
            Ok(mut completions) => completions.cancel_interaction_reservation(),
            Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner().cancel_interaction_reservation(),
            Err(std::sync::TryLockError::WouldBlock) => false,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn try_take_returned_frame_maintenance(&self) -> Option<FrameMaintenanceRefusal> {
        let (generation, owner) = self.0.frame_maintenance_executions.try_take_returned()?;
        let _ = self.0.frame_maintenance.release(generation);
        Some(FrameMaintenanceRefusal { owner, reservation_live: false })
    }


    #[cfg(not(target_arch = "wasm32"))]
    pub fn spawn_interaction_reserved<F>(&self, _key: Option<&'static str>, future: F)
    where
        F: Future<Output = AppInteractionState> + Send + 'static,
    {
        let mailbox = self.clone();
        let revision = mailbox.0.next_revision.fetch_add(1, Ordering::Relaxed);
        spawn_app_task(async move {
            let interaction = future.await;
            mailbox.0.finish(returned_completion(revision, RuntimeApply::RestoreInteraction(Some(interaction))));
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn spawn_dispatch_reserved(&self,cursor:RuntimeDispatchCursor,interaction:AppInteractionState){
        let mailbox=self.clone();let revision=mailbox.0.next_revision.fetch_add(1,Ordering::Relaxed);let owner=RuntimeDispatchReturn{mailbox,revision,cursor:Some(cursor),interaction:Some(interaction)};spawn_app_task(owner.run());
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn try_spawn_frame_deferred_reserved(&self, owner: FrameDeferredExecutionOwner) -> Result<(), FrameDeferredExecutionOwner> {
        let Some(generation) = owner.generation() else { return Err(owner) };
        let cell = Arc::new(FrameExecutionOwnerCell::new(owner));
        let registry = self.0.frame_deferred_executions.clone();
        if registry.try_publish(generation, cell.clone()).is_err() {
            return Err(cell.try_take().expect("rejected frame deferred owner remains available"));
        }
        let mailbox = self.clone();
        let revision = mailbox.0.next_revision.fetch_add(1, Ordering::Relaxed);
        let execution = FrameDeferredExecutionEnvelope::new(mailbox.clone(), registry, generation, revision);
        spawn_app_task(async move {
            let Some(mut execution) = execution.begin() else {
                mailbox.record_frame_fault("frame deferred exact owner was unavailable");
                return;
            };
            let Some(work) = execution.owner_mut().and_then(|owner| owner.work.take()) else {
                mailbox.record_frame_fault("frame deferred exact work owner was unavailable");
                return;
            };
            match work {
                FrameDeferredWork::ShellMaintenance => {}
                FrameDeferredWork::PumpSync => {
                    let Some(interaction) = execution.owner_mut().and_then(|owner| owner.interaction.as_mut()) else { return };
                    semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx::of(&mut interaction.shell).pump_sync_events().await;
                }
                FrameDeferredWork::Action(action) => {
                    if action.cancelled {
                        action.cancel();
                    } else {
                        let receipt = action.receipt;
                        let mut receipt_owner = FrameActionReceiptOwner { receipt };
                        let result = {
                            let Some(interaction) = execution.owner_mut().and_then(|owner| owner.interaction.as_mut()) else { return };
                            semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx::of(&mut interaction.shell).dispatch_gesture_action(action.descriptor).await
                        };
                        match result {
                            Ok(()) => receipt_owner.settle(semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Accepted),
                            Err(error) => {
                                receipt_owner.settle(semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Refused(&error));
                                let Some(owner) = execution.owner_mut() else { return };
                                if let Some(receipt) = receipt {
                                    if receipt.abort_correlation_on_error {
                                        owner.cursor.as_mut().map(|cursor| cursor.actions.cancel_correlation(receipt.token));
                                    }
                                }
                                if let Some(interaction) = owner.interaction.as_mut() {
                                    log_debug(&format!("[TRACE] frame deferred action failed: {error}"));
                                    interaction.shell.prefs.note_dispatch_fault(&error, &mut interaction.shell.chrome);
                                }
                            }
                        }
                    }
                }
                FrameDeferredWork::FlushTutorial => {
                    let Some(interaction) = execution.owner_mut().and_then(|owner| owner.interaction.as_mut()) else { return };
                    semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx::of(&mut interaction.shell).tutorial_flush_pending_document_ops().await;
                }
                FrameDeferredWork::Settle => {
                    let Some(interaction) = execution.owner_mut().and_then(|owner| owner.interaction.as_mut()) else { return };
                    semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx::of(&mut interaction.shell).settle_pump_step().await;
                }
            }
            if !execution.finish() {
                mailbox.record_frame_fault("frame deferred execution handback faulted");
            }
        });
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn try_spawn_frame_maintenance_reserved(&self, owner: FrameMaintenanceOwner) -> Result<(), FrameMaintenanceRefusal> {
        let Some(generation) = owner.generation() else {
            return Err(FrameMaintenanceRefusal { owner: Arc::new(FrameExecutionOwnerCell::new(owner)), reservation_live: true });
        };
        let cell = Arc::new(FrameExecutionOwnerCell::new(owner));
        if !self.0.frame_maintenance.try_reserve(generation) {
            return Err(FrameMaintenanceRefusal { owner: cell, reservation_live: true });
        }
        let registry = self.0.frame_maintenance_executions.clone();
        if registry.try_publish(generation, cell.clone()).is_err() {
            let _ = self.0.frame_maintenance.release(generation);
            return Err(FrameMaintenanceRefusal { owner: cell, reservation_live: true });
        }
        let mailbox = self.clone();
        let revision = mailbox.0.next_revision.fetch_add(1, Ordering::Relaxed);
        let worker_pool = renderer_worker_pool();
        let execution = FrameMaintenanceExecutionEnvelope::new(mailbox.clone(), registry.clone(), generation, revision);
        let job: semio_framework_async::Job = Box::new(move || {
            let Some(mut execution) = execution.begin() else {
                mailbox.record_frame_fault("frame maintenance exact owner pair was incomplete");
                return;
            };
            let stale;
            {
                let Some(owner) = execution.owner_mut() else {
                    mailbox.record_frame_fault("frame maintenance execution lost its exact owner");
                    return;
                };
                let FrameMaintenanceOwner { interaction, cursor, deadline_ms, restart } = owner;
                let (Some(interaction), Some(cursor)) = (interaction.as_mut(), cursor.as_mut()) else {
                    mailbox.record_frame_fault("frame maintenance exact owner pair was incomplete");
                    return;
                };
                let cancelled = cursor.cancel.as_ref().is_none_or(|cancel| cancel.is_cancelled_now());
                stale = !mailbox.0.frame_maintenance.is_live(generation) || mailbox.0.presentation_authority.witness_for(generation).is_none();
                let deadline_exceeded = deadline_ms.is_none_or(|deadline| worker_pool.now_ms() > deadline);
                if let Some(fault) = frame_maintenance_terminal_fault(cancelled, stale, deadline_exceeded) {
                    cursor.begin_close();
                    interaction.frame_fault = Some(fault.to_string());
                } else {
                    cursor.shell_maintenance = semio_framework_os_renderer_wgpu_shell_view_chrome::ChromeViewCtx::of(&mut interaction.shell).advance_chrome_maintenance_step();
                    if cursor.shell_maintenance {
                        cursor.phase = 0;
                    }
                    if deadline_ms.is_some_and(|deadline| worker_pool.now_ms() > deadline) {
                        interaction.frame_fault = Some("frame maintenance bounded field exceeded its deadline".to_string());
                        cursor.begin_close();
                    }
                }
                *restart = false;
                if !mailbox.0.frame_maintenance.release(generation) && !stale {
                    interaction.frame_fault = Some("frame maintenance authority release faulted".to_string());
                    cursor.begin_close();
                }
            }
            if !execution.finish() {
                mailbox.record_frame_fault("frame maintenance execution handback faulted");
            }
        });
        match renderer_worker_pool().try_submit(semio_framework_async::Lane::Io, job) {
            Ok(()) => Ok(()),
            Err(error) => {
                let reclaimed = registry.reclaim_rejected(generation, cell);
                drop(error.into_job());
                let _ = self.0.frame_maintenance.release(generation);
                Err(FrameMaintenanceRefusal { owner: reclaimed, reservation_live: true })
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn spawn_frame_maintenance_reserved<F>(&self, future: F)
    where
        F: Future<Output = (AppInteractionState, FrameDeferredCursor)> + 'static,
    {
        self.spawn_frame_deferred_reserved(future);
    }

    #[cfg(target_arch = "wasm32")]
    pub fn spawn_frame_deferred_reserved<F>(&self, future: F)
    where
        F: Future<Output = (AppInteractionState, FrameDeferredCursor)> + 'static,
    {
        let mailbox = self.clone();
        let revision = mailbox.0.next_revision.fetch_add(1, Ordering::Relaxed);
        spawn_app_task(async move {
            let (interaction, cursor) = future.await;
            mailbox.0.finish(returned_completion(revision, RuntimeApply::ResumeFrameDeferred { interaction: Some(interaction), cursor: Some(cursor) }));
        });
    }

    #[cfg(target_arch = "wasm32")]
    pub fn spawn_dispatch_reserved(&self,cursor:RuntimeDispatchCursor,interaction:AppInteractionState){
        let mailbox=self.clone();let revision=mailbox.0.next_revision.fetch_add(1,Ordering::Relaxed);let owner=RuntimeDispatchReturn{mailbox,revision,cursor:Some(cursor),interaction:Some(interaction)};spawn_app_task(owner.run());
    }

}
//#endregion 📮️RuntimeMailbox
