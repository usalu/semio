//! 🎞️ Present state and engine closers: the presenter state, the stall watch, the input wait, the resize and present phases and cursors, the presented retirement types and the engine surface and raster close delegates. See [`super`].

use super::*;

//#region 📮️RuntimeMailbox
pub struct AppPresenter {
    pub gpu: GpuContext,
    pub engine: semio_framework_os_renderer_wgpu_engine_registry::prepared_engine_canvas::EngineCanvasPresenter,
    pub gate: semio_framework_ui_prepared::PreparedRenderGate,
    pub runtime: AppMailbox,
    pub presentation_authority: RuntimePresentationAuthority,
    pub raster_operation_authority: RuntimeRasterOperationAuthority,
    pub window: Option<Arc<Window>>,
    #[cfg(target_arch = "wasm32")]
    pub offscreen_token: Option<semio_framework_ui_prepared::OffscreenPresentToken>,
    pub last_cursor: Option<(SemioCursor, bool)>,
    pub pending: Option<AppPresentCursor>,
    pub retirement: Option<AppPresentedRetirement>,
    pub retained_fault: Option<String>,
    pub surface_resize: Option<AppSurfaceResizeCursor>,
    pub stall: AppPresentStallWatch,
}

/// 🐕️ The presentation ladder's own watchdog: the last progress signature [`AppPresenter::present_step`]
/// answered `Pending` on, and how many consecutive steps have answered it unchanged.
///
/// ⚖️ `admit_next_frame` refuses to build a frame while `has_pending_presentation()`, so a cursor that
/// stops advancing stops the whole host — no frame transaction, no runtime mailbox pump, no world
/// snapshot apply, no retained document ingress, no accessibility projection. Measured on 6118 as
/// `os_host frame gate blocked=true pending=true phase=Some(Engine)` held from t≈85 s to t≈200 s, and
/// as `phase=Some(Aborted) retained-fault=true` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-wheel-zoom-a11y-live-2026-09-14.md` §3.4, `📓️wgpu-host-settle-pump-2026-09-14.md` §7).
///
/// The signature is every index a HEALTHY step moves — the phase, the engine packet, the upload page,
/// and whether a GPU cursor is held — so a legitimately long presentation (many upload pages, many
/// engine packets) resets the count on every one of them and only a genuinely frozen cursor
/// accumulates.
#[derive(Default)]
pub struct AppPresentStallWatch {
    signature: Option<AppPresentProgress>,
    pub steps: u32,
}

/// 🐕️ Every index a healthy presentation step moves: the ladder phase, the engine packet, the upload
/// page, and — because `AppPresentPhase::Render` holds ONE `gpu_cursor` for a whole composite pass —
/// that cursor's own `(phase, command, clip piece, blur mip)`. Without the inner four the outer
/// shape is frozen for as many steps as the scene has commands, and a watchdog reading the outer
/// shape alone aborts a healthy present: measured on 6118 as
/// `os_host present stalled phase=Render engine=1 upload=1 gpu-cursor=true` at t≈4.4 s on EVERY
/// example boot (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AppPresentInputWait {
    #[default]
    None,
    RuntimeLock,
    InteractionCheckout {
        site: &'static str,
        request: &'static str,
    },
    SceneIntent,
}

impl AppPresentInputWait {
    pub(super) fn awaiting_runtime(self, phase: AppPresentPhase) -> bool {
        matches!(phase, AppPresentPhase::Render | AppPresentPhase::Acknowledge) && matches!(self, Self::InteractionCheckout { .. })
    }
}

pub type AppPresentProgress = (AppPresentPhase, usize, usize, Option<(u8, usize, usize, u32)>, (u32, u32, usize), usize, AppPresentInputWait, u64);

/// 🐕️ Consecutive non-advancing `Pending` answers after which a pending presentation is aborted
/// rather than waited on. `present_step` is driven several times per frame transaction turn, so this
/// is well under a second of wall clock and three orders of magnitude above the longest healthy
/// non-advancing run this ladder has (`Uploads`/`Render` answering `Ok(false)` while the GPU queue
/// drains).
pub const APP_PRESENT_STALL_STEPS: u32 = 4_096;

