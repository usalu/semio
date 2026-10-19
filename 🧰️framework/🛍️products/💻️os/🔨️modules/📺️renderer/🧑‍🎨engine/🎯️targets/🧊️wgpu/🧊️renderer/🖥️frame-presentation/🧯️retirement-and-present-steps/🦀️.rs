//! 🧯️ Retirement and present steps: the presented retirement, the surface resize steps, the presentation gate, the close steps and the admit and present steps. See [`super`].

use super::*;

//#region 📮️RuntimeMailbox
impl AppPresentedRetirement {
    pub(super) fn new(previous: Option<semio_framework_ui_prepared::PreparedRenderPacket>) -> Self {
        Self {
            previous,
            completed_frame: None,
            raster: RasterCandidateRetirement::Infer,
            acknowledged_eviction: 0,
            acknowledged_upload_scan: 0,
            acknowledged_versions: [0; ui_gpu::MESH_GPU_KEEP_VERSION_CAPACITY],
            acknowledged_version_count: 0,
            release_previous_raster_ownership: false,
            unowned_rasters_retired: false,
        }
    }

    fn abort(previous: semio_framework_ui_prepared::PreparedRenderPacket, witness: ui_gpu::RasterTextureWitness) -> Self {
        let raster = RasterCandidateRetirement::Abort(witness);
        Self {
            previous: Some(previous),
            completed_frame: None,
            raster,
            acknowledged_eviction: 0,
            acknowledged_upload_scan: 0,
            acknowledged_versions: [0; ui_gpu::MESH_GPU_KEEP_VERSION_CAPACITY],
            acknowledged_version_count: 0,
            release_previous_raster_ownership: false,
            unowned_rasters_retired: false,
        }
    }

    pub(super) fn commit(previous: Option<semio_framework_ui_prepared::PreparedRenderPacket>, witness: ui_gpu::RasterTextureWitness) -> Self {
        Self {
            previous,
            completed_frame: None,
            raster: RasterCandidateRetirement::Commit(witness),
            acknowledged_eviction: 0,
            acknowledged_upload_scan: 0,
            acknowledged_versions: [0; ui_gpu::MESH_GPU_KEEP_VERSION_CAPACITY],
            acknowledged_version_count: 0,
            release_previous_raster_ownership: true,
            unowned_rasters_retired: false,
        }
    }

