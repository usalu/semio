//! 🏗️ Mailbox construction and authorities: the mailbox constructor, the interaction submission, the hub document status, the presentation and wake authorities, the engine surface close and the input close step. See [`super`].

use super::*;

//#region 📮️RuntimeMailbox
impl<R: MailboxRuntime> RuntimeMailbox<R> {
    /// 🏘️ Event closure observes the actual mailbox, checked-out workers and held runtime bodies.
    pub fn original_event_terminal_is_empty(&self)->bool{
        if !self.0.completions.try_lock().is_ok_and(|queue|queue.len()==0){return false}
        #[cfg(not(target_arch="wasm32"))]
        if self.0.frame_maintenance.state.load(Ordering::Acquire)!=0||self.0.frame_maintenance_executions.state.load(Ordering::Acquire)!=0||self.0.frame_deferred_executions.state.load(Ordering::Acquire)!=0{return false}
        self.try_lock().is_ok_and(|runtime|{
            #[cfg(not(target_arch="wasm32"))]
            if runtime.pending_frame_maintenance_refusal().is_some(){return false}
            runtime.interaction_available()&&runtime.pending_frame_deferred().is_none()
        })
    }
    pub fn new(runtime: R) -> Self {
        let presentation_authority = RuntimePresentationAuthority::new();
        let raster_operation_authority = RuntimeRasterOperationAuthority::new();
        #[cfg(not(target_arch = "wasm32"))]
        let (native_asset_http, native_asset_https, native_asset_http_runtime, native_asset_http_scope, native_asset_http_cancel) = {
            let pool = renderer_worker_pool();
            let runtime = Arc::new(semio_framework_os_services::TokioHostRuntime::with_pool(pool.clone()));
            let scope = runtime.open_scope_now(semio_framework_async::ScopeOwner::Service("renderer_asset_http"), None);
            let compute = Arc::new(semio_framework_os_services::ComputePool::with_pool(2, pool));
            let http_transport = Arc::new(semio_framework_os_services::SocketHttpTransport::new(compute.clone(), runtime.clone(), scope.clone()));
            let https_transport = Arc::new(semio_framework_os_kernel::os_directory::client::native::UreqStreamingHttpTransport::new(compute, runtime.clone(), scope.clone()));
            let http = Arc::new(semio_framework_os_services::HttpPool::new_with_async_transport_now(http_transport, 256 * 1024 * 1024, 1));
            let https = Arc::new(semio_framework_os_services::HttpPool::new_with_async_transport_now(https_transport, 256 * 1024 * 1024, 1));
            (http, https, runtime, scope, semio_framework_async::CancelToken::root_now())
        };
        Self(Arc::new(RuntimeMailboxInner {
            runtime: Mutex::new(runtime),
            presentation_authority,
            raster_operation_authority,
            world_cursor_wake: infinite_world::world::WorldCursorWakeAuthority::new(),
            completions: Mutex::new(RuntimeCompletionQueue::new()),
            waker: Mutex::new(None),
            next_revision: AtomicU64::new(1),
            applied_revisions: Mutex::new(std::collections::HashMap::new()),
            frame_inputs: Mutex::new(FrameBuildInputs::default()),
            retained_control_deadline_us: AtomicU64::new(u64::MAX),
            shell_clock_deadline_us: AtomicU64::new(u64::MAX),
            frame_fault: Mutex::new(None),
            #[cfg(not(target_arch = "wasm32"))]
            frame_maintenance: FrameMaintenanceAuthority::new(),
            #[cfg(not(target_arch = "wasm32"))]
            frame_maintenance_executions: Arc::new(FrameExecutionRegistry::new()),
            #[cfg(not(target_arch = "wasm32"))]
            frame_deferred_executions: Arc::new(FrameExecutionRegistry::new()),
            world3d_close_cursor: Mutex::new(0),
            world3d_asset_cursor: Mutex::new(0),
            world3d_asset_decode_cursor: Mutex::new(0),
            asset_probe: Mutex::new(None),
            asset_decode_session: Mutex::new(None),
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            staged_reference_images: Mutex::new(StagedReferenceImageAuthority::new()),
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            scene_raster_pool: infinite_world::world::world_scene_raster_pool(),
            #[cfg(not(target_arch = "wasm32"))]
            native_reference_decode: Mutex::new(None),
            #[cfg(not(target_arch = "wasm32"))]
            native_asset_transport: Mutex::new(None),
            #[cfg(all(any(test, feature = "testkit"), not(target_arch = "wasm32")))]
            native_asset_page_barrier: Mutex::new(None),
            native_asset_fetching: AtomicBool::new(false),
            native_asset_handoff: Mutex::new(None),
            #[cfg(not(target_arch = "wasm32"))]
            native_asset_http,
            #[cfg(not(target_arch = "wasm32"))]
            native_asset_https,
            #[cfg(not(target_arch = "wasm32"))]
            native_asset_http_runtime,
            #[cfg(not(target_arch = "wasm32"))]
            native_asset_http_scope,
            #[cfg(not(target_arch = "wasm32"))]
            native_asset_http_cancel,
        }))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn submit_interaction<F, Fut>(runtime: &mut R, handle: &RuntimeHandle<R>, key: Option<&'static str>, work: F) -> bool
    where
        F: FnOnce(AppInteractionState) -> Fut,
        Fut: Future<Output = AppInteractionState> + Send + 'static,
    {
        let Some(interaction) = runtime.check_out_interaction("submit-interaction") else { return false };
        let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else {
            runtime.return_interaction(interaction);
            return false;
        };
        if !mailbox.reserve_interaction_future() {
            runtime.return_interaction(interaction);
            return false;
        }
        mailbox.spawn_interaction_reserved(key, work(interaction));
        true
    }

    pub fn downgrade(&self) -> RuntimeHandle<R> {
        Arc::downgrade(&self.0)
    }

    pub fn try_lock(&self) -> std::sync::TryLockResult<std::sync::MutexGuard<'_, R>> {
        self.0.try_lock()
    }

    pub fn publish_hub_document_status(&self, document_key: String, remote: semio_framework_os_renderer_wgpu_contracts::ShellHubRemoteV1) -> bool {
        let Ok(mut runtime) = self.try_lock() else { return false };
        let Some(interaction) = runtime.interaction_mut() else { return false };
        interaction.shell.sync.publish_hub_document_status(document_key, remote);
        self.0.presentation_authority.mark_scene_changed();
        true
    }

    pub fn retire_hub_document_status(&self, document_key: &str) -> bool {
        let Ok(mut runtime) = self.try_lock() else { return false };
        let Some(interaction) = runtime.interaction_mut() else { return false };
        interaction.shell.sync.retire_hub_document_status(document_key);
        self.0.presentation_authority.mark_scene_changed();
        true
    }

    pub fn world_cursor_wake_authority(&self) -> infinite_world::world::WorldCursorWakeAuthority {
        self.0.world_cursor_wake.clone()
    }

    pub fn presentation_authority(&self) -> RuntimePresentationAuthority {
        self.0.presentation_authority.clone()
    }

    pub fn raster_operation_authority(&self) -> RuntimeRasterOperationAuthority {
        self.0.raster_operation_authority.clone()
    }

    pub fn presentation_witness_for(&self, input_generation: u64) -> Option<RuntimePresentationWitness> {
        self.0.presentation_authority.witness_for(input_generation)
    }

    pub fn observe_presentation_input_generation(&self, generation: u64) {
        self.0.presentation_authority.observe_input_generation(generation);
    }

    pub fn admit_build_presentation_witness(&self, witness: RuntimePresentationWitness) {
        self.0.presentation_authority.admit_build(witness);
    }

    pub fn acknowledge_world_cursor_wake(&self, token: &infinite_world::world::WorldCursorWakeToken) -> bool {
        self.0.world_cursor_wake.acknowledge(token)
    }

    pub fn close_world_cursor_wake_step(&self) -> bool {
        self.0.world_cursor_wake.close_step() && self.0.world_cursor_wake.terminal_is_empty()
    }

    pub fn close_component_world_step(&self, target: &semio_framework_os_renderer_wgpu_contracts::ScenePointerTarget, grant: semio_framework_value::retained_clone::RetainedCloneGrant, context: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_value::retained_clone::RetainedCloneStep,infinite_world::world::WorldDynamicFault> {
        let Ok(mut runtime)=self.try_lock() else {return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(semio_framework_value::retained_clone::RetainedCloneProgress::default()));};
        let Some(interaction)=runtime.interaction_mut() else {return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(semio_framework_value::retained_clone::RetainedCloneProgress::default()));};
        interaction.shell.surfaces.close_component_world_step(target,grant,context)
    }

