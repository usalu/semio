//! 📮️ Mailbox impl: the single runtime mailbox implementation.
use super::*;

//#region 📮️RuntimeMailbox
impl RuntimeMailbox {
    /// 🏘️ Event closure observes the actual mailbox, checked-out workers and held runtime bodies.
    pub(crate) fn original_event_terminal_is_empty(&self)->bool{
        if !self.0.completions.try_lock().is_ok_and(|queue|queue.len()==0){return false}
        #[cfg(not(target_arch="wasm32"))]
        if self.0.frame_maintenance.state.load(Ordering::Acquire)!=0||self.0.frame_maintenance_executions.state.load(Ordering::Acquire)!=0||self.0.frame_deferred_executions.state.load(Ordering::Acquire)!=0{return false}
        self.try_lock().is_ok_and(|runtime|{
            #[cfg(not(target_arch="wasm32"))]
            if runtime.pending_frame_maintenance_refusal.is_some(){return false}
            runtime.interaction_available()&&runtime.pending_frame_deferred.is_none()
        })
    }
    pub(crate) fn new(runtime: AppRuntime) -> Self {
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
            frame_inputs: Mutex::new(frame_job::FrameBuildInputs::default()),
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
            #[cfg(all(test, not(target_arch = "wasm32")))]
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

    pub(crate) fn downgrade(&self) -> AppHandle {
        Arc::downgrade(&self.0)
    }

    pub(crate) fn try_lock(&self) -> std::sync::TryLockResult<std::sync::MutexGuard<'_, AppRuntime>> {
        self.0.try_lock()
    }

    pub(crate) fn publish_hub_document_status(&self, document_key: String, remote: crate::shell::ShellHubRemoteV1) -> bool {
        let Ok(mut runtime) = self.try_lock() else { return false };
        let Some(interaction) = runtime.interaction.as_mut() else { return false };
        interaction.shell.publish_hub_document_status(document_key, remote);
        self.0.presentation_authority.mark_scene_changed();
        true
    }

    pub(crate) fn retire_hub_document_status(&self, document_key: &str) -> bool {
        let Ok(mut runtime) = self.try_lock() else { return false };
        let Some(interaction) = runtime.interaction.as_mut() else { return false };
        interaction.shell.retire_hub_document_status(document_key);
        self.0.presentation_authority.mark_scene_changed();
        true
    }

    pub(crate) fn world_cursor_wake_authority(&self) -> infinite_world::world::WorldCursorWakeAuthority {
        self.0.world_cursor_wake.clone()
    }

    pub(crate) fn presentation_authority(&self) -> RuntimePresentationAuthority {
        self.0.presentation_authority.clone()
    }

    pub(crate) fn raster_operation_authority(&self) -> RuntimeRasterOperationAuthority {
        self.0.raster_operation_authority.clone()
    }

    pub(crate) fn presentation_witness_for(&self, input_generation: u64) -> Option<RuntimePresentationWitness> {
        self.0.presentation_authority.witness_for(input_generation)
    }

    pub(crate) fn observe_presentation_input_generation(&self, generation: u64) {
        self.0.presentation_authority.observe_input_generation(generation);
    }

    pub(crate) fn admit_build_presentation_witness(&self, witness: RuntimePresentationWitness) {
        self.0.presentation_authority.admit_build(witness);
    }

    pub(crate) fn acknowledge_world_cursor_wake(&self, token: &infinite_world::world::WorldCursorWakeToken) -> bool {
        self.0.world_cursor_wake.acknowledge(token)
    }

    fn close_world_cursor_wake_step(&self) -> bool {
        self.0.world_cursor_wake.close_step() && self.0.world_cursor_wake.terminal_is_empty()
    }