    pub(super) fn step(&mut self, gpu: &mut GpuContext, gate: &semio_framework_ui_prepared::PreparedRenderGate) -> Result<bool, String> {
        if self.raster == RasterCandidateRetirement::Infer {
            self.raster = RasterCandidateRetirement::Complete;
            return Ok(false);
        }
        match self.raster {
            RasterCandidateRetirement::Abort(witness) => {
                if !gpu.abort_presented_rasters_step(witness)? {
                    return Ok(false);
                }
                self.raster = RasterCandidateRetirement::Complete;
                return Ok(false);
            }
            RasterCandidateRetirement::Commit(witness) => {
                if !gpu.commit_presented_rasters_step(witness)? {
                    return Ok(false);
                }
                self.raster = RasterCandidateRetirement::Complete;
                return Ok(false);
            }
            RasterCandidateRetirement::Infer | RasterCandidateRetirement::Complete => {}
        }
        if !gpu.close_mesh_upload_step() {
            return Ok(false);
        }
        if let Some(packet) = gate.last_valid() {
            if self.acknowledged_eviction < packet.evictions().len() {
                let Some(eviction) = packet.evictions().get(self.acknowledged_eviction) else { return Err("prepared eviction cursor exceeded its fixed list".to_string()) };
                let key = match eviction {
                    semio_framework_ui_prepared::PreparedRenderEviction::Mesh { key } => key,
                };
                if let Some(upload) = packet.uploads().get(self.acknowledged_upload_scan) {
                    if let semio_framework_ui_prepared::PreparedRenderUpload::Mesh { key: upload_key, version, .. } = upload {
                        if upload_key == key {
                            let Some(slot) = self.acknowledged_versions.get_mut(self.acknowledged_version_count) else {
                                return Err("prepared mesh eviction keep-version credits exhausted".to_string());
                            };
                            *slot = *version;
                            self.acknowledged_version_count += 1;
                        }
                    }
                    self.acknowledged_upload_scan += 1;
                    return Ok(false);
                }
                if gpu.apply_prepared_eviction_step(packet, self.acknowledged_eviction, &self.acknowledged_versions[..self.acknowledged_version_count])? {
                    self.acknowledged_eviction += 1;
                    self.acknowledged_upload_scan = 0;
                    self.acknowledged_version_count = 0;
                }
                return Ok(false);
            }
        }
        if let Some(previous) = self.previous.as_mut() {
            if !previous.retire_step() {
                return Ok(false);
            }
            self.previous = None;
            return Ok(false);
        }
        if self.release_previous_raster_ownership {
            gpu.release_previous_raster_ownership();
            self.release_previous_raster_ownership = false;
            return Ok(false);
        }
        if !self.unowned_rasters_retired {
            if !gpu.retire_unowned_raster_step()? {
                return Ok(false);
            }
            self.unowned_rasters_retired = true;
            return Ok(false);
        }
        if let Some(frame) = self.completed_frame.as_mut() {
            if !matches!(frame.close_step(frame.retained), InteractiveJobCloseStep::Complete { .. }) {
                return Ok(false);
            }
            self.completed_frame = None;
            return Ok(false);
        }
        Ok(true)
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.raster == RasterCandidateRetirement::Complete && self.previous.is_none() && !self.release_previous_raster_ownership && self.unowned_rasters_retired && self.completed_frame.as_ref().is_none_or(AppFramePresentation::terminal_is_empty)
    }
}

impl AppPresenter {
    pub fn surface_resize_available(&self) -> bool {
        self.surface_resize.is_none()
    }

    pub fn begin_surface_resize(&mut self, candidate: surface_lane::PreparedSurfaceResize) -> Result<(), surface_lane::PreparedSurfaceResize> {
        if self.surface_resize.is_some() {
            return Err(candidate);
        }
        self.surface_resize = Some(AppSurfaceResizeCursor { candidate: Some(candidate), phase: AppSurfaceResizePhase::Apply });
        Ok(())
    }

    pub fn surface_resize_step(&mut self) -> bool {
        let Some(cursor) = self.surface_resize.as_mut() else { return true };
        match cursor.phase {
            AppSurfaceResizePhase::Apply => {
                let Some(candidate) = cursor.candidate else {
                    cursor.phase = AppSurfaceResizePhase::Complete;
                    return false;
                };
                if !self.engine.observe_primary_metrics_generation(candidate.metrics_generation()) {
                    return false;
                }
                cursor.phase = AppSurfaceResizePhase::InvalidateEngine;
                false
            }
            AppSurfaceResizePhase::InvalidateEngine => {
                if !self.engine.invalidate_primary_metrics_step() {
                    return false;
                }
                cursor.phase = AppSurfaceResizePhase::ApplyGpu;
                false
            }
            AppSurfaceResizePhase::ApplyGpu => {
                let Some(candidate) = cursor.candidate else {
                    cursor.phase = AppSurfaceResizePhase::Complete;
                    return false;
                };
                if !candidate.suspended() {
                    self.gpu.resize(candidate.logical_width(), candidate.logical_height(), candidate.scale_factor());
                }
                cursor.phase = AppSurfaceResizePhase::Retire;
                false
            }
            AppSurfaceResizePhase::Retire => {
                cursor.candidate.take();
                cursor.phase = AppSurfaceResizePhase::Complete;
                false
            }
            AppSurfaceResizePhase::Complete => {
                self.surface_resize = None;
                true
            }
        }
    }