    pub fn begin_engine_surface_close(&self, token: semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::EngineSurfaceToken) -> Result<bool, ()> {
        semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::begin_engine_surface_close_token(token)
    }

    pub fn close_engine_surface_step(&self, token: semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::EngineSurfaceToken, operation: semio_framework_trace::OperationId, sequence: &mut u64, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> bool {
        let Ok(mut runtime) = self.try_lock() else {
            return false;
        };
        let Some(interaction) = runtime.interaction_mut() else {
            return false;
        };
        let now = semio_framework_job::default_now_us();
        let deadline = match now.and_then(|now| now.checked_add(1_000)) {
            Some(deadline) => deadline,
            None => return false,
        };
        let mut retained_progress = semio_framework_value::retained_clone::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            operation,
            semio_framework_trace::Generation(token.generation),
            semio_framework_job::StepBudget::new(1, deadline, grant),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            sequence,
            &mut retained_progress,
        );
        semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::close_engine_surface_step(token, &mut context, &mut interaction.input)
    }

    pub fn engine_surface_terminal_is_empty(&self, token: semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::EngineSurfaceToken) -> Result<bool, ()> {
        semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::engine_surface_terminal_nonopaque_is_empty(token)
    }

    pub fn close_input_step(&self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        let Ok(mut runtime) = self.try_lock() else {
            return Ok(false);
        };
        let returned = {
            let Ok(mut queue) = self.0.completions.try_lock() else { return Ok(false) };
            let index = queue.ready.iter().position(|completion| match completion.apply {
                RuntimeApply::ResumeFrameDeferred { .. } | RuntimeApply::ResumeDispatch { .. } | RuntimeApply::DispatchEvents(_) => true,
                #[cfg(not(target_arch = "wasm32"))]
                RuntimeApply::RestoreInteraction(_) => true,
                _ => false,
            });
            index.and_then(|index| queue.take_at(index).map(|completion| (index, completion)))
        };
        if let Some((index, mut completion)) = returned {
            let original=completion.apply.close_returned_interaction_step(&mut *runtime,cx);completion.restores_interaction=completion.apply.restores_interaction();if !matches!(original,Ok(true)){self.0.completions.lock().expect("runtime completion mailbox lock").restore_at(index,completion)}return original.map(|_|false);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if !runtime.interaction_available() && runtime.pending_frame_deferred().is_none() {
            if let Some(mut owner) = self.try_take_abandoned_frame_deferred() {
                owner.cancel_current();
                let Some((interaction, cursor)) = owner.take_pair() else { return Ok(false) };
                runtime.return_interaction(interaction);
                *runtime.pending_frame_deferred_mut() = Some(cursor);
                return Ok(false);
            }
        }
        if let Some(cursor) = runtime.pending_frame_deferred_mut().as_mut() {
            cursor.begin_close();
            if cursor.close_step() {
                *runtime.pending_frame_deferred_mut() = None;
            }
            return Ok(false);
        }
        if !runtime.frame_actions_mut().close_step() {
            return Ok(false);
        }
        let Some(interaction) = runtime.interaction_mut() else { return Ok(true) };
        if !semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::close_clipboard_io_step(&mut interaction.input,cx)?{return Ok(false)}
        Ok(interaction
            .input
            .close_step_with_receipt(|receipt| {
                settle_renderer_action_receipt(receipt, semio_framework_os_renderer_wgpu_engine_text::text_editor::TextEditorActionOutcome::Cancelled);
            })
            .is_ok_and(|complete| complete)
            && interaction.input.terminal_is_empty())
    }

}
//#endregion 📮️RuntimeMailbox