    pub(crate) fn close_world3d_dynamic_step(&self,presenter:&mut AppPresenter,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure> {
        if cx.retirement_should_yield(){return Ok(false)}
        if !self.close_renderer_asset_step() {
            return Ok(false);
        }
        let Ok(mut runtime) = self.try_lock() else {
            return Ok(false);
        };
        let Some(interaction) = runtime.interaction.as_mut() else {
            drop(runtime);
            return Ok(self.close_world_cursor_wake_step());
        };
        if !interaction.shell.close_icon_export_step(cx)? {
            return Ok(false);
        }
        if interaction.shell.advance_world3d_retirement_step() {
            if let Some(fault)=interaction.shell.world3d_retirement_fault() {self.record_frame_fault(&format!("retired terrain owner refused: {fault:?}"));}
            return Ok(false);
        }
        let Ok(mut cursor) = self.0.world3d_close_cursor.try_lock() else {
            return Ok(false);
        };
        let Some(state) = interaction.shell.world3d_states.get_mut_at(*cursor) else {
            drop(cursor);
            drop(runtime);
            return Ok(self.close_world_cursor_wake_step());
        };
        if world3d_dynamic_retirement_terminal_is_empty(state) {
            *cursor += 1;
            return Ok(false);
        }
        if state.dynamic_retirement_is_idle() {
            begin_world3d_dynamic_retirement(state);
            return Ok(false);
        }
        let terrain_grant=cx.retained_grant();
        let terrain=infinite_world::world::step_world3d_terrain_retirement(state,terrain_grant,cx);
        if !terrain.ownership.fits(terrain_grant) {return Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"world3d terrain retirement receipt exceeded caller grant").with_retained_progress(terrain.ownership),cx));}
        cx.consume_retained(terrain.ownership)?;
        match terrain.step {
            infinite_world::world::WorldTerrainMeshPublicationStep::Idle=>{},
            infinite_world::world::WorldTerrainMeshPublicationStep::Pending|infinite_world::world::WorldTerrainMeshPublicationStep::Complete=>return Ok(false),
            infinite_world::world::WorldTerrainMeshPublicationStep::Fault(fault)=>{self.record_frame_fault(&format!("world3d terrain retirement refused: {fault:?}"));return Ok(false);}
        }
        let gpu_return=infinite_world::world::world3d_return_native_mesh_gpu_step(state,cx.retained_grant(),|key,source|presenter.gpu.retire_original_mesh_source_step(key,source)).map_err(|_|semio_framework_job::OriginalTurnFailure::from(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original World GPU source return refused")))?;
        cx.consume_retained(gpu_return)?;if gpu_return.copied_items!=0{cx.consume_fuel(1);return Ok(false)}
        if step_world3d_dynamic_retirement(state, cx) && world3d_dynamic_retirement_terminal_is_empty(state) {
            *cursor += 1;
        }
        Ok(false)
    }

    pub(crate) fn close_component_world_step(&self, target: &crate::element_contracts::ScenePointerTarget, grant: semio_framework_value::retained_clone::RetainedCloneGrant, context: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_value::retained_clone::RetainedCloneStep,infinite_world::world::WorldDynamicFault> {
        let Ok(mut runtime)=self.try_lock() else {return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(semio_framework_value::retained_clone::RetainedCloneProgress::default()));};
        let Some(interaction)=runtime.interaction.as_mut() else {return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(semio_framework_value::retained_clone::RetainedCloneProgress::default()));};
        interaction.shell.close_component_world_step(target,grant,context)
    }

    pub(crate) fn begin_engine_surface_close(&self, token: engine_canvas::EngineSurfaceToken) -> Result<bool, ()> {
        engine_canvas::begin_engine_surface_close_token(token)
    }

    pub(crate) fn close_engine_surface_step(&self, token: engine_canvas::EngineSurfaceToken, operation: semio_framework_trace::OperationId, sequence: &mut u64) -> bool {
        let Ok(mut runtime) = self.try_lock() else {
            return false;
        };
        let Some(interaction) = runtime.interaction.as_mut() else {
            return false;
        };
        let now = semio_framework_job::default_now_us();
        let deadline = match now.and_then(|now| now.checked_add(1_000)) {
            Some(deadline) => deadline,
            None => return false,
        };
        let mut context = semio_framework_job::StepContext::new(
            operation,
            semio_framework_trace::Generation(token.generation),
            semio_framework_job::StepBudget::new(1, deadline),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            sequence,
        );
        engine_canvas::close_engine_surface_step(token, &mut context, &mut interaction.input)
    }

    pub(crate) fn engine_surface_terminal_is_empty(&self, token: engine_canvas::EngineSurfaceToken) -> Result<bool, ()> {
        engine_canvas::engine_surface_terminal_nonopaque_is_empty(token)
    }

    pub(crate) fn close_input_step(&self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
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
            let original=completion.apply.close_returned_interaction_step(&mut runtime,cx);completion.restores_interaction=completion.apply.restores_interaction();if !matches!(original,Ok(true)){self.0.completions.lock().expect("runtime completion mailbox lock").restore_at(index,completion)}return original.map(|_|false);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if !runtime.interaction_available() && runtime.pending_frame_deferred.is_none() {
            if let Some(mut owner) = self.try_take_abandoned_frame_deferred() {
                owner.cancel_current();
                let Some((interaction, cursor)) = owner.take_pair() else { return Ok(false) };
                runtime.return_interaction(interaction);
                runtime.pending_frame_deferred = Some(cursor);
                return Ok(false);
            }
        }
        if let Some(cursor) = runtime.pending_frame_deferred.as_mut() {
            cursor.begin_close();
            if cursor.close_step() {
                runtime.pending_frame_deferred = None;
            }
            return Ok(false);
        }
        if !runtime.frame_actions.close_step() {
            return Ok(false);
        }
        let Some(interaction) = runtime.interaction.as_mut() else { return Ok(true) };
        if !interpreter::close_clipboard_io_step(&mut interaction.input,cx)?{return Ok(false)}
        Ok(interaction
            .input
            .close_step_with_receipt(|receipt| {
                settle_renderer_action_receipt(receipt, engine_canvas::TextEditorActionOutcome::Cancelled);
            })
            .is_ok_and(|complete| complete)
            && interaction.input.terminal_is_empty())
    }

    pub(crate) fn take_renderer_asset_step(&self) -> Option<RendererAssetFetchOwner> {
        if let Some(owner) = take_next_renderer_asset() {
            return Some(RendererAssetFetchOwner::Shared(owner));
        }
        let Ok(mut runtime) = self.try_lock() else { return None };
        let interaction = runtime.interaction.as_mut()?;
        let Ok(mut cursor) = self.0.world3d_asset_cursor.try_lock() else { return None };
        let Some(surface_id) = interaction.shell.world3d_states.id_at(*cursor).map(str::to_owned) else {
            *cursor = 0;
            return None;
        };
        *cursor += 1;
        let surface_token = interaction.shell.world3d_states.token(&surface_id)?;
        let owner = take_next_world3d_asset(interaction.shell.world3d_states.get_token_mut(surface_token)?)?;
        let surface = WorldAssetMetadataId::try_from_str(&surface_id).ok()?;
        Some(RendererAssetFetchOwner::World { surface, surface_token, owner })
    }

    /// 📡️ Admits the response byte credits one fetch declared. Same three-way answer as
    /// [`Self::seal_renderer_asset_response`], and for the same reason: this runs the instant a
    /// `fetch()` resolves, which is just as likely to land inside a live apply as the seal is.
    pub(crate) fn reserve_renderer_asset_response(&self, fetch: &mut RendererAssetFetchOwner, byte_credits: usize) -> RendererAssetSealStep {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => {
                if reserve_renderer_asset_response(owner, byte_credits) {
                    RendererAssetSealStep::Granted
                } else {
                    RendererAssetSealStep::Refused("shared asset authority refused the response credits")
                }
            }
            RendererAssetFetchOwner::World { surface: _, surface_token, owner } => {
                let Ok(mut runtime) = self.try_lock() else { return RendererAssetSealStep::Busy };
                let Some(interaction) = runtime.interaction.as_mut() else { return RendererAssetSealStep::Busy };
                let Some(state) = interaction.shell.world3d_states.get_token_mut(*surface_token) else { return RendererAssetSealStep::Refused("the world surface generation retired") };
                match reserve_world3d_asset_response(state, owner, byte_credits) {
                    Ok(()) => RendererAssetSealStep::Granted,
                    Err(_) => RendererAssetSealStep::Refused("the world asset authority refused the response credits"),
                }
            }
        }
    }

    pub(crate) fn take_completed_renderer_asset_step(&self) -> Option<RendererAssetFetchOwner> {
        if let Some(owner) = take_next_completed_renderer_asset_step() {
            return Some(RendererAssetFetchOwner::Shared(owner));
        }
        let Ok(mut runtime) = self.try_lock() else { return None };
        let interaction = runtime.interaction.as_mut()?;
        let Ok(mut cursor) = self.0.world3d_asset_decode_cursor.try_lock() else { return None };
        let Some(surface_id) = interaction.shell.world3d_states.id_at(*cursor).map(str::to_owned) else {
            *cursor = 0;
            return None;
        };
        *cursor += 1;
        let surface_token = interaction.shell.world3d_states.token(&surface_id)?;
        let owner = take_next_completed_world3d_asset_step(interaction.shell.world3d_states.get_token_mut(surface_token)?)?;
        let surface = WorldAssetMetadataId::try_from_str(&surface_id).ok()?;
        Some(RendererAssetFetchOwner::World { surface, surface_token, owner })
    }

    pub(crate) fn finish_renderer_asset_owner(&self, fetch: RendererAssetFetchOwner, rejection: Option<(&'static str, WorldAssetRequestKind, String)>) -> Result<(), RendererAssetFetchOwner> {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => {
                if matches!(owner.kind(), WorldAssetRequestKind::Glb) {
                    if let Some((detail, _, _)) = rejection {
                        scenes::icon_export::asset::reject(owner.token(), detail);
                    }
                }
                finish_renderer_asset(owner).map_err(RendererAssetFetchOwner::Shared)
            }
            RendererAssetFetchOwner::World { surface, surface_token, owner } => {
                let Ok(mut runtime) = self.try_lock() else { return Err(RendererAssetFetchOwner::World { surface, surface_token, owner }) };
                let Some(interaction) = runtime.interaction.as_mut() else { return Err(RendererAssetFetchOwner::World { surface, surface_token, owner }) };
                let Some(state) = interaction.shell.world3d_states.get_token_mut(surface_token) else { return Err(RendererAssetFetchOwner::World { surface, surface_token, owner }) };
                if let Some((_, kind, url)) = rejection.filter(|_| !world3d_asset_cancellation_requested(state, owner.token())) {
                    match kind {
                        WorldAssetRequestKind::Terrain { z, x, y } => {
                            apply_world3d_terrain_tile_bytes(state, z, x, y, &[]);
                        }
                        _ => mark_world3d_asset_miss(state, &url),
                    }
                }
                finish_world3d_asset(state, owner).map_err(|owner| RendererAssetFetchOwner::World { surface, surface_token, owner })
            }
        }
    }

    /// 🗑️ Answers a decoder's refusal of ONE response. The shared kinds record their own miss right
    /// here — `apply_*` with empty bytes is exactly the miss those lanes already speak — while a
    /// World-owned kind carries its rejection to [`Self::finish_renderer_asset_owner`], which is the
    /// one place holding that surface's state. Printed unconditionally (not behind
    /// `runtime_diagnostics_enabled`): an asset that never paints is a defect report, not a trace.
    fn record_rejected_asset(rejection: &(&'static str, WorldAssetRequestKind, String)) {
        let (detail, kind, url) = rejection;
        log_debug(&format!("asset refused: {detail} — url={url}"));
        match *kind {
            WorldAssetRequestKind::UiImage { id } => interpreter::apply_ui_image_bytes(id.as_str(), url, &[]),
            kind @ WorldAssetRequestKind::MapTile { .. } => engine_canvas::apply_map_tile_bytes(kind, &[]),
            WorldAssetRequestKind::Glb | WorldAssetRequestKind::ReferenceImage | WorldAssetRequestKind::Terrain { .. } => {}
        }
    }

    /// 📥️ Drains the sealed response pages of a `Ready` probe whose kind applies WITHOUT a
    /// `World3dState` — `UiImage` and `MapTile`, the two kinds `RendererAssetFetchOwner::Shared`
    /// carries. `None` means "not one of those kinds"; a collect failure answers an empty payload so
    /// the caller still closes the probe (`apply_*` treats empty bytes as the miss they are) rather
    /// than leaving the lane parked on an undrainable response.
    fn take_shared_asset_bytes(probe: &mut RendererAssetProbe) -> Option<Vec<u8>> {
        if !matches!(probe.owner().kind(), WorldAssetRequestKind::UiImage { .. } | WorldAssetRequestKind::MapTile { .. }) {
            return None;
        }
        let owner = match probe.owner_mut() {
            RendererAssetFetchOwner::World { owner, .. } | RendererAssetFetchOwner::Shared(owner) => owner,
        };
        Some(collect_world3d_asset_bytes(owner).unwrap_or_default())
    }

    fn restore_asset_decode_boundary(&self, boundary: (RendererAssetProbe, Option<RendererAssetDecodeResult>)) -> Result<RendererAssetDecodeTurnKind, (RendererAssetProbe, Option<RendererAssetDecodeResult>)> {
        let Ok(mut slot) = self.0.asset_probe.try_lock() else { return Err(boundary) };
        if slot.is_some() {
            return Err(boundary);
        }
        let (mut probe, mut result) = boundary;
        if !matches!(result, Some(RendererAssetDecodeResult::Cancelled | RendererAssetDecodeResult::Fault(_))) {
            match self.renderer_asset_current(probe.owner()) {
                None => return Err((probe, result)),
                Some(false) => result = Some(RendererAssetDecodeResult::Cancelled),
                Some(true) => {}
            }
        }
        let kind = match result {
            Some(RendererAssetDecodeResult::Cancelled) => {
                probe.begin_close();
                RendererAssetDecodeTurnKind::Cancelled
            }
            Some(RendererAssetDecodeResult::Fault(_)) => {
                probe.begin_close();
                RendererAssetDecodeTurnKind::Fault
            }
            _ => RendererAssetDecodeTurnKind::Pending,
        };
        let rejection = matches!(result, Some(RendererAssetDecodeResult::Rejected(_))).then(|| probe.rejection.clone()).flatten();
        *slot = Some(probe);
        drop(slot);
        if let Some(rejection) = rejection {
            Self::record_rejected_asset(&rejection);
        }
        if let Some(RendererAssetDecodeResult::Fault(detail)) = result {
            self.record_frame_fault(detail);
        }
        Ok(kind)
    }

    fn pump_asset_decode_session_step(&self, closing: bool) -> Option<RendererAssetDecodeTurnKind> {
        let Ok(mut slot) = self.0.asset_decode_session.try_lock() else { return Some(RendererAssetDecodeTurnKind::Pending) };
        if let Some(active) = slot.as_mut() {
            let cancelled = if let Some(surface_token) = active.surface_token {
                self.try_lock()
                    .ok()
                    .and_then(|runtime| runtime.interaction.as_ref().map(|interaction| interaction.shell.world3d_states.get_token(surface_token).is_none_or(|state| world3d_asset_cancellation_requested(state, active.request_token))))
                    .unwrap_or(false)
            } else {
                renderer_asset_io().try_lock().is_ok_and(|authority| authority.cancellation_requested(active.request_token))
            };
            if closing || cancelled {
                active.cancel();
            }
            let wake = active.pump_one();
            let mut kind = RendererAssetDecodeTurnKind::Pending;
            if let Some(boundary) = active.take_boundary() {
                match self.restore_asset_decode_boundary(boundary) {
                    Ok(restored) => kind = restored,
                    Err(boundary) => active.boundary = Some(boundary),
                }
            }
            if wake {
                active.wake();
            }
            if active.terminal_is_empty() {
                *slot = None;
            }
            return Some(kind);
        }
        if closing {
            return None;
        }
        let Ok(mut probe_slot) = self.0.asset_probe.try_lock() else { return Some(RendererAssetDecodeTurnKind::Pending) };
        let probe = probe_slot.as_ref()?;
        if matches!(probe.phase, RendererAssetProbePhase::Ready | RendererAssetProbePhase::Closing) {
            return None;
        }
        #[cfg(not(target_arch = "wasm32"))]
        let wake = match self.0.waker.try_lock() {
            Ok(wake) => wake.clone(),
            Err(_) => return Some(RendererAssetDecodeTurnKind::Pending),
        };
        #[cfg(target_arch = "wasm32")]
        let wake = None;
        let Some(probe) = probe_slot.take() else { return Some(RendererAssetDecodeTurnKind::Pending) };
        let active = RendererAssetDecodeSession::new(probe, wake);
        active.wake();
        *slot = Some(active);
        Some(RendererAssetDecodeTurnKind::Pending)
    }

    /// 🧵️ Advances the private asset owner independently of frame construction and presentation.
    pub(crate) fn asset_decode_step(&self) -> RendererAssetDecodeTurn {
        if scenes::icon_export::asset::retire_cancelled_step() {
            return RendererAssetDecodeTurn { kind: RendererAssetDecodeTurnKind::Pending, pending: true, detail: None };
        }
        let session = self.pump_asset_decode_session_step(false);
        let kind = session.unwrap_or_else(|| if retire_cancelled_renderer_asset_step() { RendererAssetDecodeTurnKind::Pending } else { self.pump_renderer_asset_decode_step() });
        #[cfg(not(target_arch = "wasm32"))]
        if session.is_none() && !matches!(kind, RendererAssetDecodeTurnKind::Idle | RendererAssetDecodeTurnKind::Fault) {
            if let Ok(wake) = self.0.waker.try_lock() {
                if let Some(wake) = wake.as_ref() {
                    wake();
                }
            }
        }
        RendererAssetDecodeTurn {
            kind,
            pending: kind != RendererAssetDecodeTurnKind::Fault && (kind == RendererAssetDecodeTurnKind::Pending || self.has_pending_asset_decode()),
            detail: (kind == RendererAssetDecodeTurnKind::Fault).then_some("asset decoder failed; see the retained runtime fault"),
        }
    }

    fn pump_renderer_asset_decode_step(&self) -> RendererAssetDecodeTurnKind {
        let Ok(mut probe_slot) = self.0.asset_probe.try_lock() else { return RendererAssetDecodeTurnKind::Pending };
        if probe_slot.is_none() {
            let Some(owner) = self.take_completed_renderer_asset_step() else { return RendererAssetDecodeTurnKind::Idle };
            *probe_slot = Some(RendererAssetProbe::new(owner));
            return RendererAssetDecodeTurnKind::Pending;
        }
        let probe = probe_slot.as_mut().expect("asset probe initialized above");
        if matches!(probe.phase, RendererAssetProbePhase::Ready) {
            match self.renderer_asset_current(probe.owner()) {
                None => return RendererAssetDecodeTurnKind::Pending,
                Some(false) => {
                    probe.begin_close();
                    return RendererAssetDecodeTurnKind::Cancelled;
                }
                Some(true) => {}
            }
            log_debug_diagnostic(&format!("[TRACE] asset ready kind={:?} url={} bytes={}", probe.owner().kind(), probe.owner().url(), probe.owner().received_bytes()));
            if matches!(probe.owner(), RendererAssetFetchOwner::Shared(_)) && matches!(probe.owner().kind(), WorldAssetRequestKind::Glb) {
                let Some(asset) = probe.take_ready_mesh_asset() else { return RendererAssetDecodeTurnKind::Pending };
                match scenes::icon_export::asset::publish(probe.owner().owner().token(), asset) {
                    Ok(()) => probe.finish_ready_mesh(),
                    Err(asset) => {
                        probe.restore_ready_mesh_asset(asset);
                        probe.begin_close();
                    }
                }
                return RendererAssetDecodeTurnKind::Published;
            }
            if let Some(bytes) = Self::take_shared_asset_bytes(probe) {
                match probe.owner().kind() {
                    WorldAssetRequestKind::UiImage { id } => interpreter::apply_ui_image_bytes(id.as_str(), probe.owner().url(), &bytes),
                    kind @ WorldAssetRequestKind::MapTile { .. } => engine_canvas::apply_map_tile_bytes(kind, &bytes),
                    _ => {}
                }
                probe.begin_close();
                return RendererAssetDecodeTurnKind::Published;
            }
            let surface_token = match probe.owner() {
                RendererAssetFetchOwner::World { surface_token, .. } => *surface_token,
                RendererAssetFetchOwner::Shared(_) => return RendererAssetDecodeTurnKind::Pending,
            };
            let generation_is_live = {
                let Ok(runtime) = self.try_lock() else { return RendererAssetDecodeTurnKind::Pending };
                let Some(interaction) = runtime.interaction.as_ref() else { return RendererAssetDecodeTurnKind::Pending };
                interaction.shell.world3d_states.get_token(surface_token).is_some()
            };
            if !generation_is_live {
                probe.begin_close();
                return RendererAssetDecodeTurnKind::Pending;
            }
            if let WorldAssetRequestKind::Terrain { z, x, y } = probe.owner().kind() {
                let bytes = match collect_world3d_asset_bytes(match probe.owner_mut() {
                    RendererAssetFetchOwner::World { owner, .. } | RendererAssetFetchOwner::Shared(owner) => owner,
                }) {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Pending;
                    }
                };
                let Ok(mut runtime) = self.try_lock() else {
                    return RendererAssetDecodeTurnKind::Pending;
                };
                let Some(interaction) = runtime.interaction.as_mut() else {
                    return RendererAssetDecodeTurnKind::Pending;
                };
                let Some(state) = interaction.shell.world3d_states.get_token_mut(surface_token) else {
                    probe.begin_close();
                    return RendererAssetDecodeTurnKind::Pending;
                };
                if world3d_asset_cancellation_requested(state, probe.owner().owner().token()) {
                    probe.begin_close();
                    return RendererAssetDecodeTurnKind::Cancelled;
                }
                apply_world3d_terrain_tile_bytes(state, z, x, y, &bytes);
                probe.begin_close();
                return RendererAssetDecodeTurnKind::Published;
            }
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            if matches!(probe.owner().kind(), WorldAssetRequestKind::ReferenceImage) {
                let url = probe.owner().url().to_string();
                let token = probe.owner().owner().token();
                let staged = self.take_staged_reference_image(token, &url);
                if let Some(lease) = staged {
                    let Ok(mut runtime) = self.try_lock() else {
                        self.restore_staged_reference_image(token, url, lease);
                        return RendererAssetDecodeTurnKind::Pending;
                    };
                    let Some(interaction) = runtime.interaction.as_mut() else {
                        self.restore_staged_reference_image(token, url, lease);
                        return RendererAssetDecodeTurnKind::Pending;
                    };
                    let Some(state) = interaction.shell.world3d_states.get_token_mut(surface_token) else {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Pending;
                    };
                    if world3d_asset_cancellation_requested(state, probe.owner().owner().token()) {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Cancelled;
                    }
                    let _ = apply_decoded_reference_raster(state, &url, lease);
                    probe.begin_close();
                    return RendererAssetDecodeTurnKind::Published;
                }
                #[cfg(target_arch = "wasm32")]
                {
                    let Ok(mut runtime) = self.try_lock() else { return RendererAssetDecodeTurnKind::Pending };
                    let Some(interaction) = runtime.interaction.as_mut() else { return RendererAssetDecodeTurnKind::Pending };
                    let Some(state) = interaction.shell.world3d_states.get_token_mut(surface_token) else {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Pending;
                    };
                    if world3d_asset_cancellation_requested(state, probe.owner().owner().token()) {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Cancelled;
                    }
                    mark_world3d_asset_miss(state, &url);
                    probe.begin_close();
                    return RendererAssetDecodeTurnKind::Published;
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    return self.pump_native_reference_decode(probe, surface_token, token, url);
                }
            }
            let Some(asset) = probe.take_ready_mesh_asset() else { return RendererAssetDecodeTurnKind::Pending };
            let Ok(mut runtime) = self.try_lock() else {
                probe.restore_ready_mesh_asset(asset);
                return RendererAssetDecodeTurnKind::Pending;
            };
            let Some(interaction) = runtime.interaction.as_mut() else {
                probe.restore_ready_mesh_asset(asset);
                return RendererAssetDecodeTurnKind::Pending;
            };
            let Some(state) = interaction.shell.world3d_states.get_token_mut(surface_token) else {
                probe.restore_ready_mesh_asset(asset);
                probe.begin_close();
                return RendererAssetDecodeTurnKind::Pending;
            };
            if world3d_asset_cancellation_requested(state, probe.owner().owner().token()) {
                probe.restore_ready_mesh_asset(asset);
                probe.begin_close();
                return RendererAssetDecodeTurnKind::Cancelled;
            }
            match publish_world3d_asset_mesh(state, probe.owner().url(), asset) {
                Ok(()) => {
                    log_debug_diagnostic(&format!("[TRACE] asset mesh published url={}", probe.owner().url()));
                    probe.finish_ready_mesh();
                    return RendererAssetDecodeTurnKind::Published;
                }
                Err(rejected) => {
                    let stale = rejected.fault == WorldDynamicFault::StaleToken;
                    probe.restore_ready_mesh_asset(rejected.value);
                    if stale {
                        probe.begin_close();
                        drop(runtime);
                        drop(probe_slot);
                        self.record_frame_fault("asset mesh publication generation/revision witness was stale");
                        return RendererAssetDecodeTurnKind::Fault;
                    }
                    return RendererAssetDecodeTurnKind::Pending;
                }
            }
        }
        if matches!(probe.phase, RendererAssetProbePhase::Closing) {
            if !probe.close_step() {
                return RendererAssetDecodeTurnKind::Pending;
            }
            let Some(owner) = probe.take_terminal_owner() else { return RendererAssetDecodeTurnKind::Pending };
            match self.finish_renderer_asset_owner(owner, probe.rejection.clone()) {
                Ok(()) => {
                    *probe_slot = None;
                    return RendererAssetDecodeTurnKind::Cancelled;
                }
                Err(owner) => {
                    probe.owner = Some(owner);
                    return RendererAssetDecodeTurnKind::Pending;
                }
            }
        }
        RendererAssetDecodeTurnKind::Pending
    }

    fn close_renderer_asset_probe_step(&self) -> bool {
        let Ok(mut probe_slot) = self.0.asset_probe.try_lock() else { return false };
        let Some(probe) = probe_slot.as_mut() else { return true };
        if !probe.close_step() {
            return false;
        }
        let Some(owner) = probe.take_terminal_owner() else { return false };
        match self.finish_renderer_asset_owner(owner, probe.rejection.clone()) {
            Ok(()) => *probe_slot = None,
            Err(owner) => probe.owner = Some(owner),
        }
        false
    }

    pub(crate) fn return_renderer_asset_owner(&self, fetch: RendererAssetFetchOwner) -> Result<(), RendererAssetFetchOwner> {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => return_renderer_asset(owner).map_err(RendererAssetFetchOwner::Shared),
            RendererAssetFetchOwner::World { surface, surface_token, owner } => {
                let Ok(mut runtime) = self.try_lock() else { return Err(RendererAssetFetchOwner::World { surface, surface_token, owner }) };
                let Some(interaction) = runtime.interaction.as_mut() else { return Err(RendererAssetFetchOwner::World { surface, surface_token, owner }) };
                let Some(state) = interaction.shell.world3d_states.get_token_mut(surface_token) else { return Err(RendererAssetFetchOwner::World { surface, surface_token, owner }) };
                return_world3d_asset(state, owner).map_err(|owner| RendererAssetFetchOwner::World { surface, surface_token, owner })
            }
        }
    }

    /// 📡️ Seals one fetched response against the authority that admitted it.
    ///
    /// ⚖️ Three of the four ways this can decline are BACK-PRESSURE, not refusals: the runtime lock is
    /// taken, the interaction state is checked out by a live apply, or the surface has not been
    /// re-attached yet. They all used to collapse into one `false`, which the worker raised as
    /// `"asset response could not release its unused byte credits"` — a hard `asset-stream-fault`
    /// that quarantined the whole shell for a lock that was free a millisecond later. Measured on
    /// the puzzle3d playground: the second World3d surface's copy of the concrete-forest GLB sealed
    /// 40 ms into its fetch, straight into an apply that owned `AppInteractionState`, and the black
    /// canvas that followed was read as a mesh-lane bug (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
    /// `📓️w3d-world3d-glb-url-lane.md`). Only [`RendererAssetSealStep::Refused`] is a verdict about
    /// the RESPONSE; `Busy` asks the caller to come back, which is what the terrain, mesh and
    /// reference-image reservation loops already do one layer up.
    pub(crate) fn seal_renderer_asset_response(&self, fetch: &mut RendererAssetFetchOwner) -> RendererAssetSealStep {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => {
                if seal_renderer_asset_response(owner) {
                    RendererAssetSealStep::Granted
                } else {
                    RendererAssetSealStep::Refused("shared asset authority refused the seal witness")
                }
            }
            RendererAssetFetchOwner::World { surface: _, surface_token, owner } => {
                let Ok(mut runtime) = self.0.runtime.try_lock() else { return RendererAssetSealStep::Busy };
                let Some(interaction) = runtime.interaction.as_mut() else { return RendererAssetSealStep::Busy };
                let Some(state) = interaction.shell.world3d_states.get_token_mut(*surface_token) else { return RendererAssetSealStep::Refused("the world surface generation retired") };
                match seal_world3d_asset_response(state, owner) {
                    Ok(()) => RendererAssetSealStep::Granted,
                    Err(_) => RendererAssetSealStep::Refused("the world asset authority refused the seal witness"),
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn close_native_reference_decode_step(&self, token: Option<WorldAssetRequestToken>) -> bool {
        let Ok(mut slot) = self.0.native_reference_decode.try_lock() else { return false };
        let Some(job) = slot.as_ref() else { return true };
        if token.is_some_and(|token| token != job.token) {
            return true;
        }
        if !matches!(job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), NativeReferenceDecodeCloseStep::Complete) {
            return false;
        }
        *slot = None;
        true
    }

    /// 🧹️ Keeps one closing component's asset authority reachable until its exact borrowers return.
    pub(crate) fn close_component_asset_step(&self, target: &crate::element_contracts::ScenePointerTarget) -> bool {
        if !matches!(target.kind, ui_wgpu::wgpu::SurfaceKind::World3d | ui_wgpu::wgpu::SurfaceKind::IconRender) {
            return true;
        }
        let surface_token = {
            let Ok(mut runtime) = self.try_lock() else { return false };
            let Some(interaction) = runtime.interaction.as_mut() else { return false };
            let Some(token) = interaction.shell.world3d_states.token(&target.host_id) else { return true };
            let Some(state) = interaction.shell.world3d_states.get_token_mut(token) else { return false };
            infinite_world::world::begin_world3d_asset_close(state);
            token
        };
        #[cfg(not(target_arch = "wasm32"))]
        if !matches!(self.cancel_native_asset_transport_step(Some(surface_token), None), NativeAssetTransportCloseStep::Terminal) {
            return false;
        }
        {
            let Ok(session) = self.0.asset_decode_session.try_lock() else { return false };
            if let Some(session) = session.as_ref().filter(|session| session.surface_token == Some(surface_token)) {
                session.cancel();
                return false;
            }
        }
        let Ok(mut probe_slot) = self.0.asset_probe.try_lock() else { return false };
        if let Some(probe) = probe_slot.as_mut().filter(|probe| matches!(probe.owner(), RendererAssetFetchOwner::World { surface_token: token, .. } if *token == surface_token)) {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if !self.close_native_reference_decode_step(Some(probe.owner().owner().token())) {
                    return false;
                }
            }
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            self.discard_staged_reference_image(probe.owner().owner().token());
            if !probe.close_step() {
                return false;
            }
            let Some(owner) = probe.take_terminal_owner() else { return false };
            match self.finish_renderer_asset_owner(owner, probe.rejection.clone()) {
                Ok(()) => *probe_slot = None,
                Err(owner) => {
                    probe.owner = Some(owner);
                    return false;
                }
            }
            return false;
        }
        drop(probe_slot);
        let Ok(mut runtime) = self.try_lock() else { return false };
        let Some(interaction) = runtime.interaction.as_mut() else { return false };
        let Some(state) = interaction.shell.world3d_states.get_token_mut(surface_token) else { return false };
        infinite_world::world::close_world3d_asset_step(state)
    }

    pub(crate) fn close_renderer_asset_step(&self) -> bool {
        if self.pump_asset_decode_session_step(true).is_some() {
            return false;
        }
        #[cfg(not(target_arch = "wasm32"))]
        self.0.native_asset_http_cancel.cancel_now();
        #[cfg(not(target_arch = "wasm32"))]
        if !matches!(self.cancel_native_asset_transport_step(None, None), NativeAssetTransportCloseStep::Terminal) {
            return false;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if !self.close_native_reference_decode_step(None) {
            return false;
        }
        if !self.close_renderer_asset_probe_step() {
            return false;
        }
        if self.0.native_asset_fetching.load(Ordering::Acquire) {
            let _ = close_renderer_asset_step();
            return false;
        }
        if self.pump_native_asset_handoff_step(true).is_some() {
            return false;
        }
        close_renderer_asset_step()
    }

    pub(crate) fn renderer_asset_current(&self, fetch: &RendererAssetFetchOwner) -> Option<bool> {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => renderer_asset_io().try_lock().ok().map(|authority| !authority.cancellation_requested(owner.token())),
            RendererAssetFetchOwner::World { surface: _, surface_token, owner } => {
                let Ok(runtime) = self.try_lock() else { return None };
                let interaction = runtime.interaction.as_ref()?;
                Some(interaction.shell.world3d_states.get_token(*surface_token).is_some_and(|state| !world3d_asset_cancellation_requested(state, owner.token())))
            }
        }
    }

    pub(crate) fn renderer_asset_cancelled(&self, fetch: &RendererAssetFetchOwner) -> bool {
        self.renderer_asset_current(fetch) == Some(false)
    }

    pub(crate) fn browser_renderer_asset_current(&self, fetch: &RendererAssetFetchOwner) -> bool {
        self.renderer_asset_current(fetch) != Some(false)
    }

    pub(crate) fn reject_renderer_asset(&self, fetch: &RendererAssetFetchOwner) -> bool {
        match fetch {
            RendererAssetFetchOwner::World { surface: _, surface_token, owner } => {
                let Ok(mut runtime) = self.try_lock() else { return false };
                let Some(interaction) = runtime.interaction.as_mut() else { return false };
                let Some(state) = interaction.shell.world3d_states.get_token_mut(*surface_token) else { return true };
                if !world3d_asset_cancellation_requested(state, owner.token()) {
                    mark_world3d_asset_miss(state, owner.url());
                }
            }
            RendererAssetFetchOwner::Shared(owner) => {
                let Ok(authority) = renderer_asset_io().try_lock() else { return false };
                let cancelled = authority.cancellation_requested(owner.token());
                drop(authority);
                if !cancelled {
                    if matches!(owner.kind(), WorldAssetRequestKind::Glb) {
                        scenes::icon_export::asset::reject(owner.token(), "asset transport was unavailable");
                    }
                    Self::record_rejected_asset(&("asset transport was unavailable", owner.kind(), owner.url().to_string()));
                }
            }
        }
        true
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn restore_staged_reference_image(&self, token: WorldAssetRequestToken, url: String, lease: SceneRasterLease) {
        let mut staged = self.0.staged_reference_images.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        staged.restore(token, url, lease);
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub(crate) fn begin_renderer_reference_image(&self, fetch: &RendererAssetFetchOwner, width: u32, height: u32, digest: &str) -> Result<u8, &'static str> {
        if !matches!(fetch.kind(), WorldAssetRequestKind::ReferenceImage) || !self.browser_renderer_asset_current(fetch) {
            return Err("reference image request retired");
        }
        if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err("reference image source digest is invalid");
        }
        let source_digest = [u64::from_str_radix(&digest[..16], 16).map_err(|_| "reference image source digest is invalid")?, u64::from_str_radix(&digest[16..32], 16).map_err(|_| "reference image source digest is invalid")?];
        let descriptor = SceneRasterDescriptor { width, height, source_digest, source_revision: 1, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
        if descriptor.byte_len().is_none_or(|bytes| bytes > ui_wgpu::wgpu::SCENE_RASTER_ITEM_BYTES) {
            return Err("reference image exceeded fixed RGBA credits");
        }
        let owner = fetch.generation().wrapping_add(fetch.revision().rotate_left(17)).max(1);
        let mut staged = self.0.staged_reference_images.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        staged.begin(&self.0.scene_raster_pool, fetch.owner().token(), fetch.url().to_string(), descriptor, owner)
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub(crate) fn push_renderer_reference_image_rows(&self, fetch: &RendererAssetFetchOwner, offset: usize, pixels: &[u8]) -> Result<bool, &'static str> {
        if !matches!(fetch.kind(), WorldAssetRequestKind::ReferenceImage) || !self.browser_renderer_asset_current(fetch) {
            return Err("reference image request retired");
        }
        let mut staged = self.0.staged_reference_images.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        staged.push(&self.0.scene_raster_pool, fetch.owner().token(), offset, pixels)
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub(crate) fn seal_renderer_reference_image(&self, fetch: &RendererAssetFetchOwner) -> Result<bool, &'static str> {
        if !matches!(fetch.kind(), WorldAssetRequestKind::ReferenceImage) || !self.browser_renderer_asset_current(fetch) {
            return Err("reference image request retired");
        }
        let mut staged = self.0.staged_reference_images.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        staged.seal(&self.0.scene_raster_pool, fetch.owner().token())
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub(crate) fn discard_staged_reference_image(&self, token: WorldAssetRequestToken) {
        let mut staged = self.0.staged_reference_images.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        staged.discard(&self.0.scene_raster_pool, token);
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn take_staged_reference_image(&self, token: WorldAssetRequestToken, url: &str) -> Option<SceneRasterLease> {
        let mut staged = self.0.staged_reference_images.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        staged.take(token, url)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn pump_native_reference_decode(&self, probe: &mut RendererAssetProbe, surface_token: AdmittedSurfaceToken, token: WorldAssetRequestToken, url: String) -> RendererAssetDecodeTurnKind {
        let job = {
            let mut slot = self.0.native_reference_decode.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            if let Some(job) = slot.as_ref() {
                if job.token != token || job.url != url {
                    self.record_frame_fault("native reference decode owner changed before terminal handback");
                    return RendererAssetDecodeTurnKind::Pending;
                }
                job.clone()
            } else {
                let bytes = match collect_world3d_asset_bytes(match probe.owner_mut() {
                    RendererAssetFetchOwner::World { owner, .. } | RendererAssetFetchOwner::Shared(owner) => owner,
                }) {
                    Ok(bytes) => bytes,
                    Err(_) => {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Pending;
                    }
                };
                let job = Arc::new(NativeReferenceDecodeJob::new(token, url.clone(), bytes));
                *slot = Some(job.clone());
                job
            }
        };
        if self.renderer_asset_cancelled(probe.owner()) {
            job.cancel();
        }
        match job.phase.load(Ordering::Acquire) {
            0 => {
                let scheduled = job.try_schedule(self.clone());
                if !scheduled {
                    if let Some(waker) = self.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                        waker();
                    }
                }
                RendererAssetDecodeTurnKind::Pending
            }
            1 => RendererAssetDecodeTurnKind::Pending,
            2 => {
                let Some(output) = job.take_output() else { return RendererAssetDecodeTurnKind::Pending };
                let output = match output {
                    NativeReferenceDecodeOutput::Waiting(decoded) => {
                        let _ = self.0.scene_raster_pool.maintenance_step();
                        if job.rearm(decoded) {
                            if let Some(waker) = self.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                                waker();
                            }
                            return RendererAssetDecodeTurnKind::Pending;
                        }
                        if self.close_native_reference_decode_step(Some(token)) {
                            probe.begin_close();
                            return RendererAssetDecodeTurnKind::Cancelled;
                        }
                        return RendererAssetDecodeTurnKind::Pending;
                    }
                    output => output,
                };
                let Ok(mut runtime) = self.try_lock() else {
                    job.restore_output(output);
                    return RendererAssetDecodeTurnKind::Pending;
                };
                let Some(interaction) = runtime.interaction.as_mut() else {
                    job.restore_output(output);
                    return RendererAssetDecodeTurnKind::Pending;
                };
                let Some(state) = interaction.shell.world3d_states.get_token_mut(surface_token) else {
                    drop(runtime);
                    job.restore_output(output);
                    job.cancel();
                    if self.close_native_reference_decode_step(Some(token)) {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Cancelled;
                    }
                    return RendererAssetDecodeTurnKind::Pending;
                };
                let cancelled = job.cancelled.load(Ordering::Acquire);
                let stale = !world3d_reference_url_is_current(state, &url);
                if cancelled || stale {
                    if stale && !cancelled {
                        mark_world3d_asset_miss(state, &url);
                    }
                    drop(runtime);
                    job.restore_output(output);
                    job.cancel();
                    if self.close_native_reference_decode_step(Some(token)) {
                        probe.begin_close();
                        return RendererAssetDecodeTurnKind::Cancelled;
                    }
                    return RendererAssetDecodeTurnKind::Pending;
                }
                match output {
                    NativeReferenceDecodeOutput::Ready(lease) => {
                        let _ = apply_decoded_reference_raster(state, &url, lease);
                    }
                    NativeReferenceDecodeOutput::Failed => mark_world3d_asset_miss(state, &url),
                    NativeReferenceDecodeOutput::Waiting(_) => unreachable!("waiting decode was rearmed above"),
                }
                drop(runtime);
                job.phase.store(4, Ordering::Release);
                let mut slot = self.0.native_reference_decode.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                if !slot.as_ref().is_some_and(|candidate| Arc::ptr_eq(candidate, &job)) {
                    drop(slot);
                    self.record_frame_fault("native reference decode owner changed before publication handback");
                    return RendererAssetDecodeTurnKind::Fault;
                }
                *slot = None;
                probe.begin_close();
                RendererAssetDecodeTurnKind::Published
            }
            3 | 4 => {
                if self.close_native_reference_decode_step(Some(token)) {
                    probe.begin_close();
                    RendererAssetDecodeTurnKind::Cancelled
                } else {
                    RendererAssetDecodeTurnKind::Pending
                }
            }
            5 => RendererAssetDecodeTurnKind::Pending,
            6 => {
                if matches!(job.retire_payload_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), NativeReferenceDecodeCloseStep::Complete) {
                    let next = if job.cancelled.load(Ordering::Acquire) { 3 } else { 2 };
                    let _ = job.phase.compare_exchange(6, next, Ordering::AcqRel, Ordering::Acquire);
                }
                if let Some(waker) = self.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                    waker();
                }
                RendererAssetDecodeTurnKind::Pending
            }
            _ => {
                self.record_frame_fault("native reference decode entered an invalid phase");
                RendererAssetDecodeTurnKind::Fault
            }
        }
    }

    pub(crate) fn pump_native_asset_handoff_step(&self, closing: bool) -> Option<bool> {
        let Ok(mut slot) = self.0.native_asset_handoff.try_lock() else { return Some(false) };
        let handoff = slot.as_mut()?;
        if closing {
            handoff.fetch.as_mut().expect("native handoff owns its request").begin_close();
            handoff.seal = false;
        }
        if let Some(payload) = handoff.payload.as_mut() {
            let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if payload.terminal_is_empty() {
                handoff.payload = None;
            }
            return Some(true);
        }
        if handoff.reject {
            let fetch = handoff.fetch.as_mut().expect("native handoff owns its rejected request");
            if !self.reject_renderer_asset(fetch) {
                return Some(false);
            }
            fetch.begin_close();
            handoff.reject = false;
            return Some(true);
        }
        if handoff.seal {
            let fetch = handoff.fetch.as_mut().expect("native handoff owns its request");
            match self.renderer_asset_current(fetch) {
                None => return Some(false),
                Some(false) => {
                    fetch.begin_close();
                    handoff.seal = false;
                }
                Some(true) => match self.seal_renderer_asset_response(fetch) {
                    RendererAssetSealStep::Busy => return Some(false),
                    RendererAssetSealStep::Granted => handoff.seal = false,
                    RendererAssetSealStep::Refused(_) => {
                        fetch.begin_close();
                        handoff.seal = false;
                    }
                },
            }
            return Some(true);
        }
        let fetch = handoff.fetch.take().expect("native handoff owns its request");
        match self.return_renderer_asset_owner(fetch) {
            Ok(()) => {
                *slot = None;
                Some(true)
            }
            Err(fetch) => {
                handoff.fetch = Some(fetch);
                Some(false)
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn retain_native_asset_handoff(&self, fetch: RendererAssetFetchOwner, payload: Option<semio_framework_job::RetainedJobPayload>, seal: bool, reject: bool) {
        let mut slot = self.0.native_asset_handoff.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        assert!(slot.is_none(), "one native asset fetch is admitted at a time");
        *slot = Some(NativeAssetHandoff { fetch: Some(fetch), payload, seal, reject });
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn retain_native_asset_transport(&self, fetch: &RendererAssetFetchOwner, cancel: semio_framework_async::CancelToken, cancellation: semio_framework_os_services::HttpBodyCancellationHandle) -> NativeAssetTransportGuard {
        let surface_token = match fetch {
            RendererAssetFetchOwner::World { surface: _, surface_token, owner: _ } => Some(*surface_token),
            RendererAssetFetchOwner::Shared(_) => None,
        };
        let request_token = fetch.owner().token();
        let mut slot = self.0.native_asset_transport.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        assert!(slot.is_none(), "one native asset transport is admitted at a time");
        *slot = Some(NativeAssetTransportLease { surface_token, request_token, cancel, cancellation, cancellation_step: None });
        NativeAssetTransportGuard { mailbox: self.clone(), surface_token, request_token, armed: true }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn cancel_native_asset_transport_step(&self, surface_token: Option<AdmittedSurfaceToken>, request_token: Option<WorldAssetRequestToken>) -> NativeAssetTransportCloseStep {
        let cancellation = {
            let Ok(mut slot) = self.0.native_asset_transport.try_lock() else { return NativeAssetTransportCloseStep::Busy };
            let Some(lease) = slot.as_mut() else { return NativeAssetTransportCloseStep::Terminal };
            if surface_token.is_some_and(|expected| lease.surface_token != Some(expected)) || request_token.is_some_and(|expected| lease.request_token != expected) {
                return NativeAssetTransportCloseStep::Terminal;
            }
            lease.cancel.cancel_now();
            if let Some(step) = lease.cancellation_step.as_ref() {
                return NativeAssetTransportCloseStep::Pending(step.clone());
            }
            lease.cancellation_step = Some(semio_framework_os_services::HttpBodyCancellationStep::Idle);
            lease.cancellation.clone()
        };
        let step = cancellation.cancel_in_flight();
        let mut slot = self.0.native_asset_transport.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(lease) = slot.as_mut().filter(|lease| !surface_token.is_some_and(|expected| lease.surface_token != Some(expected)) && !request_token.is_some_and(|expected| lease.request_token != expected)) {
            lease.cancellation_step = Some(step.clone());
        }
        NativeAssetTransportCloseStep::Pending(step)
    }

    #[cfg(all(test, not(target_arch = "wasm32")))]
    pub(crate) fn native_asset_transport_live_count(&self) -> usize {
        usize::from(self.0.native_asset_transport.lock().expect("native asset transport slot").is_some())
    }

    #[cfg(all(test, not(target_arch = "wasm32")))]
    pub(crate) fn wait_native_asset_page_test_barrier(&self, fetch: &RendererAssetFetchOwner) {
        let RendererAssetFetchOwner::World { surface_token, .. } = fetch else { return };
        let barrier = self.0.native_asset_page_barrier.lock().expect("native asset page test barrier").as_ref().filter(|barrier| barrier.surface_token == *surface_token).cloned();
        let Some(barrier) = barrier else { return };
        barrier.entered.store(true, Ordering::Release);
        while !barrier.release.load(Ordering::Acquire) {
            std::thread::yield_now();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn pump_native_asset(&self) -> bool {
        if let Some(progressed) = self.pump_native_asset_handoff_step(false) {
            if progressed {
                if let Some(waker) = self.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                    waker();
                }
            }
            return progressed;
        }
        if self.0.native_asset_fetching.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        let Some(mut fetch) = self.take_renderer_asset_step() else {
            self.0.native_asset_fetching.store(false, Ordering::Release);
            return false;
        };
        match self.reserve_renderer_asset_response(&mut fetch, WORLD_ASSET_RESPONSE_BYTE_CAPACITY) {
            RendererAssetSealStep::Granted => {}
            admission => {
                if matches!(admission, RendererAssetSealStep::Refused(_)) {
                    fetch.begin_close();
                }
                self.retain_native_asset_handoff(fetch, None, false, false);
                self.0.native_asset_fetching.store(false, Ordering::Release);
                if let Some(waker) = self.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                    waker();
                }
                return true;
            }
        }
        let mailbox = self.clone();
        spawn_app_task(async move {
            let (payload, seal, reject) = match stream_native_renderer_asset(&mailbox, &mut fetch).await {
                Ok(()) => (None, true, false),
                Err(NativeAssetStreamFailure::Cancelled) => {
                    fetch.begin_close();
                    (None, false, false)
                }
                Err(NativeAssetStreamFailure::CancelledPage { payload }) => {
                    fetch.begin_close();
                    (Some(payload), false, false)
                }
                Err(NativeAssetStreamFailure::Unavailable(detail)) => {
                    log_debug(&format!("native renderer asset unavailable: {detail}"));
                    (None, false, true)
                }
                Err(NativeAssetStreamFailure::Fault(detail)) => {
                    fetch.begin_close();
                    log_debug(&format!("native renderer asset fetch failed: {detail}"));
                    mailbox.record_frame_fault("native renderer asset fetch failed");
                    (None, false, false)
                }
                Err(NativeAssetStreamFailure::Page { detail, payload }) => {
                    fetch.begin_close();
                    mailbox.record_frame_fault(detail);
                    (Some(payload), false, false)
                }
            };
            mailbox.retain_native_asset_handoff(fetch, payload, seal, reject);
            mailbox.0.native_asset_fetching.store(false, Ordering::Release);
            if let Some(waker) = mailbox.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                waker();
            }
        });
        true
    }

    pub(crate) fn set_waker(&self, waker: RuntimeHostWaker) {
        *self.0.waker.lock().expect("runtime completion waker lock") = Some(waker);
    }

    pub(crate) fn enqueue_apply(&self, key: Option<&'static str>, requires_interaction: bool, apply: RuntimeApply) -> bool {
        self.0.enqueue(key, requires_interaction, apply)
    }

    pub(crate) fn enqueue_dispatch_on_original_caller(&self,events:&mut ui_host::EventQueue,generation:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let mut queue=self.0.completions.lock().expect("runtime completion mailbox lock");if queue.len()>=RUNTIME_COMPLETION_CAPACITY-1||events.is_empty(){return Ok(false)}let Some(original)=RuntimeDispatchSource::admit(cx)?else{return Ok(false)};
        let input_generation=events.current_generation();let drained=events.drain_page(ui_host::WorkerContext::new(input_generation));let cursor=RuntimeDispatchCursor::new_for_generation(drained,generation,original);let completion=self.0.completion(None,true,RuntimeApply::DispatchEvents(Some(cursor)));assert!(queue.enqueue(completion),"same locked queue admits the original dispatch owner");drop(queue);self.0.presentation_authority.mark_scene_changed();if let Some(waker)=self.0.waker.lock().expect("runtime completion waker lock").as_ref(){waker()}Ok(true)
    }

    fn has_lossless_capacity(&self) -> bool {
        self.0.completions.lock().expect("runtime completion mailbox lock").len() < RUNTIME_COMPLETION_CAPACITY - 1
    }

    /// 📮️ Whether the mailbox still owes the runtime an apply — the per-frame predicate that turns a
    /// queued `DispatchEvents`/`Resize` into the next `request_frame` on an event-driven shell.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn has_pending_applies(&self) -> bool {
        !self.0.completions.lock().expect("runtime completion mailbox lock").ready.is_empty()
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn has_pending_text_work(&self) -> bool {
        self.try_lock().ok().and_then(|runtime| runtime.interaction.as_ref().map(AppInteractionState::has_pending_text_work)).unwrap_or(false)
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
    pub(crate) fn has_pending_world3d_work(&self) -> bool {
        self.try_lock()
            .ok()
            .and_then(|runtime| {
                runtime.interaction.as_ref().map(|interaction| interaction.shell.world3d_states.values().any(|state| infinite_world::world::world3d_cursor_work_pending(state) || infinite_world::world::world3d_brush_mesh_announce_pending(state)))
            })
            .unwrap_or(false)
    }

    /// 📡️ Reports only work owned by the private decoder continuation.
    fn has_pending_asset_decode(&self) -> bool {
        if self.0.asset_decode_session.try_lock().map_or(true, |owner| owner.is_some()) {
            return true;
        }
        if self.0.asset_probe.try_lock().map_or(true, |owner| owner.is_some()) {
            return true;
        }
        if renderer_asset_io().try_lock().map_or(true, |authority| authority.has_completed_step()) {
            return true;
        }
        self.try_lock().map_or(true, |runtime| runtime.interaction.as_ref().is_some_and(|interaction| interaction.shell.world3d_states.values().any(infinite_world::world::world3d_asset_decode_pending)))
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
    pub(crate) fn has_pending_settle(&self) -> bool {
        self.try_lock().ok().and_then(|runtime| runtime.interaction.as_ref().map(|interaction| interaction.shell.settle_pump_pending())).unwrap_or(false)
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn take_text_fault(&self) -> Option<String> {
        self.try_lock().ok()?.interaction.as_mut()?.text_fault.take()
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn take_frame_fault(&self) -> Option<String> {
        if let Some(fault) = self.0.frame_fault.lock().expect("runtime frame fault lock").take() {
            return Some(fault);
        }
        self.try_lock().ok()?.interaction.as_mut()?.frame_fault.take()
    }

    pub(crate) fn record_frame_fault(&self, fault: &'static str) {
        log_debug(&format!("[TRACE] frame fault recorded: {fault}"));
        let mut slot = self.0.frame_fault.lock().expect("runtime frame fault lock");
        if slot.is_none() {
            *slot = Some(fault.to_string());
        }
    }

    pub(crate) fn frame_inputs(&self, now_ms: f64) -> frame_job::FrameBuildInputs {
        let mut inputs = self.0.frame_inputs.try_lock().map(|inputs| inputs.clone()).unwrap_or_default();
        inputs.now_ms = now_ms;
        inputs
    }

    pub(crate) fn publish_retained_control_deadline(&self, deadline: Option<f64>) {
        let due_us = deadline.filter(|seconds| seconds.is_finite() && *seconds >= 0.0).map_or(u64::MAX, |seconds| (seconds * 1_000_000.0).ceil() as u64);
        self.0.retained_control_deadline_us.store(due_us, Ordering::Release);
    }

    pub(crate) fn publish_shell_clock_deadline(&self, deadline: Option<f64>) {
        let due_us = deadline.filter(|seconds| seconds.is_finite() && *seconds >= 0.0).map_or(u64::MAX, |seconds| (seconds * 1_000_000.0).ceil() as u64);
        self.0.shell_clock_deadline_us.store(due_us, Ordering::Release);
    }

    pub(crate) fn control_deadline_us(&self) -> Option<u64> {
        let due_us = self.0.retained_control_deadline_us.load(Ordering::Acquire).min(self.0.shell_clock_deadline_us.load(Ordering::Acquire));
        (due_us != u64::MAX).then_some(due_us)
    }

    pub(crate) fn retained_control_deadline(&self, host_seconds: f64) -> Option<ui_render::Deadline> {
        crate::deadlines::retained_control_deadline(self.control_deadline_us(), semio_framework_job::default_now_us(), host_seconds)
    }

    pub(crate) fn update_frame_inputs(&self, runtime: &AppRuntime) {
        if !runtime.interaction_available() {
            return;
        }
        *self.0.frame_inputs.lock().expect("runtime frame inputs lock") = frame_job::FrameBuildInputs { wheel_zoom_deadline_ms: runtime.wheel_zoom_deadline_ms, now_ms: app_now_ms() };
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn reserve_future(&self, key: Option<&'static str>) -> bool {
        self.0.completions.lock().expect("runtime completion mailbox lock").reserve(key)
    }

    pub(crate) fn reserve_interaction_future(&self) -> bool {
        self.0.completions.lock().expect("runtime completion mailbox lock").reserve_interaction()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn cancel_interaction_future_reservation(&self) -> bool {
        match self.0.completions.try_lock() {
            Ok(mut completions) => completions.cancel_interaction_reservation(),
            Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner().cancel_interaction_reservation(),
            Err(std::sync::TryLockError::WouldBlock) => false,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn try_take_abandoned_frame_maintenance(&self) -> Option<FrameMaintenanceRefusal> {
        let (generation, owner) = self.0.frame_maintenance_executions.try_take_abandoned()?;
        let _ = self.0.frame_maintenance.release(generation);
        Some(FrameMaintenanceRefusal { owner, reservation_live: false })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn try_take_abandoned_frame_deferred(&self) -> Option<FrameDeferredExecutionOwner> {
        let (_, owner) = self.0.frame_deferred_executions.try_take_abandoned()?;
        owner.try_take()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn spawn_interaction_reserved<F>(&self, _key: Option<&'static str>, future: F)
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
    pub(crate) fn spawn_dispatch_reserved(&self,cursor:RuntimeDispatchCursor,interaction:AppInteractionState){
        let mailbox=self.clone();let revision=mailbox.0.next_revision.fetch_add(1,Ordering::Relaxed);let owner=RuntimeDispatchReturn{mailbox,revision,cursor:Some(cursor),interaction:Some(interaction)};spawn_app_task(owner.run());
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn try_spawn_frame_deferred_reserved(&self, owner: FrameDeferredExecutionOwner) -> Result<(), FrameDeferredExecutionOwner> {
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
                    interaction.shell.pump_sync_events().await;
                }
                FrameDeferredWork::Action(action) => {
                    if action.cancelled {
                        action.cancel();
                    } else {
                        let receipt = action.receipt;
                        let mut receipt_owner = FrameActionReceiptOwner { receipt };
                        let result = {
                            let Some(interaction) = execution.owner_mut().and_then(|owner| owner.interaction.as_mut()) else { return };
                            interaction.shell.dispatch_gesture_action(action.descriptor).await
                        };
                        match result {
                            Ok(()) => receipt_owner.settle(engine_canvas::TextEditorActionOutcome::Accepted),
                            Err(error) => {
                                receipt_owner.settle(engine_canvas::TextEditorActionOutcome::Refused(&error));
                                let Some(owner) = execution.owner_mut() else { return };
                                if let Some(receipt) = receipt {
                                    if receipt.abort_correlation_on_error {
                                        owner.cursor.as_mut().map(|cursor| cursor.actions.cancel_correlation(receipt.token));
                                    }
                                }
                                if let Some(interaction) = owner.interaction.as_mut() {
                                    log_debug(&format!("[TRACE] frame deferred action failed: {error}"));
                                    interaction.shell.note_dispatch_fault(&error);
                                }
                            }
                        }
                    }
                }
                FrameDeferredWork::FlushTutorial => {
                    let Some(interaction) = execution.owner_mut().and_then(|owner| owner.interaction.as_mut()) else { return };
                    interaction.shell.tutorial_flush_pending_document_ops().await;
                }
                FrameDeferredWork::Settle => {
                    let Some(interaction) = execution.owner_mut().and_then(|owner| owner.interaction.as_mut()) else { return };
                    interaction.shell.settle_pump_step().await;
                }
            }
            if !execution.finish() {
                mailbox.record_frame_fault("frame deferred execution handback faulted");
            }
        });
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn try_spawn_frame_maintenance_reserved(&self, owner: FrameMaintenanceOwner) -> Result<(), FrameMaintenanceRefusal> {
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
                let FrameMaintenanceOwner { interaction, cursor, deadline_ms } = owner;
                let (Some(interaction), Some(cursor)) = (interaction.as_mut(), cursor.as_mut()) else {
                    mailbox.record_frame_fault("frame maintenance exact owner pair was incomplete");
                    return;
                };
                let cancelled = cursor.cancel.is_cancelled_now();
                stale = !mailbox.0.frame_maintenance.is_live(generation) || mailbox.0.presentation_authority.witness_for(generation).is_none();
                let deadline_exceeded = deadline_ms.is_none_or(|deadline| worker_pool.now_ms() > deadline);
                if let Some(fault) = frame_maintenance_terminal_fault(cancelled, stale, deadline_exceeded) {
                    cursor.begin_close();
                    interaction.frame_fault = Some(fault.to_string());
                } else {
                    cursor.shell_maintenance = interaction.shell.advance_chrome_maintenance_step();
                    if cursor.shell_maintenance {
                        cursor.phase = 0;
                    }
                    if deadline_ms.is_some_and(|deadline| worker_pool.now_ms() > deadline) {
                        interaction.frame_fault = Some("frame maintenance bounded field exceeded its deadline".to_string());
                        cursor.begin_close();
                    }
                }
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
    pub(crate) fn spawn_frame_maintenance_reserved<F>(&self, future: F)
    where
        F: Future<Output = (AppInteractionState, FrameDeferredCursor)> + 'static,
    {
        self.spawn_frame_deferred_reserved(future);
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn spawn_frame_deferred_reserved<F>(&self, future: F)
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
    pub(crate) fn spawn_dispatch_reserved(&self,cursor:RuntimeDispatchCursor,interaction:AppInteractionState){
        let mailbox=self.clone();let revision=mailbox.0.next_revision.fetch_add(1,Ordering::Relaxed);let owner=RuntimeDispatchReturn{mailbox,revision,cursor:Some(cursor),interaction:Some(interaction)};spawn_app_task(owner.run());
    }

}
//#endregion 📮️RuntimeMailbox
