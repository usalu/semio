//! 🔁️ Runtime apply: the completion variants a worker hands back to the runtime, with the maintenance and deferred execution owners they carry; the runtime-mailbox crate of the renderer partition (plan section 5.3, inversion I2).
use super::*;

//#region 📮️RuntimeMailbox
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct FrameMaintenanceOwner {
    pub(crate) interaction: Option<AppInteractionState>,
    pub(crate) cursor: Option<FrameDeferredCursor>,
    pub(crate) deadline_ms: Option<u64>,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameMaintenanceOwner {
    pub(crate) fn new(interaction: AppInteractionState, cursor: FrameDeferredCursor) -> Self {
        Self { interaction: Some(interaction), cursor: Some(cursor), deadline_ms: renderer_worker_pool().now_ms().checked_add(8) }
    }

    pub(crate) fn generation(&self) -> Option<u64> {
        self.cursor.as_ref().map(|cursor| cursor.generation)
    }

    pub(crate) fn take_pair(&mut self) -> Option<(AppInteractionState, FrameDeferredCursor)> {
        if self.interaction.is_none() || self.cursor.is_none() {
            return None;
        }
        Some((self.interaction.take()?, self.cursor.take()?))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct FrameDeferredExecutionOwner {
    pub(crate) interaction: Option<AppInteractionState>,
    pub(crate) cursor: Option<FrameDeferredCursor>,
    pub(crate) work: Option<FrameDeferredWork>,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameDeferredExecutionOwner {
    pub(crate) fn new(interaction: AppInteractionState, cursor: FrameDeferredCursor, work: FrameDeferredWork) -> Self {
        Self { interaction: Some(interaction), cursor: Some(cursor), work: Some(work) }
    }

    pub(crate) fn generation(&self) -> Option<u64> {
        self.cursor.as_ref().map(|cursor| cursor.generation)
    }

    pub(crate) fn cancel_current(&mut self) {
        if let Some(FrameDeferredWork::Action(action)) = self.work.take() {
            action.cancel();
        } else {
            self.work = None;
        }
    }

    pub(crate) fn take_pair(&mut self) -> Option<(AppInteractionState, FrameDeferredCursor)> {
        if self.work.is_some() || self.interaction.is_none() || self.cursor.is_none() {
            return None;
        }
        Some((self.interaction.take()?, self.cursor.take()?))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct FrameMaintenanceRefusal {
    pub(crate) owner: Arc<FrameExecutionOwnerCell<FrameMaintenanceOwner>>,
    pub(crate) reservation_live: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameMaintenanceRefusal {
    pub(crate) fn try_take(&self) -> Option<FrameMaintenanceOwner> {
        self.owner.try_take()
    }
}

pub(crate) enum RuntimeApply {
    Resize {
        width: f32,
        height: f32,
        dpr: f32,
    },
    DispatchEvents(Option<RuntimeDispatchCursor>),
    ResumeDispatch {
        interaction: Option<AppInteractionState>,
        cursor: Option<RuntimeDispatchCursor>,
    },
    ResumeFrameDeferred {
        interaction: Option<AppInteractionState>,
        cursor: Option<FrameDeferredCursor>,
    },
    #[cfg(not(target_arch = "wasm32"))]
    RestoreInteraction(Option<AppInteractionState>),
    #[cfg(not(target_arch = "wasm32"))]
    PluginReload(Option<Result<Vec<program_bridge::ProgramBridgeEntry>, String>>),
}

impl RuntimeApply {
    pub(crate) fn close_returned_interaction_step<R: MailboxRuntime>(&mut self,runtime: &mut R,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        match self{
            Self::ResumeFrameDeferred{interaction,cursor}=>{if let Some(returned)=interaction.take(){runtime.return_interaction(returned);return Ok(false)}if let Some(owner)=cursor.as_mut(){owner.begin_close();if !owner.close_step(){return Ok(false)}}cursor.take();Ok(true)},
            Self::DispatchEvents(cursor)=>{if let Some(owner)=cursor.as_mut(){if !owner.close_step(cx)?{return Ok(false)}}cursor.take();Ok(true)},
            Self::ResumeDispatch{interaction,cursor}=>{if let Some(returned)=interaction.as_mut(){if let Some(original)=cursor.as_ref(){original.validate_original(cx)?}if !interpreter::close_clipboard_io_step(&mut returned.input,cx)?{return Ok(false)}}if let Some(returned)=interaction.take(){runtime.return_interaction(returned);return Ok(false)}if let Some(owner)=cursor.as_mut(){if !owner.close_step(cx)?{return Ok(false)}}cursor.take();Ok(true)},
            #[cfg(not(target_arch="wasm32"))]Self::RestoreInteraction(interaction)=>{if let Some(returned)=interaction.take(){runtime.return_interaction(returned)}Ok(true)},
            _=>Ok(false),
        }
    }

    /// 🎟️ Whether applying this completion HANDS THE INTERACTION STATE BACK. The checkout ledger ages
    /// a live checkout against exactly this: while one of these is queued (or a reservation for one is
    /// in flight) the state has an owner, and however long that owner takes it is not a leak. An arm
    /// that carries no state carries no ownership either — the native abandoned-maintenance resume is
    /// `ResumeFrameDeferred { interaction: None, .. }` for that reason.
    pub(crate) fn restores_interaction(&self) -> bool {
        match self {
            Self::ResumeDispatch { interaction, .. } | Self::ResumeFrameDeferred { interaction, .. } => interaction.is_some(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::RestoreInteraction(interaction) => interaction.is_some(),
            _ => false,
        }
    }

    fn restore_interaction_only<R: MailboxRuntime>(&mut self, runtime: &mut R) -> bool {
        match self {
            Self::ResumeDispatch { interaction, cursor } => {
                let Some(returned) = interaction.take() else { return cursor.is_none() };
                runtime.return_interaction(returned);
                cursor.is_none()
            }
            Self::ResumeFrameDeferred { interaction, cursor } => {
                let Some(returned) = interaction.take() else { return cursor.is_none() };
                runtime.return_interaction(returned);
                cursor.is_none()
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::RestoreInteraction(interaction) => {
                let Some(returned) = interaction.take() else { return true };
                runtime.return_interaction(returned);
                true
            }
            _ => false,
        }
    }

    fn start_dispatch<R: MailboxRuntime>(cursor: &mut Option<RuntimeDispatchCursor>, runtime: &mut R, handle: &RuntimeHandle<R>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        log_debug("[TRACE] start_dispatch enter");
        let Some(cursor_value) = cursor.as_mut() else { return Ok(true) };
        if !cursor_value.close_inflight_step(cx)?{return Ok(false)}
        if cursor_value.events_terminal_is_empty()||cx.is_cancelled(){if !cursor_value.close_step(cx)?{return Ok(false)}cursor.take();return Ok(true)}
        cursor_value.validate_original(cx)?;if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}
        let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else {
            log_debug("[TRACE] start_dispatch refused: mailbox handle expired");
            return Ok(false);
        };
        let Some(mut interaction) = runtime.check_out_interaction("dispatch-event") else {
            log_debug_once_per_transition("start-dispatch-checkout", true, "[TRACE] start_dispatch refused: interaction state is checked out");
            return Ok(false);
        };
        if !interaction.input.original_clipboard_has_capacity() {
            runtime.return_interaction(interaction);
            return Ok(false);
        }
        match interaction.input.bind_original_clipboard_dispatch(cx,cursor_value.original.as_ref().expect("validated dispatch source").epoch) {
            Ok(true) => {}
            Ok(false) => { runtime.return_interaction(interaction); return Ok(false); }
            Err(error) => { runtime.return_interaction(interaction); return Err(error); }
        }
        log_debug_once_per_transition("start-dispatch-checkout", false, "[TRACE] start_dispatch: interaction state checked out for one event");
        if !mailbox.reserve_interaction_future() {
            log_debug("[TRACE] start_dispatch refused: interaction future credits exhausted");
            interaction.frame_fault = Some("runtime dispatch completion credits exhausted".to_string());
            runtime.return_interaction(interaction);
            return Ok(false);
        }
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
        let mut cursor_value=cursor.take().expect("dispatch cursor admitted above");cursor_value.inflight=Some(cursor_value.take_next().expect("non-empty dispatch cursor"));
        mailbox.spawn_dispatch_reserved(cursor_value,interaction);
        Ok(true)
    }

    pub(crate) fn start_frame_deferred<R: MailboxRuntime>(cursor: &mut Option<FrameDeferredCursor>, runtime: &mut R, handle: &RuntimeHandle<R>) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        if cursor.is_none() && !runtime.interaction_available() {
            if let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) {
                if let Some(mut owner) = mailbox.try_take_abandoned_frame_deferred() {
                    owner.cancel_current();
                    let Some((interaction, returned_cursor)) = owner.take_pair() else { return false };
                    runtime.return_interaction(interaction);
                    *cursor = Some(returned_cursor);
                    return false;
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if runtime.pending_frame_maintenance_refusal().is_none() {
            if let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) {
                *runtime.pending_frame_maintenance_refusal_mut() = mailbox.try_take_abandoned_frame_maintenance();
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(mut refusal) = runtime.pending_frame_maintenance_refusal_mut().take() {
            let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else {
                *runtime.pending_frame_maintenance_refusal_mut() = Some(refusal);
                return false;
            };
            if refusal.reservation_live {
                if !mailbox.cancel_interaction_future_reservation() {
                    *runtime.pending_frame_maintenance_refusal_mut() = Some(refusal);
                    return false;
                }
                refusal.reservation_live = false;
            }
            let Some(mut owner) = refusal.try_take() else {
                *runtime.pending_frame_maintenance_refusal_mut() = Some(refusal);
                return false;
            };
            let Some((interaction, mut returned_cursor)) = owner.take_pair() else {
                if let Err(owner) = refusal.owner.try_restore(owner) {
                    refusal.owner = Arc::new(FrameExecutionOwnerCell::new(owner));
                }
                *runtime.pending_frame_maintenance_refusal_mut() = Some(refusal);
                return false;
            };
            returned_cursor.return_shell_maintenance();
            runtime.return_interaction(interaction);
            *cursor = Some(returned_cursor);
            return false;
        }
        if cursor.as_ref().is_some_and(|cursor| cursor.closing) {
            let complete = cursor.as_mut().is_some_and(FrameDeferredCursor::close_step);
            if complete {
                cursor.take();
            }
            return complete;
        }
        let Some(cursor_value) = cursor.as_mut() else { return true };
        if cursor_value.terminal_is_empty() {
            cursor.take();
            return true;
        }
        let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else { return false };
        let checkout_site = cursor_value.checkout_site();
        let Some(mut interaction) = runtime.check_out_interaction(checkout_site) else { return false };
        if !mailbox.reserve_interaction_future() {
            interaction.frame_fault = Some("frame deferred completion credits exhausted".to_string());
            runtime.return_interaction(interaction);
            return false;
        }
        let Some(mut cursor_value) = cursor.take() else {
            runtime.return_interaction(interaction);
            return false;
        };
        let Some(work) = cursor_value.take_next() else {
            runtime.return_interaction(interaction);
            if !cursor_value.terminal_is_empty() {
                cursor_value.closing = true;
                *cursor = Some(cursor_value);
                return false;
            }
            return true;
        };
        if matches!(work, FrameDeferredWork::ShellMaintenance) {
            #[cfg(not(target_arch = "wasm32"))]
            {
                match mailbox.try_spawn_frame_maintenance_reserved(FrameMaintenanceOwner::new(interaction, cursor_value)) {
                    Ok(()) => return true,
                    Err(refusal) => {
                        *runtime.pending_frame_maintenance_refusal_mut() = Some(refusal);
                        return false;
                    }
                }
            }
            #[cfg(target_arch = "wasm32")]
            {
                mailbox.spawn_frame_maintenance_reserved(async move {
                    cursor_value.shell_maintenance = interaction.shell.advance_chrome_maintenance_step();
                    if cursor_value.shell_maintenance {
                        cursor_value.phase = 0;
                    }
                    (interaction, cursor_value)
                });
                return true;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            match mailbox.try_spawn_frame_deferred_reserved(FrameDeferredExecutionOwner::new(interaction, cursor_value, work)) {
                Ok(()) => return true,
                Err(mut owner) => {
                    let _ = mailbox.0.completions.lock().expect("runtime completion mailbox lock").cancel_interaction_reservation();
                    owner.cancel_current();
                    let Some((interaction, cursor_value)) = owner.take_pair() else { return false };
                    runtime.return_interaction(interaction);
                    *cursor = Some(cursor_value);
                    return false;
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        mailbox.spawn_frame_deferred_reserved(async move {
            match work {
                FrameDeferredWork::ShellMaintenance => {}
                FrameDeferredWork::PumpSync => {
                    interaction.shell.pump_sync_events().await;
                }
                FrameDeferredWork::Action(action) => {
                    if action.cancelled {
                        action.cancel();
                    } else {
                        let receipt = action.receipt;
                        let mut receipt_owner = FrameActionReceiptOwner { receipt };
                        match interaction.shell.dispatch_gesture_action(action.descriptor).await {
                            Ok(()) => {
                                receipt_owner.settle(engine_canvas::TextEditorActionOutcome::Accepted);
                            }
                            Err(error) => {
                                receipt_owner.settle(engine_canvas::TextEditorActionOutcome::Refused(&error));
                                if let Some(receipt) = receipt {
                                    if receipt.abort_correlation_on_error {
                                        cursor_value.actions.cancel_correlation(receipt.token);
                                    }
                                }
                                log_debug(&format!("[TRACE] frame deferred action failed: {error}"));
                                interaction.shell.note_dispatch_fault(&error);
                            }
                        }
                    }
                }
                FrameDeferredWork::FlushTutorial => interaction.shell.tutorial_flush_pending_document_ops().await,
                FrameDeferredWork::Settle => {
                    interaction.shell.settle_pump_step().await;
                }
            }
            (interaction, cursor_value)
        });
        #[cfg(target_arch = "wasm32")]
        return true;
        #[allow(unreachable_code)]
        false
    }

    fn apply_step<R: MailboxRuntime>(&mut self, runtime: &mut R, handle: &RuntimeHandle<R>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        if let Self::DispatchEvents(cursor) = self {
            log_debug(&format!("[TRACE] apply_step DispatchEvents present={} terminal-empty={:?}", cursor.is_some(), cursor.as_ref().map(RuntimeDispatchCursor::terminal_is_empty)));
        }
        match self{Self::DispatchEvents(cursor)=>return Self::start_dispatch(cursor,runtime,handle,cx),Self::ResumeDispatch{interaction,cursor}=>{if let Some(returned)=interaction.as_mut(){if let Some(original)=cursor.as_ref(){original.validate_original(cx)?;}if interpreter::drive_clipboard_io_step(&mut returned.input,cx)?{return Ok(false)}}if let Some(returned)=interaction.take(){runtime.return_interaction(returned)}return Self::start_dispatch(cursor,runtime,handle,cx)},_=>{}}
        Ok((||{match self {
            Self::DispatchEvents(_)|Self::ResumeDispatch{..}=>unreachable!(),
            Self::Resize { width, height, dpr } => runtime.resize(*width, *height, *dpr),
            Self::ResumeFrameDeferred { interaction, cursor } => {
                if let Some(returned) = interaction.take() {
                    runtime.return_interaction(returned);
                }
                return Self::start_frame_deferred(cursor, runtime, handle);
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::RestoreInteraction(interaction) => {
                if let Some(returned) = interaction.take() {
                    runtime.return_interaction(returned);
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            Self::PluginReload(result) => match result.take() {
                Some(Ok(entries)) => {
                    let handle = handle.clone();
                    RuntimeMailbox::submit_interaction(runtime, &handle, Some("plugin-boot"), move |mut interaction| async move {
                        interaction.shell.prepare_hot_reload(entries);
                        if let Err(error) = interaction.shell.boot().await {
                            log_debug(&format!("wasm program hot reload failed: {error}"));
                        } else {
                            log_debug("wasm program hot reload complete");
                        }
                        interaction
                    });
                }
                Some(Err(error)) => log_debug(&format!("wasm program reload failed: {error}")),
                None => return true,
            },
        }
        true})())
    }
}
//#endregion 📮️RuntimeMailbox