/// 🐕️ The watchdog's whole arithmetic, over the progress signature alone: `Some(shape)` on exactly the
/// step a signature has repeated [`APP_PRESENT_STALL_STEPS`] times, `None` on every other step, and a
/// reset the moment any term of the signature moves.
pub fn note_present_stall_signature(watch: &mut AppPresentStallWatch, signature: AppPresentProgress) -> Option<String> {
    if signature.6.awaiting_runtime(signature.0) {
        watch.signature = None;
        watch.steps = 0;
        return None;
    }
    if watch.signature != Some(signature) {
        watch.signature = Some(signature);
        watch.steps = 0;
        return None;
    }
    watch.steps = watch.steps.saturating_add(1);
    if watch.steps != APP_PRESENT_STALL_STEPS {
        return None;
    }
    let mut shape = format!("phase={:?} engine={} upload={} gpu-cursor={:?} upload-progress={:?}", signature.0, signature.1, signature.2, signature.3, signature.4);
    if signature.6 != AppPresentInputWait::None {
        shape.push_str(&format!(" input-wait={:?}", signature.6));
    }
    Some(shape)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AppSurfaceResizePhase {
    Apply,
    InvalidateEngine,
    ApplyGpu,
    Retire,
    Complete,
}

pub(super) struct AppSurfaceResizeCursor {
    pub(super) candidate: Option<surface_lane::PreparedSurfaceResize>,
    pub(super) phase: AppSurfaceResizePhase,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppPresentPhase {
    Fullscreen,
    Engine,
    BeginGpu,
    Ownership,
    Uploads,
    Stage,
    Render,
    CloseGpu,
    Acknowledge,
    Media,
    ProgressAcknowledge,
    Aborted,
    Directives,
}

impl AppPresentPhase {
    pub fn latency_stage(self) -> frame_latency::FrameLatencyStage {
        match self {
            Self::Fullscreen => frame_latency::FrameLatencyStage::PresenterFullscreen,
            Self::Engine => frame_latency::FrameLatencyStage::PresenterEngine,
            Self::BeginGpu => frame_latency::FrameLatencyStage::PresenterBeginGpu,
            Self::Ownership => frame_latency::FrameLatencyStage::PresenterOwnership,
            Self::Uploads => frame_latency::FrameLatencyStage::PresenterUploads,
            Self::Stage => frame_latency::FrameLatencyStage::PresenterStage,
            Self::Render => frame_latency::FrameLatencyStage::QueueSubmit,
            Self::CloseGpu => frame_latency::FrameLatencyStage::PresenterCloseGpu,
            Self::Acknowledge => frame_latency::FrameLatencyStage::PresenterAcknowledge,
            Self::Media => frame_latency::FrameLatencyStage::PresenterAcknowledge,
            Self::ProgressAcknowledge => frame_latency::FrameLatencyStage::PresenterProgressAcknowledge,
            Self::Aborted => frame_latency::FrameLatencyStage::PresenterAborted,
            Self::Directives => frame_latency::FrameLatencyStage::PresenterDirectives,
        }
    }
}

pub(super) struct AppPresentCursor {
    pub(super) media_collect:semio_framework_os_renderer_wgpu_media_slots::original_collect::OriginalPresentedRoster,
    pub(super) media_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Vec<semio_framework_os_renderer_wgpu_media_slots::PresentedMediaSlot>>>,
    pub(super) media_slots: Vec<semio_framework_os_renderer_wgpu_media_slots::PresentedMediaSlot>,
    pub(super) has_animated_primitives: bool,
    pub(super) frame: AppFramePresentation,
    pub(super) phase: AppPresentPhase,
    pub(super) engine: usize,
    pub(super) upload: usize,
    pub(super) witness: Option<semio_framework_ui_prepared::PreparedPresenterWitness>,
    pub(super) raster_witness: Option<ui_gpu::RasterTextureWitness>,
    pub(super) raster_keep: semio_framework_ui_prepared::PreparedRasterKeepCursorV1,
    pub(super) raster_keep_steps: usize,
    pub(super) input_wait: AppPresentInputWait,
    pub(super) input_progress: u64,
    pub(super) gpu_cursor: Option<ui_gpu::PreparedGpuPresentCursor>,
    pub(super) retained_control_deadline: Option<f64>,
    pub(super) shell_clock_deadline: Option<f64>,
}

pub(super) struct AppPresentedRetirement {
    pub(super) previous: Option<semio_framework_ui_prepared::PreparedRenderPacket>,
    pub(super) completed_frame: Option<AppFramePresentation>,
    pub(super) raster: RasterCandidateRetirement,
    pub(super) acknowledged_eviction: usize,
    pub(super) acknowledged_upload_scan: usize,
    pub(super) acknowledged_versions: [u64; ui_gpu::MESH_GPU_KEEP_VERSION_CAPACITY],
    pub(super) acknowledged_version_count: usize,
    pub(super) release_previous_raster_ownership: bool,
    pub(super) unowned_rasters_retired: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RasterCandidateRetirement {
    Infer,
    Abort(ui_gpu::RasterTextureWitness),
    Commit(ui_gpu::RasterTextureWitness),
    Complete,
}

pub enum AppPresentStep {
    Idle,
    Pending,
    AwaitingRuntime,
    RetryRuntime,
    Complete {
        generation: semio_framework_trace::Generation,
        cursor: SemioCursor,
        theme_dark: bool,
        fullscreen: Option<bool>,
        cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>,
        retained_control_deadline: Option<f64>,
        shell_clock_deadline: Option<f64>,
        has_animated_primitives: bool,
        media_slots: Vec<semio_framework_os_renderer_wgpu_media_slots::PresentedMediaSlot>,
    },
}

impl AppPresenter {
    pub fn engine_surface_token_at(&self, index: usize) -> Option<semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::EngineSurfaceToken> {
        self.engine.surface_token_at(index)
    }

    pub fn begin_engine_surface_close(&mut self, token: semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::EngineSurfaceToken) -> bool {
        self.engine.begin_surface_close(token)
    }

    pub fn close_engine_surface_step(&mut self, token: semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::EngineSurfaceToken) -> Result<bool, String> {
        self.engine.close_surface_step(&mut self.gpu, token)
    }

    pub fn engine_surface_terminal_is_empty(&self, token: semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::EngineSurfaceToken) -> bool {
        self.engine.surface_terminal_is_empty(token)
    }

    pub fn begin_engine_raster_close(&mut self, host_id: &str) -> Result<bool, String> {
        let key = semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::engine_raster_key(host_id).ok_or_else(|| "engine raster host identifier exceeded fixed credits".to_string())?;
        self.gpu.begin_exact_raster_close(&key)
    }

    pub fn close_engine_raster_step(&mut self, host_id: &str) -> Result<bool, String> {
        let key = semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::engine_raster_key(host_id).ok_or_else(|| "engine raster host identifier exceeded fixed credits".to_string())?;
        self.gpu.close_exact_raster_step(&key)
    }

    pub fn engine_raster_terminal_is_empty(&self, host_id: &str) -> bool {
        semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::engine_raster_key(host_id).is_some_and(|key| self.gpu.exact_raster_terminal_is_empty(&key))
    }

    pub fn engine_surfaces_terminal_is_empty(&self) -> bool {
        self.engine.terminal_is_empty()
    }
}
//#endregion 📮️RuntimeMailbox

//#region 📮️RuntimeMailbox
impl AppPresenter {
    /// 🌍️ Plan section 5.3, inversion I3: the world3d dynamic-resource close step needs the presenter's GPU and the mailbox's runtime, so it lives with the presenter and borrows the mailbox.
    pub fn close_world3d_dynamic_step(&mut self,mailbox:&AppMailbox,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure> {
        if cx.retirement_should_yield(){return Ok(false)}
        if !mailbox.close_renderer_asset_step(cx)? {
            return Ok(false);
        }
        let Ok(mut runtime) = mailbox.try_lock() else {
            return Ok(false);
        };
        let Some(interaction) = runtime.interaction_mut() else {
            drop(runtime);
            return Ok(mailbox.close_world_cursor_wake_step());
        };
        if !interaction.shell.opening.close_icon_export_step(cx)? {
            return Ok(false);
        }
        if interaction.shell.surfaces.advance_world3d_retirement_step() {
            if let Some(fault)=interaction.shell.surfaces.world3d_retirement_fault() {mailbox.record_frame_fault(&format!("retired terrain owner refused: {fault:?}"));}
            return Ok(false);
        }
        let Ok(mut cursor) = mailbox.0.world3d_close_cursor.try_lock() else {
            return Ok(false);
        };
        let Some(state) = interaction.shell.surfaces.world3d_states.get_mut_at(*cursor) else {
            drop(cursor);
            drop(runtime);
            return Ok(mailbox.close_world_cursor_wake_step());
        };
        if world3d_dynamic_retirement_terminal_is_empty(state) {
            *cursor += 1;
            return Ok(false);
        }
        if state.dynamic_retirement_is_idle() {
            begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets);
            return Ok(false);
        }
        let terrain_grant=cx.retained_grant();
        let terrain=infinite_world::world::step_world3d_terrain_retirement(&mut state.session, &mut state.terrain,terrain_grant,cx);
        if !terrain.ownership.fits(terrain_grant) {return Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"world3d terrain retirement receipt exceeded caller grant").with_retained_progress(terrain.ownership),cx));}
        cx.consume_retained(terrain.ownership)?;
        match terrain.step {
            infinite_world::world::WorldTerrainMeshPublicationStep::Idle=>{},
            infinite_world::world::WorldTerrainMeshPublicationStep::Pending|infinite_world::world::WorldTerrainMeshPublicationStep::Complete=>return Ok(false),
            infinite_world::world::WorldTerrainMeshPublicationStep::Fault(fault)=>{mailbox.record_frame_fault(&format!("world3d terrain retirement refused: {fault:?}"));return Ok(false);}
        }
        let gpu_return=infinite_world::world::world3d_return_native_mesh_gpu_step(&mut state.scene,cx.retained_grant(),|key,source|self.gpu.retire_original_mesh_source_step(key,source)).map_err(|_|semio_framework_job::OriginalTurnFailure::from(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original World GPU source return refused")))?;
        cx.consume_retained(gpu_return)?;if gpu_return.copied_items!=0{cx.consume_fuel(1);return Ok(false)}
        if step_world3d_dynamic_retirement(state, cx) && world3d_dynamic_retirement_terminal_is_empty(state) {
            *cursor += 1;
        }
        Ok(false)
    }
}
//#endregion 📮️RuntimeMailbox