    pub fn close_surface_resize_step(&mut self) -> bool {
        if let Some(cursor) = self.surface_resize.as_mut() {
            if cursor.candidate.take().is_some() {
                cursor.phase = AppSurfaceResizePhase::Retire;
                return false;
            }
            cursor.phase = AppSurfaceResizePhase::Complete;
            self.surface_resize = None;
            return false;
        }
        self.engine.invalidate_primary_metrics_step()
    }

    pub fn has_pending_presentation(&self) -> bool {
        self.pending.is_some() || self.retirement.is_some() || self.retained_fault.is_some() || self.gate.has_pending_acknowledgement()
    }

    pub fn awaiting_runtime(&self) -> bool {
        self.pending.as_ref().is_some_and(|cursor| cursor.input_wait.awaiting_runtime(cursor.phase))
    }

    pub fn holds_presented_input_publication(&self) -> bool {
        self.pending.as_ref().is_some_and(|cursor| matches!(cursor.phase, AppPresentPhase::Render | AppPresentPhase::CloseGpu | AppPresentPhase::Acknowledge | AppPresentPhase::Media))
    }

    /// 🩺️ Which of the four owners is holding the presentation gate shut — the gate that decides
    /// whether `admit_next_frame` runs a frame build at all, and therefore whether the runtime mailbox
    /// is pumped this tick.
    pub fn presentation_gate_shape(&self) -> String {
        format!(
            "pending={} phase={:?} retirement={} retained-fault={:?} gate-ack={} stall-steps={}",
            self.pending.is_some(),
            self.pending.as_ref().map(|cursor| cursor.phase),
            self.retirement.is_some(),
            self.retained_fault.as_deref(),
            self.gate.has_pending_acknowledgement(),
            self.stall.steps
        )
    }

    /// 🐕️ One watchdog tick over the pending presentation: `Some(shape)` exactly once, on the step the
    /// cursor has answered [`APP_PRESENT_STALL_STEPS`] consecutive non-advancing `Pending`s. See
    /// [`AppPresentStallWatch`].
    pub(super) fn note_present_stall(watch: &mut AppPresentStallWatch, cursor: &AppPresentCursor, upload_progress: (u32, u32, usize)) -> Option<String> {
        note_present_stall_signature(
            watch,
            (cursor.phase, cursor.engine, cursor.upload, cursor.gpu_cursor.as_ref().map(ui_gpu::PreparedGpuPresentCursor::progress), upload_progress, cursor.raster_keep_steps, cursor.input_wait, cursor.input_progress),
        )
    }

    pub fn close_cursor_wake_step(&mut self) -> bool {
        let Some(pending) = self.pending.as_mut() else { return true };
        if pending.frame.cursor_wake.take().is_some() {
            return false;
        }
        true
    }

    pub fn close_original_media_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let Some(cursor)=self.pending.as_mut()else{return Ok(true)};
        if !cursor.media_collect.terminal_is_empty(){if !semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::close_presented_media_slots(&mut cursor.media_collect,cx)?{return Ok(false)}}
        if let Some(owner)=cursor.media_close.as_mut(){if owner.terminal_is_empty(){let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}cursor.media_close=None;cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}let demand=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;let _=semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok(false)}
        if !cursor.media_slots.is_empty(){let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}let original=std::mem::take(&mut cursor.media_slots);match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>cursor.media_close=Some(owner),Err((original,source))=>{cursor.media_slots=source;return Err(original.into())}}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}Ok(true)
    }

    pub fn close_frame_owners_step(&mut self) -> Result<bool, String> {
        if !self.engine.close_active_candidate_step(&mut self.gpu)? {
            return Ok(false);
        }
        if let Some(mut cursor) = self.pending.take() {
            if let Some(input_candidate) = cursor.frame.input_candidate {
                let runtime = self.runtime.clone();
                let Ok(mut runtime) = runtime.try_lock() else {
                    self.pending = Some(cursor);
                    return Ok(false);
                };
                let Some(interaction) = runtime.interaction.as_mut() else {
                    self.pending = Some(cursor);
                    return Ok(false);
                };
                if !interaction.shell.input.discard_presented_input_candidate(input_candidate) {
                    self.pending = Some(cursor);
                    return Err("presenter close lost its staged input witness".to_string());
                }
                cursor.frame.input_candidate = None;
                self.pending = Some(cursor);
                return Ok(false);
            }
            if let Some(gpu_cursor) = cursor.gpu_cursor.as_mut() {
                gpu_cursor.begin_close();
                if !gpu_cursor.close_step() {
                    self.pending = Some(cursor);
                    return Ok(false);
                }
                cursor.gpu_cursor = None;
                self.pending = Some(cursor);
                return Ok(false);
            }
            if cursor.frame.packet.is_none() {
                cursor.frame.packet = self.gate.abort_pending();
            }
            let retirement = self.retirement.get_or_insert_with(|| AppPresentedRetirement::new(None));
            if retirement.completed_frame.is_some() {
                self.pending = Some(cursor);
                return Err("presenter close retirement capacity exhausted".to_string());
            }
            if let Some(witness) = cursor.raster_witness.take() {
                if let Err(error) = self.raster_operation_authority.release(witness) {
                    cursor.raster_witness = Some(witness);
                    self.pending = Some(cursor);
                    return Err(error.to_string());
                }
                retirement.raster = RasterCandidateRetirement::Abort(witness);
            }
            retirement.completed_frame = Some(cursor.frame);
            return Ok(false);
        }
        if let Some(packet) = self.gate.abort_pending() {
            if self.retirement.is_some() {
                return Err("presenter pending packet cannot overtake retirement".to_string());
            }
            let witness = self.raster_operation_authority.current().ok_or_else(|| "presenter raster operation witness was missing during close".to_string())?;
            self.raster_operation_authority.release(witness).map_err(str::to_owned)?;
            self.retirement = Some(AppPresentedRetirement::abort(packet, witness));
            return Ok(false);
        }
        if let Some(retirement) = self.retirement.as_mut() {
            if !retirement.step(&mut self.gpu, &self.gate)? {
                return Ok(false);
            }
            self.retirement = None;
            return Ok(false);
        }
        if let Some(fault) = self.retained_fault.as_mut() {
            if fault.pop().is_some() {
                return Ok(false);
            }
            self.retained_fault = None;
            return Ok(false);
        }
        if let Some(packet) = self.gate.take_last_valid() {
            self.retirement = Some(AppPresentedRetirement::new(Some(packet)));
            return Ok(false);
        }
        if !self.gate.close_step() {
            return Ok(false);
        }
        Ok(self.pending.is_none() && self.retirement.as_ref().is_none_or(AppPresentedRetirement::terminal_is_empty) && self.retained_fault.is_none() && self.gate.terminal_is_empty() && self.raster_operation_authority.current().is_none())
    }

    pub fn close_world_owners_step(&mut self) -> Result<bool, String> {
        if !self.close_frame_owners_step()? || !self.engine.terminal_is_empty() {
            return Ok(false);
        }
        if !self.gpu.close_mesh_upload_step() {
            return Ok(false);
        }
        if !self.gpu.close_mesh_table_step() {
            return Ok(false);
        }
        if !self.gpu.close_raster_table_step()? {
            return Ok(false);
        }
        Ok(self.world_owners_terminal_is_empty())
    }

    pub fn world_owners_terminal_is_empty(&self) -> bool {
        self.pending.is_none()
            && self.retirement.as_ref().is_none_or(AppPresentedRetirement::terminal_is_empty)
            && self.gate.terminal_is_empty()
            && self.gpu.mesh_upload_terminal_is_empty()
            && self.gpu.mesh_table_terminal_is_empty()
            && self.gpu.raster_table_terminal_is_empty()
            && self.raster_operation_authority.current().is_none()
            && self.engine.terminal_is_empty()
            && self.retained_fault.is_none()
    }

    pub fn admit_next_frame(&mut self, produce: impl FnOnce() -> Option<AppFramePresentation>) -> Option<SemioCursor> {
        if self.has_pending_presentation() {
            return None;
        }
        let frame = produce()?;
        let cursor = frame.cursor;
        self.pending = Some(AppPresentCursor {
            media_collect:Default::default(),
            media_close:None,
            media_slots: Vec::new(),
            has_animated_primitives: false,
            frame,
            phase: AppPresentPhase::BeginGpu,
            engine: 0,
            upload: 0,
            witness: None,
            raster_witness: None,
            raster_keep: semio_framework_ui_prepared::PreparedRasterKeepCursorV1::default(),
            raster_keep_steps: 0,
            input_wait: AppPresentInputWait::None,
            input_progress: 0,
            gpu_cursor: None,
            retained_control_deadline: None,
            shell_clock_deadline: None,
        });
        Some(cursor)
    }

    /// 🎬️ Advances the pending presentation by exactly one phase.
    ///
    /// **Freshness is decided ONCE, by the BUILD.** `AppPresentPhase::BeginGpu` reads
    /// `presentation_authority.admitted()` — the pair the frame build declared when it minted its
    /// `FrameBuildCursor`, never the live `current()` — and hands it to
    /// `begin_prepared`/`begin_prepared_offscreen`, whose `PreparedRenderGate::validate` refuses a
    /// packet whose `scene_revision`/`preview_generation` does not match that admitted pair; the same
    /// pair is then frozen into the cursor's `RasterTextureWitness`. Every later phase therefore
    /// compares against THAT witness and never against the authority again: it is a moving target — `OsHost`
    /// re-publishes `observe_presentation_input_generation(frame_generation)` on every
    /// `build_and_publish_snapshot`, and `🌐️browser-worker`'s `enqueueBatch` additionally rebases
    /// `frame_generation` onto the UI isolate's input generation — so a presentation that legitimately
    /// spans more than one host tick (the browser's own interactive-ceiling loop guarantees it will)
    /// would observe a different authority at acknowledgement than the one that admitted it and fault a
    /// frame that is ALREADY submitted to the GPU. That is what took the wgpu browser surface down on
    /// the first pointer event (`worker-present-failed: prepared frame authority was stale before
    /// acknowledgement`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END): a superseded frame must be
    /// acknowledged and replaced by the next one, never turned into a surface fault.
    pub fn present_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<AppPresentStep, semio_framework_job::OriginalTurnFailure> {
        if cx.should_yield(){return Ok(AppPresentStep::Pending)}
        if let Some(cursor)=self.pending.as_mut().filter(|cursor|cursor.phase==AppPresentPhase::Media){
            let Ok(runtime)=self.runtime.try_lock()else{return Ok(AppPresentStep::RetryRuntime)};let Some(interaction)=runtime.interaction.as_ref()else{return Ok(AppPresentStep::AwaitingRuntime)};if !interaction.shell.input.drive_presented_media_slots(&mut cursor.media_collect,&mut cursor.media_slots,cx)?{return Ok(AppPresentStep::Pending)}cursor.phase=AppPresentPhase::ProgressAcknowledge;return Ok(AppPresentStep::Pending)
        }
        let owner = self.pending.as_ref().map(|cursor| (cursor.frame.generation.0, cursor.phase.latency_stage())).or_else(|| {
            self.has_pending_presentation().then(|| {
                let generation = self.retirement.as_ref().and_then(|retirement| retirement.completed_frame.as_ref()).map(|frame| frame.generation.0).unwrap_or_else(frame_latency::latest_frame_generation);
                (generation, frame_latency::FrameLatencyStage::PresenterRetirement)
            })
        });
        let _latency = owner.map(|(generation, stage)| frame_latency::FrameLatencyTimer::start(frame_latency::FrameLatencyAuthority::renderer_frame(generation), stage, 1));
        self.present_step_inner(cx).map_err(|source|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,source).into())
    }

}
//#endregion 📮️RuntimeMailbox
